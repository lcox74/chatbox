use std::{net::TcpStream, sync::Arc, io::Write};
use uuid::Uuid;

pub struct Client {
    pub name: String,
    pub color: u8,
    pub conn: Arc<TcpStream>,
}

impl Client {
    pub fn new(conn: Arc<TcpStream>) -> Client {
        let uuid = Uuid::new_v4().as_simple().to_string();
        Client {
            name: format!("u-{}", uuid[0..7].to_string()),
            color: 255,
            conn,
        }
    }

    /// Send a message to the client, this will prepend a timestamp to the message
    /// 
    /// # Arguments
    /// 
    /// * `msg` - The message to send
    /// * `reset_line` - Whether to reset the line or not
    /// * `from_client` - Whether the message is from the client or not (for debugging purposes)
    /// * `serv_msg` - The message the server should print (for debugging purposes)
    /// 
    /// # Example
    /// 
    /// ```
    /// let shared_stream = Arc::new(tcpstream);
    /// let client = Client::new(Arc::clone(&shared_stream));
    /// client.send("Hello World!".to_string(), false, false, "".to_string());
    /// ```
    fn send(&self, msg: String, reset_line: bool, from_client: bool, serv_msg: String) {
        let time_str = crate::types::get_timestamp();

        if reset_line {
            self.conn.as_ref().write_all(format!("\r\u{001b}[F{} | {}\n", time_str.clone(), msg).as_bytes()).unwrap();
        } else {
            self.conn.as_ref().write_all(format!("\r{} | {}\n", time_str.clone(), msg).as_bytes()).unwrap();
        }

        if from_client {
            println!("{} | {}", time_str.clone(), serv_msg);
        }
    }

    /// Send a standard message to another client
    /// 
    /// # Arguments
    /// 
    /// * `from` - The name of the client that sent the message
    /// * `from_color` - The color of the client that sent the message
    /// * `msg` - The message to send
    /// 
    /// # Example
    /// 
    /// ```
    /// let shared_stream = Arc::new(tcpstream);
    /// let client = Client::new(Arc::clone(&shared_stream));
    /// client.send_msg("Bob".to_string(), 255, "Hello World!".to_string());
    /// ```
    pub fn send_msg(&self, from: String, from_color: u8, msg: String) {
        let from_str = format!("\u{001b}[1m\u{001b}[38;5;{}m{: >9}:\u{001b}[0m", from_color, from);
        self.send(format!("{} {}", from_str, msg), false, false, "".to_string());
    }

    /// Send a copy of the standard message to the original sender. 
    /// This will prepend the message with "You:"
    /// 
    /// # Arguments
    /// 
    /// * `msg` - The message to send
    /// 
    /// # Example
    /// 
    /// ```
    /// let shared_stream = Arc::new(tcpstream);
    /// let client = Client::new(Arc::clone(&shared_stream));
    /// client.send_self("Hello World!".to_string());
    /// ```
    pub fn send_self(&self, msg: String) {
        let srv_from_str = format!("\u{001b}[1m{: >9}:\u{001b}[0m", self.name.clone());
        let from_str = format!("\u{001b}[1m\u{001b}[38;5;{}m{: >9}:\u{001b}[0m", self.color, "You");
        self.send(format!("{} {}", from_str, msg), true, true, format!("{} {}", srv_from_str, msg));
    }

    /// Send a private message to another client
    /// 
    /// # Arguments
    /// 
    /// * `target` - The name of the client that the message is being sent to
    /// * `target_color` - The color of the client that the message is being sent to
    /// * `msg` - The message to send
    /// 
    /// # Example
    /// 
    /// ```
    /// let shared_stream = Arc::new(tcpstream);
    /// let client = Client::new(Arc::clone(&shared_stream));
    /// client.send_priv("Bob".to_string(), 255, "Hello World!".to_string());
    /// ```
    pub fn send_self_priv(&self, target: String, target_color: u8, msg: String) {
        let srv_from_str = format!("\u{001b}[1m{: >9}\u{001b}[38;5;255m -> {}:\u{001b}[0m", self.name.clone(), target);
        let from_str = format!("\u{001b}[1m\u{001b}[38;5;{}m{: >9}\u{001b}[38;5;255m -> \u{001b}[38;5;{}m{}:\u{001b}[0m", self.color, "You", target_color, target);
        self.send(format!("{} {}", from_str, msg), true, true, format!("{} {}", srv_from_str, msg));
    }

    /// Send a message to the client from the server, this usually doesn't get sent
    /// to other clients, and is more of a feedback status message
    /// 
    /// # Arguments
    /// 
    /// * `msg` - The message to send
    /// * `reset_line` - Whether to reset the line or not
    /// 
    /// # Example
    /// 
    /// ```
    /// let shared_stream = Arc::new(tcpstream);
    /// let client = Client::new(Arc::clone(&shared_stream));
    /// client.send_server("Welcome User".to_string(), false);
    /// ```
    pub fn send_server(&self, msg: String, reset_line: bool) {
        self.send(format!("{}", msg), reset_line, false, format!("{}", msg));
    }
}