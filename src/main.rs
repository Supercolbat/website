mod server;
mod routes;
mod components;
mod utils;
mod blog;
mod state;
mod reading_time;

use env_logger::Env;
use log::info;
use actix_web::rt;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    io::{self, Write},
    sync::{Arc, Mutex},
};

#[actix_web::main]
async fn main() {
    println!(r#"
  __                     __               __   
 |__|.-----.-----.--.--.|  |.-----.-----.|  |_ 
 |  ||  _  |  -__|  |  ||  ||  -__|     ||   _|
 |  ||_____|_____|___  ||__||_____|__|__||____|.dev
|___|            |_____|                       
"#);

    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);

    // Enable logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // Construct blog struct
    let blog = Arc::new(Mutex::new(blog::Blog::default().expect("Failed to construct blog")));
    info!("Initialized blog");

    // Start the server
    info!("🚀 Listening for connections on {}", address);
    info!("Press Ctrl + C to exit.");
    let server = server::create_server(address, Arc::clone(&blog));
    let handle = server.handle();

    // Spawn the server in another thread
    rt::spawn(server);

    // Start the server
    handle.resume().await;

    // Parse commands (temporary)
    // TODO: work in terminal as commands, rather than as a CLI
    loop {
        print!(">>> ");
        io::stdout().lock().flush().unwrap();

        // Read input and handle
        let mut buf = String::new();
        if let Err(_) = io::stdin().read_line(&mut buf) {
            println!("[!] Failed to read command.");
            continue;
        }
        
        // Match commands
        let line = buf.trim();
        match line {
            "help" => {
                println!("Commands: help, exit, refresh");
            }
            "exit" => {
                println!("Quitting...");
                break;
            }
            "refresh" => {
                println!("Refreshing...");
                match Arc::clone(&blog).lock().unwrap().update_articles() {
                    Ok(_) => { println!("Complete!") } 
                    Err(_) => { println!("Error!") } 
                }
            }
            _ => {
                println!("Unknown command");
            }
        }
    }

    // Gracefully stop the web server if the loops exits
    println!("exit");
    handle.stop(true).await;
}
