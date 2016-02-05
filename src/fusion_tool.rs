extern crate rustc_serialize;
use rustc_serialize::json::*;
use std::fmt;

#[derive(RustcDecodable, Debug)]
pub struct FusionTool {
	pub number: u16,
	pub pocket: u16,
	pub description: String,
	pub diameter: f32,
}

impl fmt::Display for FusionTool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Tool #{}\n    Number {}\n    {}\n    {}mm diameter", self.number, self.number, self.description, self.diameter)
	}
}