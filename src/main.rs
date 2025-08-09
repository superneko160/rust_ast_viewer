use std::env;
use std::fs;
use syn::{parse_file, Item };
use quote::quote;

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

    for item in &syntax_tree.items {
        print_ast_structure(item, 0);
    }
}

/// AST（抽象構文木）のアイテムを階層構造で表示
///
/// # 引数
///
/// * `item` - 表示するASTアイテムへの参照
/// * `indent` - インデントレベル（階層の深さを表現）
///   - 0: トップレベル（インデントなし）
///   - 1: 1階層下（2スペースのインデント）
///   - n: n階層下（n×2スペースのインデント）
///
/// # 表示される情報
///
/// - **関数**: 名前、引数の詳細、戻り値の型、ジェネリクス
/// - **構造体**: 名前、フィールドの詳細、ジェネリクス
/// - **列挙型**: 名前、バリアントの種類、ジェネリクス
/// - **実装ブロック**: 対象型、トレイト情報、アイテム数
/// - **モジュール**: 名前、内部アイテムの再帰表示
/// - **その他**: use文、型エイリアス、定数、静的変数、トレイト
///
/// # 例
///
/// ```text
/// Function: main
///   └─ Inputs: 0
/// Struct: Person
///   └─ Named Fields:
///     └─ name : String
///     └─ age : u32
/// Module: utils
///   └─ Items: 2
///     Function: helper
///       └─ Inputs: 1
/// ```
fn print_ast_structure(item: &Item, indent: usize) {
    let prefix = "  ".repeat(indent);

    match item {
        Item::Fn(func) => {
            println!("{}Function: {}", prefix, func.sig.ident);
            println!("{}  └─ Inputs: {}", prefix, func.sig.inputs.len());
            
            // 引数の詳細表示
            for (i, input) in func.sig.inputs.iter().enumerate() {
                match input {
                    syn::FnArg::Receiver(_) => {
                        println!("{}    └─ [{}] self", prefix, i);
                    }
                    syn::FnArg::Typed(pat_type) => {
                        println!("{}    └─ [{}] {} : {}", prefix, i, 
                               quote!(#pat_type.pat), quote!(#pat_type.ty));
                    }
                }
            }
 
            // 戻り値の型表示
            if let syn::ReturnType::Type(_, ty) = &func.sig.output {
                println!("{}  └─ Output: {}", prefix, quote!(#ty));
            }

            // ジェネリクスがある場合
            if !func.sig.generics.params.is_empty() {
                println!("{}  └─ Generics: {}", prefix, quote!(#func.sig.generics));
            }
        }

        Item::Struct(s) => {
            println!("{}Struct: {}", prefix, s.ident);

            // ジェネリクスがある場合
            if !s.generics.params.is_empty() {
                println!("{}  └─ Generics: {}", prefix, quote!(#s.generics));
            }

            match &s.fields {
                syn::Fields::Named(fields) => {
                    println!("{}  └─ Named Fields:", prefix);
                    for field in &fields.named {
                        if let Some(name) = &field.ident {
                            println!("{}    └─ {} : {}", prefix, name, quote!(#field.ty));
                        }
                    }
                }
                syn::Fields::Unnamed(fields) => {
                    println!("{}  └─ Tuple Fields:", prefix);
                    for (i, field) in fields.unnamed.iter().enumerate() {
                        println!("{}    └─ [{}] : {}", prefix, i, quote!(#field.ty));
                    }
                }
                syn::Fields::Unit => {
                    println!("{}  └─ Unit Struct", prefix);
                }
            }
        }

        Item::Enum(e) => {
            println!("{}Enum: {}", prefix, e.ident);

            // ジェネリクスがある場合
            if !e.generics.params.is_empty() {
                println!("{}  └─ Generics: {}", prefix, quote!(#e.generics));
            }

            println!("{}  └─ Variants:", prefix);
            for variant in &e.variants {
                match &variant.fields {
                    syn::Fields::Named(_) => {
                        println!("{}    └─ {} {{ ... }}", prefix, variant.ident);
                    }
                    syn::Fields::Unnamed(_) => {
                        println!("{}    └─ {}(...)", prefix, variant.ident);
                    }
                    syn::Fields::Unit => {
                        println!("{}    └─ {}", prefix, variant.ident);
                    }
                }
            }
        }

        Item::Impl(impl_item) => {
            println!("{}Impl: {}", prefix, quote!(#impl_item.self_ty));
            if let Some((_, trait_path, _)) = &impl_item.trait_ {
                println!("{}  └─ Trait: {}", prefix, quote!(#trait_path));
            }
            println!("{}  └─ Items: {}", prefix, impl_item.items.len());
        }

        Item::Use(use_item) => {
            println!("{}Use: {}", prefix, quote!(#use_item.tree));
        }

        Item::Mod(mod_item) => {
            println!("{}Module: {}", prefix, mod_item.ident);
            if let Some((_, items)) = &mod_item.content {
                println!("{}  └─ Items: {}", prefix, items.len());
                for sub_item in items {
                    print_ast_structure(sub_item, indent + 2);
                }
            }
        }

        Item::Type(type_item) => {
            println!("{}Type Alias: {} = {}", prefix, type_item.ident, quote!(#type_item.ty));
        }

        Item::Const(const_item) => {
            println!("{}Const: {} : {} = {}", prefix, const_item.ident, 
                   quote!(#const_item.ty), quote!(#const_item.expr));
        }

        Item::Static(static_item) => {
            println!("{}Static: {} : {}", prefix, static_item.ident, quote!(#static_item.ty));
        }

        Item::Trait(trait_item) => {
            println!("{}Trait: {}", prefix, trait_item.ident);
            println!("{}  └─ Items: {}", prefix, trait_item.items.len());
        }

        _ => {
            println!("{}Other: {:?}", prefix, std::mem::discriminant(item));
        }
    }
}
