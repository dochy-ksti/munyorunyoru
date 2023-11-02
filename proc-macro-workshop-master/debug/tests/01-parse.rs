// This test looks for a derive macro with the right name to exist. For now the
// test doesn't require any specific code to be generated by the macro, so
// returning an empty TokenStream should be sufficient.
//
// Before moving on, have your derive macro parse the macro input as a
// syn::DeriveInput syntax tree.
//
//
// Resources:
//
//   - The DeriveInput syntax tree which represents input of a derive macro:
//     https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
//
//   - An example of a derive macro implemented using Syn:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bitmask: u16,
}

fn main() {}
