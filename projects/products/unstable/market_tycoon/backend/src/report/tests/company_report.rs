use crate::model::company_id::CompanyId;
use crate::report::company_report::CompanyReport;

#[test]
fn company_report_fields() {
    let r = CompanyReport {
        company_id: CompanyId(0),
        name: "Acme".into(),
        final_budget: 5000,
        store_count: 2,
    };
    assert_eq!(r.name, "Acme");
    assert_eq!(r.store_count, 2);
}
