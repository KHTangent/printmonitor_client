use crate::error::PmResult;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_V4L2};

pub struct CameraHandler {
	cam: VideoCapture,
}

impl CameraHandler {
	pub fn new(camera_id: i32) -> PmResult<CameraHandler> {
		Ok(CameraHandler {
			cam: VideoCapture::new(camera_id, CAP_V4L2)?,
		})
	}

	pub fn get_frame(&mut self, frame: &mut Mat) -> PmResult<()> {
		self.cam.read(frame)?;
		Ok(())
	}
}
