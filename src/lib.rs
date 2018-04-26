extern crate time;

use time::{now, strftime};

pub fn time() -> String {
    let timestamp = match strftime("%s.%f", &now()) {
        Ok(t) => t,
        Err(_) => "".to_string()
    };
    timestamp
}
