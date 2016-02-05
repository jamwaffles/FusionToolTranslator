extern crate rustc_serialize;
use std::fmt;

#[derive(Debug)]
pub struct FusionTool {
	pub number: u16,
	pub description: String,
	pub cutter_type: String,
	pub diameter: f32,
}

impl fmt::Display for FusionTool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{}\t{}mm\t{} ({})", self.number, self.diameter, self.description, self.cutter_type)
	}
}