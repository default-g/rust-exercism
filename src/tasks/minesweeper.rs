const MOVEMENTS: [(i32, i32); 8] = [(-1, 1), (0, 1), (1,1), (-1,0), (1,0), (-1,-1), (0, -1), (1,-1)];

pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let rows = minefield.len() as i32;
    (0..rows).map(|y| {
        let cols = minefield[y as usize].len() as i32;
        (0..cols).map(|x| -> char {

            if minefield[y as usize].as_bytes()[x as usize] == b'*' {
                return '*'
            }

            let mines_count =  MOVEMENTS.iter()
                .map(|&(col_mov, row_mov)| (x + col_mov, y + row_mov))
                .filter(|&(x, y)| x >= 0 && x < cols && y >= 0 && y < rows)
                .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize] == b'*')
                .count();

            match mines_count {
                    0 => ' ',
                    n => char::from_digit(n as u32, 10).unwrap()
            }

        }).collect()
    }).collect()
}

