use std::io;

pub struct Input {}
impl Input {
    pub fn validate_board_input(character: char) -> Result<char, &'static str> {
        return match character {
            '1' => Ok('1'),
            '2' => Ok('2'),
            '3' => Ok('3'),
            '4' => Ok('4'),
            '5' => Ok('5'),
            '6' => Ok('6'),
            '7' => Ok('7'),
            '8' => Ok('8'),
            '9' => Ok('9'),
            _ => return Err("Це не номер на дошці!"),
        };
    }

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Ви написали хуйню і все поламали. Тільки спробуй ще раз!");

        return input;
    }

    pub fn read_char() -> char {
        return loop {
            let input = Input::get_input();

            break match input.trim().parse::<char>() {
                Ok(character) => character,
                Err(_) => {
                    println!("Хочеш вийти? Перехочеш!");
                    continue;
                }
            };
        };
    }
}
