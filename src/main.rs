mod direction;
mod game;
mod hamiltonian_matrix;
mod matrix;
mod point;
mod snake;
mod a_star;

use crate::game::Game;
use crossbeam::channel::{unbounded, TryRecvError};
use ncurses::*;
use std::thread;
use std::time::SystemTime;

fn setup_ncurses() {
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn main() {
    setup_ncurses();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    if max_x % 2 == 0 {
        max_x -= 1;
    }

    if max_y % 2 != 0 {
        max_y -= 1;
    }

    // let mut game = Game::new(1, 2, max_x-2, max_y-2);
    let mut game = Game::new(1, 2, 19 - 2, 16 - 2);

    let (trx, rev) = unbounded();

    thread::spawn(move || loop {
        let ch = getch();
        trx.send(ch).unwrap();

        thread::yield_now();
    });

    let mut running = true;
    let mut last_tick = SystemTime::now();

    while running {
        let current_mills = SystemTime::now()
            .duration_since(last_tick)
            .unwrap()
            .as_millis();

        if current_mills > game.tick_speed as u128
        {
            game.draw();

            if game.running {
                game.move_snake();

                if !game.tick() {
                    game.running = false;
                }
            }

            last_tick = SystemTime::now();
        }

        if rev.is_empty() {
            thread::sleep_ms(1);
            continue;
        }

        match rev.try_recv() {
            Ok(ch) => {
                if ch == KEY_F(1) {
                    running = false;
                }

                if ch == ' ' as i32 {
                    game.running = !game.running;
                }

                if ch == 'w' as i32 {
                    game.tick_speed += 1;
                }

                if ch == 's' as i32 {
                    game.tick_speed -= 1;
                }
            }

            Err(TryRecvError::Empty) => {
                println!("{}", "empty");
                running = false;
            }

            Err(TryRecvError::Disconnected) => {
                println!("{}", "disconnected");
                running = false;
            }
        }
    }

    endwin();
}
