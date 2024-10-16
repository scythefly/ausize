mod lan;

use protos::lan_proto as lp;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    env_logger::Builder::new().filter(None, log::LevelFilter::Debug).init();
    log::info!("Hello, world!");
    let request = lp::AllocateRequest {
        session_id: "123".to_string(),
        user_id: "123".to_string(),
        local_ip: "127.0.0.1".to_string(),
        remote_port: 123,
    };
    log::info!("request: {:?}", request);
    let addr = "[::1]:61001".parse().unwrap();
    let lan_service = lan::LanService::default();
    Server::builder()
        .add_service(lp::lan_server::LanServer::new(lan_service))
        .serve(addr)
        .await
        .unwrap();
}
