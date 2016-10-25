#[macro_use]
extern crate nom;

amed!(white_space<&str, &str>, alt!(tag_s!(" ") | tag_s!("\n")));
named!(letter, tag!("meow"));
named!(parens< &[u8], Vec<&[u8]> >, delimited!(char!('('), many0!(letter), char!(')')));

#[cfg(test)]
mod tests {
    use super::white_space;
    #[test]
    fn it_works() {
      println!("meow");
      println!("{:?}", white_space(&"\n".to_string()));
      assert!(false)
    }
}
