#[macro_use]
extern crate serde_derive;
use wasmer_plugin::*;

#[derive(Serialize, Deserialize)]
pub struct Interactive {
    pub one: u8,
    pub two: u32,
    pub three: String,
}

#[wasmer_plugin]
pub fn interact(mut interactive: Interactive) -> Interactive {
    interactive.one = interactive.one.wrapping_mul(100);
    interactive.two = interactive.two.wrapping_div(8);
    interactive.three = format!("embedded: {}", interactive.three);
    interactive
}