#![allow(unused_imports)]

use std::io::{stdout, Write};
use crossterm::{execute, Result, terminal::{ScrollUp, SetSize, size, enable_raw_mode, Clear, ClearType}, queue, cursor, style::{self, Stylize}};



fn print_screen(screen: &[[u32; 32]; 32])  -> Result<()>{
    for (i, row) in screen.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col==0{
                queue!(stdout(), cursor::MoveTo(i as u16, j as u16), style::PrintStyledContent( "█".magenta()))?;
            } else {
                queue!(stdout(), cursor::MoveTo(i as u16, j as u16), style::PrintStyledContent( "█".grey()))?;
            }
        }
    }
    
    stdout().flush()?;
    Ok(())

}

fn main()  -> Result<()>{

    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    
    let mut screen = [[0; WIDTH]; HEIGHT];
    screen[0][1] = 1;
    screen[2][3] = 1;
    screen[5][3] = 1;
    let (cols, rows) = size()?;
    //Clear(ClearType::All);
    execute!(
        stdout(),
        SetSize(32, 32)
    )?;
    //Clear(ClearType::All);
    //enable_raw_mode()?;
    
    print_screen(&screen)?;
    //execute!(stdout(), SetSize(cols, rows))?;
    
    
    // execute!(stdout(), SetSize(cols, rows))?;

    
    Ok(())
}
