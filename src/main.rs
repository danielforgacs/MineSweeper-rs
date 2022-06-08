use rand::prelude::*;
use crossterm::event::{read, Event, KeyCode};
use crossterm::{QueueableCommand};
use crossterm::cursor::MoveTo;
use std::io::{stdout, Write};
use crossterm::{style::Print};

type RawField = [[usize; HEIGHT]; WIDTH];
type SolvedField = [[Cell; HEIGHT]; WIDTH];

// const WIDTH: usize = 18;
const WIDTH: usize = 5;
// const HEIGHT: usize = 9;
const HEIGHT: usize = 4;
const MINE_PROBABILITY: f32 = 0.9;
const NEIGHBOURS: [(i32, i32); 9] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0), (0,  0), (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

const EMPTY_CELL: &str = "\u{25a1}";
const MINE_CELL: &str = "\u{2623}";
const HIDDEN_CELL: &str = "\u{2981}";
const FLAGGED_CELL: &str = "\u{2620}";

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Mine,
    Touching(u8),
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Hidden,
    Shown,
    Marked,
}

#[derive(Clone, Copy)]
struct Cell {
    cell_type: CellType,
    state: CellState,
}

impl CellType {
    fn to_text(&self) -> String {
        match self {
            CellType::Empty => EMPTY_CELL.to_string(),
            CellType::Mine => MINE_CELL.to_string(),
            CellType::Touching(x) => format!("{}", x),
        }
    }
}

impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_text())?;
        Ok(())
    }
}

impl Cell {
    fn new() -> Self {
        Self { cell_type: CellType::Empty, state: CellState::Hidden }
    }
}

fn main() {
    let (field, mine_count) = generate_field();
    let field = solve_field(field);
    run(field, mine_count).unwrap();
}

fn run(mut field: SolvedField, mut mine_count: u32) -> crossterm::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let mut flags = mine_count.clone() as i32;
    let (mut sy, mut sx) = (0, 0);
    loop {
        stdout
            .queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?
            .queue(MoveTo(WIDTH as u16 + 2, 3))?
            .queue(Print(flags))?;
        // For debugging:
        stdout
            .queue(MoveTo(WIDTH as u16 + 2, 1))?
            .queue(Print(mine_count))?;
        for (y, row) in field.iter().enumerate() {
            let y = y as u16;
            for (x, cell) in row.iter().enumerate() {
                let x = x as u16;
                if y == sy && x == sx {
                    stdout.queue(crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { r: 80, g: 30, b: 20 }))?;
                }
                let current_cell = match cell.state {
                    CellState::Hidden => HIDDEN_CELL.to_string(),
                    CellState::Shown => cell.cell_type.to_text(),
                    CellState::Marked => FLAGGED_CELL.to_string(),
                };
                stdout
                    .queue(crossterm::cursor::MoveTo(y, x))?
                    .queue(Print(current_cell))?
                    .queue(crossterm::style::ResetColor)?;
                // For debugging:
                stdout
                    .queue(crossterm::cursor::MoveTo(y + 30, x))?
                    .queue(Print(cell.cell_type))?;
            }
        }
        stdout.flush()?;
        let event = read()?;
        let action_cell = &mut field[sy as usize][sx as usize];

        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
        if event == Event::Key(KeyCode::Right.into()) {
            if sy < WIDTH as u16 - 1 { sy += 1; }
        }
        if event == Event::Key(KeyCode::Left.into()) {
            if sy > 0 { sy -= 1; }
        }
        if event == Event::Key(KeyCode::Down.into()) {
            if sx < HEIGHT as u16 - 1 { sx += 1; }
        }
        if event == Event::Key(KeyCode::Up.into()) {
            if sx > 0 { sx -= 1; }
        }
        if event == Event::Key(KeyCode::Char('m').into()) {
            match action_cell.state {
                CellState::Hidden => {
                    action_cell.state = CellState::Marked;
                    flags -= 1;
                    if action_cell.cell_type == CellType::Mine {
                        mine_count -= 1;
                        if mine_count == 0 {
                            stdout
                                .queue(MoveTo(2, HEIGHT as u16 + 2))?
                                .queue(Print("FLAGGED ALL MINES - YOU WON!"))?
                                .flush()?;
                            break;
                        }
                    }
                },
                CellState::Marked => {
                    action_cell.state = CellState::Hidden;
                    flags += 1;
                    if action_cell.cell_type == CellType::Mine {
                        mine_count += 1;
                    }
                },
                _ => {},
            };
        }
        if event == Event::Key(KeyCode::Enter.into()) {
            match action_cell.cell_type {
                CellType::Empty => {
                    reveal_around_empty(&mut field, &(sy as i32), &(sx as i32))
                },
                CellType::Mine => {
                    stdout
                        .queue(MoveTo(2, HEIGHT as u16 + 2))?
                        .queue(Print("FOUND THE MINE - YOU LOST!"))?;
                    break;
                },
                CellType::Touching(_) => { action_cell.state = CellState::Shown; },
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    println!("\n\n");
    Ok(())

}

fn reveal_around_empty(field: &mut SolvedField, sy: &i32, sx: &i32) {
    if *sy < 0 || *sy > (WIDTH as i32) - 1 || *sx < 0 || *sx > (HEIGHT as i32) - 1 {
        return;
    }
    let cell = &mut field[*sy as usize][*sx as usize];
    if cell.state == CellState::Shown {
        return;
    }
    match cell.cell_type {
        CellType::Empty => {
            cell.state = CellState::Shown;
            reveal_around_empty(field, &(sy - 1), sx);
            reveal_around_empty(field, &(sy + 1), sx);
            reveal_around_empty(field, sy, &(sx - 1));
            reveal_around_empty(field, sy, &(sx + 1));
        },
        CellType::Touching(_) => cell.state = CellState::Shown,
        _ => {},
    }
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
                newfield[y][x].cell_type = CellType::Mine;
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
                    newfield[y][x].cell_type = CellType::Empty;
                } else {
                    newfield[y][x].cell_type = CellType::Touching(mine_count);
                }
            }
        }
    }
    newfield
}
