use nom::{
	branch::{alt},
	bytes::complete::{tag},
	bytes::streaming::{take_until1, take_while},
	combinator::{map, opt, value},
	character::{is_alphabetic},
	character::streaming::{not_line_ending, line_ending},
	character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1, one_of},
	IResult,
	multi::{many_till, many0},
	sequence::{delimited, preceded, tuple, terminated, pair},
};

use super::basic::{parse_identifier, parse_string};

#[derive(Clone, Debug, PartialEq)]
pub enum DirectiveComponent<'a> {
	Capture(&'a str),
	String(&'a str)
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternNode<'a> {
	Name(&'a str),
	Anonymous(&'a str),
	Wildcard,
	Anchor,

	Field(&'a str, Box<PatternNode<'a>>),
	NegatedField(&'a str),
	Directive(&'a str, Vec<DirectiveComponent<'a>>),

	Capture(&'a str, Box<PatternNode<'a>>),
	
	ZeroOrMore(Box<PatternNode<'a>>),
	OneOrMore(Box<PatternNode<'a>>),
	Optional(Box<PatternNode<'a>>),

	Group(Vec<PatternNode<'a>>),
	Alternation(Vec<PatternNode<'a>>),
}

// utilities
fn parse_comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
	value(
		(),
		many0(
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

// basic node types
fn parse_name<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		parse_identifier,
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

fn parse_non_capturable_node<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	let node_parser = alt((
		parse_anchor,
		parse_negated_field
	));

	preceded(multispace0, terminated(node_parser, multispace0))(i)
}

fn parse_node<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	let node_parser = alt((
		parse_node_with_capture,
		parse_non_capturable_node,
	));
	
	preceded(opt(parse_comment), terminated(node_parser, opt(parse_comment)))(i)
}

fn parse_alternation<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		preceded(
			tag("["),
			many_till(parse_node, tag("]")),
		),
		|(nodes, _)| PatternNode::Alternation(nodes),
	)(i)
}

#[test]
fn alternation_test() {
	let nodes = vec![
		PatternNode::Name("a"),
		PatternNode::Name("b"),
		PatternNode::Name("c")
	];
	
	assert_eq!(parse_alternation("[a b c]"), Ok(("", PatternNode::Alternation(nodes))));
	assert_eq!(parse_alternation("[a]"), Ok(("", PatternNode::Alternation(vec![PatternNode::Name("a")]))));
}

fn parse_group<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		preceded(
			tag("("),
			many_till(parse_node, tag(")")),
		),
		|(nodes, _)| PatternNode::Group(nodes),
	)(i)
}

#[test]
fn parse_group_test() {
	let single = PatternNode::Group(
		vec![
			PatternNode::Name("a")
		]
	);

	assert_eq!(parse_group("(a)"), Ok(("", single)));

	let multiple = PatternNode::Group(
		vec![
			PatternNode::Name("a"),
			PatternNode::Name("b"),
			PatternNode::Name("c"),
		]
	);

	assert_eq!(parse_group("(a b c)"), Ok(("", multiple)));

	let nested = PatternNode::Group(
		vec![
			PatternNode::Name("a"),
			PatternNode::Group(
				vec![PatternNode::Name("b")]
			)
		]
	);

	assert_eq!(parse_group("(a (b))"), Ok(("", nested)));
}

fn parse_field<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	let field_name = terminated(parse_identifier, tag(":"));

	map(
		tuple((field_name, parse_node)),
		|(name, node)| PatternNode::Field(name, Box::new(node))
	)(i)
}

#[test]
fn field_test() {
	assert_eq!(parse_field("label:name"), Ok(("", PatternNode::Field("label", Box::new(PatternNode::Name("name"))))));
	assert_eq!(parse_field("label: name"), Ok(("", PatternNode::Field("label", Box::new(PatternNode::Name("name"))))));
}

fn parse_negated_field<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		preceded(tag("!"), parse_identifier),
		|name| PatternNode::NegatedField(name)
	)(i)
}

#[test]
fn negated_field_test() {
	assert_eq!(parse_negated_field("!name"), Ok(("", PatternNode::NegatedField("name"))));
}

fn parse_capture_label<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
	preceded(tag("@"), parse_identifier)(i)
}

#[test]
fn parse_capture_label_test() {
	assert_eq!(parse_capture_label("@name"), Ok(("", "name")));
}

fn parse_directive_name<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
	preceded(
		tag("#"),
		terminated(
			alpha1, tag("!")
		)
	)(i)
}

