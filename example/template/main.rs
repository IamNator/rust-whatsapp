use rust_whatsapp::{language_code, with_api_version, with_base_url, APIVersion, Client};
// use rust_whatsapp::template::Template;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let tmpl = template::

    let client = Client::new(
        "9414589060430990",
        "44NSNAUSF094545nLKIGSJFSKF78985395495NKSJNFDJNS0FNSNJFNSDNFSDNFJNSDKFKSDJFNJSDNFJSD",
        &[
            with_api_version(APIVersion::V16),
            with_base_url("https://graph.facebook.com"),
        ],
    );

    let mut tmpl = template::new(&str("otp_template"), language_code::en_US);
    tmpl.add_header("Daniel");
    tmpl.add_body("Daniel");
    tmpl.add_body("3243");
    tmpl.add_body("30");

    let to: &str = "2349045057266";

    let response = client.send_template(to, tmpl.done());

    if response.is_successful() {
        println!("Message sent successfully");
    } else if let Some(error) = response.error {
        eprintln!("Failed to send message: {}", error.message);
    } else {
        eprintln!("Failed to send message");
    }

    Ok(())
}
