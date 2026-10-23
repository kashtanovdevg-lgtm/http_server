use async_std::io::BufReadExt;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

#[async_std::test]
async fn test_server_returns_hello() {
    // Запускаем сервер
    let server = task::spawn(async {
        http_server::run_server().await.unwrap();
    });

    // Ждём запуска
    task::sleep(std::time::Duration::from_millis(100)).await;

    let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();

    // Отправляем запрос
    stream.write(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();

    // Создаём BufReader и читаем ответ
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response).await.unwrap();

    // Проверяем
    assert!(response.contains("200 OK"));

    // Останавливаем сервер
    server.cancel().await;
}
