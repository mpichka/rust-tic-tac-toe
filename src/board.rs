use super::player::{Player, PlayerObj};
use rand::Rng;
use std::collections::HashMap;
use std::result::Result;

pub struct Board {
    board: HashMap<char, char>,
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: Board::get_new_board(),
        }
    }

    fn get_new_board() -> HashMap<char, char> {
        let mut board: HashMap<char, char> = HashMap::new();

        board.insert('1', '1');
        board.insert('2', '2');
        board.insert('3', '3');
        board.insert('4', '4');
        board.insert('5', '5');
        board.insert('6', '6');
        board.insert('7', '7');
        board.insert('8', '8');
        board.insert('9', '9');

        board
    }

    pub fn print(&self) {
        println!(
            "
             {} │ {} │ {}
            ───┼───┼───
             {} │ {} │ {}
            ───┼───┼───
             {} │ {} │ {}    
            ",
            self.board.get(&'1').unwrap(),
            self.board.get(&'2').unwrap(),
            self.board.get(&'3').unwrap(),
            self.board.get(&'4').unwrap(),
            self.board.get(&'5').unwrap(),
            self.board.get(&'6').unwrap(),
            self.board.get(&'7').unwrap(),
            self.board.get(&'8').unwrap(),
            self.board.get(&'9').unwrap(),
        );
    }

    pub fn make_move(&mut self, player: &Player, key: char) -> Result<(), &'static str> {
        let players_char = PlayerObj::map_player_to_char(player);

        let board_value = self.board.get(&key).copied().unwrap();

        match board_value {
            '1' => self.board.insert(key, players_char),
            '2' => self.board.insert(key, players_char),
            '3' => self.board.insert(key, players_char),
            '4' => self.board.insert(key, players_char),
            '5' => self.board.insert(key, players_char),
            '6' => self.board.insert(key, players_char),
            '7' => self.board.insert(key, players_char),
            '8' => self.board.insert(key, players_char),
            '9' => self.board.insert(key, players_char),
            'X' => {
                match players_char {
                    'X' => return Err("Ви вже сюди ходили..."),
                    'O' => return Err("Сюди вже походив бот..."),
                    _ => return Err("Неможливо поставити мітку"),
                };
            }
            'O' => {
                match players_char {
                    'X' => return Err("Сюди вже походив бот..."),
                    'O' => return Err("Ви вже сюди ходили..."),
                    _ => return Err("Неможливо поставити мітку!!!"),
                };
            }
            _ => return Err("Неможливо поставити мітку!!!"),
        };

        Ok(())
    }

    pub fn make_ai_move(&mut self, player: &Player) {
        loop {
            let ai_move = char::from(rand::thread_rng().gen_range(1..=9) + 48);
            match self.make_move(player, ai_move) {
                Ok(()) => {
                    println!("Бот здійснив хід на {} клітку", ai_move);
                    break;
                }
                Err(_) => continue,
            };
        }
    }

    pub fn check_draw(&self) -> Result<char, ()> {
        const MAX_KEYS_COUNT: i8 = 9;
        let mut count: i8 = 0;

        let board_vec: Vec<char> = vec![
            *self.board.get(&'1').unwrap(),
            *self.board.get(&'2').unwrap(),
            *self.board.get(&'3').unwrap(),
            *self.board.get(&'4').unwrap(),
            *self.board.get(&'5').unwrap(),
            *self.board.get(&'6').unwrap(),
            *self.board.get(&'7').unwrap(),
            *self.board.get(&'8').unwrap(),
            *self.board.get(&'9').unwrap(),
        ];

        for item in board_vec {
            if item == 'X' || item == 'O' {
                count += 1;
            }
        }
        if count >= MAX_KEYS_COUNT {
            return Ok('d');
        }
        Err(())
    }

    pub fn check_winner(&self) -> Result<char, ()> {
        //check rows
        match self.check_items('1', '2', '3') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };
        match self.check_items('4', '5', '6') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };
        match self.check_items('7', '8', '9') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };
        //check columns
        match self.check_items('1', '4', '7') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };
        match self.check_items('2', '5', '8') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };
        match self.check_items('3', '6', '9') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };
        //check crosses
        match self.check_items('1', '5', '9') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };
        match self.check_items('3', '5', '7') {
            Ok(winner) => return Ok(winner),
            Err(_) => (),
        };

        Err(())
    }

    fn check_items(&self, i1: char, i2: char, i3: char) -> Result<char, ()> {
        let k1 = *self.board.get(&i1).unwrap();
        let k2 = *self.board.get(&i2).unwrap();
        let k3 = *self.board.get(&i3).unwrap();

        if k1 == k2 && k2 == k3 && k3 == k1 {
            return Ok(k1);
        }

        Err(())
    }
}
