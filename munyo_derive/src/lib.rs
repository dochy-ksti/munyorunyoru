mod sample1;

#[proc_macro_derive(MunyoSerialize, attributes(munyo))]
pub fn munyo(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = dbg!(item);
    //item
    proc_macro::TokenStream::new()
}
