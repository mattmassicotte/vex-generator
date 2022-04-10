use nom::{
	branch::{alt},
	combinator::{value, map},
	character::complete::{char, alpha1},
	IResult,
	sequence::delimited,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PatternNode<'a> {
	Name(&'a str),
	Anonymous(&'a str),
	Wildcard,
	Anchor,

	Group(Box<PatternNode<'a>>),
	// Field(&'a str, Box<PatternNode<'a>>),
	// NegatedField(&'a str),
	//
	// Capture(&'a str, Box<PatternNode<'a>>),
	//
	// ZeroOrMore(Box<PatternNode<'a>>),
	// OneOrMore(Box<PatternNode<'a>>),
	// Optional(Box<PatternNode<'a>>),
	//
	// Directive(&'a str, Vec<&'a str>),
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
	delimited(
		char('"'),
		alpha1,
		char('"')
	)(i)
}

#[test]
fn string_test() {
  assert_eq!(parse_string(r#""abc""#), Ok(("", "abc")));
}

fn parse_name<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		alpha1,
		|value: &str| PatternNode::Name(value),
	)(i)
}

#[test]
fn parse_name_test() {
	assert_eq!(parse_name("abc"), Ok(("", PatternNode::Name("abc"))));
}

fn parse_anonymous<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		parse_string,
		|value: &str| PatternNode::Anonymous(value),
	)(i)
}

#[test]
fn parse_anonymous_test() {
	assert_eq!(parse_anonymous(r#""abc""#), Ok(("", PatternNode::Anonymous("abc"))));
}

fn parse_wildcard<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	value(
		PatternNode::Wildcard,
		char('_')
	)(i)
}

#[test]
fn wildcard_test() {
  assert_eq!(parse_wildcard("_"), Ok(("", PatternNode::Wildcard)));
}

fn parse_anchor<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	value(
		PatternNode::Anchor,
		char('.')
	)(i)
}

#[test]
fn anchor_test() {
  assert_eq!(parse_anchor("."), Ok(("", PatternNode::Anchor)));
}

fn parse_node<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	alt((
		parse_name,
		parse_anonymous,
		parse_wildcard,
		parse_anchor
	))(i)
}

fn parse_group<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		delimited(
			char('('),
			parse_node,
			char(')')
		),
		|node: PatternNode| PatternNode::Group(Box::new(node)),
	)(i)
}

#[test]
fn group_test() {
  assert_eq!(parse_group("(name)"), Ok(("", PatternNode::Group(Box::new(PatternNode::Name("name"))))));
}
