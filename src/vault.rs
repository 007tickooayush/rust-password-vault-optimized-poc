use crate::utils::read_input;
use serde::{Deserialize, Serialize};
use crate::prompt::Prompt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vault<'vault> {
    pub service: &'vault str,
    pub username: &'vault str,
    pub password: &'vault str,
}

impl<'vault> Vault<'vault> {
    pub fn from(service: &'vault str, username: &'vault str, password: &'vault str) -> Self {
        Self {
            service,
            username,
            password,
        }
    }

    pub fn empty() -> Self {
        Self {
            service: "",
            username: "",
            password: "",
        }
    }


}


