# Mistakes!
I'm so used to C, I pretty messed up all the Rust. The code in the Rust folder
is correct now, but here's what my code looked like before I fixed it.
Can you count all the errors?

```Rust
// Convert temperatures from fahrenheit to celsius
fn fahrenheit_to_celsius(i32 fahrenheit_temperature) -> f32 {
    return (fahrenheit_temperature - 32) / 1.8;
}

fn main() {
    for fahrenheit in (-40..41) {
        f32 celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}°F is {}°C", fahrenheit, celsius);
    }
}
```

1. C-like variable definitions. f32 celsius is wrong. I forgot to use let. Even
in the fahrenheit_to_celsius function I declared the variable wrong

2. Rust doesn't simply let you do math with different types. So
`(fahrenheit_temperature - 32) / 1.8` was plain wrong.

3. Not an error, but I didn't need parentheses around (-40..41)

Just made me laugh a bit that I tried to write C in Rust and had a bad combination of the two.

## My C

And here is my first attempt at the same code in C. I wrote the Rust first,
so let's see. Do you see my mistakes here too?

```C
#include <stdio.h>

// Convert fahrenheit to celsius
float fahrenheit_to_celsius(int fahrenheit) {
    return (fahrenheit - 32) / 1.8;
}

int main() {
    for (int fahrenheit = -40; fahrenheit < 41; fahrenheit++) {
        float celsius = fahrenheit_to_celsius(fahrenheit);
        printf("%d°F is %d°C", fahrenheit, celsius);
    }
}
```

1. Yes, the compiler did warn me about this one, I wrote %d in my format string
for a float.

2. I alfo forgot the newline character in printf.

3. Also you can't use the degrees character in C. But I knew that, I just wanted
to try it. It looks like this in my terminal: `32┬░F is 0.000000┬░C`
