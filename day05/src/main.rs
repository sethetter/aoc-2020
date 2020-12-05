use std::{cmp, fs};

fn main() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let max_seat_id = lines.trim().split("\n")
        .map(seat_from_line)
        .map(|seat| (seat.0 * 8) + seat.1)
        .fold(0, |max, id| cmp::max(max, id));

    println!("Part 1: {}", max_seat_id);
}

fn seat_from_line(line: &str) -> (usize, usize) {
    let row_dirs: Vec<char> = line.chars().take(7).collect();
    let col_dirs: Vec<char> = line.chars().skip(7).collect();

    (locate(row_dirs, (0..128).collect()), locate(col_dirs, (0..8).collect()))
}

fn locate(dirs: Vec<char>, seats: Vec<usize>) -> usize {
    let result = dirs.iter().fold(seats, |s, c| {
        let half  = s.clone().into_iter().count() / 2;
        match c {
            'F' | 'L' => s.clone().into_iter().take(half).collect(),
            'B' | 'R' => s.clone().into_iter().skip(half).collect(),
            _ => panic!("Invalid character encountered"),
        }
    });
    result[0]
}

#[test]
fn test_seat_from_line() {
    assert_eq!(seat_from_line("BFFFBBFRRR"), (70, 7));
    assert_eq!(seat_from_line("FFFBBBFRRR"), (14, 7));
    assert_eq!(seat_from_line("BBFFBBFRLL"), (102, 4));
}