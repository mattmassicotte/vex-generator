use nom::{
	branch::{alt},
	combinator::{recognize, value},
	character::streaming::{not_line_ending},
	character::complete::{alpha1, alphanumeric1, char, multispace0},
	IResult,
	multi::many0_count,
	sequence::{delimited, pair, terminated},
	bytes::complete::tag,
};

pub fn parse_identifier<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
	let seperators = alt((tag("_"), tag("-")));
	
	recognize(
		pair(
			alt((alpha1, tag("_"))),
			many0_count(alt((alphanumeric1, seperators)))
		)
	)(i)
}

#[test]
fn parse_identifier_test() {
	assert_eq!(parse_identifier("abc"), Ok(("", "abc")));
	assert_eq!(parse_identifier("abc1"), Ok(("", "abc1")));
	assert_eq!(parse_identifier("a-b"), Ok(("", "a-b")));
	assert_eq!(parse_identifier("a_c"), Ok(("", "a_c")));
}

pub fn parse_string<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
	delimited(
		char('"'),
		alphanumeric1,
		char('"')
	)(i)
}

#[test]
fn string_test() {
  assert_eq!(parse_string(r#""abc""#), Ok(("", "abc")));
}

pub fn parse_comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
	value(
		(),
		many0_count(
			pair(
				char(';'),
				terminated(not_line_ending, multispace0)
			)
		)
	)(i)
}

#[test]
fn comment_test() {
  assert_eq!(parse_comment(";abcdef\n"), Ok(("", ())));
  assert_eq!(parse_comment(";\n"), Ok(("", ())));
  assert_eq!(parse_comment(";a\n;b\n;c\n"), Ok(("", ())));
}
