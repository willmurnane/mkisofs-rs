extern crate byteorder;
extern crate chrono;

extern crate structopt;

use structopt::StructOpt;

use mkisofs::option::Opt;

fn main() {
    let mut opt = Opt::from_args();
    mkisofs::create_iso(&mut opt).unwrap();
}
