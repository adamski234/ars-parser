#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;

pub fn safe_create() -> Box<dyn key_base::Key + Send + Sync> {
	return Box::new(std_sleep {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("sleep"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_sleep {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

unsafe impl Send for std_sleep {}
unsafe impl Sync for std_sleep {}

impl key_base::Key for std_sleep {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], _environment: &mut key_base::environment::Environment) -> Result<String, String> {
	match humantime::parse_duration(&parameter[0]) {
		Ok(result) => {
			std::thread::sleep(result);
			return Ok(String::new());
		}
		Err(error) => {
			return Err(format!("Invalid time passed to `sleep`: `{}`", error));
		}
	}
}