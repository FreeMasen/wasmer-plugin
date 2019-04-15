pub use wasmer_plugin_macro::wasmer_plugin;
use serde::{Deserialize, Serialize};

pub fn convert_slice<'i, D>(slice: &'i [u8]) -> D 
where D: Deserialize<'i> {
    match bincode::deserialize(slice) {
        Ok(ret) => ret,
        Err(e) => {
            panic!("error deserializing {}", e);
        },
    }
}

pub fn convert_ret<S>(ret: S) -> Vec<u8> 
where S: Serialize {
    match bincode::serialize(&ret) {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("error serializing {}", e)
        }
    }
}