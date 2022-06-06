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

type RawField = [[usize; WIDTH]; HEIGHT];
type SolvedField = [[FieldType; WIDTH]; HEIGHT];

const WIDTH: usize = 7;
const HEIGHT: usize = 5;
const NEIGHBOURS: [(i32, i32); 9] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0), (0,  0), (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

#[derive(Clone, Copy)]
enum FieldType {
    Empty,
    Mine,
    Touching(u8),
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Empty => f.write_str("\u{25cb}")?,
            FieldType::Mine => f.write_str("\u{25cf}")?,
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
    let mut field: RawField = [[0; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if rng.gen::<f32>() > 0.9_f32 {
                field[y][x] = 1;
            } else {
                field[y][x] = 0;
            }
        }
    }
    field
}

fn solve_field(field: RawField) -> SolvedField {
    let mut newfield: SolvedField = [[FieldType::Empty; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if field[y][x] == 1 {
                newfield[y][x] = FieldType::Mine;
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
                    newfield[y][x] = FieldType::Empty;
                } else {
                    newfield[y][x] = FieldType::Touching(mine_count);
                }
            }
        }
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
