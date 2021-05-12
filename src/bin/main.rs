#[macro_use]

extern crate cmd_lib;
extern crate dirs;
extern crate runas;
extern crate seahorse;
extern crate imt_cli;

use seahorse::{App, Command, Context, Flag, FlagType};
use runas::{Command as RunasCommand};
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::{Command as StdCmd, Output as StdOutput};
use std::io::prelude::*;

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
        .command(addfile_command())
        .command(updatefile_command())
        .command(find_command())
        .command(rollback_command())
        .command(rollforward_command())
        .command(wormhole_recv_command())
        .command(wormhole_send_command());

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
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("create")
            .arg("--store-name")
            .arg(n)
            .arg(mnemonic)
            .status()
            .unwrap()
            .success();
    } else {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("create")
            .arg(mnemonic)
            .status()
            .unwrap()
            .success();
    }
}

fn addfile_action(c: &Context) {
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");

    let mut tags_vec: Vec<String> = vec![];

    let mut args = c.args.iter();
    let arg_count = args.clone().count();
    let mut file = "";
    let mut tags = "";
    match arg_count {
        0 => {
        },
        1 => {
            file = args.next().unwrap();
        },
        _ => {
            file = args.next().unwrap();
            for tag in args {
                tags_vec.push(tag.to_string())
            }
        }
    }

    if file != "" {
        let mut file_path = PathBuf::from(file);
        file_path = fs::canonicalize(&file_path).expect("failed tr full path of file");
        let filename = file_path.as_path().file_name().expect("can't get file name");
        let stage_path = format!("{}/immutag/{}/{}", path_string, "stage", filename.to_owned().to_str().unwrap());
        fs::copy(file_path.to_str().expect("can't convert file path to str"), stage_path).expect("fail to rename file");
    } else {
        panic!("add file: no file given")
    }

    if let Some(n) = c.string_flag("store-name") {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("add")
            .arg("--store-name")
            .arg(n)
            .arg(file)
            .args(&tags_vec)
            .status()
            .unwrap()
            .success();
    } else {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("add")
            .arg(file)
            .args(&tags_vec)
            .status()
            .unwrap()
            .success();
    }
}

fn updatefile_action(c: &Context) {
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");

    let mut args_iter = c.args.iter();
    let addr = args_iter.next().unwrap();
    let file = args_iter.next().unwrap();

    let mut addr_option = "";

    let mut stage_filename = std::ffi::OsString::new();

    if c.bool_flag("addr") {
        addr_option = "--addr";
    }

    if file != "" {
        let mut file_path = PathBuf::from(file);
        file_path = fs::canonicalize(&file_path).expect("failed tr full path of file");
        let filename = file_path.as_path().file_name().expect("can't get file name").to_os_string();

        let stage_path = format!("{}/immutag/{}/{}", path_string, "stage", filename.to_owned().to_str().unwrap());
        fs::copy(file_path.to_str().expect("can't convert file path to str"), stage_path).expect("fail to rename file");

        stage_filename = file_path.as_path().file_name().expect("can't get file name").to_os_string();
    } else {
        panic!("add file: no file given")
    }

    if let Some(n) = c.string_flag("store-name") {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("update")
            .arg("--store-name")
            .arg(n)
            .arg(addr)
            .arg(stage_filename)
            .status()
            .unwrap()
            .success();
    } else {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("update")
            .arg(addr)
            .arg(stage_filename)
            .status()
            .unwrap()
            .success();
    }
}

fn find_action(c: &Context) {
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");

    let mut addr_option = "";

    let find_file = |path_string| {
        // Need permissions to write in ~/immutag.
        let output_path = format!("{}/{}/{}", path_string, "immutag", ".find_output");

        // read .find_output
        let mut input = std::fs::File::open(output_path).expect("failed to open .find_output");

        let mut input_buffer = String::new();

        input.read_to_string(&mut input_buffer).expect("fail read .find_output");

        let mut find_file_path = Path::new(&input_buffer);

        let find_file_path = find_file_path.strip_prefix("/root/").expect("failed to strip /root prefix from .find_output");

        let find_file_pathbuf = path.join(find_file_path);

        let mut f = find_file_pathbuf.to_str().unwrap().to_string();
        f.pop();

        let rm_res = fs::remove_file(path.join("immutag/file"));


        std::os::unix::fs::symlink(&f, path.join("immutag/file")).expect("fail to create file link");

        let mut input = std::fs::File::open(&f).expect("failed to open .find_output");

        let mut input_buffer = String::new();

        input.read_to_string(&mut input_buffer).expect("fail read .find_output");
    };

    if let Some(n) = c.string_flag("store-name") {
        if c.bool_flag("addr") {
            StdCmd::new("sudo")
                .arg("docker")
                .args(&["run", "-it"])
                .arg("-v")
                .arg(docker_mount)
                .arg("immutag:0.0.11")
                .arg("find")
                .arg("--store-name")
                .arg(n)
                .arg("--addr")
                .status()
                .unwrap()
                .success();
        } else {
            StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("find")
            .arg("--store-name")
            .arg(n)
            .status()
            .unwrap()
            .success();

            find_file(path_string);
        }
    } else {
        if c.bool_flag("addr") {
            StdCmd::new("sudo")
                .arg("docker")
                .args(&["run", "-it"])
                .arg("-v")
                .arg(docker_mount)
                .arg("immutag:0.0.11")
                .arg("find")
                .arg("--addr")
                .status()
                .unwrap()
                .success();
        } else  {
            StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("find")
            .status()
            .unwrap()
            .success();

            find_file(path_string);
        }
    }

    // convert into path
    // Drain /root and prepend dir_path

    //if rm_res.is_ok() {
    //    println!("ok");

    //}
}

