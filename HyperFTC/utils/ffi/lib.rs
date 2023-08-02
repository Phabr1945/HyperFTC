extern crate proc_macro;
use proc_macro::TokenStream;

/// Simplifies the creation of FFI functions
///
/// # Example
/// ```
/// #[utils::ffi(type = "system")]
/// pub fn Java_Main_greet<'a>(
///     mut env: JNIEnv<'a>, _class: JClass<'a>, input: JString<'a>
/// ) -> jstring {
///     // First, we have to get the string out of Java. Check out the `strings`
///     // module for more info on how this works.
///     let input: String = env.get_string(&input).expect("Couldn't get java string!").into();
///
///     // Then we have to create a new Java string to return. Again, more info
///     // in the `strings` module.
///     let output = env.new_string(
///         format!("Hello, {}!", input)
///     ).expect("Couldn't create java string!");
/// 
///     // Finally, extract the raw pointer to return.
///     output.into_raw()
/// }
/// ```
#[proc_macro_attribute]
pub fn ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_backup = item.clone();
    let res = parse_func(item).to_string();
    let settings = parse_settings(attr);
    let res = res.replace("__FFI_RAW_MODIFIERS__", &settings.0);
    let res = res.replace("__FFI_EXTERN_MODIFIER__", &settings.1);
    res.parse().unwrap_or(item_backup)
}

/// parses the function it should be attached to
fn parse_func(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let generics = &input.sig.generics;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;
    let result = quote::quote! {
        #(#attrs)*
        #[no_mangle]
        #vis __FFI_RAW_MODIFIERS__ extern __FFI_EXTERN_MODIFIER__ fn #name #generics(#inputs) #ret {
            #body
        }
    };
    result.into()
}

/// parses the settings (can be none, const, unsafe or both)
fn parse_settings<'a>(attr: TokenStream) -> (String, String) {
    let modifiers_str = attr.to_string();
    let mut res = String::new();
    if modifiers_str.contains("const") {
        res.push_str("const ");
    }
    if modifiers_str.contains("unsafe") {
        res.push_str("unsafe ");
    }
    let t = if modifiers_str.contains("type=") {
        let type_str = modifiers_str.split("type=").collect::<Vec<&str>>()[1];
        type_str.split(" ").collect::<Vec<&str>>()[0]
    } else {
        "C"
    };
    (res, format!("\"{}\"",t))
}