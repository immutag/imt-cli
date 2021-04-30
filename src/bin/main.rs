#[macro_use]

extern crate dirs;
extern crate runas;
extern crate seahorse;
extern crate imt_cli;

use seahorse::{App, Command, Context, Flag, FlagType};
use runas::{Command as RunasCommand};
use std::env;
use std::path::Path;
use std::process::{Command as StdCmd, Output as StdOutput};

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
        .command(create_command())
        .command(addfile_command());

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
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");
    let mut mnemonic = "";
    mnemonic = c.args.iter().next().unwrap();

    if let Some(n) = c.string_flag("store-name") {
        let status = RunasCommand::new("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("create")
            .arg("--store-name")
            .arg(n)
            .arg(mnemonic)
            .status()
            .unwrap();
    } else {
        let status = RunasCommand::new("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("create")
            .arg(mnemonic)
            .status()
            .unwrap();
    }
}

fn addfile_action(c: &Context) {
    let mut args = c.args.iter();
    let arg_count = args.clone().count();
    let mut file = "";
    match arg_count {
        0 => {
        },
        1 => {
            file = args.next().unwrap();
        },
        _ => {
            file = args.next().unwrap();
            let mut tags_vec: Vec<String> = vec![];
            for path in args {

            }
        }
    }
    if let Some(n) = c.string_flag("store-name") {
        let status = RunasCommand::new("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg("immutag:0.0.11")
            .arg("create")
            .arg("--store-name")
            .status()
            .unwrap();
    } else {
        let status = RunasCommand::new("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg("immutag:0.0.11")
            .arg("create")
            .status()
            .unwrap();
    }
}

fn create_command() -> Command {
    Command::new()
        .name("create")
        .usage("cli create [mnemonic]")
        .action(create_action)
        .flag(
            Flag::new(
                "store-name",
                "cli create [mnemonic] --store-name(-n) [name]",
                FlagType::String,
            )
            .alias("n"),
        )
}

fn addfile_command() -> Command {
    Command::new()
        .name("add")
        .usage("cli add [file] [tags...]")
        .action(addfile_action)
        .flag(
            Flag::new(
                "store-name",
                "cli add [file] [tags...]  --store-name(-n) [name]",
                FlagType::String,
            )
            .alias("n"),
        )
}

    //sudo docker run -it \
    //-v "$HOME"/immutag:/root/immutag \
    //immutag:0.0.11 \
    //bash
