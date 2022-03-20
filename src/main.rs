mod camera_handler;
mod command;
mod error;
mod tasks;

use camera_handler::CameraHandler;
use dotenv::dotenv;
use error::PmResult;
use std::env;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use url::Url;

#[tokio::main]
async fn main() -> PmResult<()> {
	// First, read and prepare settings
	dotenv().ok();
	let camera_index = env::var("CAMERA_INDEX").unwrap_or(String::from("0"));
	let camera_index =
		i32::from_str_radix(&camera_index, 10).expect("CAMERA_INDEX must be an integer");
	let server_address = env::var("SERVER_ADDRESS").expect("Please provide a server address");
	let server_port = env::var("SERVER_PORT").unwrap_or(String::from("8889"));
	let server_port =
		i32::from_str_radix(&server_port, 10).expect("SERVER_PORT must be an integer");
	let ws_url = Url::parse(format!("ws://{}:{}", &server_address, server_port).as_ref())
		.expect("Invalid URL provided");
	// Initialize our camera handler, for use later
	let mut ch = CameraHandler::new(camera_index).expect("Could not initialize camera.");
	ch.warmup().expect("Unable to get pictures from camera");
	// Connect to server
	let (ws_stream, _) = connect_async(ws_url)
		.await
		.expect("Failed to connect to server");
	// Initialize tokio tasks
	let (tx, rx) = mpsc::channel(32);
	let manager_task = tokio::spawn(async move {
		tasks::camera_task::camera_process(&mut ch, rx).await;
	});
	let ws_task = tokio::spawn(async move {
		tasks::ws_task::ws_process(ws_stream, tx).await;
	});
	manager_task.await.unwrap();
	ws_task.await.unwrap();
	Ok(())
}
