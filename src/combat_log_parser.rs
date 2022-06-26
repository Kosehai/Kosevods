use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};
struct Parser {
    //Loading the entire BufReader into the struct
    //No clue if this will completely demolish memory, guess we will see
    filename: String,
    reader: BufReader<File>,
    pos: u64,
    finish: bool,
}

impl Parser {
    fn new(logpath: &str) -> Parser {
        let f = File::open(logpath).unwrap();
        let mut reader = BufReader::new(f);
        let pos: u64 = reader.seek(SeekFrom::Start(0)).unwrap();
        let filename: String = logpath.to_owned();
        Parser {
            filename: filename,
            reader: reader,
            pos: pos,
            finish: false
        }
    }

    fn read_new_events_loop(&self){
        loop {
            let mut line = String::new();
            let resp = self.reader.read_line(&mut line);
        }
    }
}

fn parse_log() -> io::Result<()> {
    let wowcombatlog = File::open("/home/mw/WoWCombatLog.txt")?;
    let reader = BufReader::new(wowcombatlog);

    for line in reader.lines(){
        println!("{}", line?);
    }
    Ok(())
}