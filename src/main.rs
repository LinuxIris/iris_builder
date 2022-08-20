use serde::Deserialize;

mod packages_x86_64_file;

#[derive(Deserialize)]
struct Config {
    packages: PackageList,
}

#[derive(Deserialize)]
struct PackageList {
    packages: Vec<String>,
}

fn main() {
    println!("This software was designed to run only on arch/iris. If you are not running on arch/iris, this probably will not work at all!");

    let args: Vec<String> = std::env::args().collect();
    let args: Vec<String> = args[1..].to_vec();
    let argc = args.len();
    if argc > 1 {
        println!("You can only specify 1 argument!");
        return;
    }
    if argc > 0 {
        match args[0].as_ref() {
            "help" => println!("Not yet finished... sorry :)"),
            "setup" => setup(),
            "build" => build(),
            _ => println!("Unknown argument specified! Acceptable arguments:\n\thelp\n\tsetup\n\tbuild")
        }
        return;
    }
}

fn setup() {
    use std::process::Command;
    println!("Setting up project...");
    println!("Cloning folders, make sure git is installed!");
    Command::new("git")
        .args(["clone", "https://github.com/LinuxIris/iris-minimal"])
        .status()
        .expect("Failed to clone repository! Is git installed?");

    println!("Setup done! You can now safely run the build command!");
}

fn build() {
    config();
    todo!();
}

fn config() {
    println!("Reading config file...");
    let file_str = std::fs::read_to_string("config.toml").expect("Failed to open config file!");
    let config: Config = toml::from_str(&file_str).expect("Failed to parse config file!");

    println!("Packages to be installed: {}", config.packages.packages.len());
    let packages_file_str = packages_x86_64_file::build(&config.packages.packages);
    std::fs::write("/iris-minimal/archiso/packages.x86_64", packages_file_str).expect("Failed to overwrite packages.x86_64!");
}
