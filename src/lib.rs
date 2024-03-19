use aoko::no_std::pipelines::tap::Tap;
use rand::{rngs::ThreadRng, Rng};

pub fn print_random_ascii(tr: &mut ThreadRng, n: u16) {
    (0..n).for_each(|_| print!("{}", tr.gen_range(33u8..127) as char));
    println!();
}

fn gen_special(tr: &mut ThreadRng) -> char {
    tr.gen_range(33u8 ..= 47 + 7 + 6 + 4).tap_mut(|n| {
        if (48..=54).contains(n) {
            *n += 10; // bit shift 10
        }
        if (55..=60).contains(n) {
            *n += 36; // bit shift 36
        }
        if (61..=64).contains(n) {
            *n += 62; // bit shift 62
        }
    }) as char
}

#[test]
fn test_gen_special() {
    use aoko::assert_eqs;
    assert_eqs! {
        (48 + 10u8) as char, ':';
        (54 + 10u8) as char, '@';
        (55 + 36u8) as char, '[';
        (60 + 36u8) as char, '`';
        (61 + 62u8) as char, '{';
        (64 + 62u8) as char, '~';
    }
}

pub fn print_special_random_ascii(tr: &mut ThreadRng, n: u16) {
    (0..n).for_each(|_| print!("{}", gen_special(tr)));
    println!();
}

use clap::Parser;

/// Print random ascii
#[derive(Parser)]
#[clap(version = "0.0.0", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify generate ascii number
    #[clap()]
    pub n: u16,

    /// Generate special ascii
    #[clap(short)]
    pub s: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
