use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};

pub enum ParserAction {
    None,
    SeekToEnd
}

pub struct Parser {
    //Loading the entire BufReader into the struct
    //No clue if this will completely demolish memory, guess we will see
    filename: String,
    reader: BufReader<File>,
    pos: u64,
    finish: bool,
}

impl Parser {
    pub fn new(logpath: &str) -> Result<Parser, io::Error> {
        let f = match File::open(logpath){
            Ok(x) => x,
            Err(err) => return Err(err)
        };
        let mut reader = BufReader::new(f);
        let pos: u64 = reader.seek(SeekFrom::Start(0)).unwrap();
        let filename: String = logpath.to_owned();
        Ok(Parser {
            filename: filename,
            reader: reader,
            pos: pos,
            finish: false
        })
    }

    pub fn read_new_events_loop<F: ?Sized>(&mut self, callback: &mut F)
        where F: FnMut(String) -> ParserAction,
        {
            loop {
                let mut line = String::new();
                let resp = self.reader.read_line(&mut line);
                match resp {
                    Ok(len) => {
                        if len > 0 {
                            self.pos += len as u64;
                            self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                            match callback(line.replace("\n", "")) {
                                ParserAction::SeekToEnd => {
                                    println!("Found new lines in combatlog");
                                    self.reader.seek(SeekFrom::End(0)).unwrap();
                                }
                                ParserAction::None => {}
                            }
                            line.clear();
                        } else {
                            if self.finish {
                                break;
                            } else {
                                //IO error need to reopen file
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
        }
}