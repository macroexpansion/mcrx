use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    impl_builder(&ast)
}

fn impl_builder(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        unimplemented!()
    };
    let optionized_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty.clone();
        quote! { #ident: std::option::Option<#ty> }
    });
    let methods = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty.clone();
        quote! {
            pub fn #ident(&mut self, value: #ty) -> &mut Self {
                self.#ident = Some(value);
                self
            }
        }
    });
    let builder_struct_ident = syn::Ident::new(&format!("{ident}Builder"), ident.span());
    let builder_struct_attrs = fields.iter().map(|field| {
        let ident = &field.ident;
        quote! { #ident: None }
    });

    let generated = quote! {
        pub struct #builder_struct_ident {
            #(#optionized_fields,)*
        }

        impl #builder_struct_ident {
            #(#methods)*
        }

        impl #ident {
            pub fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    #(#builder_struct_attrs,)*
                }
            }
        }
    };

    TokenStream::from(generated)
}
