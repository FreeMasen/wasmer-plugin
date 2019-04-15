
use wasmer_runtime::{
    Instance,
    instantiate,
    imports,
    Ctx,
    func,
};
use std::process::Command;

use std::{
    fs::File,
    io::Read,
    path::PathBuf,
};

use example_runner_lib::Interactive;


const START: usize = 5;

fn main() {
    println!("Creating initial shared data");
    let target_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    let wasm_path = target_path
                    .join("wasm32-unknown-unknown")
                    .join("release")
                    .join("example_plugin.wasm");
    if true {// !wasm_path.exists()  {
        println!("compiling example plugin");
        let output = Command::new("cargo")
                .arg("build")
                .arg("--target")
                .arg("wasm32-unknown-unknown")
                .arg("-p")
                .arg("example_plugin")
                .arg("--release")
                .arg("--target-dir")
                .arg(&target_path)
                .output()
                .expect("Faild to compile plugin");
            assert!(output.status.success());
    }
    assert!(wasm_path.exists());
    println!("running example plugin");
    let mut val = Interactive::default();
    for i in 0..10 {
        let updated = match run_sub_processor(&wasm_path, val) {
            Ok(updated) => {
                println!("{} Successfully updated\n{:#?}", i, updated);
                updated
            }
            Err(e) => panic!("Failed to update\n{}", e),
        };
        val = updated;
    }
}

fn run_sub_processor(path: &PathBuf, inter: Interactive) -> Result<Interactive, String> {
    let mut inst = read_and_instantiate_wasm(path)?;
    let len = serialize_and_inject(&mut inst, &inter)?;
    let inter = run_wasm(&mut inst, len)?;
    Ok(inter)
}

fn read_and_instantiate_wasm(path: &PathBuf) -> Result<Instance, String> {
    let import_object = imports! {
            // Define the "env" namespace that was implicitly used
            // by our sample application.
            "env" => {
                // name        // the func! macro autodetects the signature
                "print_str" => func!(print_str),
            },
    };
    println!("reading in the wasm module");
    let mut wasm = Vec::new();
    let mut f = File::open(path).map_err(|e| format!("failed to open file {:?} - {}", path, e))?;
    f.read_to_end(&mut wasm).map_err(|e| format!("failed to read file {:?} - {}", path, e))?;
    println!("instantiating the wasm module");
    let inst = instantiate(&wasm, imports! {}).map_err(|e| format!("failed to instantiate {:?}\n{}", path, e))?;
    Ok(inst)
}

fn serialize_and_inject(inst: &mut Instance, inter: &Interactive) -> Result<usize, String> {
    println!("serializing the shared data");
    let serialized = bincode::serialize(inter).map_err(|e| format!("error serializing {}", e))?;
    let mem = inst.context_mut().memory(0);
    let len = serialized.len();
    println!("inserting the bytes into wasm memory (bytes {}-{})", START, len + START);
    // Notice that we are starting at byte 4 and not byte 0
    // This is because we are going to reserve bytes 0-3 for
    // the new length of the return value 
    for (cell, byte) in mem.view()[START..len + START].iter().zip(serialized.iter()) {
        cell.set(*byte)
    }
    Ok(len)
}

fn run_wasm(inst: &mut Instance, len: usize) -> Result<Interactive, String> {
    println!("binding the wasm function _interact");
    let func = inst.func::<(i32, i32), i32>("_interact").map_err(|e| format!("failed to bind _interact \n{}", e))?;
    println!("executing _interact");
    let ptr = func.call(START as _, len as i32).map_err(|e| format!("failed to execute _interact\n{}", e))?;
    extract_value(inst, ptr)
}


fn extract_value(inst: &wasmer_runtime::Instance, ptr: i32) -> Result<Interactive, String> {
    println!("extracting updated bytes: {}", ptr);
    let updated_mem = inst.context().memory(0);
    let view = updated_mem.view();
    let mut new_len_bytes: [u8; 4] = [0;4];
    for i in 0..4 {
        new_len_bytes[i] = view.get(1 + i).map(|c| c.get()).ok_or(format!("unable to get new length part {}", i))?;
    }
    let new_len = u32::from_ne_bytes(new_len_bytes);
    println!("new length: {}", new_len);
    let view = updated_mem.view();
    println!("extracting data");
    let end: usize = ptr as usize + new_len as usize;
    let buf: Vec<u8> = view[ptr as usize..end].iter().map(|c| c.get()).collect();
    println!("deserializing bytes");
    let updated_book = bincode::deserialize(&buf).map_err(|e| format!("Unable to reconstruct\n{}", e))?;
    Ok(updated_book)
}

fn print_str(ctx: &mut Ctx, ptr: u32, len: u32) {
    let memory = ctx.memory(0);
    let str_vec: Vec<_> = memory.view()[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(|cell| cell.get())
        .collect();
    let string = String::from_utf8_lossy(&str_vec).to_string();
    println!("wasm: {:?}", string);
}