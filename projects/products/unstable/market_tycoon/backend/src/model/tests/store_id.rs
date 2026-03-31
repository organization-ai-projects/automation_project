use crate::model::store_id::StoreId;

#[test]
fn display() {
    assert_eq!(format!("{}", StoreId(7)), "store-7");
}
