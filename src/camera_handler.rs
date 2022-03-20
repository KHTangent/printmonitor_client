use crate::error::PmResult;
use opencv::core::Vector;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_V4L2};
use opencv::imgcodecs::imencode;

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

	/// Retrieves an encoded jpg from the camera
	/// 
	/// # Returns
	/// A vector of bytes representing the encoded jpg
	pub fn get_jpeg(&mut self) -> PmResult<Vec<u8>> {
		let mut mat = Mat::default();
		self.get_frame(&mut mat)?;
		let mut output = Vector::<u8>::new();
		let params = Vector::<i32>::new();
		imencode("jpg", &mat, &mut output, &params)?;
		Ok(output.to_vec())
	}

	/// Some cameras don't work well right after startup, so this function just
	/// takes a few pictures as "warm-up", and discards them
	pub fn warmup(&mut self) -> PmResult<()> {
		let mut frame = Mat::default();
		for _ in 0..10 {
			self.get_frame(&mut frame)?
		}
		Ok(())
	}
}
