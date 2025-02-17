use bitcoin::BigUint;
use bitcoin::PrivateKey;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use figlet_rs::FIGfont;

#[tokio::main]
async fn main() {
    // Generate ASCII art
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font
        .convert("Bitcoin")
        .expect("Failed to generate ASCII art");
    println!("{}", figure);

    // Menu options
    let options = vec![
        "Convert Hexadecimal to Decimal",
        "Generate a random private key",
        "Exit",
    ];

    // Loop to keep the menu active until "Exit" is selected
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => convert_hex_to_dec(),
            1 => generate_random_private_key(),
            2 => {
                println!("Exiting...");
                break; // Exit the loop if "Exit" is selected
            }
            _ => println!("Invalid option"),
        }
    }
    // Private key
    let private_key =
        PrivateKey::new(b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949");
    println!("Private Key: {}", private_key);
    println!("Signature: {}", private_key.sign(b"55"));
}

// Request a BigUint in hexadecimal from the user
fn convert_hex_to_dec() {
    let input: String = Input::new()
        .with_prompt("Enter a large integer in hexadecimal format")
        .interact_text()
        .expect("Failed to read input");

    match BigUint::parse_bytes(input.as_bytes(), 16) {
        Some(num) => println!("Decimal: {}", num),
        None => println!("Invalid hexadecimal number format"),
    }
}

// Generate a random private key
fn generate_random_private_key() {
    let private_key = PrivateKey::random();
    println!("Random Private Key: {}", private_key);
}
