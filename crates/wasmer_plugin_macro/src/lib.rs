#![recursion_limit="128"]
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
   Item as SynItem,
};

use proc_macro2::{
   Ident,
   Span,
};
use quote::quote;

#[proc_macro_attribute]
pub fn wasmer_plugin(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
   let tokens2 = proc_macro2::TokenStream::from(tokens);
   let parse2 = syn::parse2::<SynItem>(tokens2).expect("Failed to parse");
   match parse2 {
      SynItem::Fn(func) => handle_func(func),
      _ => panic!("Only functions are currently supported"),
   }
}

fn handle_func(func: syn::ItemFn) -> TokenStream {
   match &func.vis {
      syn::Visibility::Public(_) => (),
      _ => panic!("fns marked with wasmer_plugin must be public"),
   }
   if func.decl.inputs.len() != 1 {
      panic!("fns marked with wasmer_plugin can only take 1 argument");
   }
   let ident = func.ident.clone();
   let wasm_ident = Ident::new(&format!("_{}", ident.to_string()), Span::call_site());
   let ret = quote! {
      #func
      #[no_mangle]
      #[doc(hidden)]
      #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
      #[allow(clippy::all)]
      pub fn #wasm_ident(ptr: i32, len: i32) -> i32 {
         let slice: &[u8] = unsafe { ::std::slice::from_raw_parts(ptr as _, len as _) };
         let arg = wasmer_plugin::convert_slice(slice);
         let ret = #ident(arg);
         let serialized = wasmer_plugin::convert_ret(&ret);
         unsafe { ::std::ptr::replace(0 as _, serialized.len() as u32)};
         serialized.into_boxed_slice().as_ptr() as i32
      }
   };
   ret.into()
}