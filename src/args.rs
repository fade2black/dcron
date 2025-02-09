use clap::{Arg, Command};

#[derive(Debug)]
pub enum DcronCommand {
    Upload(String),
    List
}

pub fn get_args() -> DcronCommand {
    let matches = build_command().get_matches();

    match matches.subcommand() {
        Some(("upload", sub_m)) => {
            let filename = sub_m.get_one::<String>("filename").unwrap();
            DcronCommand::Upload(filename.into())
        }
        Some(("list", _)) => {
            DcronCommand::List
        }
        _ => {
            panic!("No valid subcommand provided. Use 'upload' or 'list'.");
        }
    }

}

fn build_command() -> Command {
    Command::new("dcron")
        .version("1.0")
        .author("Bayram <bkuliyev@gmail.com>")
        .about("Distributed cron jobs")
        .subcommand(
            Command::new("upload")
                .about("Upload a file")
                .arg(
                    Arg::new("filename")
                        .help("The file with cron jobs to upload")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("list").about("List cron jobs"))
}
