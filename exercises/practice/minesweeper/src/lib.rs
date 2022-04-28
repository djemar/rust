use std::io::Write;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut s: String= String::new();

    for (x, row) in minefield.iter().enumerate() {
        for (y, col) in row.as_bytes().iter().enumerate() {
            println!("{0} at {1},{2}", col, x, y);
            if *col == 32u8 {
                s.push(char::from(count_mines(x, y, minefield)));
            } else {
                s.push('*');
            }
        }
        result.push(String::from(&s));
        &s.clear();
    }
    result
}

fn count_mines(x: usize, y: usize, minefield: &[&str]) -> u8 {
    let x_min = if x == 0 { 0 } else { x - 1 };
    let x_max = if x == minefield.len() - 1 { minefield.len() - 1 } else { x + 1 };
    let y_min = if y == 0 { 0 } else { y - 1 };
    let y_max = if y == minefield[0].len() - 1 { minefield[0].len() - 1 } else { y + 1 };
    let mut count: u8 = 0;

    for i in x_min..=x_max {
        for j in y_min..=y_max {
            if *minefield[i].as_bytes().get(j).unwrap() == 42u8 {
                count += 1;
            }
        }
    }
    println!("Mines surrounding {0},{1} are: {2}", x, y, count);
    if count == 0 { 32u8 } else { 48u8 + count }
}