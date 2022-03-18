// This will eventually be Break, a game by Alex Abbott.
// To start, we'll just have a 12X12 board with 60 of the same piece for each player and you can move them around.
// Next step will be to be able to attack with them.
// Then implement soldier classes and attributes (Scout (S), Attacker (T), Archer (R), and Defender (D))
// Then implement the different Commander classes (Squad Commander (Q), Battalion Commander (B), Culture Commander (C), King Commander (K))
// Then implement dice
// And finally, the different cultures.

use std::io;            // Allows user input
use array2d::Array2D;


struct Piece {
    id:       String,
    attack:   u8,
    health:   i8,
    range:    u8,
    movement: u8,
    row:      u8,
    col:      u8,
}
impl Piece {
    fn get_player(&self) -> String {
        return self.id[..1].to_string();
    }
    fn get_map_code(&self) -> String {
        if self.get_player() == String::from("1"){
            return self.id[1..2].to_ascii_uppercase();
        }
        return self.id[1..2].to_ascii_lowercase();
    }
}

struct Game {
    pieces         : Vec<Piece>,
    player_one_name: String,
    player_two_name: String,
    player_one_turn: bool,
}

impl Game {
    fn setup(&mut self){
        // Put 24 player 1 pieces on the top 2 rows and 24 player 2 pieces on the bottom 2 rows.
        for piece_count in 0..48{
            let piece_count : u8 = piece_count;
            let id : String;
            // 1 signifying playerOne, s signifying soldier, and the pieceCount signifying its id.
            if piece_count / 24u8 == 0{
                id = String::from("1s");
            }else{
                id = String::from("2s");
            }
            let piece = Piece{
                id : id + piece_count.to_string().as_str(),
                attack: 1,
                health: 1,
                range: 1,
                movement: 1,
                col: piece_count % 12, // just keep walking along the rows
                // step up every 12 pieces, but also jump an extra 8 rows after the first 2 rows have been placed
                row: piece_count / 12 + (piece_count / 24) * 8
            };
            self.pieces.push(piece);
        }
    }
    fn run(&mut self){
        // Runs the whole game
        loop{
            // Display current board
            self.display_board();
            // Say whose turn it is
            if self.player_one_turn{
                println!("{}, it's your turn!", self.player_one_name)
            }else{
                println!("{}, it's your turn!", self.player_two_name)
            }
            // Take a turn.
            self.take_turn();
            
            // Check if the game is over
            let mut player_one_alive = false;
            let mut player_two_alive = false;
            for piece in &self.pieces{
                if piece.get_player() == "1"{
                    player_one_alive = true;
                }else if piece.get_player() == "2"{
                    player_two_alive = true;
                }
                if player_one_alive && player_two_alive{
                    break;
                }
            }
            if !player_one_alive{
                println!("{} wins!", self.player_two_name);
                return;
            }else if !player_two_alive{
                println!("{} wins!", self.player_one_name);
                return;
            }
            // Next player.
            self.player_one_turn = !self.player_one_turn;
        }
    }
    fn take_turn(&mut self){
        loop{
            // Get the piece that will be moved.
            let source_piece_index: usize;
            loop{
                let row: u8;
                let col: u8;
                print!("Which piece would you like to move? ");
                (row, col) = self.get_user_coords();
                // Get the piece at that location.
                let piece_index = self.index_of_piece_at_coords(row, col);
                if piece_index == -1{
                    println!("No piece at location.");
                    continue;
                }
                let piece = &self.pieces[piece_index as usize];

                // Make sure it's their piece.
                if (piece.get_player() == "1") != self.player_one_turn{
                    println!("You don't own that piece.");
                    continue;
                }

                // Everything should be good! Use that index.
                source_piece_index = piece_index as usize;
                break;
            }
            // We have the piece, now ask the user where it goes.
            let row: u8;
            let col: u8;
            print!("Where would you like to move it? ");
            (row, col) = self.get_user_coords();

            // See if we have enough movement
            {
                let piece_to_move = &self.pieces[source_piece_index];
                let horiz_move : i8 = piece_to_move.row as i8 - row as i8;
                let vert_move : i8 = piece_to_move.col as i8 - col as i8;
                if piece_to_move.movement < (horiz_move.abs() + vert_move.abs()) as u8{
                    // Nope. Go back to picking a piece.
                    println!("Not enough movement to go there.");
                    continue;
                }
            }
            // See if there's a piece at that location
            let piece_index = self.index_of_piece_at_coords(row, col);
            if piece_index == -1{
                // No piece, just move there.
                self.pieces[source_piece_index].row = row;
                self.pieces[source_piece_index].col = col;
                return;
            }else{
                // Check if the piece there is friendly
                let target_piece = &self.pieces[piece_index as usize];
                if (target_piece.get_player() != "1") == self.player_one_turn{
                    // It's an enemy piece, take it out.
                    // Swap_remove is much faster than remove, but does not preserve order.
                    println!("Captured piece '{}'", target_piece.id);
                    self.pieces.swap_remove(piece_index as usize);
                    self.pieces[source_piece_index].row = row;
                    self.pieces[source_piece_index].col = col;
                    return;
                } else{
                    // it's a friendly, so we can't go there. Go back to piece selection.
                    println!("That's a friendly piece!");
                    continue;
                }
            }
        }
    }

