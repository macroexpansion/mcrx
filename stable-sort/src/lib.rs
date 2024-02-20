use proc_macro::TokenStream;
use syn;

#[proc_macro_attribute]
pub fn stable_sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut out = input.clone();

    let ty = syn::parse_macro_input!(input as syn::Item);
    assert!(args.is_empty());

    if let Err(e) = sort(ty) {
        out.extend(TokenStream::from(e.to_compile_error()));
    }

    out
}

fn sort(input: syn::Item) -> Result<(), syn::Error> {
    match input {
        syn::Item::Enum(_) => todo!(),
        syn::Item::Struct(value) => sort_struct(value),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected struct or enum",
        )),
    }
}

fn sort_struct(item: syn::ItemStruct) -> Result<(), syn::Error> {
    if let syn::Fields::Named(fields) = item.fields {
        let mut idents = Vec::new();
        for name in fields.named.iter() {
            let ident = name.ident.as_ref().unwrap().to_string();
            if idents.last().map(|last| &ident < last).unwrap_or(false) {
                let next_index = idents.binary_search(&ident).unwrap_err();

                return Err(syn::Error::new(
                    name.ident.as_ref().unwrap().span(),
                    format!("{} should sort before {}", ident, idents[next_index]),
                ));
            }
            idents.push(ident);
        }

        Ok(())
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected struct with braces",
        ))
    }
}
