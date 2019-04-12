# Wasmer Plugin

## What
This crate is designed to ease the development of plugins that will be 
run as web assembly modules via [wasmer](https://github.com/wasmerio/wasmer).

The components here include a `proc_macro` attribute and a few helper functions 
to ease the serialization/deserialization of data structures in wasm memory.

## Why
Let's first cover all of the steps that would need to happen to allow this.

1. Set up (`parent scope`)
    1. Instantiate the Wasm Module 
    1. Provide a shared data structure to start with 
    1. Serialize the shared data 
    1. Insert serialized data into Wasm Shared Memory 
    1. Execute exported function 
1. Plug In (`plugin scope`)
    1. Deserialize shared data
    1. Manipulate shared data
    1. Serialize shared data
    1. Insert new length of data into known memory location
    1. Return pointer to start of new data
1. Repeat (`parent scope`)
    1. Extract shared data from wasm memory
    1. Deserialize shared data
    1. Move on to the next plugin?

Of all of the steps above, a plugin developer really should have to think about 2.2 --
All of the other steps should be handled by the parent process.

## How
`wasmer-plugin` utilizes [bincode](https://github.com/TyOverby/bincode) to pass
data structures from the parent scope into the wasm scope and vice versa.

First, we need to import the contents of this crate
by putting `use wasmer_plugin::*` at the top of the file
which will make 3 things available.
1. The attribute `#[wasmer_plugin]`
1. The function `convert_slice<'a, T>(&[u8]) -> impl Deserialize<'a>` (used by the attribute)
1. The function `convert_ret<T>(T: impl Serialize) -> Vec<u8>` (used by the attribute)

### The Attribute
When a plugin developer marks a function with the
attribute `#[wasmer_plugin]`, it will add 
a public wrapper function that will take care of 
the messy memory bits or transitioning from one
scope to another.

### The Functions
Those two function are essentially just a way to guarantee
that bincode's serialize and deserialize implementations
are going to be available for the attribute.


## Limitations
- Currently this is only setup to handle a plugin that will
take one argument and return one argument. 
- The scheme used to move data around
is not very efficient from a space (or probably speed) 
perspective

## Example
In the (./examples)[./examples] folder there is a file called `simple.rs` 
that includes an example of what a plugin might look like. It has a companion
file `simple_expanded.rs` that includes the result of running 
`cargo +nightly expand --example simple --target wasm32-unknown-unknown`.

> note, these files are not traditional rust examples so running 
> `cargo build --example simple` will fail.