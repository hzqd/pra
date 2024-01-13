use pra::{get_args, print_random_ascii, print_special_random_ascii, Args};

fn main() {
    let Args { n, s } = get_args();

    match s {
        false => print_random_ascii(n),
        true => print_special_random_ascii(n),
    }
}
