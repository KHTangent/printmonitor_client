use tokio::sync::oneshot;

use crate::error::PmResult;
use opencv::prelude::*;

#[derive(Debug)]
pub enum Command {
	GetPicture { resp: Responder<Mat> },
}

type Responder<T> = oneshot::Sender<PmResult<T>>;
