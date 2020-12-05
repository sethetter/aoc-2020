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

    (locate(row_dirs, (0, 127)), locate(col_dirs, (0, 8)))
}

fn locate(dirs: Vec<char>, bounds: (usize, usize)) -> usize {
    let (x, _y) = dirs.iter().fold(bounds, |(x, y), c| {
        let half = ((y+1) - (x+1)) / 2;
        match c {
            'F' | 'L' => (x, y - half),
            'B' | 'R' => (x + half, y),
            _ => panic!("Invalid character encountered"),
        }
    });
    x
}

#[test]
fn test_seat_from_line() {
    assert_eq!(seat_from_line("BFFFBBFRRR"), (70, 7));
    assert_eq!(seat_from_line("FFFBBBFRRR"), (14, 7));
    assert_eq!(seat_from_line("BBFFBBFRLL"), (102, 4));
}