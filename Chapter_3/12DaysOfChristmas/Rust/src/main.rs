// My solution is to store an array of tuples, each tuple will
// contain the "nth" day and the gift for that day.
// the program will loop through all 12 days
// On each day it will iterate through the list starting from the day
// and counting down towards 0 printing out all the gives given
// for that day.

const FIRST_DAY_LYRICS: &str = "A partridge in a pear tree";
const NUMBER_OF_VERSES: usize = 12;

// Store all lyrics as tuples
const LYRICS_BY_DAYS: [(&str, &str); NUMBER_OF_VERSES] = [
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

// Return "first", "second", "third..." I don't really like the syntax
// for accessing the tuple, so I'm hiding it in this function
// This code was supposed to hide the tuple access but I don't know how to implement it...
// I'm only on chapter 3 and I can't figure out how to get it to compile
// fn get_ordinal_for_day(day: usize) -> &'static str {
//     return LYRICS_BY_DAYS[day].0;
// }

// Returns the gift for that day of the song.
// Same reasoning as get_ordinal_for_day, using this function to hide
// ugly accessing of the tuple
// See comment above. For now I'm leaving the ugly tuple
// fn get_gift_for_day(day: usize) -> &'static str {
//     return LYRICS_BY_DAYS[day].1;
// }

// Print the traditional: On the nth day of christmas...
fn print_start_of_verse(day: u8) {
    let ordinal = LYRICS_BY_DAYS[day as usize].0;
    println!("On the {} day of christmas my true love sent to me", ordinal);
}

fn print_first_day_verse() {
    println!("{}", FIRST_DAY_LYRICS);
}

// Print the looping song!
fn print_verse_for_day(today: u8) {
    let is_first_day = today == 0;
    if is_first_day {
        // The very first day of the song doesn't say "and a partridge"
        // it just says "A partridge..." so it's a different string.
        print_first_day_verse();
    } else {
        let upper_limit_including_today = today + 1;
        // Loop while printing the gift for every previous day
        for day in (0..upper_limit_including_today).rev() {
            let gift = LYRICS_BY_DAYS[day as usize].1;
            // Print the gift for today
            println!("{}", gift);
        }
    }
}

// Prints a separator so we can see the output for each day.
fn print_separator() {
    println!("-------------------------------");
}

// Print the twelve days of christmas song
fn print_the_twelve_days_of_christmas() {
    for day in 0..12 {
        print_start_of_verse(day);
        print_verse_for_day(day);
        print_separator();
    }
}

fn main() {
    print_the_twelve_days_of_christmas();
}
