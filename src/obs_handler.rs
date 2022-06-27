use anyhow::Result;
use obws::Client;
use regex::Regex;
mod combat_log_parser;
pub use self::combat_log_parser::{Parser, ParserAction};
use chrono::{NaiveDateTime, Datelike};


pub fn recording_handler(ip: String, port: u16, pass: String, wowcombatlog: String) -> Result<()> {
    println!("spawned worker");
    let mut parser = Parser::new(wowcombatlog).unwrap();
    let client = Client::connect(ip, port).await?;
    client.login(Some(pass)).await?;
    let re = Regex::new(r"^\d?\d/\d?\d \d\d:\d\d:\d\d").unwrap();

    let mut recording: bool = false;

    let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

    //lmao was compileing the regex within the watch loop
    //tough it made the loop was infinite but i just made it dogshit slow
    parser.read_new_events_loop(&mut move |line: String| {
        if re.is_match(line.as_str()){
            let mut log_date = String::new();
            for cap in re.captures_iter(line.as_str()) {
                //format: 6/14 00:48:36
                /*
                Going back over the parser, i did some dump shit was no point of parseing whole file
                could just get the len from the metadata... This code should be fine now
                not really any point converting the date now, but its nice to have i guess for later
                */
                let current_date = chrono::Utc::now();
                let date_str = current_date.year().to_string() + "/" + &cap[0];
                let date = NaiveDateTime::parse_from_str(&date_str, "%Y/%m/%d %H:%M:%S");
                log_date = date.unwrap().to_string();
            }

            if line.contains("ARENA_MATCH_START") && recording == false {
                recording = true;
                rt.block_on(async {
                    start_stop_recording(recording, &client).await;
                })  
            }

            if line.contains("ARENA_MATCH_END") && recording == false {
                recording = false;
                rt.block_on(async {
                    start_stop_recording(recording, &client).await;
                }) 
            }
        }
        ParserAction::None
    });

    Ok(())
}

async fn start_stop_recording(recording: bool, client: &Client) {
    if recording == true {
        client.recording().start_recording().await;
    } else {
        client.recording().stop_recording().await;
    }
}

pub async fn test_connection(ip: String, port: u16, pass: String) -> Result<()> {
    let client = Client::connect(ip, port).await?;
    client.login(Some(pass)).await?;
    client.general().get_version().await?;
    Ok(())
}

