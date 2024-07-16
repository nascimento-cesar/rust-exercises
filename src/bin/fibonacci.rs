use std::io;

fn main() {
    let mut number_index = String::new();

    loop {
        io::stdin()
            .read_line(&mut number_index)
            .expect("Failed to read line");

        let number_index: u16 = match number_index.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                eprintln!("Invalid number. Please enter a valid number.");
                continue;
            }
        };

        let fibonacci_number = match number_index {
            0 => 0,
            1 => 1,
            _ => find_fibonacci_number(number_index, 1, 0),
        };

        println!("{fibonacci_number}");
    }
}

fn find_fibonacci_number(mut number_index: u16, current_number: u32, previous_nummber: u32) -> u32 {
    if number_index == 2 {
        current_number + previous_nummber
    } else {
        number_index -= 1;
        find_fibonacci_number(
            number_index,
            current_number + previous_nummber,
            current_number,
        )
    }
}
