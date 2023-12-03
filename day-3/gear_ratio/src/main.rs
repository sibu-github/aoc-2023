fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let chars = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", get_sum_part_nos(&chars));
    println!("{}", get_sum_gear_ratio(&chars));
}

fn get_sum_gear_ratio(chars: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for (row, line) in chars.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '*' {
                let mut pair = vec![];
                if check_up(chars, row, col, CheckFor::Number) {
                    let n = get_num_at_pos(chars, row - 1, col);
                    pair.push(n);
                } else {
                    if check_up_left(chars, row, col, CheckFor::Number) {
                        let n = get_num_at_pos(chars, row - 1, col - 1);
                        pair.push(n);
                    }
                    if check_up_right(chars, row, col, CheckFor::Number) {
                        let n = get_num_at_pos(chars, row - 1, col + 1);
                        pair.push(n);
                    }
                }

                if check_left(chars, row, col, CheckFor::Number) {
                    let n = get_num_at_pos(chars, row, col - 1);
                    pair.push(n);
                }

                if check_right(chars, row, col, CheckFor::Number) {
                    let n = get_num_at_pos(chars, row, col + 1);
                    pair.push(n);
                }

                if check_down(chars, row, col, CheckFor::Number) {
                    let n = get_num_at_pos(chars, row + 1, col);
                    pair.push(n);
                } else {
                    if check_down_left(chars, row, col, CheckFor::Number) {
                        let n = get_num_at_pos(chars, row + 1, col - 1);
                        pair.push(n);
                    }
                    if check_down_right(chars, row, col, CheckFor::Number) {
                        let n = get_num_at_pos(chars, row + 1, col + 1);
                        pair.push(n);
                    }
                }

                if pair.len() > 2 {
                    println!("row {row} col {col} {:?}", pair);
                    panic!("more numbers found")
                }
                if pair.len() == 2 {
                    // println!("pair {} {}", pair[0], pair[1]);
                    let s = pair[0] * pair[1];
                    sum += s;
                }
            }
        }
    }
    sum
}

fn get_num_at_pos(chars: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let ch = chars[row][col];
    if !ch.is_numeric() {
        return 0;
    }
    let mut num = ch.to_digit(10).unwrap();
    let mut curr = col + 1;
    let mut multiplier = 10;
    loop {
        if curr < chars[0].len() {
            let ch = chars[row][curr];
            if ch.is_numeric() {
                let n = ch.to_digit(10).unwrap();
                num = (num * 10) + n;
                multiplier *= 10;
                curr += 1;
                continue;
            }
        }
        break;
    }
    if col > 0 {
        curr = col - 1;
        loop {
            let ch = chars[row][curr];
            if ch.is_numeric() {
                let n = ch.to_digit(10).unwrap();
                num = (n * multiplier) + num;
                multiplier *= 10;
            } else {
                break;
            }
            if curr == 0 {
                break;
            } else {
                curr -= 1;
            }
        }
    }

    num
}

enum CheckFor {
    Symbol,
    Number,
}

fn check_up(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if row == 0 {
        false
    } else {
        let ch = chars[row - 1][col];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}
fn check_up_left(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if row == 0 || col == 0 {
        false
    } else {
        let ch = chars[row - 1][col - 1];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}
fn check_left(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if col == 0 {
        false
    } else {
        let ch = chars[row][col - 1];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}
fn check_down_left(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if row == chars.len() - 1 || col == 0 {
        false
    } else {
        let ch = chars[row + 1][col - 1];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}
fn check_down(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if row == chars.len() - 1 {
        false
    } else {
        let ch = chars[row + 1][col];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}
fn check_down_right(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if row == chars.len() - 1 || col == chars[0].len() - 1 {
        false
    } else {
        let ch = chars[row + 1][col + 1];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}
fn check_right(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if col == chars[0].len() - 1 {
        false
    } else {
        let ch = chars[row][col + 1];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}
fn check_up_right(chars: &Vec<Vec<char>>, row: usize, col: usize, check_for: CheckFor) -> bool {
    if row == 0 || col == chars[0].len() - 1 {
        false
    } else {
        let ch = chars[row - 1][col + 1];
        match check_for {
            CheckFor::Number => ch.is_numeric(),
            CheckFor::Symbol => !ch.is_numeric() && ch != '.',
        }
    }
}

fn is_part_number(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    check_up(chars, row, col, CheckFor::Symbol)
        || check_up_left(chars, row, col, CheckFor::Symbol)
        || check_left(chars, row, col, CheckFor::Symbol)
        || check_down_left(chars, row, col, CheckFor::Symbol)
        || check_down(chars, row, col, CheckFor::Symbol)
        || check_down_right(chars, row, col, CheckFor::Symbol)
        || check_right(chars, row, col, CheckFor::Symbol)
        || check_up_right(chars, row, col, CheckFor::Symbol)
}

fn get_sum_part_nos(chars: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for (row, line) in chars.iter().enumerate() {
        let mut num = 0;
        let mut is_part = false;
        for (col, ch) in line.iter().enumerate() {
            if ch.is_numeric() {
                let n = ch.to_digit(10).unwrap();
                num = (num * 10) + n;
                if !is_part {
                    is_part = is_part_number(&chars, row, col);
                }
            } else {
                if is_part {
                    sum += num;
                }
                num = 0;
                is_part = false;
            }
        }
        if is_part {
            sum += num;
        }
    }
    sum
}
