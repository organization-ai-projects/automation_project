use crate::model::company_id::CompanyId;
use crate::model::store::Store;
use crate::model::store_id::StoreId;

#[test]
fn new_store() {
    let s = Store::new(StoreId(1), CompanyId(0), "Main St".into());
    assert_eq!(s.id(), StoreId(1));
    assert_eq!(s.owner(), CompanyId(0));
    assert_eq!(s.name(), "Main St");
}
