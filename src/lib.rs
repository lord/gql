#[macro_use]
extern crate nom;
extern crate regex;

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
  first: take_while1_s!(name_first_char) ~
  rest: take_while_s!(name_char) ,
  || {Token::Name(format!("{}{}", first, rest))}
));
named!(integer_part<&str, &str>, re_find!("^-?(?:0|(?:[1-9][0-9]*))"));
named!(fractional_part<&str, &str>, re_find!("^\\.[0-9]+"));
named!(exponent_part<&str, &str>, re_find!("^[eE][+-]?[0-9]+"));
named!(int_value<&str, Token>, chain!(
  num: integer_part ,
  || {Token::IntValue(num.parse::<i64>().unwrap())} // TODO REMOVE UNWRAP
));
named!(float_value<&str, Token>, chain!(
  i_part: integer_part ~
  f_part: fractional_part? ~
  e_part: exponent_part? ,
  || {
    Token::FloatValue(
      format!("{}{}{}", i_part, f_part.unwrap_or(""), e_part.unwrap_or(""))
        .parse::<f64>().unwrap())
    } // TODO REMOVE UNWRAP
));
named!(string_contents<&str, &str>,
  re_find!("^(?:(?:\\\\u[0-9A-Fa-f]{4})|(?:\\\\[\"\\\\/bfnrt])|(?:[^\"\\n\\r\\\\]))*"));
named!(string_value<&str, Token>, chain!(
  tag_s!("\"") ~
  contents: string_contents ~
  tag_s!("\"") ,
  || {Token::StringValue(contents.to_string())}
));
named!(token<&str, Token>, alt_complete!(
  punctuator |
  name |
  int_value |
  float_value |
  string_value
));
named!(tokens< &str, Vec<Token> >, many0!(delimited!(
  many_ignored,
  token,
  many_ignored
)));

#[cfg(test)]
mod tests {
    use super::tokens;
    #[test]
    fn it_works() {
      println!("{:?}", tokens(&"123 \n \"m\\re\\\"ow\"".to_string()));
      assert!(false)
    }
}
