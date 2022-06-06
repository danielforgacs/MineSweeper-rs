/*
☐☐☐☐☐
☐☐x☐☐
☐☐☐☐☐
☐☐xx☐
☐☐☐☐☐

☐111☐
☐1x1☐
☐2221
☐1xx1
☐1221
*/

use rand::prelude::*;

type RawField = Vec<Vec<usize>>;
type SolvedField = Vec<Vec<FieldType>>;

const WIDTH: usize = 7;
const HEIGHT: usize = 5;
const NEIGHBOURS: [(i32, i32); 9] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0), (0, 0), (1, 0),
    (-1, 1), (0, 1), (1, 1)
    ];

#[derive(Debug)]
enum FieldType {
    Empty,
    Mine,
    Touching(u8),
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Empty => f.write_str("\u{20dd}")?,
            FieldType::Mine => f.write_str("\u{229b}")?,
            FieldType::Touching(x) => f.write_str(&format!("{}", x))?,
        }
        Ok(())
    }
}

fn main() {
    let field = generate_field();

    for row in &field {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
    println!();

    let field = solve_field(field);

    assert_eq!(field.len(), HEIGHT);
    assert_eq!(field[0].len(), WIDTH);

    for row in field {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
}

fn generate_field() -> RawField {
    let mut rng = rand::thread_rng();
    let mut field: RawField = Vec::new();
    for _ in 0..HEIGHT {
        let mut row = Vec::new();
        for _ in 0..WIDTH {
            if rng.gen::<f32>() > 0.9_f32 {
                row.push(1);
            } else {
                row.push(0);
            }
        }
        field.push(row);
    }
    field
}

fn solve_field(field: RawField) -> SolvedField {
    let mut newfield: SolvedField = Vec::new();
    for y in 0..HEIGHT {
        let mut row: Vec<FieldType> = Vec::new();
        for x in 0..WIDTH {
            let field_value = field[y][x];
            if field_value == 1 {
                row.push(FieldType::Mine);
            } else {
                let mut mine_count = 0;
                for neighbour in NEIGHBOURS {
                        let (ny, nx) = (y as i32 + neighbour.0, x as i32 + neighbour.1);
                        if ny < 0 || ny > HEIGHT as i32 - 1 || nx < 0 || nx > WIDTH as i32 - 1 {
                            continue;
                        }
                        let (ny, nx) = (ny as usize, nx as usize);
                        if ny == y && nx == x {
                            continue;
                        }
                        if field[ny][nx] == 1 {
                            mine_count += 1;
                        }
                }
                if mine_count == 0 {
                    row.push(FieldType::Empty);
                } else {
                    row.push(FieldType::Touching(mine_count));
                }
            }
        }
        newfield.push(row);
    }
    newfield
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_field() {
    }
}
