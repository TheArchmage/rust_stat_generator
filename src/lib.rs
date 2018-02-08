use std::io;
use std::error::Error;
use std::fmt;

pub fn print_welcome_message() {
    println!("Welcome to TheArchmage's Magnificent Stat-Generator!");
    println!(
        "With this program you can roll, arrange, and save stat-blocks \
         for the Dungeons and Dragons Fantasy Roleplaying Game!"
    );
}

#[derive(Debug)]
pub enum RollStyle {
    roll_4d6_drop_low,
    roll_3d6_reroll_1s,
    roll_3d6_in_order,
}

impl RollStyle {
    pub fn new() -> RollStyle {
        RollStyle::determine_roll_style().unwrap_or_else(|err| {
            println!("Error: {}", err);
            RollStyle::new()
        })
    }
    
    fn determine_roll_style() -> Result<RollStyle, Box<Error>> {
        println!(
            "Please select which method your GM has chosen for rolling \
            stats:"
        );
        println!("1 -> roll 4d6-drop-low");
        println!("2 -> roll 3d6-reroll-1's");
        println!("3 -> roll 3d6-in-order");

        let mut rollstyle_num = String::new();
        io::stdin().read_line(&mut rollstyle_num)?;
        
        let mut rollstyle: RollStyle;

        match rollstyle_num.trim() {
            "1" => rollstyle = RollStyle::roll_4d6_drop_low,
            "2" => rollstyle = RollStyle::roll_3d6_reroll_1s,
            "3" => rollstyle = RollStyle::roll_3d6_in_order,
            _ => return Err(Box::new(RollStyleInputError)),
        }
        
        
        println!("You have selected {:?}", rollstyle);

        Ok(rollstyle)
    }
}

#[derive(Debug)]
struct RollStyleInputError;

impl fmt::Display for RollStyleInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid selection.")
    }
}

impl Error for RollStyleInputError {
    fn description(&self) -> &str {
        "Incorrect selection of roll-style input options"
    }
}