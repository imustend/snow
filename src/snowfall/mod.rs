use std::{
    fmt::{Display, Write},
    fs::File,
    io::{stdout, BufRead, BufReader, Write as ioWrite},
    path::Path,
};

use crossterm::{
    cursor,
    terminal::{self},
    ExecutableCommand,
};

pub struct Snowfall {
    pub field: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
    done: bool,
    current_iteration: u16,
    max_iteration: u16,
}

impl Snowfall {
    pub fn new(file: &Path, max_iteration: u16) -> Self {
        let mut field: Vec<Vec<char>> = vec![vec![]];

        let file = File::open(file.to_str().unwrap());
        let mut reader = BufReader::new(file.unwrap());
        let mut hw = String::new();
        let _ = reader.read_line(&mut hw);
        let mut hw = hw.split_whitespace();
        let height = hw.next().unwrap().parse::<usize>().unwrap();
        let width = hw.next().unwrap().parse::<usize>().unwrap();

        for (i, line) in reader.lines().enumerate() {
            field.push(vec![]);
            let line = line.unwrap();
            for c in line.chars() {
                field[i].push(c);
            }
        }

        Snowfall {
            field,
            height,
            width,
            done: false,
            current_iteration: 0,
            max_iteration,
        }
    }

    fn simulate_frame(&mut self) {
        if self.done || self.current_iteration >= self.max_iteration {
            print!("simulating frame even tho its done!!");
            return;
        }
        // set done to true, if the iteration is not done any change will overwirte it to false again
        self.done = true;

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if self.field[y][x] == '*' {
                    // removing the snow if its on the last line of the field
                    // its "falling off the screen"
                    if self.height == y {
                        self.field[y][x] = '.';
                        self.done = false;
                        continue;
                    }

                    // if the cell below the snow is empty move the snow to that cell
                    if self.field[y + 1][x] == '.' {
                        self.field[y][x] = '.';
                        self.field[y + 1][x] = '*';
                        self.done = false;
                        continue;
                    }
                }
            }
        }
    }

    pub fn simulate(&mut self) {
        let mut stdout = stdout();

        for i in 0..self.max_iteration {
            self.current_iteration = i;
            self.simulate_frame();

            std::thread::sleep(std::time::Duration::from_secs_f32(0.5));

            stdout
                .execute(cursor::MoveUp((self.height as u16) + 1))
                .unwrap();

            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
            let _ = write!(stdout, "{}", self);

            if self.done {
                println!("done");
                break;
            }
        }
    }
}

impl Display for Snowfall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(self.field[y][x])?;
            }
            f.write_char('\n')?;
        }
        let _ = f.write_fmt(format_args!("step: {}, \n", self.current_iteration));

        Ok(())
    }
}
