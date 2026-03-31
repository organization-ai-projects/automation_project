use crate::agents::agent_id::AgentId;
use crate::market::good::Good;
use crate::market::order::{Order, OrderSide};

#[derive(Debug, Clone)]
pub struct Fill {
    pub buyer: AgentId,
    pub seller: AgentId,
    pub good: Good,
    pub quantity: u64,
    pub price: i64,
}

pub struct OrderBook;

impl OrderBook {
    /// Deterministic order matching.
    ///
    /// Buys sorted descending by price, then ascending by agent_id.
    /// Sells sorted ascending by price, then ascending by agent_id.
    /// Matches highest-bid with lowest-ask when bid >= ask, at ask price.
    pub fn clear(orders: &[Order]) -> Vec<Fill> {
        let mut buys: Vec<&Order> = orders
            .iter()
            .filter(|o| o.side == OrderSide::Buy)
            .collect();
        let mut sells: Vec<&Order> = orders
            .iter()
            .filter(|o| o.side == OrderSide::Sell)
            .collect();

        // Deterministic sort: buys descending by price, then ascending by agent_id
        buys.sort_by(|a, b| {
            b.price
                .cmp(&a.price)
                .then_with(|| a.agent_id.0.cmp(&b.agent_id.0))
        });

        // Sells ascending by price, then ascending by agent_id
        sells.sort_by(|a, b| {
            a.price
                .cmp(&b.price)
                .then_with(|| a.agent_id.0.cmp(&b.agent_id.0))
        });

        let mut fills = Vec::new();
        let mut buy_remaining: Vec<u64> = buys.iter().map(|o| o.quantity).collect();
        let mut sell_remaining: Vec<u64> = sells.iter().map(|o| o.quantity).collect();

        let mut bi = 0;
        let mut si = 0;

        while bi < buys.len() && si < sells.len() {
            if buy_remaining[bi] == 0 {
                bi += 1;
                continue;
            }
            if sell_remaining[si] == 0 {
                si += 1;
                continue;
            }

            let buy = buys[bi];
            let sell = sells[si];

            if buy.price < sell.price {
                break;
            }

            let trade_qty = buy_remaining[bi].min(sell_remaining[si]);
            let trade_price = sell.price;

            fills.push(Fill {
                buyer: buy.agent_id,
                seller: sell.agent_id,
                good: buy.good,
                quantity: trade_qty,
                price: trade_price,
            });

            buy_remaining[bi] -= trade_qty;
            sell_remaining[si] -= trade_qty;
        }

        fills
    }
}
