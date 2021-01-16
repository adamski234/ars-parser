#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use futures::executor;
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
	return Box::new(std_pin {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("pin"),
		parameters_required: vec![0],
	};
}

#[allow(non_camel_case_types)]
struct std_pin {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_pin {}
unsafe impl Sync for std_pin {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_pin {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn run_key(&self, _parameter: &[String], environment: &mut Environment<Manager, DB>) -> Result<String, String> {
		if let events::EventType::Message(event) = &environment.event_info {
			let message_id = event.message_id.clone();
			let channel_id = event.channel_id.clone();
			match executor::block_on(environment.discord_context.cache.message(channel_id, message_id)) {
				Some(message) => {
					if let Err(error) = executor::block_on(message.pin(&environment.discord_context.http)) {
						return Err(format!("Could not pin message: `{}`", error));
					}
				}
				None => {
					return Err(String::from("Message could not be found"));
				}
			}
		}
		return Ok(String::new());
	}
}