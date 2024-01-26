use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use rand::Rng;

// Enum representing different game states
#[derive(Debug, Clone, Copy)]
enum GameState {
    Start,
    Room1,
    Room2,
    Room3,
    Win,
    GameOver,
    Purchase,
}

// Enum representing different items in the game
enum Item {
    Key,
    Coin(u32),
}

fn main() {
    let mut inventory = load_game().unwrap_or_else(|_| vec![Item::Coin(0)]);
    play_game(GameState::Start, &mut inventory);
}

fn play_game(mut game_state: GameState, mut inventory: &mut Vec<Item>) {
    match game_state {
        GameState::Start => {
            let coin_count = get_coin_count(&inventory);
            println!("Welcome to the Text Adventure Game!");
            println!("You find yourself in a dark room. There are three doors in front of you.");
            println!("You have {} coins.", coin_count);
            println!("Choose a door to enter (1, 2, 3):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "1" => play_game(random_game_state(), &mut inventory),
                "2" => play_game(random_game_state(), &mut inventory),
                "3" => play_game(random_game_state(), &mut inventory),
                _ => {
                    println!("Invalid choice! You stumble in the darkness.");
                    play_game(GameState::GameOver, &mut inventory);
                }
            }
        }
        GameState::Room1 => {
            println!("You enter Room 1. It's dark and musty. A mysterious sound echoes.");
            println!("You need to find a key to unlock the door.");

            if inventory.iter().any(|item| matches!(item, Item::Key)) {
                println!("You use the key to unlock the door. The door creaks open.");
                play_game(GameState::Win, &mut inventory);
            } else {
                println!("You search the room, trying to find the key.");
                play_game(random_game_state(), &mut inventory);
            }
        }
        GameState::Room2 => {
            println!("You enter Room 2. It's dimly lit with a strange aura. A table stands in the center.");
            println!("On the table, there's a key and a coin. Do you want to pick them up? (yes/no):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "yes" => {
                    println!("You picked up the key and the coin. The room shivers.");
                    inventory.push(Item::Key);
                    inventory.push(Item::Coin(1));
                    play_game(random_game_state(), &mut inventory);
                }
                "no" => {
                    println!("You decide to leave the key and the coin on the table. The room remains still.");
                    play_game(random_game_state(), &mut inventory);
                }
                _ => {
                    println!("Invalid choice! The room reacts strangely.");
                    play_game(GameState::GameOver, &mut inventory);
                }
            }
        }
        GameState::Room3 => {
            println!("You enter Room 3. A giant spider blocks your way!");
            println!("You can't proceed this way. Go back to another room.");

            play_game(random_game_state(), &mut inventory);
        }
        GameState::Win => {
            println!("Congratulations! You unlocked the door and won the game!");
        }
        GameState::GameOver => {
            println!("Game Over! You made a wrong choice. The darkness consumes you.");
        }
        GameState::Purchase => {
            let coin_count = get_coin_count(&inventory);

            println!("Welcome to the item shop! ");
            if coin_count > 0 {
                println!("You have {} coins.", coin_count);
            } else {
                println!("Player has 0 coins.");
            }

            if coin_count >= 5 {
                println!("You can purchase a loophole to win the game (L - 5 coins).");
            }

            println!("Choose an item to purchase (1. Key - 2 coins, 2. Back):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "1" => {
                    if coin_count >= 3 {
                        println!("You purchased a key! The shopkeeper nods.");
                        inventory.push(Item::Key);
                        // Consume 2 coins
                        inventory.retain(|item| !matches!(item, Item::Coin(_)));
                        inventory.push(Item::Coin(3));
                    } else {
                        println!("Not enough coins to purchase the key. The shopkeeper frowns.");
                    }
                }
                "L" => {
                    if coin_count >= 5 {
                        println!("You purchased a loophole and won the game! The universe bends to your will.");
                        play_game(GameState::Win, &mut inventory);
                        return;
                    } else {
                        println!("Not enough coins to purchase the loophole. The shopkeeper shakes his head.");
                    }
                }
                "2" => {
                    play_game(random_game_state(), &mut inventory);
                }
                _ => {
                    println!("Invalid choice! The shopkeeper looks confused.");
                }
            }

            play_game(random_game_state(), &mut inventory);
        }
        GameState::GameOver | GameState::Win => {
            println!("Do you want to save the game? (yes/no):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "yes" => {
                    let key_code = generate_key_code();
                    save_game_with_key(&inventory, key_code.clone()).expect("Failed to save the game.");
                    println!("Game saved with key code: {}. The universe remembers.", key_code);
                }
                "no" => {
                    println!("Thanks for playing! The adventure ends here.");
                }
                _ => {
                    println!("Invalid choice! The universe is indifferent.");
                }
            }
        }
    }
}

fn random_game_state() -> GameState {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => GameState::Room1,
        1 => GameState::Room2,
        2 => GameState::Room3,
        _ => unreachable!(),
    }
}

fn generate_key_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..=999999))
}

fn save_game_with_key(inventory: &Vec<Item>, key_code: String) -> io::Result<()> {
    let mut file = File::create("save_game.txt")?;
    writeln!(file, "Key Code: {}", key_code)?;

    for item in inventory {
        match item {
            Item::Key => writeln!(file, "Key")?,
            Item::Coin(count) => writeln!(file, "Coin {}", count)?,
        }
    }
    Ok(())
}

fn load_game() -> io::Result<Vec<Item>> {
    let mut file = match File::open("save_game.txt") {
        Ok(file) => file,
        Err(_) => return Ok(vec![]),
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut lines = content.lines();
    let key_line = lines.next().unwrap_or_default();
    let key_code = key_line.trim_start_matches("Key Code: ");

    // For simplicity, assuming the structure of the save file is fixed
    let inventory: Vec<Item> = lines
        .map(|line| {
            if line == "Key" {
                Item::Key
            } else if let Some(coin_count_str) = line.strip_prefix("Coin ") {
                if let Ok(coin_count) = coin_count_str.parse::<u32>() {
                    Item::Coin(coin_count)
                } else {
                    Item::Coin(0)
                }
            } else {
                Item::Coin(0)
            }
        })
        .collect();

    Ok(inventory)
}

fn get_coin_count(inventory: &Vec<Item>) -> u32 {
    inventory
        .iter()
        .filter_map(|item| match item {
            Item::Coin(count) => Some(*count),
            _ => None,
        })
        .sum()
}