#[test]
fn parse_directive_name_test() {
	assert_eq!(parse_directive_name("#set!"), Ok(("", "set")));
}
	
fn parse_directive_argument<'a>(i: &'a str) -> IResult<&'a str, DirectiveComponent> {
	map(
		preceded(multispace0, terminated(parse_capture_label, multispace0)),
		|name| DirectiveComponent::Capture(name)
	)(i)
}

fn parse_directive<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		pair(
			parse_directive_name,
			many_till(parse_directive_argument, tag(")"))
		),
		|(name, args)| PatternNode::Directive(name, args.0),
	)(i)
}

#[test]
fn parse_directive_test() {
	let directive_single_capture = PatternNode::Directive(
		"set",
		vec![
			DirectiveComponent::Capture("thing")
		]
	);

	assert_eq!(parse_directive("#set! @thing)"), Ok(("", directive_single_capture)));

	let directive_double_capture = PatternNode::Directive(
		"set",
		vec![
			DirectiveComponent::Capture("thing"),
			DirectiveComponent::Capture("thing")
		]
	);
	
	assert_eq!(parse_directive("#set! @thing @thing)"), Ok(("", directive_double_capture)));
}

fn parse_basic_node<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	let node_parser = alt((
		parse_name,
		parse_anonymous,
		parse_wildcard,
		parse_group
	));

	preceded(multispace0, terminated(node_parser, multispace0))(i)
}

fn parse_node_with_capture<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		tuple((parse_node_with_quantification, opt(parse_capture_label))),
		|(node, maybe_label)| {
			if let Some(label) = maybe_label {
				PatternNode::Capture(label, Box::new(node))
			} else {
				node
			}
		}
	)(i)
}

#[test]
fn parse_node_with_capture_test() {
	assert_eq!(parse_node_with_capture("name @thing"), Ok(("", PatternNode::Capture("thing", Box::new(PatternNode::Name("name"))))));
}

fn parse_zero_or_more<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		terminated(
			parse_basic_node,
			tag("*")
		),
		|node| PatternNode::ZeroOrMore(Box::new(node))
	)(i)
}

#[test]
fn parse_zero_or_more_test() {
	assert_eq!(parse_zero_or_more("name*"), Ok(("", PatternNode::ZeroOrMore(Box::new(PatternNode::Name("name"))))));
}

fn parse_one_or_more<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		terminated(
			parse_basic_node,
			tag("+")
		),
		|node| PatternNode::OneOrMore(Box::new(node))
	)(i)
}

#[test]
fn parse_one_or_more_test() {
	assert_eq!(parse_one_or_more("name+"), Ok(("", PatternNode::OneOrMore(Box::new(PatternNode::Name("name"))))));
}

fn parse_optional<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	map(
		terminated(
			parse_basic_node,
			tag("?")
		),
		|node| PatternNode::Optional(Box::new(node))
	)(i)
}

#[test]
fn parse_optional_test() {
	assert_eq!(parse_optional("name?"), Ok(("", PatternNode::Optional(Box::new(PatternNode::Name("name"))))));
}

fn parse_node_with_quantification<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	alt((
		parse_zero_or_more,
		parse_one_or_more,
		parse_optional,
		parse_basic_node
	))(i)
}

pub fn parse_pattern<'a>(i: &'a str) -> IResult<&'a str, PatternNode> {
	parse_node(i)
}
