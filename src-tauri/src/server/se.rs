use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = tokio::sync::mpsc::UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<String, Vec<Tx>>>>;

pub async fn run_websocket_server() {
    let state = PeerMap::default();
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let state = state.clone();
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            let room_id = "default".to_string(); // TODO: 解析真实 room_id
            let username = "anonymous".to_string(); // TODO: 解析真实 username
            let member_num = "0".to_string();

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            {
                let mut map = state.lock().unwrap();
                map.entry(room_id.clone()).or_default().push(tx.clone());
            }

            // 发送任务
            let send_task = tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    ws_sender.send(msg).await.ok();
                }
            });

            // 接收任务
            while let Some(Ok(msg)) = ws_receiver.next().await {
                // 假设收到的消息是文本且为json，解析后广播给房间内所有人
                if msg.is_text() {
                    let text = msg.into_text().unwrap();
                    // 尝试解析收到的json消息
                    if let Ok(json_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                        let map = state.lock().unwrap();
                        if let Some(peers) = map.get(&room_id) {
                            for peer in peers {
                                let _ = peer.send(Message::Text(json_msg.to_string().into()));
                            }
                        }
                    }
                }
            }

            send_task.abort();
            // 断开时移除
            let mut map = state.lock().unwrap();
            if let Some(peers) = map.get_mut(&room_id) {
                peers.retain(|p| !p.same_channel(&tx));
            }
        });
    }
}