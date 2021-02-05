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
