#[macro_use]
extern crate nom;
extern crate regex;

mod lexer;

#[cfg(test)]
mod tests {
    use super::lexer;
    #[test]
    fn it_works() {
      println!("{:?}", lexer::lex(include_str!("blah.graphql")));
      assert!(false)
    }
}
