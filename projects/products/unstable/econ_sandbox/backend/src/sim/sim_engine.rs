use std::collections::BTreeMap;

use crate::agents::agent::Agent;
use crate::agents::agent_id::AgentId;
use crate::config::sim_config::SimConfig;
use crate::diagnostics::error::Error;
use crate::events::event_log::EventLog;
use crate::events::sim_event::{SimEvent, SimEventKind};
use crate::market::good::Good;
use crate::market::order::{Order, OrderSide};
use crate::market::order_book::OrderBook;
use crate::policy::policy_engine::PolicyEngine;
use crate::policy::policy_kind::PolicyKind;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_hash::RunHash;
use crate::report::sim_report::SimReport;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::time::tick::Tick;
use crate::time::tick_clock::TickClock;

pub struct SimEngine {
    agents: BTreeMap<AgentId, Agent>,
    clock: TickClock,
    event_log: EventLog,
    policies: Vec<PolicyKind>,
    prices: BTreeMap<Good, i64>,
}

impl SimEngine {
    pub fn run_sim(config: &SimConfig) -> Result<(SimReport, ReplayFile), Error> {
        let mut engine = Self::new(config);
        engine.run_all();

        let snapshot =
            StateSnapshot::capture(&engine.agents, &engine.prices, engine.clock.current());
        let snapshot_hash = SnapshotHash::compute(&snapshot);

        let event_count = engine.event_log.len();
        let run_hash = RunHash::compute(config.seed, config.ticks, event_count, &snapshot_hash);

        let report = SimReport {
            run_hash,
            seed: config.seed,
            ticks: config.ticks,
            event_count,
            snapshot_hash,
        };

        let replay = ReplayFile::new(
            config.seed,
            config.ticks,
            engine.event_log.events().to_vec(),
        );

        Ok((report, replay))
    }

    fn new(config: &SimConfig) -> Self {
        let agents = Self::create_default_agents(config.seed);
        let policies = Self::default_policies();
        let prices = Self::initial_prices();

        Self {
            agents,
            clock: TickClock::new(config.ticks),
            event_log: EventLog::new(),
            policies,
            prices,
        }
    }

    fn run_all(&mut self) {
        while !self.clock.is_done() {
            self.clock.advance();
            self.tick_once();
        }
    }

    fn tick_once(&mut self) {
        let tick = self.clock.current();
        let mut tick_events: Vec<SimEvent> = Vec::new();

        // Phase 1: Production
        let agent_ids: Vec<AgentId> = self.agents.keys().copied().collect();
        for &aid in &agent_ids {
            if let Some(agent) = self.agents.get_mut(&aid) {
                let production: Vec<(Good, u64)> =
                    agent.production.iter().map(|(&g, &a)| (g, a)).collect();
                agent.produce();
                for (good, amount) in production {
                    if amount > 0 {
                        tick_events.push(SimEvent {
                            tick,
                            kind: SimEventKind::Produced {
                                agent_id: aid,
                                good,
                                amount,
                            },
                        });
                    }
                }
            }
        }

        // Phase 2: Order submission + market clearing
        let orders = self.generate_orders(tick);
        let fills = OrderBook::clear(&orders);

        for fill in &fills {
            if let Some(buyer) = self.agents.get_mut(&fill.buyer) {
                buyer.cash -= fill.price * fill.quantity as i64;
                *buyer.inventory.entry(fill.good).or_insert(0) += fill.quantity;
            }
            if let Some(seller) = self.agents.get_mut(&fill.seller) {
                seller.cash += fill.price * fill.quantity as i64;
                let held = seller.inventory.entry(fill.good).or_insert(0);
                *held = held.saturating_sub(fill.quantity);
            }

            tick_events.push(SimEvent {
                tick,
                kind: SimEventKind::OrderMatched {
                    buyer: fill.buyer,
                    seller: fill.seller,
                    good: fill.good,
                    quantity: fill.quantity,
                    price: fill.price,
                },
            });

            // Update price based on last trade
            self.prices.insert(fill.good, fill.price);
        }

        // Phase 3: Consumption
        for &aid in &agent_ids {
            if let Some(agent) = self.agents.get_mut(&aid) {
                let consumption: Vec<(Good, u64)> =
                    agent.consumption.iter().map(|(&g, &a)| (g, a)).collect();
                // Capture how much was actually consumed (limited by inventory)
                let mut consumed_amounts: Vec<(Good, u64)> = Vec::new();
                for &(good, amount) in &consumption {
                    let held = agent.inventory.get(&good).copied().unwrap_or(0);
                    let actual = amount.min(held);
                    consumed_amounts.push((good, actual));
                }
                agent.consume();
                for (good, actual) in consumed_amounts {
                    if actual > 0 {
                        tick_events.push(SimEvent {
                            tick,
                            kind: SimEventKind::Consumed {
                                agent_id: aid,
                                good,
                                amount: actual,
                            },
                        });
                    }
                }
            }
        }

        // Phase 4: Policies
        PolicyEngine::apply(&self.policies, &mut self.agents, tick, &mut tick_events);

        // Record all events
        for event in tick_events {
            self.event_log.push(event);
        }
    }

