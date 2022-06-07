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
use crossterm::event::{read, Event, KeyCode};
use crossterm::{QueueableCommand, cursor};
use crossterm::cursor::MoveTo;
// use crossterm::Print;
use std::io::{stdout, Write};
// use std::io::{Write, };
use crossterm::{queue, style::Print};

type RawField = [[usize; HEIGHT]; WIDTH];
type SolvedField = [[Cell; HEIGHT]; WIDTH];

const WIDTH: usize = 18;
const HEIGHT: usize = 9;
const MINE_PROBABILITY: f32 = 0.97;
const NEIGHBOURS: [(i32, i32); 9] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0), (0,  0), (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

#[derive(Clone, Copy, PartialEq)]
enum FieldType {
    Empty,
    Mine,
    Touching(u8),
}

#[derive(Clone, Copy, PartialEq)]
enum FieldState {
    Hidden,
    Shown,
}

#[derive(Clone, Copy)]
struct Cell {
    cell_type: FieldType,
    state: FieldState,
}

impl FieldType {
    fn to_text(&self) -> String {
        match self {
            FieldType::Empty => "\u{25cb}".to_string(),
            FieldType::Mine => "\u{25cf}".to_string(),
            FieldType::Touching(x) => format!("{}", x),
        }
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_text())?;
        Ok(())
    }
}

impl Cell {
    fn new() -> Self {
        Self { cell_type: FieldType::Empty, state: FieldState::Hidden }
    }
}

fn main() {
    let (field, mine_count) = generate_field();

    for row in &field {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
    println!();

    let field = solve_field(field);

    assert_eq!(field.len(), WIDTH);
    assert_eq!(field[0].len(), HEIGHT);

    run(field, mine_count);
}

fn run(mut field: SolvedField, mut mine_count: u32) -> crossterm::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    // stdout.queue(crossterm::terminal::Clear{clea})
    let (mut sy, mut sx) = (0, 0);
    loop {
        stdout.queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
        for (y, row) in field.iter().enumerate() {
            let y = y as u16;
            for (x, cell) in row.iter().enumerate() {
                let x = x as u16;
                if y == sy && x == sx {
                    // stdout.queue(crossterm::style::SetBackgroundColor(crossterm::style::Color::Green))?;
                    // stdout.queue(crossterm::style::SetForegroundColor(crossterm::style::Color::Red))?;
                    stdout.queue(crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { r: 80, g: 30, b: 20 }))?;
                }
                let mut current_cell = cell.cell_type.to_text();
                if cell.state == FieldState::Hidden {
                    current_cell = "X".to_string()
                };
                stdout
                    .queue(crossterm::cursor::MoveTo(y, x))?
                    .queue(Print(current_cell))?
                    .queue(crossterm::style::ResetColor)?;
            }
            // println!();
        }
        stdout.flush()?;
        let event = read()?;
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
        if event == Event::Key(KeyCode::Right.into()) {
            if sy < WIDTH as u16 - 1 {
                sy += 1;
            }
        }
        if event == Event::Key(KeyCode::Left.into()) {
            if sy > 0 {
                sy -= 1;
            }
        }
        if event == Event::Key(KeyCode::Down.into()) {
            if sx < HEIGHT as u16 - 1 {
                sx += 1;
            }
        }
        if event == Event::Key(KeyCode::Up.into()) {
            if sx > 0 {
                sx -= 1;
            }
        }
        if event == Event::Key(KeyCode::Enter.into()) {
            field[sy as usize][sx as usize].state = FieldState::Shown;
            if field[sy as usize][sx as usize].cell_type == FieldType::Mine {
                stdout.queue(MoveTo(2, HEIGHT as u16 + 2))?
                    .queue(Print("FOUND THE MINE - YOU LOST!"))?;
                break;
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    println!("\n\n");
    Ok(())

}

fn generate_field() -> (RawField, u32) {
    let mut rng = rand::thread_rng();
    let mut field: RawField = [[0; HEIGHT]; WIDTH];
    let mut mine_count = 0;
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            if rng.gen::<f32>() > MINE_PROBABILITY {
                field[y][x] = 1;
                mine_count += 1;
            } else {
                field[y][x] = 0;
            }
        }
    }
    (field, mine_count)
}

fn solve_field(field: RawField) -> SolvedField {
    let mut newfield: SolvedField = [[Cell::new(); HEIGHT]; WIDTH];
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            if field[y][x] == 1 {
                newfield[y][x].cell_type = FieldType::Mine;
            } else {
                let mut mine_count = 0;
                for neighbour in NEIGHBOURS {
                    let (ny, nx) = (y as i32 + neighbour.0, x as i32 + neighbour.1);
                    if ny < 0 || ny > WIDTH as i32 - 1 || nx < 0 || nx > HEIGHT as i32 - 1 {
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
                    newfield[y][x].cell_type = FieldType::Empty;
                } else {
                    newfield[y][x].cell_type = FieldType::Touching(mine_count);
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
