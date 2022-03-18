mod camera_handler;
mod error;
use camera_handler::CameraHandler;
use dotenv::dotenv;
use error::PmResult;
use opencv::core::Vector;
use opencv::imgcodecs::imwrite;
use opencv::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> PmResult<()> {
	// First, read and prepare settings
	dotenv().ok();
	let camera_index = env::var("CAMERA_INDEX").unwrap_or(String::from("0"));
	let camera_index =
		i32::from_str_radix(&camera_index, 10).expect("CAMERA_INDEX must be an integer");
	// Initialize our camera handler, for use later
	let mut ch = CameraHandler::new(camera_index).expect("Could not initialize camera.");
	// Some cameras don't really work right after turning on, so we take 10 pictures as a 
	// "warm-up", and discard them
	let mut frame = Mat::default();
	for _ in 0..10 {
		ch.get_frame(&mut frame).expect("Unable to capture image");
	}
	let sf = Vector::<i32>::new(); // Can be used for flags when saving, but we don't need any
	imwrite("./saved.jpg", &mut frame, &sf).unwrap();
	Ok(())
}
