use local_ip_address::local_ip;

fn main() {
    let ip = local_ip().unwrap();
    println!("Hello, IP! {}", ip);
}
