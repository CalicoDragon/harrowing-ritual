use arboard::Clipboard;
use std::io;
use std::thread;
use std::time::Duration;

const READING_THE_SIGNS: bool = true;
const DC_BY_LVL: [u8; 10] = [15, 16, 18, 19, 20, 22, 23, 24, 26, 27];

fn main() {
    // Inizialization and Inputs
    println!("Welcome to Harrowing Ritual Automated!");
    println!("READING_THE_SIGNS: {}", READING_THE_SIGNS);
    println!("DC_BY_LVL: {:?}", DC_BY_LVL);

    println!("What level is the ritual?");
    let mut ritual_level: String = String::new();
    io::stdin()
        .read_line(&mut ritual_level)
        .expect("Failed to read line");
    let ritual_level: usize = ritual_level
        .trim()
        .parse()
        .expect("The level of the ritual must be a number.");
    if ritual_level < 1 || ritual_level > DC_BY_LVL.len() {
        panic!("The ritual level must be between 1 and {}", DC_BY_LVL.len());
    }

    println!("What is your full skill bonus?");
    let mut skill_bonus: String = String::new();
    io::stdin()
        .read_line(&mut skill_bonus)
        .expect("Failed to read line");
    let skill_bonus: u8 = skill_bonus
        .trim()
        .parse()
        .expect("The skill bonus must be a number.");

    println!("List the characters: (e.g. 'Laika Gabriel Miku Gideon')");
    let mut characters: String = String::new();
    io::stdin()
        .read_line(&mut characters)
        .expect("Failed to read line");
    characters.pop();

    // Creation of table
    let characters: Vec<&str> = characters.split_whitespace().collect();
    let mut table = Vec::new();
    for character in characters {
        for _ in 0..ritual_level {
            table.push((
                character,
                get_success(ritual_level, skill_bonus),
                get_suit(character),
            ));
        }
    }

    println!("Copying data to clipboard...");
    copy_table(table);
}

// Rolls and turns the roll into a String containing the grade of success
fn get_success(ritual_level: usize, skill_bonus: u8) -> String {
    let ritual_level = ritual_level - 1;
    let skill_roll = roll(20);
    let result = if skill_roll == 1 || skill_roll + skill_bonus <= DC_BY_LVL[ritual_level] - 10 {
        "Critical Failure"
    } else if skill_roll == 20 || skill_roll + skill_bonus >= DC_BY_LVL[ritual_level] + 10 {
        "Critical Success"
    } else if skill_roll + skill_bonus <= DC_BY_LVL[ritual_level] {
        "Failure"
    } else if skill_roll + skill_bonus >= DC_BY_LVL[ritual_level] {
        "Success"
    } else {
        "ERROR IN GET_SUCCESS()"
    };
    return result.to_string();
}

// Rolls and turns the roll into a String containing the wildcard suit
fn get_suit(name: &str) -> String {
    if !READING_THE_SIGNS {
        return suit_to_string(roll(6));
    }

    let roll1 = roll(6);
    let roll2 = roll(6);

    if roll1 == 6 {
        return suit_to_string(roll1);
    }
    if roll2 == 6 {
        return suit_to_string(roll2);
    }
    if roll1 == roll2 {
        return suit_to_string(roll1);
    }

    let roll1 = suit_to_string(roll1);
    let roll2 = suit_to_string(roll2);
    println!("{name} card: {roll1} (1) or {roll2} (2)?");

    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("The choice must be a number.");
    if choice != 1 || choice != 2 {
        panic!("The choice must be 1 or 2");
    }

    match choice {
        1 => {
            return roll1;
        }
        2 => {
            return roll1;
        }
        _ => {
            panic!("The choice must be either 1 or 2");
        }
    }
}

fn suit_to_string(suit: u8) -> String {
    return match suit {
        1 => "Strikes",
        2 => "Reflex Saves",
        3 => "Fortitude Saves",
        4 => "Skill Checks",
        5 => "Will Saves",
        6 => "All rolls",
        _ => "ERROR IN SUIT_TO_STRING()",
    }
    .to_string();
}

fn roll(size: u8) -> u8 {
    return rand::random_range(1..=size);
}

// Copies the data to clipboard imitating the format of FoundryTTV journals
fn copy_table(table: Vec<(&str, String, String)>) {
    let mut clipboard = Clipboard::new().unwrap();

    let mut text: String = String::new();
    for elem in table.clone() {
        text += &format!("{}\n\n{}\n\n{}\n\n-\n\n\n", elem.0, elem.1, elem.2);
    }
    text = format!(
        "Characters\n\nGrade of Success\n\nPossible Check\n\nUsed\n\n\n{}",
        text
    );

    let mut html: String = String::new();
    for elem in table {
        html += &format!(
            "<tr><td><p>{}</p></td><td><p>{}</p></td><td><p>{}</p></td><td><p></p></td></tr>",
            elem.0, elem.1, elem.2
        );
    }
    html = format!(
        "<table data-pm-slice=\"1 1 []\"><tbody><tr><th><p>Characters</p></th><th><p>Grade of Success</p></th><th><p>Possible Check</p></th><th><p>Used</p></th></tr>{}</tbody></table>",
        html
    );

    clipboard
        .set_html(html, Some(text))
        .expect("Copying to clipboard failed!");

    // At least on wayland I need sleep for it to work, not sure why
    thread::sleep(Duration::from_millis(100));
}
