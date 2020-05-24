#![allow(unused_variables)]
/**
 * Compiles ARS into bytecode defined in docs/
 * # Arguments
 * * `ars_string` - string containing ARS code
 * # Returns
 * `Vec<u8>` containing compiled code
 */
pub fn compile_ars(ars_string: String) /*-> Vec<String>*/ { //Commented out to disable compiler errors
	let mut compiled = Vec::<u8>::new();
	let mut tree_from_ars = create_ars_tree(ars_string);
	println!("{:?}", tree_from_ars);
}

fn create_ars_tree(ars_string: String) -> Vec<ARSTreeItem> {
	let words = split_into_keys(ars_string); //Words is a Vec<String> containing first level parsed and split keys
	let mut top_level_items = Vec::<ARSTreeItem>::new();
	for word in words.iter() {
		if word.is_empty() {
			continue; //Disregard all empty strings
		}
		let split_parts_of_current_word: Vec<&str> = word.split(':').collect();
		let key = String::from(split_parts_of_current_word[0]); //First part of the content split by `:` is the key
		let param = String::from(split_parts_of_current_word[1..].join("")); //The rest of the content is considered to be the parameter
		let current_part = ARSTreeItem {
			key: ARSStringOrTree::Text(key),
			parameter: ARSStringOrTree::Text(param),
		};
		top_level_items.push(current_part);
	}
	for item in &top_level_items {
		item.parse_recursive();
	}
	return top_level_items;
}

/**
 * Splits the argument into a vector of keys
 * # Arguments
 * * `ars_string` - string containing ARS code enclosed in brackets
 * # Returns
 * `Vec<String>` containing all split keys
 */
fn split_into_keys(ars_string: String) -> Vec<String> {
	let mut current_word = String::new();
	let mut words = Vec::<String>::new();
	let mut opened_brackets: u8 = 0;
	let mut character_count = 0; //Fixes bug where unclosed brackets would get dropped
	//To prevent text before the first bracket from being suffixed to the content of the first bracket
	//Split the input into keys with brackets
	for current_char in ars_string.chars() {
		character_count += 1;
		if opened_brackets == 0 {
			if current_char == '{' {
				opened_brackets += 1;
				if !current_word.is_empty() {
					words.push(current_word.clone());
				}
				current_word = current_char.to_string();
			} else {
				current_word.push(current_char);
			}
		} else if opened_brackets == 1 {
			current_word.push(current_char);
			if current_char == '{' {
				opened_brackets += 1;
			} else if current_char == '}' {
				opened_brackets -= 1;
				if !current_word.is_empty() {
					words.push(current_word.clone());
				}
				current_word = String::new();
			}
		} else {
			current_word.push(current_char);
			if current_char == '{' {
				opened_brackets += 1;
			} else if current_char == '}' {
				opened_brackets -= 1;
			}
		}
		if character_count == ars_string.len() {
			if !current_word.is_empty() {
				words.push(current_word.clone());
			}
		}
	}
	if words.len() == 0 {
		words.push(ars_string);
	}
	return words;
}
#[derive(Debug)]
struct ARSTreeItem {
	key: ARSStringOrTree,
	parameter: ARSStringOrTree,
}

impl ARSTreeItem {
	pub fn parse_recursive(&self) {
		unimplemented!();
	}
}

#[derive(Debug)]
enum ARSStringOrTree {
	Text(String),
	Keys(Vec<ARSTreeItem>),
}

#[cfg(test)]
mod tests {
	mod split_into_keys_tests {
		use super::super::*;
		#[test]
		fn split_into_keys_correct() {
			//Tests for splitting a correctly formed string
			assert_eq!(
				split_into_keys(String::from("abc{def}{ghi:{jkm}}")),
				vec!["abc", "{def}", "{ghi:{jkm}}"]
			);
		}
		#[test]
		fn split_into_keys_unclosed_brackets_end() {
			//Tests for splitting a string with unclosed brackets at the end
			assert_eq!(split_into_keys(
				String::from("abc{{}")),
				vec!["abc", "{{}"]
			);
		}
	}
	mod create_ars_tree_tests {
		use super::super::*;
		#[test]
		fn create_ars_tree_correct() {
			//Tests for a correctly formed string
			//It needs to be replaced when the ars tree will be parsed to the end
			let tree = create_ars_tree(String::from("abc{def}{ghi:{jkm}}"));
			match &tree[0].key {
				ARSStringOrTree::Text(abc) => {
					assert_eq!(abc, "abc")
				}
				ARSStringOrTree::Keys(_) => {
					panic!("`ARSStringOrTree` was of variant `Keys`");
				}
			}
		}
	}
}
