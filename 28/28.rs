fn main() {
    let size : usize = 1001;

    // Generate array
    let spiral_numbers = generate_spiral(size);
    //print_spiral(&spiral_numbers, size);

    // Calculate diagonal sum
    let sum = calculate_diagonal_sum(&spiral_numbers, size);

    println!("Diagonal sum: {}", sum);
}

fn generate_spiral(size: usize) -> std::vec::Vec<std::vec::Vec<i32>> {
    assert!(size > 0);

    let mut spiral_numbers : std::vec::Vec<std::vec::Vec<i32>> = vec![vec![0; size]; size];
    let mut current_position = ((size - 1) / 2, (size - 1) / 2);
    let mut last_direction = 'n';

    for i in 1 .. size * size + 1 {
        // Set value
        let (x, y) = current_position;
        spiral_numbers[x][y] = i as i32;

        // Set next field
        if last_direction == 'n' {
            if spiral_numbers[x + 1][y] == 0 {
                current_position = (x + 1, y);
                last_direction = 'e';
            } else {
                current_position = (x, y - 1);
            }
        } else if last_direction == 'e' {
            if spiral_numbers[x][y + 1] == 0 {
                current_position = (x, y + 1);
                last_direction = 's';
            } else {
                current_position = (x + 1, y);
            }
        } else if last_direction == 's' {
            if spiral_numbers[x - 1][y] == 0 {
                current_position = (x - 1, y);
                last_direction = 'w';
            } else {
                current_position = (x, y + 1);
            }
        } else if last_direction == 'w' {
            if spiral_numbers[x][y - 1] == 0 {
                current_position = (x, y - 1);
                last_direction = 'n';
            } else {
                current_position = (x - 1, y);
            }
        }
    } 
    
    return spiral_numbers;
}

fn calculate_diagonal_sum(spiral_numbers: &std::vec::Vec<std::vec::Vec<i32>>, size: usize) -> i64 {
    assert!(size % 2 == 1);

    let mut sum : i64 = 0;
    // Sum descending diagonal
    for i in 0 .. size {
        let (x, y) = (i, i);
        sum += spiral_numbers[x][y] as i64;
    }

    // Sum ascending diagonal
    for i in 0 .. size {
        let (x, y) = (0 + i, size - 1 - i);
        sum += spiral_numbers[x][y] as i64;
    }

    // Subtract doubly summed center
    sum -= spiral_numbers[(size - 1) / 2][(size - 1) / 2] as i64;

    return sum;
}

fn print_spiral(spiral_numbers: &std::vec::Vec<std::vec::Vec<i32>>, size: usize) {
    for y in 0 .. size {
        for x in 0 .. size {
            print!("{} ", spiral_numbers[x][y]);
        }

        println!("");
    }
}
