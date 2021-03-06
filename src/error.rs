#![allow(dead_code)]

use std::fmt;

pub type PmResult<T> = Result<T, PmError>;

/// Custom error for PrintMonitor_client
#[derive(fmt::Debug)]
pub struct PmError {
	kind: PmErrorKind,
	message: String,
}

#[derive(fmt::Debug)]
enum PmErrorKind {
	Internal,
}

impl From<opencv::Error> for PmError {
	fn from(e: opencv::Error) -> Self {
		PmError {
			kind: PmErrorKind::Internal,
			message: e.message,
		}
	}
}

impl PmError {
	pub fn internal(e: impl ToString) -> PmError {
		PmError {
			kind: PmErrorKind::Internal,
			message: e.to_string(),
		}
	}
}
