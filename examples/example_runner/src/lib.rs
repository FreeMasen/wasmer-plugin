#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Interactive {
    pub one: u8,
    pub two: u32,
    pub three: String,
}

impl ::std::default::Default for Interactive {
    fn default() -> Self {
        Self {
            one: 2,
            two: 2, 
            three: String::from("Interactive"),
        }
    }
}