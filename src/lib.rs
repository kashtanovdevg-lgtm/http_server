use async_std::fs;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task::spawn;
use futures::stream::StreamExt;

// Публичная функция, доступная из тестов
pub async fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Сервер запущен на http://127.0.0.1:7878");

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;

    Ok(())
}

// Публичная функция для обработки соединений
pub async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).await.unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
