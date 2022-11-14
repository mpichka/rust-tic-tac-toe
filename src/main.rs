mod board;
mod input;
mod player;

use board::Board;
use input::Input;
use player::PlayerObj;

fn main() {
    loop {
        let mut board = Board::new();
        clean_screen();
        println!("Вітаю в грі хрестики-нолики!");

        println!("Виберіть свого бійця: [Х]рестик чи н[О]лик!");
        let input = Input::read_char();
        let mut player = PlayerObj::select_player(input);
        println!("Ви граєте за {}.", PlayerObj::map_player_to_string(&player));

        let winner = loop {
            board.print();
            PlayerObj::print_next_player(&player);

            // Player move
            loop {
                println!(
                    "Впиздячте номер клітки куди ви бажаєте хуйнути свій {}, будь ласка:",
                    PlayerObj::map_player_to_string(&player)
                );
                let input = Input::read_char();
                let key = match Input::validate_board_input(input) {
                    Ok(key) => key,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
                match board.make_move(&player, key) {
                    Ok(()) => {
                        println!("Заєбість!");
                        break;
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
            }
            match board.check_winner() {
                Ok(winner) => break winner,
                Err(_) => (),
            };
            match board.check_draw() {
                Ok(draw) => break draw,
                Err(_) => (),
            };
            player = PlayerObj::switch_move(player);

            // AI move
            clean_screen();
            board.make_ai_move(&player);
            match board.check_winner() {
                Ok(winner) => break winner,
                Err(_) => (),
            };
            match board.check_draw() {
                Ok(draw) => break draw,
                Err(_) => (),
            };
            player = PlayerObj::switch_move(player);
        };

        board.print();

        match winner {
            'X' => println!("Перемогли хрестики!"),
            'O' => println!("Перемогли нолики!"),
            _ => println!("Перемогла дружба!"),
        }

        println!("Бажаєте зіграти ще раз [Y]?");
        let input = Input::read_char();

        if input == 'Y' || input == 'y' {
            continue;
        }
    }
}

fn clean_screen() {
    const SCREEN_HEIGHT: u8 = 20;
    let mut i = 0;
    while i < SCREEN_HEIGHT {
        println!("");
        i += 1;
    }
}
