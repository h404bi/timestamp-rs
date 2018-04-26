mod lib;

pub use lib::time;

fn main() {
    println!("{}", time());
    std::process::exit(0);
}
