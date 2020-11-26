#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use key_base::environment::events;
use futures::executor;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_pin {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_pin {
		info: create_key_info(),
		function: key_function,
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
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_pin {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(_parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
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