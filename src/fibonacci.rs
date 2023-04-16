pub fn fibonacci_sequence_inclusive(index: i32) -> Vec<i32> {
    lucas_sequence_inclusive(0, 1, index)
}

pub fn fibonacci_seq_printed(index: i32) {
    println!("Fibonacci sequence until index=[{}]", index);

    fibonacci_sequence_inclusive(index)
        .iter()
        .for_each(|output| println!("{}", output));
}

pub fn lucas_sequence_inclusive(first_num: i32, second_num: i32, index: i32) -> Vec<i32> {
    if index < 0 {
        panic!("Target index for sequence cannot be smaller than 0!")
    }

    match index {
        0 => vec![first_num],
        1 => vec![first_num, second_num],
        _ => {
            let mut sequence = vec![first_num, second_num];

            for _ in 2..=index {
                let next_num = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];

                sequence.push(next_num);
            }

            sequence
        }
    }
}

pub fn lucas_sequence_seq_printed(first_num: i32, second_num: i32, index: i32) {
    println!(
        "Lucas sequence for first_num=[{}], second_num=[{}], and final index=[{}]",
        first_num, second_num, index
    );

    lucas_sequence_inclusive(first_num, second_num, index)
        .iter()
        .for_each(|output| println!("{}", output));
}
