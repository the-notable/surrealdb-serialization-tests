use proc_macro::TokenStream;
use std::iter::Peekable;
use proc_macro2::{Group, Ident, Literal, Span, TokenTree};
use proc_macro2::token_stream::IntoIter;
use quote::{quote, ToTokens};
use syn::{Expr, Token};
use syn::token::Comma;

fn syn_parse2<T: syn::parse::Parse>(input: &mut Peekable<IntoIter>) -> T {
    syn::parse2::<T>(input.next().unwrap().into()).unwrap()
}

#[proc_macro]
pub fn test_surreal_serialization(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let input_grp = syn::parse::<Group>(input_iter.next().unwrap().into()).unwrap();
    let mut input_grp_iter = input_grp.stream().into_iter();
    let mut test_targets = vec![];
    while let Some(v) = input_grp_iter.next() {
        if let Ok(grp) = syn::parse2::<Group>(v.to_token_stream()) {
            let mut iter = grp.stream().into_iter().peekable();
            let src_ty = syn_parse2::<Ident>(&mut iter);
            let _ = syn_parse2::<Comma>(&mut iter);
            let src_val = if let Some(tt) = iter.next() {
                match tt {
                    TokenTree::Group(_) => panic!(),
                    TokenTree::Ident(ty) => {
                        let _ = syn_parse2::<Token![:]>(&mut iter);
                        let _ = syn_parse2::<Token![:]>(&mut iter);
                        let const_or_method = syn_parse2::<Ident>(&mut iter);
                        let maybe_group = syn::parse2::<Group>(iter.peek().unwrap().to_token_stream());
                        let expr_str = if let Ok(g) = maybe_group {
                            // We peeked and successfully parsed a Group, but we still need to
                            // advance the iterator, throwing away the now-unneeded value
                            let _ = iter.next().unwrap();
                            format!("{ty}::{const_or_method}{}", g.to_string())
                        } else {
                            format!("{ty}::{const_or_method}")
                        };
                        syn::parse_str::<Expr>(expr_str.as_str()).unwrap()
                    },
                    TokenTree::Punct(neg_sign) => {
                        let neg_sign = neg_sign.to_string();
                        assert_eq!("-", neg_sign);
                        let int_abs_val = syn_parse2::<Literal>(&mut iter);
                        let int_str = format!("-{}", int_abs_val.to_string());
                        syn::parse_str::<Expr>(int_str.as_str()).unwrap()
                    }
                    TokenTree::Literal(v) => {
                        syn::parse2::<Expr>(v.to_token_stream()).unwrap()
                    }
                }
            } else {
                panic!()
            };
            let _ = syn_parse2::<Comma>(&mut iter);
            let tar_srl_ty = syn_parse2::<Ident>(&mut iter);
            let _ = syn_parse2::<Comma>(&mut iter);
            let sdk_test_pf = syn_parse2::<Ident>(&mut iter);
            test_targets.push((src_ty, src_val, tar_srl_ty, sdk_test_pf));
        }
    }

    let mut tests = proc_macro2::TokenStream::new();
    for (test_number, (src_ty, src_val, tar_srl_ty, sdk_test_pf)) in test_targets.iter().enumerate() {

        let mut src_val_str = src_val.to_token_stream().to_string();
        src_val_str.retain(|c| !c.is_whitespace());

        let sdk_test_name = format!("test_{test_number}_{src_ty}_{tar_srl_ty}_{sdk_test_pf}");
        let sdk_test_name = Ident::new(sdk_test_name.as_str(), Span::call_site());

        let tar_srl_ty = match tar_srl_ty.to_string().as_str() {
            "int" => "int",
            "dec" => "decimal",
            "flt" => "float",
            "str" => "string",
            "def" => "default",
            _ => panic!("invalid surrealdb type")
        };

        let schema_sql = format!(r#"
            DEFINE TABLE test SCHEMAFULL;
            DEFINE FIELD test_value ON TABLE test TYPE {tar_srl_ty};
        "#);
        
        let define_schema = (tar_srl_ty != "default").then(|| {
            quote! { db.query(#schema_sql).await.unwrap(); }
        });

        let src_struct_name = format!("Test{src_ty}Src");
        let src_struct_name = Ident::new(&src_struct_name.as_str(), Span::call_site());

        let target_struct_name = format!("Test{src_ty}Target");
        let target_struct_name = Ident::new(&target_struct_name.as_str(), Span::call_site());

        let test_should_panic = (sdk_test_pf.to_string() == "f").then(|| {
            quote! { #[should_panic] }
        });
        
        let sdk_test = quote! {

            #test_should_panic
            #[tokio::test]
            async fn #sdk_test_name() {
                use surrealdb::Surreal;
                use surrealdb::sql::Thing;
                use surrealdb::engine::local::Mem;
                use serde::{Serialize, Deserialize};
                use surrealdb::Error;
                use rust_decimal::Decimal;
                
                let db = Surreal::new::<Mem>(()).await.unwrap();
                db.use_ns("test").use_db("test").await.unwrap();
                
                #define_schema

                #[derive(Serialize)]
                struct #src_struct_name {
                    test_value: #src_ty
                }

                #[derive(Deserialize)]
                struct #target_struct_name {
                    id: Thing,
                    test_value: #src_ty
                }

                let src = #src_struct_name { test_value: #src_val };
                
                let created: Vec<#target_struct_name> = db
                    .create("test")
                    .content(&src)
                    .await
                    .expect("Error creating record");

                let target = created.first().unwrap();
                assert_eq!(src.test_value, target.test_value);
            }
        };

        tests.extend(sdk_test);
    }
    
    tests.into()
}