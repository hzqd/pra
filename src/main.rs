use pra::{get_args, print_random_ascii, print_special_random_ascii, Args};
use rand::thread_rng;

fn main() {
    let (tr, Args { n, s }) = (&mut thread_rng(), get_args());

    match s {
        false => print_random_ascii(tr, n),
        true => print_special_random_ascii(tr, n),
    }
}
