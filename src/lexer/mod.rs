//! 词法分析器
//! 拆分代码为Token stream，使用迭代器形式返回token
//! 为支持LL(1)分析，可前向看一个token
//! # Usage
//! ```
//! use std::fs::File;
//! use std::fs::File;
//! use std::io::Read;
//! use std::io::Read;
//! use lelang::lexer::LELexer;
//! let mut f = File::open("benches/test_case/lexer_test.le").unwrap();
//! let mut buffer = String::new();
//! f.read_to_string(&mut buffer).unwrap();
//! let lexer = LELexer::new(&buffer).unwrap();
//! for token in lexer{
//!     eprintln!("{:?}",token);
//! }
//! ```

pub use token_iterator::*;

mod token_iterator;
mod number_parser;
mod string_literal_parser;

