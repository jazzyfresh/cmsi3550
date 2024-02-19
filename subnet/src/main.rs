use std::env::args;

fn main() {
    // Get operating system args passed into this program
    let subnet_args: Vec<String> = args().collect();

    // Iterate through args with index
    for (i, arg) in subnet_args.iter().enumerate() {
        println!("Arg {i}: {arg}");
    }

    // // args() gives you an iterator already
    // // you don't have to collect into a vector
    // for (i, arg) in args().enumerate() {
    //     println!("Arg {i}: {arg}");
    // }
}
