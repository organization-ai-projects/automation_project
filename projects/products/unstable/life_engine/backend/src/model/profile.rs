use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EmploymentStatus {
    Employed,
    Unemployed,
    SelfEmployed,
    Student,
    Retired,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: String,
    pub status: Option<EmploymentStatus>,
    pub income_before: Option<f64>,
    pub location: Option<String>,
}
