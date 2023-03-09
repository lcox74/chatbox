use std::{net::TcpStream, sync::Arc, io::Write};
use chrono::prelude::*;
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

    pub fn send(&self, msg: String, reset_line: bool) {
        let dt = Utc::now();
        let time_str = format!("[{:0>2}\u{001b}[34;1m:\u{001b}[0m{:0>2}]", dt.hour(), dt.minute());

        if reset_line {
            self.conn.as_ref().write_all(format!("\r\u{001b}[F{} | {}", time_str, msg).as_bytes()).unwrap();
        } else {
            self.conn.as_ref().write_all(format!("\r{} | {}", time_str, msg).as_bytes()).unwrap();
        }
    }

    pub fn send_msg(&self, from: String, from_color: u8, msg: String) {
        let from_str = format!("\u{001b}[1m\u{001b}[38;5;{}m{: >9}:\u{001b}[0m", from_color, from);
        self.send(format!("{} {}", from_str, msg), false);
    }

    pub fn send_self(&self, msg: String) {
        let from_str = format!("\u{001b}[1m\u{001b}[38;5;{}m{: >9}:\u{001b}[0m", self.color, "You");
        self.send(format!("{} {}", from_str, msg), true);
    }

    pub fn send_self_priv(&self, target: String, target_color: u8, msg: String) {
        let from_str = format!("\u{001b}[1m\u{001b}[38;5;{}m{: >9}\u{001b}[38;5;255m -> \u{001b}[38;5;{}m{}:\u{001b}[0m", self.color, "You", target_color, target);
        self.send(format!("{} {}", from_str, msg), true);
    }

    pub fn send_server(&self, msg: String, reset_line: bool) {
        self.send(format!("{}\n", msg), reset_line);
    }
}