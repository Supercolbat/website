//! Website software program by Joey Lent
//! Licensed under MIT

mod server;
mod routes;
mod components;
mod utils;
mod blog;
mod state;
mod reading_time;
mod logging;

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
    logging::init_logging();
    
    // Construct blog struct
    let blog = Arc::new(Mutex::new(blog::Blog::default().expect("Failed to construct blog")));
    info!("Initialized blog");
    let css = Arc::new(Mutex::new(state::PageStyle::new()));

    if ! cfg!(debug_assertions) {
        css.lock().unwrap().update_styles();
        info!("Compiled SCSS");
    }

    // Start the server
    println!("🚀 Listening for connections on {}", address);
    println!("Press Ctrl + C to exit.");
    let server = server::create_server(address, Arc::clone(&blog), Arc::clone(&css));
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
        let mut buf = String::default();
        if io::stdin().read_line(&mut buf).is_err() {
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
                match blog.lock().unwrap().update_articles() {
                    Ok(_) => { println!("Complete!") } 
                    Err(_) => { println!("Error!") } 
                }
            }
            _ => {
                println!("Unknown command");
            }
        }
    }

    // Gracefully stop the web server if the loop exits
    println!("exit");
    handle.stop(true).await;
}
