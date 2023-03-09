use std::sync::{Arc, Mutex};

pub type SharedClient = Arc<Mutex<crate::client::Client>>;
pub type SharedClientList = Arc<Mutex<Vec<SharedClient>>>;

// Get reference from context
#[macro_export]
macro_rules! get_ctx {
    ($ctx:expr) => {{
        $ctx.lock().unwrap()
    }};
}