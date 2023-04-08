use magic_packet_builder::{brodcast_address, create_magic_packet, send_magic_packet};
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
struct Opt {
    /// MAC address of the machine
    #[structopt(short, long)]
    mac: String,

    #[structopt(short, long, default_value = "5")]
    timeout: u64,

    #[structopt(short, long, default_value = "9")]
    broadcast_port: u16,
}

fn main() {
    let opt = Opt::from_args();

    let broadcast_addr = brodcast_address(Some(opt.broadcast_port)).unwrap_or_default();
    let packet = create_magic_packet(&opt.mac);
    match send_magic_packet(&packet, &broadcast_addr, opt.timeout) {
        Ok(_) => {
            print!("Done");
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            std::process::exit(1);
        }
    }
}
