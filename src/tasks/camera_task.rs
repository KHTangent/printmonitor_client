use crate::camera_handler::CameraHandler;
use crate::command::Command;
use crate::error::PmError;
use tokio::sync::mpsc;

pub async fn camera_process(ch: &mut CameraHandler, mut rx: mpsc::Receiver<Command>) {
	while let Some(cmd) = rx.recv().await {
		match cmd {
			Command::GetPicture { resp } => {
				let jpeg = ch.get_jpeg();
				if let Ok(jpeg) = jpeg {
					let _ = resp.send(Ok(jpeg));
				} else {
					let _ = resp.send(Err(PmError::internal("Error getting picture")));
				}
			}
		}
	}
}
