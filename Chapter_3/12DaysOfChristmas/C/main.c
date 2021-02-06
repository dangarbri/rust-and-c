// Note, comment was written for Rust, but same principle applies.
// My solution is to store an array of tuples, each tuple will
// contain the "nth" day and the gift for that day.
// the program will loop through all 12 days
// On each day it will iterate through the list starting from the day
// and counting down towards 0 printing out all the gives given
// for that day.
#include <stdint.h>
#include <stdio.h>

// C compensation.
typedef char bool;

const char* FIRST_DAY_LYRICS = "A partridge in a pear tree";
const int NUMBER_OF_VERSES = 12;

// I probably shouldn't have used tuples in Rust. But
// I don't know the syntax for structured types. So I think
// this looks better than the tuples.
struct DailyLyric {
    const char* ordinal;
    const char* lyrics;
};

// Store all lyrics as tuples
const struct DailyLyric LYRICS_BY_DAYS[] = {
    {"first", "and a partridge in a pear tree."},
    {"second", "Two turtle doves,"},
    {"third", "Three French hens,"},
    {"fourth", "four calling birds,"},
    {"fifth", "five gold rings,"},
    {"sixth", "six geese a-laying,"},
    {"seventh", "seven swans a-swimming,"},
    {"eigth", "eight maids a-milking,"},
    {"ninth", "nine ladies dancing,"},
    {"tenth", "ten lords a-leaping,"},
    {"eleventh", "eleven pipers piping,"},
    {"twelfth", "twelve drummers drumming,"}
};

// Print the traditional: On the nth day of christmas...
void print_start_of_verse(uint8_t day) {
    const char* ordinal = LYRICS_BY_DAYS[day].ordinal;
    printf("On the %s day of christmas my true love sent to me\n", ordinal);
}

void print_first_day_verse() {
    printf("%s\n", FIRST_DAY_LYRICS);
}

// Print the looping song!
void print_verse_for_day(uint8_t today) {
    bool is_first_day = today == 0;
    if (is_first_day) {
        // The very first day of the song doesn't say "and a partridge"
        // it just says "A partridge..." so it's a different string.
        print_first_day_verse();
    } else {
        // C doesn't have the nice for-in syntax
        // I also had a bug here because I left "day" as an unsigned integer
        // and was counting down below 0.
        for (int8_t day = today; day >= 0; day--) {
            const char* gift = LYRICS_BY_DAYS[day].lyrics;
            // Print the gift for today
            printf("%s\n", gift);
        }
    }
}

// Prints a separator so we can see the output for each day.
void print_separator() {
    puts("-------------------------------");
}

// Print the twelve days of christmas song
void print_the_twelve_days_of_christmas() {
    for (uint8_t day = 0; day < NUMBER_OF_VERSES; day++) {
        print_start_of_verse(day);
        print_verse_for_day(day);
        print_separator();
    }
}

void main() {
    print_the_twelve_days_of_christmas();
}
