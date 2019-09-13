#![feature(plugin)]
#![plugin(roman_numerals)]

fn main() {
    println!("{}", rn!(MMXIX));
    assert_eq!(rn!(MMXIX), 2019);
}
