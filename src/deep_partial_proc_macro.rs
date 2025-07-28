// これは概念的な実装例です（実際にはproc-macroクレートで作成）

// Cargo.toml に以下が必要:
// [lib]
// proc-macro = true
// 
// [dependencies]
// proc-macro2 = "1.0"
// quote = "1.0"
// syn = { version = "1.0", features = ["full"] }


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(DeepPartial)]
pub fn deep_partial_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let partial_name = syn::Ident::new(&format!("{}Partial", name), name.span());
    
    let fields = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => &fields_named.named,
                _ => panic!("DeepPartial only supports structs with named fields"),
            }
        }
        _ => panic!("DeepPartial only supports structs"),
    };
    
    // 各フィールドをOption<T>に変換
    let partial_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        
        // 型を解析してPartial版に変換
        let partial_type = convert_type_to_partial(field_type);
        
        quote! {
            pub #field_name: Option<#partial_type>
        }
    });
    
    // ビルダーメソッドを生成
    let builder_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let method_name = syn::Ident::new(&format!("with_{}", field_name.as_ref().unwrap()), field_name.span());
        
        quote! {
            pub fn #method_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = Some(#field_name);
                self
            }
        }
    });
    
    let expanded = quote! {
        #[derive(Debug, Clone, Default)]
        pub struct #partial_name {
            #(#partial_fields,)*
        }
        
        impl #partial_name {
            pub fn new() -> Self {
                Self::default()
            }
            
            #(#builder_methods)*
            
            pub fn merge(self, other: Self) -> Self {
                // マージロジックも自動生成可能
                Self {
                    #(#field_name: other.#field_name.or(self.#field_name),)*
                }
            }
        }
        
        impl From<#name> for #partial_name {
            fn from(complete: #name) -> Self {
                Self {
                    #(#field_name: Some(complete.#field_name),)*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}

fn convert_type_to_partial(ty: &syn::Type) -> syn::Type {
    // 型の解析と変換ロジック
    // Vec<T> → Vec<T>
    // SomeStruct → SomeStructPartial
    // Option<T> → Option<T>
    // etc.
    ty.clone() // 簡略化
}


// 使用例（実際のコード）
pub fn usage_example() {
    
    #[derive(DeepPartial)]
    struct User {
        name: String,
        age: u32,
        address: Address,
    }
    
    #[derive(DeepPartial)]
    struct Address {
        street: String,
        city: String,
    }
    
    // 以下が自動生成される:
    //
    // struct UserPartial {
    //     name: Option<String>,
    //     age: Option<u32>,
    //     address: Option<AddressPartial>,  // 再帰的にPartial化！
    // }
    //
    // impl UserPartial {
    //     fn with_name(mut self, name: String) -> Self { ... }
    //     fn with_age(mut self, age: u32) -> Self { ... }
    //     fn merge(self, other: Self) -> Self { ... }
    // }
    
    
    println!("Procedural Macroの使用例");
}