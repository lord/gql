#[macro_use]
extern crate nom;

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

#[cfg(test)]
mod tests {
    use super::ignored;
    #[test]
    fn it_works() {
      println!("{:?}", ignored(&" ".to_string()));
      assert!(false)
    }
}
