use crate::agents::agent_id::AgentId;
use crate::market::good::Good;
use crate::market::order::{Order, OrderSide};
use crate::market::order_book::OrderBook;

#[test]
fn same_orders_produce_same_fills() {
    let orders = vec![
        Order {
            agent_id: AgentId(1),
            good: Good::Food,
            side: OrderSide::Buy,
            price: 120,
            quantity: 5,
        },
        Order {
            agent_id: AgentId(2),
            good: Good::Food,
            side: OrderSide::Sell,
            price: 100,
            quantity: 3,
        },
        Order {
            agent_id: AgentId(3),
            good: Good::Food,
            side: OrderSide::Sell,
            price: 110,
            quantity: 4,
        },
        Order {
            agent_id: AgentId(4),
            good: Good::Food,
            side: OrderSide::Buy,
            price: 115,
            quantity: 2,
        },
    ];

    let fills_a = OrderBook::clear(&orders);
    let fills_b = OrderBook::clear(&orders);

    assert_eq!(fills_a.len(), fills_b.len());
    for (a, b) in fills_a.iter().zip(fills_b.iter()) {
        assert_eq!(a.buyer, b.buyer);
        assert_eq!(a.seller, b.seller);
        assert_eq!(a.good, b.good);
        assert_eq!(a.quantity, b.quantity);
        assert_eq!(a.price, b.price);
    }
}

#[test]
fn highest_bid_matches_lowest_ask() {
    let orders = vec![
        Order {
            agent_id: AgentId(1),
            good: Good::Tools,
            side: OrderSide::Buy,
            price: 200,
            quantity: 1,
        },
        Order {
            agent_id: AgentId(2),
            good: Good::Tools,
            side: OrderSide::Buy,
            price: 150,
            quantity: 1,
        },
        Order {
            agent_id: AgentId(3),
            good: Good::Tools,
            side: OrderSide::Sell,
            price: 100,
            quantity: 1,
        },
        Order {
            agent_id: AgentId(4),
            good: Good::Tools,
            side: OrderSide::Sell,
            price: 180,
            quantity: 1,
        },
    ];

    let fills = OrderBook::clear(&orders);
    assert!(!fills.is_empty());

    // Highest bid (200 from agent 1) matches lowest ask (100 from agent 3)
    assert_eq!(fills[0].buyer, AgentId(1));
    assert_eq!(fills[0].seller, AgentId(3));
    assert_eq!(fills[0].price, 100); // trade at ask price
}

#[test]
fn no_match_when_bid_below_ask() {
    let orders = vec![
        Order {
            agent_id: AgentId(1),
            good: Good::Luxuries,
            side: OrderSide::Buy,
            price: 50,
            quantity: 1,
        },
        Order {
            agent_id: AgentId(2),
            good: Good::Luxuries,
            side: OrderSide::Sell,
            price: 100,
            quantity: 1,
        },
    ];

    let fills = OrderBook::clear(&orders);
    assert!(fills.is_empty());
}

#[test]
fn deterministic_ordering_by_agent_id() {
    let orders = vec![
        Order {
            agent_id: AgentId(5),
            good: Good::Food,
            side: OrderSide::Buy,
            price: 100,
            quantity: 1,
        },
        Order {
            agent_id: AgentId(3),
            good: Good::Food,
            side: OrderSide::Buy,
            price: 100,
            quantity: 1,
        },
        Order {
            agent_id: AgentId(1),
            good: Good::Food,
            side: OrderSide::Sell,
            price: 80,
            quantity: 2,
        },
    ];

    let fills = OrderBook::clear(&orders);
    // Agent 3 should match first (lower agent_id at same price)
    assert_eq!(fills[0].buyer, AgentId(3));
    assert_eq!(fills[0].seller, AgentId(1));
}
