use proc_macro::TokenStream;
use quote::quote;
use std::mem::size_of;
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn gen_types(input: TokenStream) -> TokenStream {
    let count: usize = parse_macro_input!(input as LitInt)
        .base10_digits()
        .parse()
        .expect("expecting an usize");

    (0..=count)
        .map(|i| {
            let i = 1<<i as usize;
            let size = size_of::<usize>() * i;
            let fields = (0..i)
                .map(|j| {
                    format!("num_{}", j)
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let name: proc_macro2::TokenStream = format!("Struct{}", size).parse().unwrap();
            quote! {
                #[derive(Clone, Default)]
                pub struct #name {
                    #(#fields: usize,)*
                }

                impl #name {
                    fn get_size() -> usize {
                        ::std::mem::size_of::<#name>()
                    }
                }
            }
        })
        .fold(quote! {}, |acc, new| {
            quote! {
                #acc
                #new
            }
        })
        .into()
}

#[proc_macro]
pub fn gen_tests(input: TokenStream) -> TokenStream {
    let count: usize = parse_macro_input!(input as LitInt)
        .base10_digits()
        .parse()
        .expect("expecting an usize");

    let cases = (0..=count)
        .map(|i| {
            let i = 1<<i as usize;
            let size = size_of::<usize>() * i;
            let test_name = format!("clone_{}", size);
            let name: proc_macro2::TokenStream = format!("Struct{}", size).parse().unwrap();
            quote! {
                c.bench_function(#test_name, |b| {
                    let v = crate::#name::default();
                    b.iter(|| ::criterion::black_box(&v).clone())
                });
            }
        })
        .fold(quote! {}, |acc, new| {
            quote! {
                #acc
                #new
            }
        });
    
    quote! {
        pub fn benchmark(c: &mut ::criterion::Criterion) {
            #cases
        }
    }.into()
}