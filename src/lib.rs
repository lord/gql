#[macro_use]
extern crate nom;
extern crate regex;

mod lexer;
pub use lexer::lex;

#[cfg(test)]
mod tests {
    use super::lexer;
    #[test]
    fn it_works() {
      lexer::lex(include_str!("blah.graphql"));
    }
}
