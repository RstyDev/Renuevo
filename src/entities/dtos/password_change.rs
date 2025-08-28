use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct PasswordChange {
    pub id: String,
    pub new_password: String,
    pub old_password: String,
}
