use std::env;
use std::fs;
use syn::{parse_file, Item, ItemFn, ItemStruct, ItemEnum, ItemImpl };

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <file-path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("file reading error: {}", err);
            return;
        }
    };

    let syntax_tree = match parse_file(&content) {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("parsing error: {}", err);
            return;
        }
    };

    for item in syntax_tree.items {
        match item {
            Item::Fn(ItemFn { sig, .. }) => {
                println!("関数: {}", sig.ident);
            }
            Item::Struct(ItemStruct { ident, fields, .. }) => {
                println!("構造体: {} (フィールド数: {})", ident, fields.len());
            }
            Item::Enum(ItemEnum { ident, variants, .. }) => {
                println!("列挙型: {} (バリアント数: {})", ident, variants.len());
            }
            Item::Impl(ItemImpl { self_ty, .. }) => {
                println!("実装ブロック: {}", quote::quote!(#self_ty));
            }
            Item::Use(use_item) => {
                println!("use文: {}", quote::quote!(#use_item));
            }
            Item::Mod(mod_item) => {
                println!("モジュール: {}", mod_item.ident);
            }
            _ => {
                println!("others: {:?}", std::mem::discriminant(&item));
            }
        }
    }
}

