use async_std::io::BufReadExt;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use futures::stream::FuturesUnordered;
use std::time::Instant;

#[async_std::test]
async fn test_1000_parallel_requests() {
    // Запускаем сервер
    let server = task::spawn(async {
        http_server::run_server().await.unwrap();
    });

    // Ждём запуска
    task::sleep(std::time::Duration::from_millis(100)).await;

    let start = Instant::now();
    let mut tasks = FuturesUnordered::new();

    // Создаём 1000 параллельных задач
    for _ in 0..1000 {
        tasks.push(task::spawn(async {
            let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();

            // Отправляем запрос
            stream.write(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();

            // Читаем ответ
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response).await.unwrap();

            assert!(response.contains("200 OK"));
        }));
    }

    // Ждём завершения всех задач
    while let Some(_) = tasks.next().await {}

    let duration = start.elapsed();
    println!("1000 параллельных запросов: {:?}", duration);
    println!("Запросов в секунду: {:.0}", 1000.0 / duration.as_secs_f64());

    // Останавливаем сервер
    server.cancel().await;
}
