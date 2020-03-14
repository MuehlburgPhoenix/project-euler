use std::ops::RangeInclusive;

fn main() {
    let mut sum: i128 = 0;

    for current_number in 1 .. 10000000 {
        if is_palindromic(current_number) {
            let consecutive_squares = find_consecutive_squares(current_number);
            if consecutive_squares.start() < consecutive_squares.end() {
                sum += current_number as i128;
                println!("{} with {:?}", current_number, consecutive_squares);
            }
        }
    }

    println!("Sum: {}", sum);
}

fn is_palindromic(number: i64) -> bool {
    let text_number = number.to_string();
    let characters_to_check = text_number.len() / 2;

    // Check characters from outside to inside
    let mut is_palindromic = true;
    for i in 0 .. characters_to_check {
        if text_number.chars().nth(i).unwrap() != text_number.chars().nth(text_number.len() - 1 - i).unwrap() {
            is_palindromic = false;
        }
    }

    return is_palindromic;
}

fn find_consecutive_squares(number: i64) -> RangeInclusive<i64> {
    // Square can not be bigger than the square root, so start there downwards
    let highest_possible_square = (number as f64).sqrt() as i64;

    for upper_starting_number in (1 ..= highest_possible_square).rev() {
        let mut sum = 0;

        for current_number in (1 ..= upper_starting_number).rev() {
            sum += current_number * current_number;

            if sum == number {
                return current_number ..= upper_starting_number
            } else if sum > number {
                break;
            }
        }

        // If the sum is smaller than the number, lower upper starting numbers
        // cannot yield better results
        if sum < number {
            break;
        }
    }

    return 1 ..= 0;
}

// fn find_consecutive_squares_starting_at(number: i64, upperStartingNumber: i64) -> Option<i64> {
//     let mut lowerEndingNumber: Option<i64> = None;

//     let mut sum = 0;
//     for i in (1 .. upperStartingNumber + 1).rev() {
//         sum += i * i;

//         // Return lower ending number of the consecutive squared numbers
//         if sum == number {
//             lowerEndingNumber = Some(i as i64);
//             break;
//         } else if sum > number {
//             break;
//         }
//     }

//     return lowerEndingNumber;
// }
