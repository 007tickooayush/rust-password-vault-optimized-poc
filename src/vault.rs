use crate::utils::read_input;
use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
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

    // TODO: implement a trait based approach to dynamically decide the method of storage(DB or local files)
    pub async fn write_to_file(&self) -> Result<(), std::io::Error> {
        let json_formatted = format!("{}\n", self.to_json()?);

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("credentials.json").await {
            Ok(mut file) => {
                if let Err(error) = file.write_all(json_formatted.as_bytes()).await {
                    Err(error)
                } else {
                    Ok(())
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }


    fn to_json(&self) -> Result<String, std::io::Error> {
        match serde_json::to_string(&self) {
            Ok(json) => Ok(json),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn from_json(json_string: &'vault str) -> Result<Self, std::io::Error> {
        match serde_json::from_str(json_string) {
            Ok(vault) => Ok(vault),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Unsupported, e.to_string()))
        }
    }
}


#[test]
fn test_from_json() {
    let json = r#"{"service":"aws-s3","username":"ceo","password":"ceo"}"#;
    let vault = Vault::from_json(json).unwrap();

    println!("{:?}", vault.clone());

    assert_eq!(vault.service, "aws-s3");
    assert_eq!(vault.username, "ceo");
    assert_eq!(vault.password, "ceo");
}