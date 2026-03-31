use crate::model::company_id::CompanyId;

#[test]
fn display() {
    assert_eq!(format!("{}", CompanyId(3)), "company-3");
}

#[test]
fn ordering() {
    assert!(CompanyId(1) < CompanyId(2));
}
