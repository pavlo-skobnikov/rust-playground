pub fn fizz_buzz(num: i32) -> Vec<String> {
    (1..=num)
        .map(|iteration| {
            let mut output = String::new();

            if iteration % 3 == 0 {
                output.push_str("Fizz");
            }
            if iteration % 5 == 0 {
                output.push_str("Buzz");
            }
            if iteration % 7 == 0 {
                output.push_str("Baz");
            }

            if output.is_empty() {
                iteration.to_string()
            } else {
                output
            }
        })
        .collect()
}

pub fn fizz_buzz_seq_printed(num: i32) {
    println!("FizzBuzz for {}:", num);

    fizz_buzz(num)
        .iter()
        .for_each(|output| println!("{}", output));
}
