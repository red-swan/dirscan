use dirscan::*;


static SAMPLE1: &str = "//Hello there!\n//What's going on?";
static SAMPLE2: &str = "/* What's the deal with airline food?\nIt keeps getting worse and worse\nI can't take it anymore!*/";
static SAMPLE3: &str = " //Global Variable\nlet x = 5;\n/*TODO:\n\t// Add the number of cats as a variable\n\t//Shouldn't take too long\n*/";
static SAMPLE4: &str = "//First\n//Second//NotThird\n//Third";

#[test]
fn count_sample1_correctly() {
  assert_eq!(2, count_comment_lines(&SAMPLE1))
}

#[test]
fn count_sample2_correctly() {
  assert_eq!(3, count_comment_lines(&SAMPLE2))
}


#[test]
fn count_sample3_correctly() {
  assert_eq!(4, count_comment_lines(&SAMPLE3))
}

#[test]
fn count_sample4_correctly() {
  assert_eq!(3, count_comment_lines(&SAMPLE4))
}