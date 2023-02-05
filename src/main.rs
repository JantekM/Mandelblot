#![allow(unused_imports, non_snake_case)]

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize, Color},
    terminal::{enable_raw_mode, size, Clear, ClearType, ScrollUp, SetSize},
    Result,
};
use std::{
    io::{stdout, Write, stdin, Read},
    path::Iter, ops::{Add, Mul},
};
use colors_transform::{Hsl, Color as ctColor};

use clap::{arg, command, value_parser, ArgAction, Command};

const PIECE_SIZE: usize = 128 ;
const MAX_ITERS: u64 = 30;



#[derive(Copy, Clone)]
enum IterResult {
    MaxExceeded,
    Converges { num_iterations: u64, rest: f64 },
    Uncalculated,
}

struct ScreenPiece {
    //position path
    //numIterations
    //rest for exact color
    iter_result: Option<[IterResult; PIECE_SIZE * PIECE_SIZE]>,
    pos_x: f64,
    pos_y: f64,
    edge_size: f64,
}

impl ScreenPiece {
    fn new(x: f64, y: f64, size: f64)->ScreenPiece{
        let piece = ScreenPiece{
            iter_result: None,
            pos_x: x,
            pos_y: y,
            edge_size: size,
        };
        return piece;
    }
    fn populate(&mut self) {
        let mut iter_result = [IterResult::Uncalculated; PIECE_SIZE * PIECE_SIZE];
        let step: f64 = self.edge_size / PIECE_SIZE as f64;
        for i in 0..PIECE_SIZE {
            for j in 0..PIECE_SIZE {
                let x = self.pos_x + step * i as f64;
                let y = self.pos_y + step * j as f64;
                iter_result[i * PIECE_SIZE + j] = calculate(x, y);
            }
        }
        self.iter_result = Some(iter_result);
    }
}

fn calculate(x0: f64, y0: f64) -> IterResult {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let mut curr_mod: f64;
    let mut iteration: u64 = 0;
    let result: IterResult;
    loop {
        curr_mod = x * x + y * y;
        if curr_mod > 2.0 {
            result = IterResult::Converges {
                num_iterations: iteration,
                rest: curr_mod,
            };
            break;
        }
        if iteration > MAX_ITERS.into() {
            result = IterResult::MaxExceeded;
            break;
        }
        let xtemp = x * x - y * y + x0;
        y = 2.0 * x * y + y0;
        x = xtemp;
        iteration += 1;
    }
    return result;
}

fn print_screen(piece: &ScreenPiece) -> Result<()> {
    for i in 0..PIECE_SIZE{
        for j in 0..PIECE_SIZE{
            match piece.iter_result.unwrap()[i*PIECE_SIZE + j]{
                IterResult::Converges { num_iterations, rest } => {
                    queue!(
                        stdout(),
                        cursor::MoveTo(i as u16, j as u16),
                        style::PrintStyledContent("█".with(choose_color(num_iterations, rest)))
                    )?;
                },
                IterResult::MaxExceeded => {
                   queue!(
                        stdout(), 
                        cursor::MoveTo(i as u16, j as u16),
                        style::PrintStyledContent("█".black())
                    )?; 
                },
                IterResult::Uncalculated => panic!("Tried to print uncalculated values")
            }
        }
    }
   

    stdout().flush()?;
    Ok(())
}

fn choose_color(iters: u64, rest: f64) -> Color{
    let hue = (iters as f32 / MAX_ITERS as f32)*359.0;
    let _light: f64 = if iters == MAX_ITERS.into() {
        0.10
    } else { (rest - 4.0)
                    .log(10.0)
                    .min(0.5)
                    .add(0.5)
                    .mul(100.0)
    };
    let color = Hsl::from(hue.floor(), 100.0, 50.0);
    Color::Rgb { r: color.get_red().mul(255.0) as u8, 
        g: color.get_green().mul(255.0) as u8, 
        b: color.get_blue().mul(255.0) as u8 }
}

fn main() -> Result<()> {

    

    let mut piece = ScreenPiece::new(-1.5, -1.2, 2.5);
    piece.populate();


    let (_cols, _rows) = size()?;
    //Clear(ClearType::All);
    execute!(stdout(), SetSize(PIECE_SIZE as u16, PIECE_SIZE as u16))?;
    Clear(ClearType::All);
    enable_raw_mode()?;

    print_screen(&piece)?;
    //execute!(stdout(), SetSize(cols, rows))?;

    // execute!(stdout(), SetSize(cols, rows))?;
    //stdin().read(&mut [0]).unwrap();
    Ok(())
}
