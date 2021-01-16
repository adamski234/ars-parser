#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use serenity::model::id::UserId;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::{
		Environment,
		events::*,
	},
};
pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_user {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Optional, chooses information to return, possible values: id, nickname, username, avatar, discriminator
Optional, user ID to target
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("user"),
		parameters_required: vec![2],
	};
}

#[allow(non_camel_case_types)]
struct std_user<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_user<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_user<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_user<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	let guild_id = environment.guild_id.clone();
	let user_id;
	if parameter.len() == 2 {
		let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
		if matcher.is_match(&parameter[1]) {
			user_id = UserId::from(parameter[1].parse::<u64>().unwrap());
		} else {
			return Err(String::from("Invalid user ID passed to `user`"));
		}
	} else {
		match &environment.event_info {
			EventType::MemberJoin(event) => {
				user_id = event.user_id.clone();
			}
			EventType::MemberLeave(event) => {
				user_id = event.user_id.clone();
			}
			EventType::MemberUpdate(event) => {
				user_id = event.user_id.clone();
			}
			EventType::Message(event) => {
				user_id = event.user_id.clone();
			}
			EventType::VoiceUpdate(event) => {
				user_id = event.user_id.clone();
			}
			EventType::ReactionAdd(event) => {
				user_id = event.user_id.clone();
			}
			EventType::ReactionRemove(event) => {
				user_id = event.user_id.clone();
			}
			_ => {
				return Err(String::from("`user` was called on an invalid event with no ID"));
			}
		}
	}
	match futures::executor::block_on(environment.discord_context.cache.member(guild_id, user_id)) {
		Some(user) => {
			match parameter[0].as_str() {
				"id" => {
					return Ok(user.user.id.to_string());
				}
				"nickname" => {
					match user.nick {
						Some(nick) => {
							return Ok(nick);
						}
						None => {
							return Ok(user.user.name);
						}
					}
				}
				"username" => {
					return Ok(user.user.name);
				}
				"avatar" => {
					match user.user.avatar_url() {
						Some(url) => {
							return Ok(url);
						}
						None => {
							return Ok(String::new());
						}
					}
				}
				"discriminator" => {
					return Ok(user.user.discriminator.to_string());
				}
				_ => {
					return Err(String::from("Invalid property passed to `user`"));
				}
			}
		}
		None => {
			return Err(String::from("Could not find the member in `user"));
		}
	}
}