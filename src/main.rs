mod obs_handler;
use self::obs_handler::Handler;
use std::thread;
slint::include_modules!();



fn main() {
    let main_window = MainWindow::new();
    let mut main_window_weak = main_window.as_weak();

    main_window_weak = main_window.as_weak();
    main_window.on_record_clicked(move || {
        let main_window = main_window_weak.unwrap();
        println!("Clicked");
        let ip = main_window.get_obs_ip().to_string();
        let port = main_window.get_obs_port().parse::<u16>().unwrap();
        let pass = main_window.get_obs_pass().to_string();
        let wowcombatlog = main_window.get_combatlog_path().to_string();
        thread::spawn(move || {
            let handler = Handler::new(ip, port, pass, wowcombatlog);
            handler.recording_handler();
        });     
            
    });
    
    main_window.run();
}
