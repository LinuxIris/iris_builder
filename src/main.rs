use std::process::Command;
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

fn old_main() {
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
            "help" => println!("Iris builder tool\n\nHelp: Shows this message\n\nSetup: Clones the main repository and sets the project up\n\nClean: Cleans up pacman cache\n\nBuild: Configures and builds the project\n\nUpdate: Pulls updates from the main repository"),
            "setup" => setup(),
            "clean" => clean(),
            "build" => build(),
            "update" => update(),
            _ => println!("Unknown argument specified! Acceptable arguments:\n\thelp\n\tsetup\n\tclean\n\tbuild\n\tupdate")
        }
        return;
    }
}

fn main() {
    use clap::{Arg, Command};

    let matches = Command::new("iris_builder")
        .about("Iris distro builder")
        .version("0.0.1")
        .arg_required_else_help(true)
        .author("Lucky4Luuk")
        .subcommand(Command::new("setup").about("Initial setup of required files"))
        .subcommand(Command::new("clean").about("Cleans up package cache"))
        .subcommand(Command::new("build").about("Builds the repo. Warning: might take long!"))
        .subcommand(Command::new("update").about("Pulls any new iris-minimal repo changes"))
        .get_matches();

    match matches.subcommand() {
        Some(("setup", subm)) => setup(),
        Some(("clean", subm)) => clean(),
        Some(("update", subm)) => update(),
        Some(("build", subm)) => build(),
        _ => panic!("How did we get here?")
    }
}

fn setup() {
    println!("Setting up project...");
    println!("Cloning folders, make sure git is installed!");

    Command::new("git")
        .args(["clone", "https://github.com/LinuxIris/iris-minimal"])
        .status()
        .expect("Failed to clone repository! Is git installed?");

    clean();

    println!("Setup done! You can now safely run the build command!");
}

fn clean() {
    println!("Cleaning up pacman cache...");
    Command::new("sudo")
        .args(["bash", "scripts/pac_clean.sh"])
        .status()
        .expect("Failed to clean pacman cache!");
}

fn update() {
    println!("Starting update process...");
    println!("Updating iris-minimal...");
    Command::new("git")
        .args(["pull"])
        .current_dir("iris-minimal")
        .status()
        .expect("Failed to git pull new version of iris-minimal!");
    clean();
    println!("Update complete!");
}

fn build() {
    config();

    Command::new("bash")
        .args(["40-build-the-iso-local-again.sh"])
        .current_dir("iris-minimal/installation-scripts")
        .status()
        .expect("Failed to run bash script!");

    println!("Project succesfully built!");
}

fn config() {
    println!("Building project...");
    println!("Reading config file...");
    let file_str = std::fs::read_to_string("config.toml").expect("Failed to open config file!");
    let config: Config = toml::from_str(&file_str).expect("Failed to parse config file!");

    println!("Packages to be installed: {}", config.packages.packages.len());
    let packages_file_str = packages_x86_64_file::build(&config.packages.packages);
    std::fs::write("iris-minimal/archiso/packages.x86_64", packages_file_str).expect("Failed to overwrite packages.x86_64!");
}
