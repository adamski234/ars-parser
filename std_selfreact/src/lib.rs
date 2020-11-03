#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Required, the reaction to add, either the ID or the Unicode emoji itself (NOT the name)
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("selfreact"),
		parameters_required: vec![1],
	};
	return Box::into_raw(Box::new(std_selfdelete {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_selfdelete {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_selfdelete {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
	let reaction;
	if matcher.is_match(&parameter[0]) {
		//Guild reaction
		let emoji_id = serenity::model::id::EmojiId::from(parameter[0].parse::<u64>().unwrap());
		reaction = serenity::model::channel::ReactionType::from(emoji_id);
	} else {
		//Normal unicode reaction
		reaction = serenity::model::channel::ReactionType::Unicode(parameter[0].clone());
	}
	environment.reactions_to_add.push(reaction);
	return String::new();
}