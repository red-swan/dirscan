use std::io;
use nom::{
  Err, IResult, Parser,
  branch::alt,
  bytes::complete::{is_not, tag, take_until},
  character::complete::{char, line_ending},
  combinator::{value, eof, map, not, success},
  error::{ErrorKind, ParseError},
  multi::many0,
  sequence::{pair, tuple, preceded},
};


fn get_input() -> String {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}


fn single_line_comment(s: &str) -> IResult<&str,(&str,&str)> {
  pair( tag("//"),is_not("\n\r"))(s)
}

pub fn count_single_line_comment(i: &str) -> IResult<&str, usize> {
  value(1,
    single_line_comment
  )(i)
}

fn multi_line_comments(i: &str) -> IResult<&str, (&str,&str,&str)> {
  tuple((tag("/*"),take_until("*/"),tag("*/")))(i)
}

fn count_multi_line_comments(i: &str) -> IResult<&str, usize> {
  map(multi_line_comments, |(_,s,_)| s.lines().count()) (i) 
}

fn _count_comment_lines(s: &str) -> IResult<&str,usize> {
  alt((count_single_line_comment, count_multi_line_comments))(s)
}

fn _find_comment(s: &str) -> IResult<&str,usize> {
  preceded(
    take_until("/"), 
    _count_comment_lines
  )(s)
}

pub fn count_comment_lines(s: &str) -> usize {
  match many0(_find_comment)(s) {
    std::result::Result::Ok(("",v)) => v.iter().sum(),
    _ => usize::MAX
  }
}

