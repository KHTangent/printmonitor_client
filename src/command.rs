use tokio::sync::oneshot;

use crate::error::PmResult;

#[derive(Debug)]
pub enum Command {
	GetPicture { resp: Responder<Vec<u8>> },
}

type Responder<T> = oneshot::Sender<PmResult<T>>;
