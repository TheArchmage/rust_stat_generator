extern crate rand;

use std::io;
use std::io::Write;
use std::error::Error;
use std::fmt;
use std::process;
use rand::Rng;


pub fn print_welcome_message() {
    println!("Welcome to TheArchmage's Magnificent Stat-Generator!");
    println!(
        "With this program you can roll, arrange, and save stat-blocks \
         for the Dungeons and Dragons Fantasy Roleplaying Game!"
    );
}

pub enum RollStyle {
    roll_4d6_drop_low,
    roll_3d6_reroll_1s,
    roll_3d6_in_order,
}

impl RollStyle {
    fn new() -> RollStyle {
        RollStyle::roll_4d6_drop_low
    }

    fn request_new(&mut self) {
        self.determine_roll_style().unwrap_or_else(|err| {
            println!("Error: {}", err);
            self.request_new()
        })
    }
    
    fn determine_roll_style(&mut self) -> Result<(), Box<Error>> {
        println!(
            "Please select which method your GM has chosen for rolling \
            stats:"
        );
        println!("1 -> roll {}", RollStyle::roll_4d6_drop_low);
        println!("2 -> roll {}", RollStyle::roll_3d6_reroll_1s);
        println!("3 -> roll {}", RollStyle::roll_3d6_in_order);

        let mut rollstyle_num = String::new();
        io::stdin().read_line(&mut rollstyle_num)?;
        
        match rollstyle_num.trim() {
            "1" => *self = RollStyle::roll_4d6_drop_low,
            "2" => *self = RollStyle::roll_3d6_reroll_1s,
            "3" => *self = RollStyle::roll_3d6_in_order,
            _ => return Err(Box::new(InvalidInputError)),
        }

        println!("You have selected {}", *self);

        Ok(())
    }
}

impl fmt::Display for RollStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RollStyle::roll_4d6_drop_low => write!(f, "4d6-drop-low"),
            RollStyle::roll_3d6_reroll_1s => write!(f, "3d6-reroll-1's"),
            RollStyle::roll_3d6_in_order => write!(f, "3d6-in-order"),
        }
    }
}

#[derive(Debug)]
struct InvalidInputError;

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input")
    }
}

impl Error for InvalidInputError {
    fn description(&self) -> &str {
        "Incorrect input for user-prompts"
    }
}

pub struct Stat<'a> {
    name: &'a str,
    value: i32,
}

impl Stat<'a> {
    fn new(name: &'a str, value: i32) -> Stat<'a> {
        Stat {
            name,
            value,
        }
    }
}

pub struct Character {
    pub name: String,
    pub stats: Vec<Stat<'a>>,
    pub rollstyle: RollStyle,
}

impl Character {
    pub fn new() -> Character {
        Character {
            name: String::from("Default"),
            stats: Vec::new(
                Stat::new("STR", 10),
                Stat::new("DEX", 10),
                Stat::new("CON", 10),
                Stat::new("INT", 10),
                Stat::new("WIS", 10),
                Stat::new("CHA", 10),
            ),
            rollstyle: RollStyle::new(),
        }
    }

    pub fn update_rollstyle(&mut self) {
        self.rollstyle.request_new();
    }

    pub fn roll_stats(&mut self) -> Result<Option<Vec<i32>>, Box<Error>> {
        println!("Your current roll-style is: {}.", self.rollstyle);
        println!("We will now roll stats for the character '{}'.", self.name);
        print!("Would you like to continue? [y/n] ");
        io::stdout().flush().unwrap();
        
        let mut accept = String::new();
        io::stdin().read_line(&mut accept)?;

        match accept.trim() {
            "y" => (),
            "n" => {
                println!("Aborting.");
                process::exit(0);
            },
            _ => {
                println!("Invalid input.");
                return self.roll_stats()
            },
        }

        

        match self.rollstyle {
            RollStyle::roll_4d6_drop_low => {
                let mut rolls = Vec::new();

                for number in 0..6 {
                    let raw_rolls = roll_4d6();
                    let mut sorted_rolls = raw_rolls.clone();
                    sorted_rolls.sort();
                    let mut roll = sorted_rolls.iter().sum();
                    roll = roll - sorted_rolls.first().unwrap();
                    rolls.push(roll);
                    let mut raw_rolls_iter = raw_rolls.iter();
                    println!("Stat {}: {} {} {} {} -> {}", 
                        number+1, 
                        raw_rolls_iter.next().unwrap(), 
                        raw_rolls_iter.next().unwrap(), 
                        raw_rolls_iter.next().unwrap(),
                        raw_rolls_iter.next().unwrap(),
                        rolls[number]);
                }
                return Ok(Some(rolls))
            },
            RollStyle::roll_3d6_reroll_1s => {
                let mut rolls = Vec::new();

                for number in 0..6 {
                    let raw_rolls = roll_3d6_no_ones();
                    rolls.push(raw_rolls.iter().sum());
                    let mut raw_rolls_iter = raw_rolls.iter();
                    println!("Stat {}: {} {} {} -> {}", 
                        number+1, 
                        raw_rolls_iter.next().unwrap(), 
                        raw_rolls_iter.next().unwrap(), 
                        raw_rolls_iter.next().unwrap(),
                        rolls[number]);
                }
                return Ok(Some(rolls))
                
            },
            RollStyle::roll_3d6_in_order => {
                
                for number in 0..6 {
                    let rolls = roll_3d6();
                    self.stats[number].value = rolls.iter().sum();
                    let mut rolls_iter = rolls.iter();
                    println!(
                        "{}: {} {} {} -> {}",
                        self.stats[number].name,
                        rolls_iter.next().unwrap(),
                        rolls_iter.next().unwrap(),
                        rolls_iter.next().unwrap(),
                        self.stats[number].value,
                    );
                }
                return Ok(None)
            },

        }

        Ok(None)

    }
}

fn roll_3d6() -> Vec<i32> {
    let mut rolls = Vec::new();
    for number in 1..4 {
        rolls.push(rand::thread_rng().gen_range(1, 7));
    }
    rolls
}

fn roll_3d6_no_ones() -> Vec<i32> {
    let mut rolls = Vec::new();
    for number in 1..4 {
        rolls.push(rand::thread_rng().gen_range(2, 7));
    }
    rolls
}

fn roll_4d6() -> Vec<i32> {
    let mut rolls = Vec::new();
    for number in 1..5 {
        rolls.push(rand::thread_rng().gen_range(1, 7));
    }
    rolls
}
