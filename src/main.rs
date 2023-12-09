pub mod ascii_bros;
pub mod game;
pub mod actor;

use game::*;
use ascii_bros::*;
use actor::{Actor, sprite::Sprite};

use actor::sprite::tile::TILE_AREA;
use crossterm::{execute, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::event::KeyModifiers;
use crossterm::terminal::{self, Clear, ClearType, DisableLineWrap};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::style::Print;

use std::io::{stdout, Stdout, Write};
use std::time::{Duration, SystemTime};
use std::error::Error;
use std::thread;
use fnv::FnvHashMap;

pub struct Window {
    stdout_hndl: Stdout,
    height: u16,                // The height of the window being rendered (this is not necessarily equal to the height of the terminal)
    width: u16,                 // The width of the window being rendered (this is not necessarily equal to the width of the terminal)
    color_buf: Vec<Color>,      // Buffer containing color data for each pixel in pix_buf
    fg_frame_buf: Vec<char>,    // Buffer containing each glyph to be printed when rendering the foreground
    bg_frame_buf: Vec<char>,    // Buffer containing each glyph to be printed when rendering the background
}

impl Window {
    fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        Self {
            stdout_hndl: stdout(),
            height, width,
            color_buf: Vec::new(),
            fg_frame_buf: Vec::new(), 
            bg_frame_buf: Vec::new(),
        }
    }

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        execute!(
            self.stdout_hndl, 
            Clear(ClearType::All), 
            Hide, 
            DisableLineWrap
        )?;
        Ok(())
    }

    // TODO: Return errors
    fn render_frame(&mut self, game: &mut Game) {
        self.clear().expect("Could not execute clear");

        let mut render_batch = game.actor_list
            .iter()
            .filter(|actor| { 
                (actor.props.y_pos.round() as u16) < self.height &&
                actor.props.y_pos >= 0.0 &&
                (actor.props.x_pos.round() as u16) < self.width &&
                actor.props.x_pos >= 0.0
            })
            .collect::<Vec<&Actor>>();
        render_batch.sort_by(|a, b| a.props.sprite.z_order.cmp(&b.props.sprite.z_order));
        
        let stride = (TILE_AREA as f64).sqrt() as usize;

        for actor in render_batch {
            let sprite_x_origin = actor.props.x_pos.round() as usize;
            let sprite_y_origin = actor.props.y_pos.round() as usize;

            let mut tile_ids_iter = actor.props.sprite.tile_ids.iter();

            for y_offset in (sprite_y_origin..(sprite_y_origin + actor.props.sprite.height)).step_by(stride) {
                for x_offset in (sprite_x_origin..(sprite_x_origin + actor.props.sprite.width)).step_by(stride) {
                    // TODO: This is assuming TILE_AREA = 4. Make to be dynamic

                    let curr_tile_id = &tile_ids_iter.next().expect("Failed to get next tile id");
                    let x_offset: u16 = x_offset.try_into().unwrap();
                    let y_offset: u16 = y_offset.try_into().unwrap();

                    self.stdout_hndl.queue(MoveTo(x_offset, y_offset)).unwrap();
                    self.stdout_hndl.queue(
                        Print(game.tile_atlas[curr_tile_id].pix_buf[0] as char)
                    ).unwrap();

                    self.stdout_hndl.queue(MoveTo(x_offset + 1, y_offset)).unwrap();
                    self.stdout_hndl.queue(
                        Print(game.tile_atlas[curr_tile_id].pix_buf[1] as char)
                    ).unwrap();

                    self.stdout_hndl.queue(MoveTo(x_offset, y_offset + 1)).unwrap();
                    self.stdout_hndl.queue(
                        Print(game.tile_atlas[curr_tile_id].pix_buf[2] as char)
                    ).unwrap();

                    self.stdout_hndl.queue(MoveTo(x_offset + 1, y_offset + 1)).unwrap();
                    self.stdout_hndl.queue(
                        Print(game.tile_atlas[curr_tile_id].pix_buf[3] as char)
                    ).unwrap();
                }
            }
        }

        // Insert newlines at end of screen boundary
        for row in 0..(self.height - 1) {
            self.stdout_hndl.queue(MoveTo(self.width - 2, row)).unwrap();
            self.stdout_hndl.queue(Print('\n')).unwrap();
        } 

        self.stdout_hndl.flush().unwrap();
    }
}

fn main() {
    let mut win = Window::new();

    // Create a game object
    let mut game = Game {
        tile_atlas: FnvHashMap::default(),
        actor_list: Vec::new(),
        next_tile_id: 0,
    };
    
    // Enable raw mode
    terminal::enable_raw_mode().unwrap();

    /*** Game start ***/

    Game::on_start(&mut game);
    let mut delta_time: u128 = 0;

    let mut quit = false;

    /*** Game loop ***/

    while !quit {
        let delta_time_start = SystemTime::now();

        // Process events (non-blocking)
        if poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(k_evt) => {
                    match k_evt.code {
                        KeyCode::Char(c) => {
                            if c == 'c' && k_evt.modifiers.contains(KeyModifiers::CONTROL) {
                                quit = true;
                            } else {
                                print!("{}", c);
                            }
                        }, 
                        _ => todo!(),
                    }
                },
                Event::Resize(width, height) => {
                    win.width = width;
                    win.height = height;
                },
                _ => todo!(),
            }
        }

        // Call Game's on_update function to render the frame and so-forth
        Game::on_update(&mut game, &mut win, &delta_time);

        // Calculate how long everything took for this frame
        let delta_time_end = SystemTime::now();
        delta_time = delta_time_end.duration_since(delta_time_start).unwrap().as_millis();

        // Sleep for any spare time
        //let frame_delta = ((SECOND_IN_MILLIS / TARGET_FPS) - delta_time) as u32;
        //if frame_delta > 0 {
        //    thread::sleep(Duration::from_millis(delta_time));
        //}
        thread::sleep(Duration::from_millis(10));
    }

    terminal::disable_raw_mode().unwrap();
}
