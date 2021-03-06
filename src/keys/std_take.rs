#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::model::id::{UserId, RoleId};
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::{
		Environment,
		events,
	},
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_take {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("take"),
		parameters_required: vec![1, 2],
	};
}

#[allow(non_camel_case_types)]
struct std_take {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_take {}
unsafe impl Sync for std_take {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_take {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
		let guild_id = environment.guild_id;
		let user_id;
		if parameter.len() == 1 {
			match &environment.event_info {
				events::EventType::Message(event) => {
					user_id = event.user_id;
				}
				events::EventType::MemberJoin(event) => {
					user_id = event.user_id;
				}
				events::EventType::MemberUpdate(event) => {
					user_id = event.user_id;
				}
				events::EventType::VoiceUpdate(event) => {
					user_id = event.user_id;
				}
				events::EventType::ReactionAdd(event) => {
					user_id = event.user_id;
				}
				events::EventType::ReactionRemove(event) => {
					user_id = event.user_id;
				}
				_ => {
					return Err(String::from("`take` was called on an invalid event without an ID"));
				}
			}
		} else if matcher.is_match(&parameter[1]) {
				user_id = UserId::from(parameter[1].parse::<u64>().unwrap());
		} else {
			return Err(String::from("Invalid user ID passed to `take"));
		}
		match environment.discord_context.cache.guild(guild_id).await {
			Some(guild) => {
				let mut role_id;
				if matcher.is_match(&parameter[0]) {
					role_id = RoleId::from(parameter[0].parse::<u64>().unwrap());
					if !guild.roles.contains_key(&role_id) {
						//Safeguard against 18 characters long role names composed only of digits
						match guild.role_by_name(&parameter[0]) {
							Some(role) => {
								role_id = role.id;
							}
							None => {
								return Err(String::from("Could not find the role in `take`"));
							}
						}
					}
				} else {
					match guild.role_by_name(&parameter[0]) {
						Some(role) => {
							role_id = role.id;
						}
						None => {
							return Err(String::from("Could not find the role in `take`"));
						}
					}
				}
				match guild.member(&environment.discord_context.http, user_id).await {
					Ok(mut member) => {
						if let Err(error) = member.remove_role(&environment.discord_context.http, role_id).await {
							return Err(format!("Could not take the role in `take`: `{}`", error));
						}
					}
					Err(error) => {
						return Err(format!("Could not get the member in `take`: `{}`", error));
					}
				}
				return Ok(String::new());
			}
			None => {
				return Err(String::from("Could not find guild"));
			}
		}
	}
}