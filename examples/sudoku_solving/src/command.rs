use clap::{Arg, ArgGroup, Command};

pub fn build_command() -> Command {
    let command = Command::new("Sudoku Solver Example")
        .version("0.1.0")
        .author("Markus Mayer")
        .arg(
            Arg::new("normal")
                .long("sudoku")
                .help("Solve a regular Sudoku")
                .action(clap::ArgAction::SetTrue)
                .group("type"),
        )
        .arg(
            Arg::new("nonomino")
                .long("nonomino")
                .help("Solve a Nonomino-type game")
                .action(clap::ArgAction::SetTrue)
                .group("type"),
        )
        .arg(
            Arg::new("hypersudoku")
                .long("hyper")
                .help("Solve a Hypersoduko-type game")
                .action(clap::ArgAction::SetTrue)
                .group("type"),
        )
        .group(ArgGroup::new("type").required(true));
    command
}
