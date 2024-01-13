use pra::{get_args, print_random_ascii, print_special_random_ascii, Args};
use rand::thread_rng;

fn main() {
    let (mut tr, Args { n, s }) = (thread_rng(), get_args());

    match s {
        false => print_random_ascii(&mut tr, n),
        true => print_special_random_ascii(&mut tr, n),
    }
}
