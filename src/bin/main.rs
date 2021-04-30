#[macro_use]
extern crate seahorse;
extern crate imt_cli;

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .flag(Flag::new("bye", "cli [name] --bye(-b)", FlagType::Bool).alias("b"))
        .flag(Flag::new("age", "cli [name] --age(-a)", FlagType::Int).alias("a"))
        .flag(Flag::new("age", "cli [name] --age(-a)", FlagType::Int).alias("a"))
        .command(calc_command())
        .command(create_command());

    app.run(args);
}

fn default_action(c: &Context) {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }

    if let Some(age) = c.int_flag("age") {
        println!("{:?} is {} years old", c.args, age);
    }
}

fn calc_action(c: &Context) {
    match c.string_flag("operator") {
        Some(op) => {
            let sum: i32 = match &*op {
                "add" => c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum(),
                "sub" => c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum(),
                _ => panic!("undefined operator..."),
            };

            println!("{}", sum);
        }
        None => panic!(),
    }
}

fn calc_command() -> Command {
    Command::new()
        .name("calc")
        .usage("cli calc [nums...]")
        .action(calc_action)
        .flag(
            Flag::new(
                "operator",
                "cli calc [nums...] --operator(-op) [add | sub]",
                FlagType::String,
            )
            .alias("op"),
        )
}

fn create_action(c: &Context) {
    let mut mnemonic = "";
    mnemonic = c.args.iter().next().unwrap();
    println!("{}", mnemonic)
}

fn create_command() -> Command {
    Command::new()
        .name("create")
        .usage("cli create [mnemonic]")
        .action(create_action)
}
