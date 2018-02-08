extern crate stat_generator;

fn main() {

    stat_generator::print_welcome_message();

    let mut character = stat_generator::Character::new();

    character.update_rollstyle();

    character.roll_stats();
}
