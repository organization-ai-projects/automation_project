use crate::model::company::Company;
use crate::model::company_id::CompanyId;

#[test]
fn new_company() {
    let c = Company::new(CompanyId(1), "Acme".into(), 5000);
    assert_eq!(c.id(), CompanyId(1));
    assert_eq!(c.name(), "Acme");
    assert_eq!(c.budget(), 5000);
}

#[test]
fn adjust_budget() {
    let mut c = Company::new(CompanyId(1), "Acme".into(), 5000);
    c.adjust_budget(-1000);
    assert_eq!(c.budget(), 4000);
    c.adjust_budget(500);
    assert_eq!(c.budget(), 4500);
}
