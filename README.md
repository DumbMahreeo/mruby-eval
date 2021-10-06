# mruby-eval

This is a package I made for another lib, but here's how to use it:

```rust
use mruby-eval::{eval_to, evaluators::eval_to_string};

eval_to_string("1+2") // returns an Option<String>, in this case Some(String::from("3"))

eval_to!(i32, "1+2") // eval_to!<type, expr> returns an Err<type, Parse#Error>
                     // (# in ParseError is the type, ex: ParseIntError)
                     //
                     // So in this case it's Ok(3)

```
