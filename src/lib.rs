pub use wasmer_plugin_macro::wasmer_plugin;
use serde::{Deserialize, Serialize};

pub fn convert_slice<'i, D>(slice: &'i [u8]) -> D 
where D: Deserialize<'i> {
    bincode::deserialize(slice).expect("Failed to deserialize slice")
}

pub fn convert_ret<S>(ret: &S) -> Vec<u8> 
where S: Serialize {
    bincode::serialize(ret).expect("Failed to serialize return value")
}