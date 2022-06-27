mod obs_handler;
use obs_handler::{test_connection, recording_handler};
use std::thread;
slint::include_modules!();



fn main() {
    let main_window = MainWindow::new();
    let mut main_window_weak = main_window.as_weak();
    main_window.on_connect_clicked(move || {
        let main_window = main_window_weak.unwrap();
        let ip = main_window.get_obs_ip().to_string();
        let port = main_window.get_obs_port().parse::<u16>().unwrap();
        let pass = main_window.get_obs_pass().to_string();
        let conn_test = check_conn(ip, port, pass);
        main_window.set_obs_connected(conn_test);
    });

    main_window_weak = main_window.as_weak();
    main_window.on_record_clicked(move || {
        let main_window = main_window_weak.unwrap();
        println!("Clicked");
        let ip = main_window.get_obs_ip().to_string();
        let port = main_window.get_obs_port().parse::<u16>().unwrap();
        let pass = main_window.get_obs_pass().to_string();
        let wowcombatlog = main_window.get_combatlog_path().to_string();
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .spawn_blocking(move || {
            recording_handler(ip, port, pass, wowcombatlog)
        });
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