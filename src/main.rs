use clap::{App, Arg};
use slack_hook2::{PayloadBuilder, Slack};
use std::env;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;

fn read_stdin(stdin: &io::Stdin) -> Result<Vec<String>, ()> {
    let mut buffer = String::new();
    let mut lines: Vec<String> = vec![];
    loop {
        stdin.read_line(&mut buffer).map_err(|_e| ())?;
        if buffer.trim() == "" {
            break;
        }
        lines.push(String::from(buffer.trim()));
        buffer = String::from("");
    }

    Ok(lines)
}

fn make_msg(done: Vec<String>, todo: Vec<String>, tomorrow: bool) -> Result<String, String> {
    if done.len() == 0 || todo.len() == 0 {
        return Err(String::from("Empty Done or Todo!"));
    }

    let stardate: u64 = ((SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64)
        / ((3600 * 24) as f64)) as u64;

    let mut buf = String::new();
    buf.push_str("#checkin\n");
    buf.push_str(&format!("Stardate {}\n", stardate));
    if tomorrow {
        buf.push_str("Today:\n");
    } else {
        buf.push_str("Yesterday:\n");
    }
    for line in done {
        buf.push_str(&format!("* {}\n", line));
    }
    if tomorrow {
        buf.push_str("Tomorrow:\n");
    } else {
        buf.push_str("Today:\n");
    }
    for line in todo {
        buf.push_str(&format!("* {}\n", line));
    }

    Ok(buf)
}

struct SlackHook {
    username: String,
    channel: String,
    url: String,
}

impl SlackHook {
    fn new() -> SlackHook {
        SlackHook {
            url: env::var("STARLOG_HOOK")
                .expect("Please add STARLOG_HOOK to your environment variables"),
            channel: env::var("STARLOG_CHANNEL")
                .expect("Please add STARLOG_CHANNEL to your environment variables"),
            username: env::var("STARLOG_USERNAME")
                .expect("Please add STARLOG_USERNAME to your environment variables"),
        }
    }

    fn send(&self, done: Vec<String>, todo: Vec<String>, tomorrow: bool) -> Result<(), ()> {
        let slack = Slack::new(&self.url).unwrap();
        let p = PayloadBuilder::new()
            .text(make_msg(done, todo, tomorrow).unwrap())
            .channel(&self.channel)
            .username(&self.username)
            .build()
            .unwrap();

        Runtime::new()
            .expect("Failed to start async runtime")
            .block_on(slack.send(&p))
            .unwrap();
        Ok(())
    }
}

fn main() {
    let args = App::new("Starlog")
        .version("0.1.0")
        .author("Guillaume Pasquet")
        .about("Write your daily update")
        .arg(
            Arg::with_name("tomorrow")
                .short("t")
                .long("tomorrow")
                .takes_value(false)
                .help("Enter what you did today and what you'll do tomorrow"),
        )
        .get_matches();

    let stdin = io::stdin();
    let slack = SlackHook::new();

    if args.is_present("tomorrow") {
        println!("Enter what you did today, end with empty line");
        let done = read_stdin(&stdin).unwrap();
        println!("Enter what you'll do tomorrow, end with empty line");
        let todo = read_stdin(&stdin).unwrap();

        slack.send(done, todo, true).unwrap();
    } else {
        println!("Enter what you did yesterday, end with empty line");
        let done = read_stdin(&stdin).unwrap();
        println!("Enter what you'll do today, end with empty line");
        let todo = read_stdin(&stdin).unwrap();

        slack.send(done, todo, false).unwrap();
    }

    println!("Recorded!");
}
