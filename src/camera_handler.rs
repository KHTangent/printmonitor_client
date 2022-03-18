use crate::error::PmResult;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_V4L2};

/// Wrapper around the OpenCV camera struct, to make it easier to use the features we need
pub struct CameraHandler {
	cam: VideoCapture,
}

impl CameraHandler {
	/// Creates a new camera handler
	/// 
	/// # Arguments
	/// 
	/// * `camera_id` - OpenCV camera ID. Usually you want 0, to use the system default camera
	pub fn new(camera_id: i32) -> PmResult<CameraHandler> {
		Ok(CameraHandler {
			cam: VideoCapture::new(camera_id, CAP_V4L2)?,
		})
	}

	/// Gets a frame from the camera (i.e. takes a picture)
	/// 
	/// # Arguments
	/// 
	/// * `frame` - Mat struct to store the frame in
	pub fn get_frame(&mut self, frame: &mut Mat) -> PmResult<()> {
		self.cam.read(frame)?;
		Ok(())
	}
}
