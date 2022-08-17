use go_echarger_api::{connection::http::DirectHttpChargerConnection, GoECharger};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // program name
    let hostname = args.next().expect("no hostname given");

    let goe = GoECharger::new(DirectHttpChargerConnection::new(hostname));
    println!("{:?}", goe.latest_status().await.unwrap());
}
