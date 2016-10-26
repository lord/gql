#[macro_use]
extern crate nom;

#[derive(Debug)]
enum Token {
  Punctuator(char),
  Name(String),
  IntValue(i64),
  FloatValue(f64),
  StringValue(String),
}

// Ignored Tokens
fn line_terminator_char(x: char) -> bool {
  x == '\u{000D}' || x == '\u{000A}'
}
named!(unicode_bom<&str, &str>, tag_s!("\u{feff}"));
named!(white_space<&str, &str>, alt!(tag_s!(" ") | tag_s!("\t")));
named!(line_terminator<&str, &str>, alt!(
  tag_s!("\u{000D}\u{000A}") |
  tag_s!("\u{000D}") |
  tag_s!("\u{000A}")));
named!(comment<&str, &str>, preceded!(
  tag_s!("#"),
  take_till_s!(line_terminator_char)));
named!(comma<&str, &str>, tag_s!(","));
named!(ignored<&str, &str>, alt_complete!(
  unicode_bom |
  white_space |
  line_terminator |
  comment |
  comma));
named!(many_ignored<&str, Vec<&str> >, many0!(ignored));

// Lexical Tokens
fn name_char(x: char) -> bool {
  x.is_alphanumeric() || x == '_'
}
fn name_first_char(x: char) -> bool {
  x.is_alphabetic() || x == '_'
}
named!(punctuator_str<&str, &str>, alt!(
  tag_s!("!") |
  tag_s!("$") |
  tag_s!("(") |
  tag_s!(")") |
  tag_s!("...") |
  tag_s!(":") |
  tag_s!("=") |
  tag_s!("@") |
  tag_s!("[") |
  tag_s!("]") |
  tag_s!("{") |
  tag_s!("|") |
  tag_s!("}")));
named!(punctuator<&str, Token>, chain!(
  punc: punctuator_str ,
  ||{Token::Punctuator(punc.chars().nth(0).unwrap())}
));
named!(name<&str, Token>, chain!(
  first: take_while_s!(name_first_char) ~
  rest: take_while_s!(name_char) ,
  || {Token::Name(format!("{}{}", first, rest))}
));

#[cfg(test)]
mod tests {
    use super::name;
    #[test]
    fn it_works() {
      println!("{:?}", name(&"uentha1298ueto_uentoh".to_string()));
      assert!(false)
    }
}
