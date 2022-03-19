use crate::camera_handler::CameraHandler;
use crate::command::Command;
use crate::error::PmError;
use opencv::prelude::*;
use tokio;
use tokio::sync::mpsc;

pub async fn camera_process(ch: &mut CameraHandler, mut rx: mpsc::Receiver<Command>) {
	while let Some(cmd) = rx.recv().await {
		match cmd {
			Command::GetPicture { resp } => {
				let mut frame = Mat::default();
				let mat_res = ch.get_frame(&mut frame);
				if let Ok(_) = mat_res {
					let _ = resp.send(Ok(frame));
				} else {
					let _ = resp.send(Err(PmError::internal("Error getting picture")));
				}
			}
		}
	}
}
