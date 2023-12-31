use dirscan::*;



static SAMPLES: [&str;8] = 
  [
    "No comments here",
    "//Hello there!\n//General Kenobi",
    "/* What's the deal with airline food?\nIt keeps getting worse and worse\nI can't take it anymore!*/",
    " //Global Variable\nlet x = 5;\n/*TODO:\n\t// Add the number of cats as a variable\n\t//Shouldn't take too long\n*/\nlet c = 500;",
    "//First\n//Second//NotThird\n//Third",
    "x = 3*4 /* not 3*5 */",
    "/* foo */ /* unterminated comment",
    ""
  ];

#[test]
pub fn count_all_comments() {
  let tests: Vec<usize> = SAMPLES.iter().map(|x| count_comments(&x).unwrap().1).collect();
  assert_eq!(tests, vec![0,2,3,4,3,1,1,0])
}