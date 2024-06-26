use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    listener.incoming().for_each_concurrent(None, |stream| async move {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        println!("Connection established: {}", addr);

        let mut websocket = tokio_tungstenite::accept_async(stream).await.unwrap();
        
        while let Some(msg) = websocket.next().await {
            match msg {
                Ok(message) => {
                    if message.is_text() || message.is_binary() {
                        websocket.send(message).await.unwrap();
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }).await;
}