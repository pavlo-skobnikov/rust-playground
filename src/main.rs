mod fibonacci;
mod fizzbuzz;

fn main() {
    fizzbuzz::fizz_buzz_seq_printed(105);

    fibonacci::fibonacci_seq_printed(20);

    fibonacci::lucas_sequence_seq_printed(20, 23, 20);
}
