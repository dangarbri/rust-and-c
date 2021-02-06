# I don't get Rust.
Ok so I wrote the Rust program first. My main problem
was with trying to abstract an array access. I don't know
what's wrong with the code below, but it doesn't compile
and it's a quite confusing, so I continued without these
functions.

```Rust
const LYRICS_BY_DAYS: [(&str, &str); NUMBER_OF_VERSES] = [
    ("first", "and a partridge in a pear tree."),
    // ... other days
];

fn get_ordinal_for_day(day: usize) -> &'static str {
    return LYRICS_BY_DAYS[day].0;
}

fn get_gift_for_day(day: usize) -> &'static str {
    return LYRICS_BY_DAYS[day].1;
}
```

# Porting to C
I did get the program running, then I ported it over to C.
I had some interesting problems here. For the most
part it's actually quite easy to port over, until it isn't...

## No Tuples
C doesn't have tuples, and I probably shouldn't have used them.
For C, I implemented the lyrics array with a struct. This allowed
me to not even need to abstract the array access because
`LYRICS_BY_DAYS[day].ordinal` already looks good.

```C
struct DailyLyric {
    const char* ordinal;
    const char* lyrics;
};

// Store all lyrics as tuples
const struct DailyLyric LYRICS_BY_DAYS[] = {
    {"first", "and a partridge in a pear tree."},
    // ...
};
```

## Unsigned Countdown...
I had a bug in C that caused the program to crash. I was simply
porting the code by changing `fn` to `void` and changing `let` to
a real C type. Until I reached this point in Rust:

```Rust
let upper_limit_including_today = today + 1;
// Loop while printing the gift for every previous day
for day in (0..upper_limit_including_today).rev() {
    let gift = LYRICS_BY_DAYS[day as usize].1;
    // Print the gift for today
    println!("{}", gift);
}
```

Now intuitively, I just changed the first line to:

```C
uint8_t upper_limit_including_today = today + 1;
```

But then C doesn't have this loop structure:

```Rust
for day in (0..upper_limit_including_today).rev() {
```

So I had to change it over to a counting down for loop.
The code looked like this:

```C
uint8_t upper_limit_including_today = today + 1;
for (uint8_t day = upper_limit_including_today; day >= 0; day--) {
```

At this point, if you're used to programming at this level, you
will probably see the problem. But I was running along thinking
"these languages are so similar, it's so easy to port between them."
Until I executed the code...

If you don't see it, the code above is wrong for 2 reasons. One for
each line of code.

1. `today + 1` is incorrect.

In rust, it is correct because I
needed to be inclusive `(0..11)` does not include 11, so I needed to use `11 + 1` to include 11. Then I reverse the
collection so it's counting down. In C, it will become 12
which is out of bounds.

2. for (uint8_t day = upper_limit_including_today; day >= 0; day--)

I'm counting down to 0 on an **unsigned** integer and checking
that it's `>= 0`. It's ALWAYS going to be `>= 0`!

After fixing that loop, the code ran almost perfectly...

## VIRUS!?

When I tried to execute my program compiled in C, Windows would not let me...
I don't know what I did in the code that is making it show up as a virus.
Also if I compile with optimizations, then it's no longer detected as a virus.

![Windows Defender Threat Found](virus.png "Title")

# The Result

In the end, both programs produce the following output.

```
On the first day of christmas my true love sent to me
A partridge in a pear tree
-------------------------------
On the second day of christmas my true love sent to me
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the third day of christmas my true love sent to me
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the fourth day of christmas my true love sent to me
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the fifth day of christmas my true love sent to me
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the sixth day of christmas my true love sent to me
six geese a-laying,
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the seventh day of christmas my true love sent to me
seven swans a-swimming,
six geese a-laying,
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the eigth day of christmas my true love sent to me
eight maids a-milking,
seven swans a-swimming,
six geese a-laying,
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the ninth day of christmas my true love sent to me
nine ladies dancing,
eight maids a-milking,
seven swans a-swimming,
six geese a-laying,
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the tenth day of christmas my true love sent to me
ten lords a-leaping,
nine ladies dancing,
eight maids a-milking,
seven swans a-swimming,
six geese a-laying,
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the eleventh day of christmas my true love sent to me
eleven pipers piping,
ten lords a-leaping,
nine ladies dancing,
eight maids a-milking,
seven swans a-swimming,
six geese a-laying,
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
On the twelfth day of christmas my true love sent to me
twelve drummers drumming,
eleven pipers piping,
ten lords a-leaping,
nine ladies dancing,
eight maids a-milking,
seven swans a-swimming,
six geese a-laying,
five gold rings,
four calling birds,
Three French hens,
Two turtle doves,
and a partridge in a pear tree.
-------------------------------
```

