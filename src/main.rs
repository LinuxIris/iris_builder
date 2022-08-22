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

fn main() {
    use clap::Command;

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
        Some(("setup", _)) => setup(),
        Some(("clean", _)) => clean(),
        Some(("update", _)) => update(),
        Some(("build", _)) => build(),
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
