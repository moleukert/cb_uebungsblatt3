mod lexer;
mod c1_parser;

// Type definition for the Result that is being used by the parser. You may change it to anything
// you want
pub type ParseResult = Result<(), String>;

pub use lexer::C1Lexer;
pub use lexer::C1Token;

// You will need a re-export of your C1Parser definition. Here is an example:
//mod c1_parser;
pub use c1_parser::C1Parser;