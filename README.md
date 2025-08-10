# rust_ast_viewer

## Usage

```sh
cargo run -- example.rs
```

### Example

- [redis_sandbox](https://github.com/superneko160/redis_sandbox)

```sh
$ cargo run -- ../redis_sandbox/src/main.rs

Other: Discriminant(2)
Use: use redis :: Commands ; . tree
Use: use std :: env ; . tree
Function: connection_handling
  └─ Inputs: 0
  └─ Output: redis :: RedisResult < redis :: Connection >
Function: set_to_redis
  └─ Inputs: 3
    └─ [0] mut con : redis :: Connection . pat : mut con : redis :: Connection . ty
    └─ [1] key : String . pat : key : String . ty
    └─ [2] value : String . pat : value : String . ty
  └─ Output: redis :: RedisResult < () >
Function: get_from_redis
  └─ Inputs: 2
    └─ [0] mut con : redis :: Connection . pat : mut con : redis :: Connection . ty
    └─ [1] key : String . pat : key : String . ty
  └─ Output: redis :: RedisResult < String >
Function: parse_args
  └─ Inputs: 0
  └─ Output: Result < (String , Vec < String >) , String >
Function: show_usage
  └─ Inputs: 0
Function: execute_command
  └─ Inputs: 3
    └─ [0] con : redis :: Connection . pat : con : redis :: Connection . ty
    └─ [1] command : & str . pat : command : & str . ty
    └─ [2] params : & [String] . pat : params : & [String] . ty
  └─ Output: Result < () , String >
Function: execute_add_command
  └─ Inputs: 2
    └─ [0] con : redis :: Connection . pat : con : redis :: Connection . ty
    └─ [1] params : & [String] . pat : params : & [String] . ty
  └─ Output: Result < () , String >
Function: execute_get_command
  └─ Inputs: 2
    └─ [0] con : redis :: Connection . pat : con : redis :: Connection . ty
    └─ [1] params : & [String] . pat : params : & [String] . ty
  └─ Output: Result < () , String >
Function: run
  └─ Inputs: 0
  └─ Output: Result < () , String >
Function: main
  └─ Inputs: 0
```
## 使用クレート

- [syn - Rust](https://docs.rs/syn/latest/syn/)
- [quote - Rust](https://docs.rs/quote/latest/quote/)
