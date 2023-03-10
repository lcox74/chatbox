use std::sync::{Arc, Mutex};

use chrono::prelude::*;

pub type SharedClient = Arc<Mutex<crate::client::Client>>;
pub type SharedClientList = Arc<Mutex<Vec<SharedClient>>>;

// Get reference from context
#[macro_export]
macro_rules! get_ctx {
    ($ctx:expr) => {{
        $ctx.lock().unwrap()
    }};
}

pub fn get_timestamp() -> String {
    let dt = Utc::now();

    format!("[{:0>2}\u{001b}[34;1m:\u{001b}[0m{:0>2}]", dt.hour(), dt.minute())
}