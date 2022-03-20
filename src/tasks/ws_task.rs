use crate::command::Command;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{oneshot, mpsc};
use tokio::{self, net::TcpStream};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub async fn ws_process(
	mut ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
	tx: mpsc::Sender<Command>,
) {
	while let Some(msg) = ws.next().await {
		if let Ok(msg) = msg {
			match msg {
				Message::Text(txt) => {
					if txt.contains("picture") {
						let (otx, orx) = oneshot::channel();
						let _ = tx.send(Command::GetPicture { resp: otx }).await;
						let mat = orx.await.unwrap().unwrap();
						let _= ws.send(Message::Binary(mat));
					}
				},
				_ => {
					continue;
				}
			}
		} else {
			continue;
		}
	}
}
