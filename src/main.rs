mod combat_log_parser;
pub use self::combat_log_parser::{Parser, ParserAction};
use chrono::{NaiveDateTime, Datelike};
use obs_handler::test_connection;
slint::include_modules!();
mod obs_handler;
use regex::Regex;

fn main() {
    let wowcombatlog = "/home/mw/WoWCombatLog.txt";
    let mut parser = Parser::new(wowcombatlog).unwrap();
    let re = Regex::new(r"^\d?\d/\d?\d \d\d:\d\d:\d\d").unwrap();
    //lmao was compileing the regex within the watch loop
    //tough the loop was infinite but i just made it dogshit slow
    parser.read_new_events_loop(&mut move |line: String| {
        if re.is_match(line.as_str()){
            for cap in re.captures_iter(line.as_str()) {
                //format: 6/14 00:48:36
                /*
                There is some demonic parseing going on here, will probally have to refactor for better speed
                but in theory should only be slow when initialy loading file.
                Need to parse the date to filter out old log entries before program was started
                Ideas for refac is: save cur byte position in config file so then we dont need to do the cringe date shit
                */

                /*
                in retrospect, i will for sure need a better implementation
                even tough il spin this shit up on a seperate thread to parse the log file
                its still taking a good like 10 secs to run trough this shit will all the cringe
                regex shit... im only working with like a 100MB file, and some of these fucker are multi gig

                Atlease after testing useing a ReadBuf is not fucking the memory atleast so its memory safe
                got some insane cpu usage atm but might just be my shit laptop
                */
                let current_date = chrono::Utc::now();
                let date_str = current_date.year().to_string() + "/" + &cap[0];
                let date = NaiveDateTime::parse_from_str(&date_str, "%Y/%m/%d %H:%M:%S");
                println!("{}", date.unwrap());
            }
        }
        ParserAction::None
    });
}

fn main_old() {
    let main_window = MainWindow::new();
    let main_window_weak = main_window.as_weak();
    main_window.on_connect_clicked(move || {
        let main_window = main_window_weak.unwrap();
        let ip = main_window.get_obs_ip().to_string();
        let port = main_window.get_obs_port().parse::<u16>().unwrap();
        let pass = main_window.get_obs_pass().to_string();
        let conn_test = check_conn(ip, port, pass);
        main_window.set_obs_connected(conn_test);
    });
    
    main_window.run();
}

fn check_conn(ip: String, port: u16, pass: String) -> bool {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {{
            test_connection(ip, port, pass).await.is_ok()
        }})    
}