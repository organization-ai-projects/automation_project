use crate::events::event_log::EventLog;
use crate::model::company::Company;
use crate::model::company_id::CompanyId;
use crate::model::good::Good;
use crate::model::inventory::Inventory;
use crate::model::store_id::StoreId;
use crate::pricing::pricing_engine::PricingEngine;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::time::tick::Tick;

#[test]
fn updates_prices_with_markup() {
    let mut inv = Inventory::new(StoreId(1));
    inv.add_stock(Good::Widget, 10);
    let company = Company::new(CompanyId(0), "Acme".into(), 10000);
    let policy = PricingPolicy::default();
    let mut log = EventLog::new();

    PricingEngine::update_prices(&Tick(0), &company, &inv, &policy, &mut log);
    assert!(!log.events().is_empty());
}
