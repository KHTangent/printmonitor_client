mod camera_handler;
mod command;
mod error;
mod tasks;
use camera_handler::CameraHandler;
use dotenv::dotenv;
use error::PmResult;
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> PmResult<()> {
	// First, read and prepare settings
	dotenv().ok();
	let camera_index = env::var("CAMERA_INDEX").unwrap_or(String::from("0"));
	let camera_index =
		i32::from_str_radix(&camera_index, 10).expect("CAMERA_INDEX must be an integer");
	// Initialize our camera handler, for use later
	let mut ch = CameraHandler::new(camera_index).expect("Could not initialize camera.");
	ch.warmup().expect("Unable to get pictures from camera");
	// Initialize tokio tasks
	let (_tx, rx) = mpsc::channel(32);
	let manager_task = tokio::spawn(async move {
		tasks::camera_task::camera_process(&mut ch, rx).await;
	});
	manager_task.await.unwrap();
	Ok(())
}
