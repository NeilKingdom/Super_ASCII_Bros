#![allow(non_snake_case)]

pub mod ascii_bros;
pub mod game;
pub mod actor;

use crate::game::*;
use crate::ascii_bros::*;
use crate::actor::{Actor, sprite::Sprite};

use crate::libc::STDIN_FILENO;
use crate::libc::ioctl;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::thread;
use std::io::{self, Read};
use nix::libc;
use std::sync::mpsc::channel;

#[derive(Default)]
struct ShellInfo {
    cols: usize,
    lines: usize,
}

impl ShellInfo {
    fn default() -> Self {
        // Initialize a winsize struct to store the terminal's column & line count
        let mut winsize = libc::winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        // Use the TIOCGWINSZ ioctl command to query the terminal dimensions
        if unsafe { ioctl(STDIN_FILENO, libc::TIOCGWINSZ, &mut winsize) } == -1 {
            panic!("Failed to retrieve terminal size");
        }

        ShellInfo { cols: winsize.ws_col as usize, lines: winsize.ws_row as usize }
    }
}

#[derive(Default)]
pub struct Window {
    height: usize,              // The height of the window being rendered (this is not necessarily equal to the height of the terminal)
    width: usize,               // The width of the window being rendered (this is not necessarily equal to the width of the terminal)
    color_buf: Vec<Color>,      // Buffer containing color data for each pixel in pix_buf
    fg_frame_buf: Vec<char>,    // Buffer containing each glyph to be printed when rendering the foreground
    bg_frame_buf: Vec<char>,    // Buffer containing each glyph to be printed when rendering the background
}

impl Window {
    fn default() -> Self {
        Window {
            height: 25,
            width: 80,
            color_buf: Vec::new(),
            fg_frame_buf: Vec::new(),
            bg_frame_buf: Vec::new(),
        }
    }

    fn clear(&mut self) {
        print!("{}", SCRN_CLR);       // Clear the screen
        print!("{}", CUR_POS_RST);    // Move cursor to position (0, 0)
        print!("{}", CUR_HIDE);       // Hide cursor
        print!("{}", LN_WRP_OFF);     // Disable line wrap

        self.color_buf.clear();
        self.fg_frame_buf.clear();
    }

    fn render_frame(&mut self, game: &mut Game) {
        self.clear();

        let screen_area = self.width * self.height;
        // TODO: Error handling
        self.fg_frame_buf.reserve_exact(screen_area);
        while self.fg_frame_buf.len() != self.fg_frame_buf.capacity() {
            self.fg_frame_buf.push(' ');
        }

        for actor in &game.actor_list {
            let sprite_y_offset = actor.y_pos.round() as usize;
            let sprite_x_offset = actor.x_pos.round() as usize;
            let sprite_offset = (sprite_y_offset * self.width) + sprite_x_offset; // Top-left corner of sprite
            let mut draw_pos = sprite_offset;

            /*** Render frame buffer ***/

            for tile_id in &actor.sprite.tile_ids {
                // TODO: This is assuming TILE_AREA = 4. Make to be dynamic
                let mut pixel = game.tile_atlas[&tile_id].pix_buf[0];
                self.fg_frame_buf[draw_pos] = pixel as char;

                pixel = game.tile_atlas[&tile_id].pix_buf[1];
                self.fg_frame_buf[draw_pos + 1] = pixel as char;

                pixel = game.tile_atlas[&tile_id].pix_buf[2];
                self.fg_frame_buf[draw_pos + self.width] = pixel as char;

                pixel = game.tile_atlas[&tile_id].pix_buf[3];
                self.fg_frame_buf[draw_pos + self.width + 1] = pixel as char;

                draw_pos += 2;

                if (draw_pos - sprite_offset) % 16 == 0 {
                    draw_pos += (self.width * 2) - 16;
                }
            }
        }

        // Insert newlines at end of screen boundary
        for idx in (self.width..screen_area).step_by(self.width) {
            self.fg_frame_buf[idx - 1] = '\n';
        }

        /*** Render pixel attributes ***/

        /*
        let col_pix_pairs: Vec<(Color, u8)> = ;
        for (color, pixel) in col_pix_pairs.iter() {
            print!("\x1b[0;{}m", color, pixel); // Reset attributes before printing color attribute and pixel glyph
        }
        */

        // TODO: Clone not ideal
        let output_str: String = self.fg_frame_buf.clone().into_iter().collect();
        print!("{}", output_str);
    }
}

fn main() -> ! {
    // Get $COLUMNS and $ROWS
    let shell_info: ShellInfo = ShellInfo::default();

    // Create a new window
    let mut win = Window::default();
    if win.height > shell_info.lines {
        panic!("Window is too small");
    }

    //win.width = shell_info.cols;
    //win.height = shell_info.lines;

    // Create a game object
    let mut game = Game {
        tile_atlas: HashMap::new(),
        actor_list: Vec::new(),
        next_tile_id: 0,
    };

    // Create a transmit/receive buffer for keyboard input
    let (tx, rx) = channel();

    // Get the current terminal settings
    let orig_termios = Termios::from_fd(0).expect("from_fd failed");

    // Spawn thread for polling keyboard input
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        // Disable canonical mode (line buffering) and echo
        let mut new_termios = orig_termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(0, TCSANOW, &new_termios).expect("tcsetattr failed");

        loop {
            // Read and process input
            let mut input = [0u8; 1];
            while handle.read_exact(&mut input).is_ok() {
                let c = input[0] as char;
                tx.send(c).expect("send failed");
            }
        }
    });

    /*** Game start ***/

    Game::on_start(&mut game);
    let mut delta_time = 0.0;

    /*** Game loop ***/

    loop {
        let delta_time_start = SystemTime::now();

        // Receive user input from channel (non-blocking)
        match rx.try_recv() {
            Ok(c) => {
                match KeyPress(c) {
                    KeyPress(' ') => println!("jump"),
                    KeyPress('a') => println!("left"),
                    KeyPress('d') => println!("right"),
                    // TODO: Remove (this is only for debug purposes)
                    KeyPress('q') => {
                        // Exit raw mode
                        tcsetattr(0, TCSANOW, &orig_termios).expect("tcsetattr failed");
                    },
                    _ =>  {}
                }
            },
            Err(_) =>  {}
        }

        // Call Game's on-update function to render the frame and so-forth
        Game::on_update(&mut game, &mut win, &delta_time);

        // Calculate how long everything took for this frame
        let delta_time_end = SystemTime::now();
        delta_time = delta_time_end.duration_since(delta_time_start).unwrap().as_millis() as f32;

        // Sleep for any spare time
        let frame_delta = ((SECOND_IN_MILLIS / TARGET_FPS) - delta_time) as u32;
        if frame_delta > 0 {
            thread::sleep(Duration::new(0, frame_delta));
        }
    }
}
