mod server;
mod routes;
mod components;
mod utils;

use env_logger::Env;

// const ADDR: &str = "127.0.0.1";
const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!(r#"
  __                     __               __   
 |__|.-----.-----.--.--.|  |.-----.-----.|  |_ 
 |  ||  _  |  -__|  |  ||  ||  -__|     ||   _|
 |  ||_____|_____|___  ||__||_____|__|__||____|.dev
|___|            |_____|                       
"#);

    // Enable logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start the server
    println!("🚀 Listening for connections on {}:{}", ADDR, PORT);
    println!("Press Ctrl + C to exit.");
    server::start_server(ADDR, PORT).await
}
