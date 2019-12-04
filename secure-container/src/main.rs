fn main() {
    let start = 123257;
    let end = 647015;

    let mut valid = Vec::<i32>::new();

    for password in start..=end {
        if contains_atleast_one_pair(password)
            && never_decreases(password)
            && contains_atleast_one_exact_pair(password)
        {
            valid.push(password);
        }
    }

    println!("Number of valid passwords: {}", valid.len());
}

fn contains_atleast_one_pair(password: i32) -> bool {
    let mut previous_char = '!';

    for c in password.to_string().chars() {
        if c == previous_char {
            return true;
        }
        previous_char = c;
    }

    false
}

fn contains_atleast_one_exact_pair(password: i32) -> bool {
    let mut previous_char = '!';
    let mut consecutive_count = 0;
    let mut consecutive_counts = Vec::<i32>::new();

    for c in password.to_string().chars() {
        if c == previous_char {
            consecutive_count += 1;
        } else {
            consecutive_counts.push(consecutive_count);
            consecutive_count = 1;
        }

        previous_char = c;
    }

    consecutive_counts.push(consecutive_count);

    for count in consecutive_counts {
        if count == 2 {
            return true;
        }
    }

    false
}

fn never_decreases(password: i32) -> bool {
    let mut previous_digit = 0;

    for c in password.to_string().chars() {
        let digit = c.to_digit(10).unwrap();

        if digit < previous_digit {
            return false;
        }

        previous_digit = digit;
    }

    true
}
