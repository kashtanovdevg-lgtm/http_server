use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Прослушивание входящих TCP-соединений на локальном хост-порту 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Блокировать навсегда, обрабатывая каждый запрос, поступающий на этот IP-адрес
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Считывает первые 1024 байта данных из потока
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // Веб-сервер отвечает приветствием или возвращает 404 ошибку (страница не найдена)
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Записывает ответ обратно в поток и очищает поток,
    // чтобы гарантировать отправку ответа обратно клиенту
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
