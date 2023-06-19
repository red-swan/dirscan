use std::io;
use nom::{
  Err, IResult, Parser,
  branch::alt,
  bytes::complete::{is_not, tag, take_until, take_till},
  character::complete::{char, line_ending, anychar},
  combinator::{value, eof, map, not, success, all_consuming, peek, opt},
  error::{ErrorKind, ParseError},
  multi::{many0, many_till},
  sequence::{pair, tuple, preceded, delimited, terminated},
};


fn get_input() -> String {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

// match single comment 
fn parse_single_line_comment(s: &str) -> IResult<&str,&str> {
  preceded( tag("//"),is_not("\n"))(s)
}

// mat
fn parse_multi_line_comments(i: &str) -> IResult<&str, &str> {
  delimited(tag("/*"),take_until("*/"),tag("*/"))(i)
}

fn parse_comment(s: &str) -> IResult<&str, &str> {
  alt((parse_single_line_comment, parse_multi_line_comments))(s)
}

fn parse_end_of_file(s:&str) -> IResult<&str,&str> {
  eof(s)
}

fn skip_not_comment(s: &str) -> IResult<&str, &str> {
  let (s_rest, _) = many_till(anychar, 
              peek(alt((parse_comment, parse_end_of_file)))
  )(s)?;
  Ok((s_rest,""))
}



pub fn extract_comments(s: &str) -> IResult<&str,Vec<&str>> {
  let (tail,_) = opt(skip_not_comment)(s)?;

    many0( 
      terminated(
        parse_comment,
        opt(skip_not_comment)
      )
    )(tail)
}

// fn count_multi_line_comments(i: &str) -> IResult<&str, usize> {
//   map(multi_line_comments, |s| s.lines().count()) (i) 
// }

// fn _count_comment_lines(s: &str) -> IResult<&str,usize> {
//   alt((count_single_line_comment, count_multi_line_comments))(s)
// }

// fn f(s: &str) -> IResult<&str,usize> {
//   alt(
//     (_count_comment_lines, 
//      preceded(first, second)))(s)
// }


// fn _find_comment(s: &str) -> IResult<&str,usize> {
//   alt(
//         (_count_comment_lines, preceded(take_until(_count_comment_lines), _count_comment_lines))
//   )(s)
//   // alt(
//   //   (_count_comment_lines, preceded(not(_count_comment_lines), _count_comment_lines))
//   // )(s)
//   // preceded(
//   //   take_until("/"), 
//   //   _count_comment_lines
//   // )
// }

// pub fn asd(s: &str) -> IResult<&str,Vec<&str>> {

//   many0(
//     alt((comments, 
//           preceded(
//           take_until(not(comments)),
//           comments
//         )))
//   )(s)

// }

// // pub fn count_comment_lines(s: &str) -> usize {
// //   match many0(_find_comment)(s) {
// //     std::result::Result::Ok(("",v)) => v.iter().sum(),
// //     _ => usize::MAX
// //   }
// // }


// fn f(s: &str) -> IResult<&str,&str> {
//   tag("ab")(s)
// }

// pub fn aaa(s: &str) -> IResult<&str, Vec<&str>> {
//   many0(
//     alt(
//       (tag("ab"), preceded(take_until("ab"), tag("ab")))
//     )
//   )(s)
// }