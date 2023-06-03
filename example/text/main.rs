#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        "9414589060430990",
        "44NSNAUSF094545nLKIGSJFSKF78985395495NKSJNFDJNS0FNSNJFNSDNFSDNFJNSDKFKSDJFNJSDNFJSD",
        &[
            with_api_version(APIVersion::V16),
            with_base_url("https://graph.facebook.com"),
        ],
    );

    let response = client.send_text("1234567890", "Hello, World!").await?;

    if response.is_successful() {
        println!("Message sent successfully");
    } else if let Some(error) = response.error {
        eprintln!("Failed to send message: {}", error.message);
    } else {
        eprintln!("Failed to send message");
    }

    Ok(())
}
