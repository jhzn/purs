use super::git;
use ansi_term::Colour::{Cyan, Purple, White, Yellow};
use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};
use git2::{self, Repository};
use regex::Regex;
use std::env;
use std::fmt;
use sys_info;
use tico::tico;

struct Precmd {
    time: chrono::DateTime<chrono::Local>,
    user: String,
    host: String,
    cwd: String,
    git_status: Option<git::Info>,
}

impl fmt::Display for Precmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let git_status = match &self.git_status {
            Some(status) => format!("{}", status),
            None => "".to_string(),
        };

        write!(
            f,
            "{} {}@{} {} {}",
            Yellow.paint(format!("{}", self.time.format("%Y-%m-%d %H:%M:%S"))),
            Cyan.paint(&self.user),
            Purple.paint(&self.host),
            White.bold().paint(&self.cwd),
            git_status,
        )
    }
}

fn friendly_path(cwd: &str) -> String {
    match dirs::home_dir() {
        Some(path) => String::from(
            Regex::new(path.to_str().unwrap())
                .unwrap()
                .replace(cwd, "~"),
        ),
        _ => String::from(""),
    }
}

fn shorten_path(cwd: String) -> String {
    tico(&friendly_path(&cwd))
}

pub fn display(sub_matches: &ArgMatches<'_>) {
    let cwd = env::current_dir().unwrap();
    let git_repo = match Repository::discover(&cwd) {
        Ok(repo) => git::get_status(&repo, sub_matches.is_present("git-detailed")),
        Err(_e) => None,
    };
    let cwd_style = match sub_matches.is_present("shortened-path") {
        true => shorten_path(cwd.to_str().unwrap().to_string()),
        false => friendly_path(cwd.to_str().unwrap()),
    };

    let precmd = Precmd {
        time: Local::now(),
        git_status: git_repo,
        user: env::var("USER").unwrap(),
        host: sys_info::hostname().unwrap(),
        cwd: cwd_style,
    };

    println!("{} ", precmd);
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
    let v = &[
        Arg::with_name("git-detailed")
            .long("git-detailed")
            .help("Prints detailed git status"),
        Arg::with_name("shortened-path")
            .long("shortened-path")
            .help("~/c/rust instead of ~/.config/rust"),
    ];
    SubCommand::with_name("precmd").args(v)
}
