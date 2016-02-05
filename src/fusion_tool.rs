extern crate rustc_serialize;
use rustc_serialize::json::*;

#[derive(RustcDecodable)]
pub struct FusionTool {
	pub number: u16,
	pub pocket: u16,
	pub description: String,
	pub diameter: f32,
}