    fn display_board(&self){
        let blank : String = String::from(" ");
        let mut board = Array2D::filled_with(blank, 12, 12);
        // Populate the board
        for piece in &self.pieces{
            board[(piece.row as usize, piece.col as usize)] = piece.get_map_code();
        }
        // Print the board
        println!("    ! @ # $ % ^ & * ( ) _ + ");
        println!("   -------------------------");
        let mut row_count = 0;
        for row_iter in board.rows_iter() {
            // The row indicator
            if row_count < 9{
                print!("{}  ", row_count + 1);
            }else if row_count == 9{
                print!("0  ");
            }else if row_count == 10{
                print!("-  ");
            }else if row_count == 11{
                print!("=  ");
            }else{
                print!("ERROR");
            }
            // And all the stuff in the row
            for square in row_iter {
                print!("|{}", square);
            }
            println!("|");
            println!("   -------------------------");
            row_count += 1;
        }
        print!("\n");
    }

    fn decode_coords(&self, coords: String) -> (i8, i8){
        // Turns a string of coordinates into a (row, col) tuple
        if coords.len() != 2{
            return (-1, -1);
        }
        let col = match &coords[..1] {
            "!" => 0,
            "@" => 1,
            "#" => 2,
            "$" => 3,
            "%" => 4,
            "^" => 5,
            "&" => 6,
            "*" => 7,
            "(" => 8,
            ")" => 9,
            "_" => 10,
            "+" => 11,
             _  => -1 
        };
        let row = match &coords[1..] {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            "6" => 5,
            "7" => 6,
            "8" => 7,
            "9" => 8,
            "0" => 9,
            "-" => 10,
            "=" => 11,
             _  => -1
        };
        return (row, col);
    }

    fn get_user_coords(&self) -> (u8, u8){
        let mut row : i8;
        let mut col : i8;
        loop{
            // Ask for a position
            println!("Enter a pair of coordinates (ex $8): ");
            let mut coords = String::new();
            io::stdin()
            .read_line(&mut coords)
            .expect("Failed to read line");
            coords = coords.trim().to_string();

            // Decode the position.
            (row, col) = self.decode_coords(coords);
            if row == -1 || col == -1{
                println!("Invalid coordinates.");
                continue;
            }
            break;
        }
        return (row as u8, col as u8);
    }

    fn index_of_piece_at_coords(&self, row : u8, col : u8) -> i8{
        let mut index :i8 = -1;
        // Try to find the index of the piece at that place.
        for piece in &self.pieces{
            index += 1;
            if piece.row == row && piece.col == col{
                return index;
            }
        }
        // Return -1 to show it wasn't found.
        return -1;
    }
}

fn main() {
    println!("Welcome to Break!");

    println!("Player 1, please enter your name: ");
    let mut name1 = String::new();
    io::stdin()
    .read_line(&mut name1)
    .expect("Failed to read line");
    name1 = name1.trim().to_string();

    println!("Player 2, please enter your name: ");
    let mut name2 = String::new();
    io::stdin()
    .read_line(&mut name2)
    .expect("Failed to read line");
    name2 = name2.trim().to_string();
    
    let mut game = Game {
        pieces         :Vec::new(),
        player_one_name:name1,
        player_two_name:name2,
        player_one_turn:true
    };

    game.setup();
    game.run();
}
