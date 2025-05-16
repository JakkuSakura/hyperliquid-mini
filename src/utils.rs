#[allow(unused_imports)]
use crate::HYPERLIQUID;

use uuid::Uuid;

pub(crate) fn trim_float_in_string_for_hashing(x: &mut String) -> &str {
    if x.contains('.') {
        while x.ends_with('0') {
            x.pop();
        }
        if x.ends_with('.') {
            x.pop();
        }
    }
    x.as_str()
}

pub(crate) fn uuid_to_hex_string(uuid: Uuid) -> String {
    format!("0x{}", uuid.simple())
}
