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
        let ty = &field.ty;
        if ty_inner_type("Option", ty).is_some() {
            quote! { #ident: #ty }
        } else {
            quote! { #ident: std::option::Option<#ty> }
        }
    });

    let methods = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = if let Some(ty) = ty_inner_type("Option", &field.ty) {
            ty
        } else {
            &field.ty
        };
        quote! {
            pub fn #ident(&mut self, value: #ty) -> &mut Self {
                self.#ident = Some(value);
                self
            }
        }
    });

    let builder_struct_ident = syn::Ident::new(&format!("{ident}Builder"), ident.span());
    let builder_struct_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        quote! { #ident: None }
    });
    let builder_struct_build = fields.iter().map(|field| {
        let ident = &field.ident;
        if ty_inner_type("Option", &field.ty).is_some() {
            quote! { #ident: self.#ident.clone() }
        } else {
            quote! { #ident: self.#ident.clone().ok_or(concat!(stringify!(#ident), " is not set"))? }
        }
    });

    let generated = quote! {
        pub struct #builder_struct_ident {
            #(#optionized_fields,)*
        }

        impl #builder_struct_ident {
            #(#methods)*

            pub fn build(&mut self) -> std::result::Result<#ident, std::boxed::Box<dyn std::error::Error>> {
                Ok(#ident {
                    #(#builder_struct_build,)*
                })
            }
        }

        impl #ident {
            pub fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    #(#builder_struct_fields,)*
                }
            }
        }
    };

    TokenStream::from(generated)
}

fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }
    None
}
