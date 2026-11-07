use qrcode::QrCode;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use tiny_http::{Response, Server};

pub struct SambungHpServer {
    pub pin: String,
    pub local_ip: String,
    pub port: u16,
    pub is_running: bool,
    pub qr_matrix: Arc<Mutex<Vec<Vec<bool>>>>,
}

impl SambungHpServer {
    pub fn new() -> Self {
        let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
        let pin = "8492".to_string();
        let port = 7878;

        let connect_url = format!("http://{}:{}/?pin={}", local_ip, port, pin);
        let matrix = generate_qr_matrix(&connect_url);
        let qr_arc = Arc::new(Mutex::new(matrix));

        let server_instance = Self {
            pin,
            local_ip,
            port,
            is_running: true,
            qr_matrix: qr_arc.clone(),
        };

        start_http_listener(port, qr_arc);
        server_instance
    }
}

fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

fn generate_qr_matrix(url: &str) -> Vec<Vec<bool>> {
    if let Ok(code) = QrCode::new(url.as_bytes()) {
        let width = code.width();
        let colors = code.into_colors();
        let mut matrix = vec![vec![false; width]; width];
        for y in 0..width {
            for x in 0..width {
                matrix[y][x] = colors[y * width + x] == qrcode::Color::Dark;
            }
        }
        matrix
    } else {
        vec![vec![false; 21]; 21]
    }
}

fn start_http_listener(port: u16, _qr: Arc<Mutex<Vec<Vec<bool>>>>) {
    thread::spawn(move || {
        let addr = format!("0.0.0.0:{}", port);
        if let Ok(server) = Server::http(&addr) {
            for request in server.incoming_requests() {
                let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VenomIsland Sambung HP</title>
    <style>
        body { font-family: monospace; background: #09090B; color: #FFFFFF; padding: 24px; text-align: center; }
        .box { border: 2px solid #BEF264; border-radius: 12px; padding: 20px; max-width: 400px; margin: auto; }
        input[type=file] { margin: 20px 0; color: #BEF264; }
        .btn { background: #BEF264; color: #09090B; font-weight: bold; border: none; padding: 12px 24px; border-radius: 6px; cursor: pointer; }
    </style>
</head>
<body>
    <div class="box">
        <h2 style="color:#BEF264;">VENOM // SAMBUNG HP</h2>
        <p>Terkoneksi ke PC Windows</p>
        <input type="file" id="fileInput" multiple><br>
        <button class="btn" onclick="alert('File terkirim ke Shelf VenomIsland!')">KIRIM KE PC</button>
    </div>
</body>
</html>"#;
                let response = Response::from_string(html).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap()
                );
                let _ = request.respond(response);
            }
        }
    });
}