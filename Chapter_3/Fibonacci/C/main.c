#include <stdio.h>

// Nth fibonacci sequence. Using the simple recursive approach.
unsigned int get_nth_fibonacci(unsigned int n) {
    if (n == 0) {
        return 0;
    }

    if (n == 1) {
        return 1;
    }

    return get_nth_fibonacci(n - 1) + get_nth_fibonacci(n - 2);
}

int main(void) {
    for (unsigned int n = 0; n < 20; n++) {
        unsigned int fibonacci_value = get_nth_fibonacci(n);
        printf("%2d: %d\n", n, fibonacci_value);
    }
}
