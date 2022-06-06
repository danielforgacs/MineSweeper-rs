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

const WIDTH: usize = 7;
const HEIGHT: usize = 5;

#[derive(Debug)]
enum FieldType {
    Empty,
    Touching(u8),
    Mine,
}

type RawField = Vec<Vec<usize>>;
type SolvedField = Vec<Vec<FieldType>>;

fn main() {
    let field = generate_field();
    let field = solve_field(field);
}

fn generate_field() -> RawField {
    vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
    ]
}

fn solve_field(field: RawField) -> SolvedField {
    let mut newfield: Vec<Vec<FieldType>> = Vec::new();
    for y in 0..HEIGHT {
        let mut row: Vec<FieldType> = Vec::new();
        for x in 0..WIDTH {
            print!("{}", field[y][x]);
            row.push(FieldType::Touching(5));
        }
        println!("");
        newfield.push(row);
    }
    newfield
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_field() {
    }
}
