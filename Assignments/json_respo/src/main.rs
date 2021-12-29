pub mod test;

use crate::response::root::json;
pub mod response {
    pub mod root;
}

fn main() {
    let value="Kchitiz".to_string();
    json(value).ok();
}
