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
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
    ]
}

fn solve_field(field: RawField) -> SolvedField {
    for x in 0..5 {
        for y in 0..5 {
            print!("{}", field[x][y]);
        }
        println!("");
    }
    vec![vec![]]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_field() {
    }
}
