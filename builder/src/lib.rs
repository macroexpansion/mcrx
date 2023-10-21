use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();

    impl_builder(&ast)
}

fn impl_builder(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let builder_struct_ident = Ident::new(&format!("{ident}Builder"), ident.span());

    let generated = quote! {
        pub struct #builder_struct_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #builder_struct_ident {
            pub fn executable(&mut self, value: String) -> &mut Self {
                self.executable = Some(value);
                self
            }

            pub fn args(&mut self, value: Vec<String>) -> &mut Self {
                self.args = Some(value);
                self
            }

            pub fn env(&mut self, value: Vec<String>) -> &mut Self {
                self.env = Some(value);
                self
            }

            pub fn current_dir(&mut self, value: String) -> &mut Self {
                self.current_dir = Some(value);
                self
            }
        }

        impl #ident {
            pub fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    TokenStream::from(generated)
}
