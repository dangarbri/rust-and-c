# Fail
I was so close on this one! I'm playing this little game with myself to get
the code right on the first try. But I still didn't get it even for this
simple problem. Here is my original C. I wrote it before writing the Rust.
Can you spot the error?

```C
#include <stdio.h>

// Nth fibonacci sequence. Using the simple recursive approach.
unsigned int get_nth_fibonacci(unsigned int n) {
    if (n == 0) {
        return 0;
    }

    if (n == 1) {
        return 1;
    }

    return n + get_nth_fibonacci(n - 1);
}

int main(void) {
    for (unsigned int n = 0; n < 20; n++) {
        unsigned int fibonacci_value = get_nth_fibonacci(n);
        printf("%2d: %d\n", n, fibonacci_value);
    }
}
```

1. The only error is `return n + get_nth_fibonacci(n - 1);`. Like what was I
thinking? I had the algorithm in a tab in a browser and somehow messed up, but
oh well. Caught it quickly, and hopefully won't make the same mistake in Rust.

## In Rust
Can you spot the error in this code?

```Rust
// Return the nth value in the fibonacci sequence
fn get_nth_fibonacci_value(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return get_nth_fibonacci_value(n - 1) + get_nth_fibonacci_value(n - 2);
}

fn main() {
    for n in 0..20 {
        let nth_value = get_nth_fibonacci_value(n);
        println!("{}: {}", n, nth_value);
    }
}
```

You can't because there aren't any! I did get this one on the first try, but the
code is pretty much identical to C, so it was easier. Moving forward I'll write
the Rust first, then see how it looks in C.