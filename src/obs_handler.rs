use anyhow::Result;
use obws::{Client, responses::Version};
use regex::Regex;
mod combat_log_parser;
pub use self::combat_log_parser::{Parser, ParserAction};
use chrono::{NaiveDateTime, Datelike};

pub struct Handler {
    client: Client,
    pub ip: String,
    pub port: u16,
    pub pass: String,
    pub logpath: String
}

impl Handler {
    pub fn new(ip: String, port: u16, pass: String, logpath: String) -> Handler{
        let cli = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {{
            let client = Client::connect(&ip, port).await.unwrap();
            client.login(Some(&pass)).await.unwrap();
            client
        }});
        Handler {
            ip: ip,
            port: port,
            pass: pass,
            logpath: logpath,
            client: cli
        }

    }

    pub fn check_conn(&self) -> Result<Version, obws::Error>{
        let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
        let conn = rt.block_on(async {
            self.client.general().get_version().await
        });
        conn
    }

    pub fn recording_handler(&self){
        let re = Regex::new(r"^\d?\d/\d?\d \d\d:\d\d:\d\d").unwrap();

        let mut recording: bool = false;

        let mut parser = Parser::new(&self.logpath).unwrap();
    
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

                println!("new line: {}", line);
                
                if line.contains("ARENA_MATCH_START") && recording == false {
                    recording = true;
                    tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        &self.client.recording().start_recording().await.unwrap();
                    })  
                }
    
                if line.contains("ARENA_MATCH_END") && recording == false {
                    recording = false;
                    tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        &self.client.recording().stop_recording().await.unwrap();
                    }) 
                }
            }
            ParserAction::None
        });
    }
}

