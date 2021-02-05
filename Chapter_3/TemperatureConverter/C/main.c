#include <stdio.h>

// Convert fahrenheit to celsius
float fahrenheit_to_celsius(int fahrenheit) {
    return (fahrenheit - 32) / 1.8;
}

int main() {
    for (int fahrenheit = -40; fahrenheit < 41; fahrenheit++) {
        float celsius = fahrenheit_to_celsius(fahrenheit);
        printf("%dF is %fC\n", fahrenheit, celsius);
    }
}