fn rollback_action(c: &Context) {
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");

    if let Some(n) = c.string_flag("store-name") {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("rollback")
            .arg("--store-name")
            .arg(n)
            .status()
            .unwrap()
            .success();
    } else {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("rollback")
            .status()
            .unwrap()
            .success();
    }
}

fn rollforward_action(c: &Context) {
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");

    if let Some(n) = c.string_flag("store-name") {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("rollforward")
            .arg("--store-name")
            .arg(n)
            .status()
            .unwrap()
            .success();
    } else {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("rollforward")
            .status()
            .unwrap()
            .success();
    }
}

fn wormhole_recv_action(c: &Context) {
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");

    if let Some(n) = c.string_flag("store-name") {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("wormhole-recv")
            .arg("--store-name")
            .arg(n)
            .status()
            .unwrap()
            .success();
    } else {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("wormhole-recv")
            .status()
            .unwrap()
            .success();
    }
}

fn wormhole_send_action(c: &Context) {
    let home_dir = dirs::home_dir().unwrap();
    let mut path = Path::new(&home_dir);
    let mut path_string = path.to_str().unwrap().to_string();
    let docker_mount = format!("{}/{}:/root/immutag", path_string, "immutag");

    if let Some(n) = c.string_flag("store-name") {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("wormhole-send")
            .arg("--store-name")
            .arg(n)
            .status()
            .unwrap()
            .success();
    } else {
        StdCmd::new("sudo")
            .arg("docker")
            .args(&["run", "-it"])
            .arg("-v")
            .arg(docker_mount)
            .arg("immutag:0.0.11")
            .arg("wormhole-send")
            .status()
            .unwrap()
            .success();
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

fn updatefile_command() -> Command {
    Command::new()
        .name("update")
        .usage("cli update [addr] [file]")
        .action(updatefile_action)
        .flag(
            Flag::new(
                "store-name",
                "cli update [file] [addr] --store-name(-n) [name]",
                FlagType::String,
            )
            .alias("n"),
        )
}

fn find_command() -> Command {
    Command::new()
        .name("find")
        .usage("cli find")
        .action(find_action)
        .flag(
            Flag::new(
                "store-name",
                "cli find --store-name(-n) [name]",
                FlagType::String,
            )

            .alias("n"),
        )
        .flag(
            Flag::new(
                "addr",
                "cli find --addr(-a)",
                FlagType::Bool,
            )

            .alias("a"),
        )
}

fn rollback_command() -> Command {
    Command::new()
        .name("rollback")
        .usage("cli rollback")
        .action(rollback_action)
        .flag(
            Flag::new(
                "store-name",
                "cli rollback --store-name(-n) [name]",
                FlagType::String,
            )
            .alias("n"),
        )
}

fn rollforward_command() -> Command {
    Command::new()
        .name("rollforward")
        .usage("cli rollforward")
        .action(rollforward_action)
        .flag(
            Flag::new(
                "store-name",
                "cli rollforward --store-name(-n) [name]",
                FlagType::String,
            )
            .alias("n"),
        )
}

fn wormhole_send_command() -> Command {
    Command::new()
        .name("wormhole-send")
        .usage("cli wormhole-send")
        .action(wormhole_send_action)
        .flag(
            Flag::new(
                "store-name",
                "cli wormhole-send --store-name(-n) [name]",
                FlagType::String,
            )
            .alias("n"),
        )
}

fn wormhole_recv_command() -> Command {
    Command::new()
        .name("wormhole-recv")
        .usage("cli wormhole-recv]")
        .action(wormhole_recv_action)
        .flag(
            Flag::new(
                "store-name",
                "cli wormhole-recv --store-name(-n) [name]",
                FlagType::String,
            )
            .alias("n"),
        )
}
