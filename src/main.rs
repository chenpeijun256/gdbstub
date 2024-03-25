mod bin_file;
mod mem;
mod perips;
mod config;
mod rv32_actor;
mod utils;
mod intrrupt;
mod gdbserver;

fn main() {
    let args:Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        gdbserver::server_start(&args[1]);
    } else {
        println!("Please input with following format:");
        println!("test file: zemulator filename.");
        println!("--------------------------------");
    }
}
