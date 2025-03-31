// use tokio::io::AsyncReadExt;

pub async fn read_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())

    // let mut input = String::new();
    // tokio::io::stdin().read_to_string(&mut input).await?;
    // Ok(input.trim().to_string())
}