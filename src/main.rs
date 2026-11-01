use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}

impl Grid {
    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.cols) + x
    }

    fn fill_grid(&mut self, character: char) {
        let total_cells = self.get_index(self.cols, self.rows);
        self.data = vec![character; total_cells];
    }

    fn change_cell(&mut self, column: usize, row: usize, change_to: char) {
        let cell = self.get_index(column, row);
        self.data[cell] = change_to;
    }
}

fn main() {
    loop {
        let path_txt =
            std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("grid.txt");
        let path_json = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("grid.json");
        let mut grid: Grid = if !path_json.is_file() {
            first_launch()
        } else {
            get_grid_from_json(&path_json)
        };
        write_to_file(&grid, &path_txt, &path_json);
        let readable_grid =
            std::fs::read_to_string(&path_txt).expect("readable_grid failed to read path_txt!");
        println!("{}", readable_grid);
        println!(
            "\nHere's your grid! What do you want to do with it?\n1. Change one character\n2. Change all characters\n3. Delete\n4. Nothing!"
        );
        let opt = input();
        let opt: i32 = opt.trim().parse().expect("Not a valid option!");
        match opt {
            1 => {
                println!(
                    "Type in the row and column of the character you want to change, followed by the character you want to change it to.\nFormat:\nr3 c2 A\nc5 r3 ["
                );
                let input = parse_row_column_char();
                change_grid_cell(input, &mut grid);
            }
            2 => {
                println!("Type in the character you want to turn everything into!");
                let mut input = input();
                let input = input.remove(0);
                grid.fill_grid(input);
            }
            3 => {
                fs::remove_file(&path_json).expect("Couldn't delete file!");
                fs::remove_file(&path_txt).expect("Couldn't delete file!");
                return;
            }
            4 => {
                return;
            }
            _ => {
                println!("Unknown option!");
                continue;
            }
        }
        write_to_file(&grid, &path_txt, &path_json);
    }
}

fn input() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("WAH");
    string
}

fn get_grid_from_json(json_filepath: &PathBuf) -> Grid {
    let mut file_json = OpenOptions::new()
        .read(true)
        .open(json_filepath)
        .expect("get_grid_from_json failed to read file_json!");
    let grid: Grid = serde_json::from_reader(&mut file_json)
        .expect("get_grid_from_json let grid failed to read &mut file_json!");
    drop(file_json);
    grid
}

fn write_to_file(grid: &Grid, txt_filepath: &PathBuf, json_filepath: &PathBuf) {
    let mut file_txt = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(txt_filepath)
        .expect("TXT open error!");
    let mut file_json = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(json_filepath)
        .expect("JSON open error!");
    serde_json::to_writer_pretty(&mut file_json, grid).expect("Failed to write to file!");
    write!(
        file_txt,
        "\n--- GRID ---\nRows: {}\nColumns: {}\n",
        grid.rows, grid.cols,
    )
    .expect("Failed to write to file!");
    for rows in 0..grid.rows {
        for cols in 0..grid.cols {
            let index = grid.get_index(cols, rows);
            write!(file_txt, " {}", grid.data[index]).expect("Failed to write to file!");
        }
        writeln!(file_txt).expect("Failed to write to file!");
    }
    drop(file_txt);
    drop(file_json);
}

fn change_grid_cell(mut character: Vec<String>, grid: &mut Grid) {
    let mut change_to: char = '#';
    let mut x: usize = 1;
    let mut y: usize = 1;
    for i in 0..character.len() {
        if i < 2 {
            let identifier = character[i].remove(0);
            let number: usize = character[i].parse().expect("Invalid!");
            match identifier {
                'r' | 'R' => y = number,
                'c' | 'C' => x = number,
                _ => {
                    eprintln!("Invalid!");
                    std::process::exit(1);
                }
            }
        } else {
            change_to = character[i].remove(0);
        }
    }
    grid.change_cell(x, y, change_to);
}

fn parse_row_column() -> Vec<String> {
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("WAH");
    let size: Vec<String> = size.split_whitespace().map(String::from).collect();
    if size.len() > 2 {
        eprintln!("ERR: Too many arguments!");
        std::process::exit(1);
    }
    size
}
fn parse_row_column_char() -> Vec<String> {
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("WAH");
    let size: Vec<String> = size.split_whitespace().map(String::from).collect();
    if size.len() > 3 {
        eprintln!("ERR: Too many arguments!");
        std::process::exit(1);
    }
    size
}

fn first_launch() -> Grid {
    println!(
        "Looks like this is a first launch!
Type in the amount of Rows and Columns to start.
Format:
r3 c2
c5 r3"
    );
    let mut size = parse_row_column();
    let mut grid: Grid = Grid {
        rows: 0,
        cols: 0,
        data: vec!['0'],
    };
    for i in 0..size.len() {
        let identifier = size[i].remove(0);
        let number: usize = size[i].parse().expect("Invalid!");
        match identifier {
            'r' | 'R' => grid.rows = number,
            'c' | 'C' => grid.cols = number,
            _ => {
                eprintln!("Invalid!");
                std::process::exit(1);
            }
        }
    }
    if grid.rows == 0 || grid.cols == 0 {
        eprintln!("Rows or columns can't be 0!");
        std::process::exit(1);
    }
    grid.fill_grid('#');
    println!("Rows: {}\nColumns: {}", grid.rows, grid.cols);
    grid
}
