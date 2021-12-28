pub mod test;

use crate::response::root::json;
pub mod response {
    pub mod root;
}

fn main() {
    json().ok();
}
