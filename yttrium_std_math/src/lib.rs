//I spent more time on this single piece of junk than I spent on the interpreter
//Thanks, interop
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
#[allow(unused_imports)]
use cxx::{CxxString, UniquePtr};
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_math {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_math {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("math"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_math {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_math {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], _environment: &mut key_base::environment::Environment) -> Result<String, String> {
	#[allow(unused_unsafe)]
	return Ok(unsafe { ffi::calculate(&parameter[0]).to_str().unwrap().to_string() });
}

#[cxx::bridge]
mod ffi {
	unsafe extern "C++" {
		include!("yttrium_std_math/cpp/qalc.hpp");
		fn calculate(expression: &str) -> UniquePtr<CxxString>;
	}
}