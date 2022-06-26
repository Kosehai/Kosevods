use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use obs_handler::test_connection;
mod combat_log_parser;
slint::include_modules!();
mod obs_handler;

fn main_old() -> io::Result<()> {
    let wowcombatlog = File::open("/home/mw/WoWCombatLog.txt")?;
    let reader = BufReader::new(wowcombatlog);

    for line in reader.lines(){
        println!("{}", line?);
    }
    Ok(())
}

fn main() {
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