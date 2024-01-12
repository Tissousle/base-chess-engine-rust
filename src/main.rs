extern crate pleco;

use pleco::{Board, Player, BitMove};
use std::{io, time::Instant};

//const MINIMUM_EVAL: i32 = -2_147_483_647;
//const MAXIMUM_EVAL: i32 = 2_147_483_647;

struct Engine {
    board: Board,
    search_stopped: bool,
    active: bool,
    wtime: u32,
    btime: u32,
    movetime: u32,
    depth: u8,
    instant: Instant,
    nodes: u128,
}

impl Engine {

    fn new() -> Engine {
        Engine { 
            board: Board::start_pos(), 
            search_stopped: true, 
            active: true, 
            wtime: 0, 
            btime: 0, 
            movetime: 0, 
            depth: 255,
            instant: Instant::now(),
            nodes: 0,
        }
    }

    fn out_of_time(&self) -> bool {
        if &self.instant.elapsed().as_millis() > &self.movetime.into() {
            true
        }
        else {
            false
        }
    }

    fn re_initialize(&mut self) {
        self.wtime = 0;
        self.btime = 0;
        self.movetime = 0;
        self.nodes = 0;
    }
}

#[allow(unused)]
fn search(engine:&mut Engine,board:&mut Board) -> (BitMove, i32) {

    let moves = board.generate_moves();

    let random: i8 = rand::random();

    return (moves[random.rem_euclid(moves.len() as i8) as usize], random as i32)

}



fn start_search(engine:&mut Engine) {

    let mut shallow_board = (*engine).board.shallow_clone();
    
    let mut depth = 0;

    let mut best_move_info: (BitMove, i32) = (BitMove::null(), 0);

    (*engine).instant = Instant::now();

    let _perspective = {
        if (*engine).board.turn() == Player::White {1} else {-1}
    };

    while !(*engine).out_of_time() && depth < (*engine).depth {
        let past_best_move_info = best_move_info;

        depth += 1;

        best_move_info = search(engine, &mut shallow_board);

        if (*engine).out_of_time() {
            /*if past_best_move_info.1 * perspective > best_move_info.1 * perspective {
                best_move_info = past_best_move_info;
            }*/
            best_move_info = past_best_move_info;
        }

        println!("info depth {depth} time {} nodes {} score cp {} pv {}",(*engine).instant.elapsed().as_millis(),(*engine).nodes,best_move_info.1, best_move_info.0);

    }

    println!("bestmove {}", best_move_info.0);
}

#[allow(unused_assignments)]
fn com(text:&String, engine:&mut Engine) {
    let split_line = text.trim().split(" ");

    let lvec: Vec<&str> = split_line.collect();

    (*engine).re_initialize();

    match lvec[0] {
        "position" => {
            
            match lvec[1] {

                "startpos" => {
                    (*engine).board = Board::start_pos();
                    // to determine whether an input is "position startpos"
                    // or "position startpos moves xxxx xxxx"
                    let mut there_are_moves = false;

                    for word in lvec {

                        if word.trim() == "moves" {
                            there_are_moves = true;
                            continue;
                        }

                        if there_are_moves {
                            let success = (*engine).board.apply_uci_move(word.trim());
                            assert!(success);
                        }

                    }
                }
                    
                "fen" => {

                    let mut there_are_moves = false;

                    let mut fen_constructing = false;

                    let mut fen_string: String = String::new().to_owned();

                    for word in &lvec {

                        match word.trim() {

                            "fen" => {
                                fen_constructing = true;
                                continue;
                            }

                            "moves" => {
                                fen_constructing = false;
                                there_are_moves = true;
                                break;
                            }

                            _ => {
                                if fen_constructing {
                                    fen_string += word;
                                    fen_string += " ";
                                    continue;
                                } else {
                                    continue;
                                }
                            },

                        }
                    }

                    (*engine).board = Board::from_fen(&fen_string).unwrap_or_default();

                    if there_are_moves {
                        
                        let mut flag = false;

                        for word in &lvec {
                            if !flag {
                                match word.trim() {

                                    "moves" => {
                                        flag = true;
                                        continue;
                                    },
    
                                    _ => continue,
    
                                }
                            }
                            
                            let success = (*engine).board.apply_uci_move(word.trim());
                            assert!(success);

                        }
                        
                    }

                }

                _ => {
                    println!("Unknown command: {}", text.trim())
                }
            }
            
            
            
            
        }
        
        "go" => {

            for i in 1..lvec.len() {

                match lvec[i] {

                    "depth" => {
                        (*engine).depth = lvec[i+1].trim().parse::<u8>().unwrap_or_default(); 
                    }

                    "wtime" => {
                        (*engine).wtime = lvec[i+1].trim().parse::<u32>().unwrap_or_default();
                    }

                    "btime" => {
                        (*engine).btime = lvec[i+1].trim().parse::<u32>().unwrap_or_default();
                    }

                    "movetime" => {
                        (*engine).movetime = lvec[i+1].trim().parse::<u32>().unwrap_or_default();
                    }

                    _ => continue,

                }
            }

            (*engine).movetime = {
                if (*engine).movetime != 0 {
                    (*engine).movetime
                }
                else if ((*engine).wtime != 0) || ((*engine).btime != 0) {
                    if (*engine).board.turn() == Player::White {
                        10*f32::sqrt((*engine).wtime as f32) as u32
                    } else {
                        10*f32::sqrt((*engine).btime as f32) as u32
                    }
                }
                else {
                    8000
                }
            };


            (*engine).search_stopped = false;
            start_search(engine)
        }

        "d" => {
            (*engine).board.pretty_print()
        }
        "uci" => {
            println!("id name TissousleBot");
            println!("id author Tissousle");
            println!("");
            println!("option name Hash type spin default 16 min 1 max 4096");
            println!("uciok");
        },
        "isready" => 
            println!("readyok"),
        "ucinewgame" => 
            (),
        "stop" => 
            (*engine).search_stopped = true,
        "quit" =>
            (*engine).active = false,
        _ => 
            println!("Unknown command: {}", text.trim()),
    }
}

fn main() {
    println!("Hello, world!");

    let mut engine = Engine::new();

    while engine.active {

        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");

        com(&text, &mut engine);

    }

}
