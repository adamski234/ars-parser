#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use key_base::environment::events;
use key_base::databases::{
	DatabaseManager,
	Database,
};
use serenity::model::id::UserId;

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_ban {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Optional, reason, defaults to nothing
Optional, days to remove messages from, default 0
Optional, user id, defaults to the sender
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("ban"),
		parameters_required: vec![0, 1, 2, 3],
	};
}
#[allow(non_camel_case_types)]
struct std_ban<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_ban<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_ban<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_ban<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
	let guild_id = environment.guild_id.clone();
	let user_id;
	let mut days_to_remove = 0;
	if parameter.len() >= 2 {
		if let Ok(value) = parameter[1].parse() {
			days_to_remove = value;
		}
	}
	if parameter.len() == 3 && matcher.is_match(&parameter[2]) {
		user_id = UserId::from(parameter[2].parse::<u64>().unwrap());
	} else {
		match &environment.event_info {
			events::EventType::Message(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::MemberJoin(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::MemberUpdate(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::VoiceUpdate(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::ReactionAdd(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::ReactionRemove(event) => {
				user_id = event.user_id.clone();
			}
			_ => {
				return Err(String::from("`ban` called without user ID on invalid event"));
			}
		}
	}
	let member;
	match futures::executor::block_on(environment.discord_context.cache.member(guild_id, user_id)) {
		Some(memb) => {
			member = memb;
		}
		None => {
			return Err(String::from("User does not exist"));
		}
	}
	let result;
	if parameter.len() >= 2 {
		let reason = &parameter[0];
		result = futures::executor::block_on(member.ban_with_reason(&environment.discord_context.http, days_to_remove, reason));
	} else {
		result = futures::executor::block_on(member.ban(&environment.discord_context.http, days_to_remove));
	}
	if let Err(error) = result {
		return Err(error.to_string());
	}
	return Ok(String::new());
}