#[macro_use]
extern crate nom;
extern crate regex;

mod lexer;

#[cfg(test)]
mod tests {
    use super::lexer;
    #[test]
    fn it_works() {
      println!("{:?}", lexer::lex(&"123 \n \"m\\re\\\"ow\"".to_string()));
      assert!(false)
    }
}
