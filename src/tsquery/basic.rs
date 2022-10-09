use nom::{
	branch::{alt},
	combinator::{recognize},
	character::complete::{alpha1, alphanumeric1, char, one_of},
	IResult,
	multi::many0_count,
	sequence::{delimited, pair},
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
