use std::env::args;

fn main() {
    let my_args: Vec<String> = args().collect();

    let ipv6_address = my_args[1].to_string();

    let hextets: [&str; 8] = ipv6_address
        .split(":")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();
    for hextet in hextets {
        println!("{hextet}");
    }
}
