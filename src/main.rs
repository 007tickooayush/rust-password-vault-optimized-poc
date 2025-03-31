use crate::prompt::Prompt;

mod vault;
mod utils;
mod prompt;

#[tokio::main]
async fn main() {

    // for TESTING the implementation of the prompt
    // loop {
    //     test_user_input().await;
    // }
}


async fn test_user_input() {
    let prompt = Prompt::new();

    let service = prompt.prompt_input("Enter service: ").await.unwrap();
    let username = prompt.prompt_input("Enter username: ").await.unwrap();
    let password = prompt.prompt_input("Enter password: ").await.unwrap();
    let vault = vault::Vault::from(
        &service,
        &username,
        &password
    );


    println!("Vault created successfully");
    println!("Vault values: {:?}", vault);
}