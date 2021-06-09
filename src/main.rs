// System miodules
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Env details
    dotenv().ok();

    println!("Hello, world!");
}
