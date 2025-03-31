use crate::utils::read_input;
use crate::vault::Vault;

pub struct Prompt;

impl Prompt {
    pub fn new() -> Self {
        Self
    }

    pub async fn prompt_input(&self, prompt: &str) -> Result<String, std::io::Error> {
        println!("{prompt}");
        let input = read_input().await?;
        let input = input.trim();
        Ok(input.to_string())
    }
}