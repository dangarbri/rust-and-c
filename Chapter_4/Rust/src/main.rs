// In chapter 3 I tried to create a function to return a value
// from the lyrics array because I didn't like how the default
// accessor syntax looked:
// LYRICS_BY_DAY[day].0/1
// Mainly the syntax is bad because of the tuple accessor, 0 and 1 have no
// meaning, so I wanted to provide functions to hide the poor syntax.
// I know the syntax could be improved by using structs instead of tuples,
// but I haven't learned the syntax for structs yet, that's chapter 5.
// In chapter 4 I learned about references and borrowing, which seemed to be
// the root of my problem with the functions, so I've come back to try
// to implement it now.

// Store all lyrics as tuples
const LYRICS_BY_DAYS: [(&str, &str); 12] = [
    ("first", "and a partridge in a pear tree."),
    ("second", "Two turtle doves,"),
    ("third", "Three French hens,"),
    ("fourth", "four calling birds,"),
    ("fifth", "five gold rings,"),
    ("sixth", "six geese a-laying,"),
    ("seventh", "seven swans a-swimming,"),
    ("eigth", "eight maids a-milking,"),
    ("ninth", "nine ladies dancing,"),
    ("tenth", "ten lords a-leaping,"),
    ("eleventh", "eleven pipers piping,"),
    ("twelfth", "twelve drummers drumming,")
];

// The compiler made me add static, but I still haven't learned about lifetimes.
fn get_ordinal(day: usize) -> &'static str {
    LYRICS_BY_DAYS[day].0
}

fn get_lyric(day: usize) -> &'static str {
    LYRICS_BY_DAYS[day].1
}

fn main() {
    let ordinal = get_ordinal(0);
    println!("Got ordinal {}", ordinal);

    let lyric = get_lyric(0);
    println!("Got lyric {}", lyric);
}