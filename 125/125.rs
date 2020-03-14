use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() {
    find_palindromic_numbers_with_consecutive_squares_constructively(10000000);
    find_palindromic_numbers_with_consecutive_squares_by_number(10000000);
}

fn find_palindromic_numbers_with_consecutive_squares_constructively(excluding_upper_limit: i64) {
    let mut sum: i128 = 0;
    let mut found_numbers: HashSet<i64> = HashSet::new();
    let highest_possible_square = (excluding_upper_limit as f64).sqrt() as i64;

    // Non-including ranges, since at least two consecutive squares needed
    for current_range_start in 1 .. highest_possible_square {
        for current_range_end in current_range_start + 1 .. highest_possible_square {
            let mut retrieved_number: i64 = 0;
            for number in current_range_start ..= current_range_end {
                retrieved_number += number * number;
            }

            // Contain retrieved number if palindromic and in limit as well as not retrieved yet
            // (p.e. 554455 by 331..=335 and 9..=118 and 9343439 by 102..=307 and 657..=677)
            if retrieved_number < excluding_upper_limit
                    && is_palindromic(retrieved_number)
                    && !found_numbers.contains(&retrieved_number) {
                sum += retrieved_number as i128;
                found_numbers.insert(retrieved_number);
                println!("{} by {:?}", retrieved_number, (current_range_start ..= current_range_end));
            }

            // Abort if current range is already above limit
            if retrieved_number >= excluding_upper_limit {
                break;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn find_palindromic_numbers_with_consecutive_squares_by_number(excluding_upper_limit: i64) {
    let mut sum: i128 = 0;

    for current_number in 1 .. excluding_upper_limit {
        if is_palindromic(current_number) {
            let consecutive_squares = find_consecutive_squares(current_number);

            if consecutive_squares.start() < consecutive_squares.end() {
                sum += current_number as i128;
                println!("{} by {:?}", current_number, consecutive_squares);
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
