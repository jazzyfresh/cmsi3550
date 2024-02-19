use std::env::args;

fn main() {
    // // Get operating system args passed into this program
    // let subnet_args: Vec<String> = args().collect();

    // Iterate through args with index
    for (i, arg) in args().enumerate() {
        println!("Arg {i}: {arg}");

        // subnet_args[0] is the name of the executable
        // subnet_args[1] the target ip address + cidr notation
        if i == 1 {
            calculate_subnet(arg);
        }
    }

    // // args() gives you an iterator already
    // // you don't have to collect into a vector
    // for (i, arg) in args().enumerate() {
    //     println!("Arg {i}: {arg}");
    // }
}

fn calculate_subnet(target_subnet: String) {
    // how to split strings
    let parts = target_subnet.split("/");
    for part in parts {
        println!("Subnet part: {part}");
    }

    // // how to split strings into named variables
    // let [target_ip, cidr]: [&str; 2] = target_subnet
    //     .split("/")
    //     .collect::<Vec<&str>>()
    //     .try_into()
    //     .unwrap_or_default();
    //
    // println!("Target IP: {target_ip}");
    // println!("CIDR: {cidr}");
}
