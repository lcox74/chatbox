use std::{net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, io::Read};


pub mod client;
pub mod command;
pub mod types;
use types::{SharedClient, SharedClientList, get_timestamp};


const HOST : &str = "0.0.0.0:3000";

fn main() {
    println!("Running Chatbox Server on {}...", HOST);

    // Create TCP listener
    let listener: TcpListener = TcpListener::bind(HOST).unwrap();

    // Create a shared Vector of Clients
    let shared_clients: SharedClientList = Arc::new(Mutex::new(Vec::new()));

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Clone the Arc pointer to the clients vector
                let copy_clients = Arc::clone(&shared_clients);

                let shared_stream = Arc::new(stream);
                let cloned_stream = Arc::clone(&shared_stream);

                // Spawn a new thread
                std::thread::spawn(move || {
                    // Create new client
                    let client = SharedClient::new(Mutex::new(client::Client::new(cloned_stream.clone())));

                    // Add the new client to the shared vector
                    copy_clients.lock().unwrap().push(client.clone());

                    // Accept a new client
                    handle_client(copy_clients, client, cloned_stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

}


fn handle_client(ctx: SharedClientList, client: SharedClient, stream: Arc<TcpStream>) { 

    let client_name = get_ctx!(client).name.clone();
    println!("{} | \u{001b}[1m{}({})\u{001b}[0m joined the server", get_timestamp(), client_name.clone(), stream.peer_addr().unwrap());

    // Send welcome
    get_ctx!(client).send_server("Welcome to the \u{001b}[4mChatbox Server\u{001b}[0m!".to_string(), false);
    get_ctx!(client).send_server(format!("Your name is \u{001b}[1m{}\u{001b}[0m, feel free to change it", client_name.clone()), false);

    // List connected users
    command::list_cmd(ctx.clone(), client.clone(), false);

    // Send message to all clients
    command::send_join_msg(ctx.clone(), client_name.clone());

    // Recv Loop
    loop {
        // Get Response
        let mut buffer = [0; 512];

        match  stream.as_ref().read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    let client_name = get_ctx!(client).name.clone();
                    let client_color: u8 = get_ctx!(client).color.clone();

                    // Remove client from list
                    let index = get_ctx!(ctx).iter().position(|c| get_ctx!(c).name == client_name).unwrap();
                    get_ctx!(ctx).remove(index);

                    // Send message to all clients
                    command::send_leave_msg(ctx.clone(), client_name.clone(), client_color.clone());
                    println!("{} | \u{001b}[1m{}({})\u{001b}[0m left the server", get_timestamp(), client_name.clone(), stream.peer_addr().unwrap());

                    break;
                }

                if size == 1 {
                    get_ctx!(client).send_server("".to_string(), true);
                    continue;
                }
            },
            Err(_) => {
                let client_name = get_ctx!(client).name.clone();
                let client_color: u8 = get_ctx!(client).color.clone();

                // Remove client from list
                let index = get_ctx!(ctx).iter().position(|c| get_ctx!(c).name == client_name).unwrap();
                get_ctx!(ctx).remove(index);

                // Send message to all clients
                command::send_leave_msg(ctx.clone(), client_name.clone(), client_color.clone());
                println!("{} | \u{001b}[1m{}({})\u{001b}[0m left the server", get_timestamp(), client_name.clone(), stream.peer_addr().unwrap());

                break;
            }
        }
        let message = String::from_utf8_lossy(&buffer[..]).trim().to_string();
        let messages: Vec<&str> = message.split("\n").collect();

        // Process multiple commands or messages
        for msg in messages {
            if msg.len() > 0 {
                // Check if message is a command where the first character is a '/'
                if msg.clone().starts_with("/") {
                    // Extract from Message
                    let args: Vec<&str> = msg.split_whitespace().collect();
                    let command = args[0].to_uppercase();
        
                    // Handle command
                    match command.as_str() {
                        "/NICK" => command::nick_cmd(ctx.clone(), client.clone(), args[1..].to_vec()),
                        "/PRIVMSG" => command::privmsg_cmd(ctx.clone(), client.clone(), args[1..].to_vec()),
                        "/COLOR" => command::color_cmd(client.clone(), args[1..].to_vec()),
                        "/LIST" => command::list_cmd(ctx.clone(), client.clone(), true),
                        _ => {
                            get_ctx!(client).send_server(format!("Unknown command: {}", command), true);
                        }
                    }
                } else if msg.as_bytes()[0] != 0 {
                    command::send_normal_msg(ctx.clone(), client.clone(), msg.to_string());
                } else {
                    // NOP
                }
            }
        }
        
    }
}
