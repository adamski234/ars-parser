#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_parameter {
		info: create_key_info(),
	});
}


/*
Parameters:
Optional, the string to split on. If empty returns the entire parameter string
Required, the index of the split string to return
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("parameter"),
		parameters_required: vec![0, 2],
	};
}

#[allow(non_camel_case_types)]
struct std_parameter {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_parameter {}
unsafe impl Sync for std_parameter {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_parameter {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		if let key_base::environment::events::EventType::Message(event) = &mut environment.event_info {
			if parameter.is_empty() {
				return Ok(event.parameter.clone());
			} else {
				match parameter[1].parse::<usize>() {
					Ok(index) => {
						if !event.split_parameters.contains_key(&parameter[0]) {
							//This is hacky and I don't like this
							let split = event.parameter.split(&parameter[0]).map(String::from).collect();
							event.split_parameters.insert(parameter[0].clone(), split);
						}
						let split = event.split_parameters.get(&parameter[0]).unwrap();
						if split.len() > index {
							return Ok(split[index].clone());
						} else {
							return Err(format!("`parameter` split by `{}` didn't have `{}` elements", parameter[0], index + 1));
						}
					}
					Err(error) => {
						return Err(format!("Invalid number passed to `parameter`: `{}`", error.to_string()));
					}
				}
			}
		} else {
			return Err(String::from("`parameter` called on invalid event type"));
		}
	}
}