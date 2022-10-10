use serde::{Deserialize};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum NamedElement {
	#[serde(rename(deserialize = "SYMBOL"))]
	Symbol { name: String },
	#[serde(rename(deserialize = "PATTERN"))]
	Pattern { value: String },
	#[serde(rename(deserialize = "STRING"))]
	String { value: String },
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Grammar {
	name: String,
	word: Option<String>,
	extras: Vec<NamedElement>,
	conflicts: Vec<Vec<String>>,
	precedences: Vec<Vec<NamedElement>>,
	inline: Vec<String>,
	supertypes: Vec<String>,
}

pub fn parse_grammar<'a>(i: &'a str) -> Grammar {
	return serde_json::from_str(i).unwrap();
}

#[cfg(test)]
mod tests {
	use std::path::PathBuf;
	use std::fs;
	use super::{parse_grammar, NamedElement};

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
	fn read_basic_structure() {
		let data = r#"{
	"name": "abc",
	"extras": [],
	"conflicts": [],
	"precedences": [],
	"externals": [],
	"inline": [],
	"supertypes": []
}"#;

		let grammar = parse_grammar(&data);
		
		assert_eq!(grammar.name, "abc");
	}
	
	#[test]
	fn read_word_field() {
		let data = r#"{
	"name": "abc",
	"word": "def",
	"extras": [],
	"conflicts": [],
	"precedences": [],
	"externals": [],
	"inline": [],
	"supertypes": []
}"#;

		let grammar = parse_grammar(&data);
		
		assert_eq!(grammar.word, Some("def".to_string()));
	}
	
	#[test]
	fn read_inlines() {
		let data = r#"{
	"name": "abc",
	"word": "def",
	"extras": [],
	"conflicts": [],
	"precedences": [],
	"externals": [],
	"inline": ["a","b","c"],
	"supertypes": []
}"#;

		let grammar = parse_grammar(&data);
		
		assert_eq!(grammar.inline, vec!["a", "b", "c"]);
	}

	#[test]
	fn read_supertypes() {
		let data = r#"{
	"name": "abc",
	"word": "def",
	"extras": [],
	"conflicts": [],
	"precedences": [],
	"externals": [],
	"inline": [],
	"supertypes": ["a","b","c"]
}"#;

		let grammar = parse_grammar(&data);
		
		assert_eq!(grammar.supertypes, vec!["a", "b", "c"]);
	}
	
	#[test]
	fn read_conflicts() {
		let data = r#"{
	"name": "abc",
	"word": "def",
	"extras": [],
	"conflicts": [["a","b"], ["a","b","c"]],
	"precedences": [],
	"externals": [],
	"inline": [],
	"supertypes": []
}"#;

		let grammar = parse_grammar(&data);
		
		assert_eq!(grammar.conflicts, vec![vec!["a", "b"], vec!["a", "b", "c"]]);
	}

	#[test]
	fn read_precedences() {
		let data = r#"{
	"name": "abc",
	"word": "def",
	"extras": [],
	"conflicts": [],
	"precedences": [
		[
			{
				"type": "STRING",
				"value": "a"
			}
		],
		[
			{
				"type": "STRING",
				"value": "b"
			},
			{
				"type": "STRING",
				"value": "c"
			}
		]
	],
	"externals": [],
	"inline": [],
	"supertypes": []
}"#;

		let grammar = &parse_grammar(&data);
		
		let expected = vec![
			vec![
				NamedElement::String{value: "a".to_string()},
			],
			vec![
				NamedElement::String{value: "b".to_string()},
				NamedElement::String{value: "c".to_string()},
			],
		];

		assert_eq!(grammar.precedences, expected);
	}
}
