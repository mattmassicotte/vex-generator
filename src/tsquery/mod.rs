pub mod basic;
pub mod parser;

#[cfg(test)]
mod tests {
	use std::path::PathBuf;
	use std::fs;
	use super::parser::{parse_pattern, PatternNode};

	fn resource_path(name: &str) -> PathBuf {
		let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		path.push("resources/test");
		path.push(name);
		
		return path;
	}
	
	fn resource_data(name: &str) -> String {
		let path = resource_path(name);
		
		return fs::read_to_string(path).expect("Unable to read file");
	}
	
	#[test]
	fn node_with_children() {
		let data = resource_data("node_with_children.scm");

		let expected = PatternNode::Group(
			vec![
				PatternNode::Name("binary_expression"),
				PatternNode::Group(
					vec![PatternNode::Name("number_literal")],
				),
				PatternNode::Group(
					vec![PatternNode::Name("number_literal")],
				),
			]
		);

		assert_eq!(parse_pattern(&data), Ok(("", expected)));
	}
}
