use std::io;
use nom::{
  IResult,
  branch::alt,
  bytes::complete::{is_not, tag, take_until},
  character::complete::anychar,
  combinator::{value, eof, peek, opt},
  multi::{many0, many_till},
  sequence::{preceded, delimited, terminated},
};


fn get_input() -> String {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

// match single comment 
fn parse_single_line_comment(s: &str) -> IResult<&str,usize> {
  value(1,
    preceded( tag("//"),is_not("\n"))
  )(s)
}

// mat
fn parse_multi_line_comments(s: &str) -> IResult<&str, usize> {
  let (a,b) = 
  delimited(
    tag("/*"),
    take_until("*/"),
    tag("*/")
  )(s)?;

  Ok((a,b.lines().count()))
}

fn parse_comment(s: &str) -> IResult<&str, usize> {
  alt((parse_single_line_comment, parse_multi_line_comments))(s)
}

fn parse_end_of_file(s:&str) -> IResult<&str,usize> {
  value(0,eof)(s)
}

fn skip_not_comment(s: &str) -> IResult<&str, usize> {
  let (s_rest, _) = many_till(anychar, 
              peek(alt((parse_comment, parse_end_of_file)))
  )(s)?;
  Ok((s_rest,0))
}



pub fn count_comments(s: &str) -> IResult<&str,usize> {
  let (tail,_) = opt(skip_not_comment)(s)?;

  let (rest, nums) = 
    many0( 
      terminated(
        parse_comment,
        opt(skip_not_comment)
      )
    )(tail)?;

    Ok((rest, nums.iter().sum()))
}