    fn generate_orders(&self, _tick: Tick) -> Vec<Order> {
        let mut orders = Vec::new();

        for (&aid, agent) in &self.agents {
            for &good in Good::all() {
                let held = agent.inventory.get(&good).copied().unwrap_or(0);
                let consumes = agent.consumption.get(&good).copied().unwrap_or(0);
                let produces = agent.production.get(&good).copied().unwrap_or(0);
                let base_price = self.prices.get(&good).copied().unwrap_or(100);

                // Agents sell excess production
                if held > consumes && produces > 0 {
                    let sell_qty = held.saturating_sub(consumes);
                    if sell_qty > 0 {
                        orders.push(Order {
                            agent_id: aid,
                            good,
                            side: OrderSide::Sell,
                            price: base_price,
                            quantity: sell_qty,
                        });
                    }
                }

                // Agents buy what they need but don't produce
                if consumes > 0 && produces == 0 && held < consumes {
                    let buy_qty = consumes - held;
                    if buy_qty > 0 && agent.cash >= base_price {
                        orders.push(Order {
                            agent_id: aid,
                            good,
                            side: OrderSide::Buy,
                            price: base_price + 10,
                            quantity: buy_qty,
                        });
                    }
                }
            }
        }

        orders
    }

    fn create_default_agents(seed: u64) -> BTreeMap<AgentId, Agent> {
        let mut agents = BTreeMap::new();

        // Farmer: produces food, consumes tools
        let mut farmer = Agent::new(AgentId(seed % 1000), "Farmer".into(), 1000);
        farmer.production.insert(Good::Food, 5);
        farmer.consumption.insert(Good::Tools, 1);
        agents.insert(farmer.id, farmer);

        // Smith: produces tools, consumes food
        let mut smith = Agent::new(AgentId(seed % 1000 + 1), "Smith".into(), 1000);
        smith.production.insert(Good::Tools, 3);
        smith.consumption.insert(Good::Food, 2);
        agents.insert(smith.id, smith);

        // Merchant: produces luxuries, consumes food and tools
        let mut merchant = Agent::new(AgentId(seed % 1000 + 2), "Merchant".into(), 2000);
        merchant.production.insert(Good::Luxuries, 2);
        merchant.consumption.insert(Good::Food, 1);
        merchant.consumption.insert(Good::Tools, 1);
        agents.insert(merchant.id, merchant);

        agents
    }

    fn default_policies() -> Vec<PolicyKind> {
        vec![PolicyKind::FlatTax { rate_pct: 5 }]
    }

    fn initial_prices() -> BTreeMap<Good, i64> {
        let mut prices = BTreeMap::new();
        prices.insert(Good::Food, 100);
        prices.insert(Good::Tools, 150);
        prices.insert(Good::Luxuries, 200);
        prices
    }
}
