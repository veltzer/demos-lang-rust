use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn reverse_string(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal
    let input = parse_macro_input!(input as LitStr);
    let input_string = input.value();
    
    // Reverse the string at compile time
    let reversed = input_string.chars().rev().collect::<String>();
    
    // Generate the code that will create this string at runtime
    let expanded = quote! {
        String::from(#reversed)
    };
    
    TokenStream::from(expanded)
}

// Usage:
let reversed = reverse_string!("Hello, World!");  // Creates "!dlroW ,olleH"
