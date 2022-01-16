use aoko::no_std::functions::ext::AnyExt1;
use rand::{thread_rng, Rng};
use std::io::stdin;

fn print_random_ascii(n: usize) {
    (0..n).for_each(|_| print!("{}", thread_rng().gen_range(33u8, 127) as char));
    println!();
}

fn main() {
    loop {
        println!("Please input the size you want to generate:");
        String::new()
            .also_mut(|s| stdin().read_line(s).unwrap())
            .trim_end().parse::<usize>().unwrap()
            .let_owned(|x| print_random_ascii(x));
        println!();
    }
}
