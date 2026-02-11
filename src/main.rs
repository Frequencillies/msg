use std::fmt::Display;
use std::io::Write;
use std::process::{Command};
use std::net::TcpStream;
use abes_nice_things::{self, FromBinary, ToBinary, input};

use crate::display::term_size;
pub mod display;

#[derive(Clone)]
struct Snippet {
    text: String
}
impl ToBinary for Snippet {
    fn to_binary(&self, binary: &mut dyn Write) -> Result<(), std::io::Error> {
        (*self.text).to_string().to_binary(binary)
    }
}
impl FromBinary for Snippet {
    fn from_binary(binary: &mut dyn std::io::Read) -> Result<Self, std::io::Error>
    where
        Self: Sized {
        Ok(Snippet { text: String::from_binary(binary)?} )
    }
}
impl Display for Snippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        let width = term_size().width;
        let block = display::enclose_text(&self.text, width);
        let line = "-".repeat(term_size().width);
        write!(f, "{}\n{}\n", block, line)
    }
}

struct State {
    log: Vec<Snippet>,
}

fn run(command: String) {
    let _ = Command::new(command).output();
}

impl State {
    fn render(&self) { 
        run(String::from("clear")); // clear terminal
        for msg in &self.log {
            // display all messages
            print!("{msg}") 
        }
    }
}
impl ToBinary for State {
    fn to_binary(&self, binary: &mut dyn Write) -> Result<(), std::io::Error> {
        self.log.to_binary(binary)
    }
}
impl FromBinary for State {
    fn from_binary(binary: &mut dyn std::io::Read) -> Result<Self, std::io::Error>
    where
        Self: Sized {
        Ok(State{log: Vec::from_binary(binary)?})
    }
}
fn main() {
    if std::env::args().any(|arg| arg == "host") {
        host();
    } 
    else if std::env::args().any(|arg| arg == "client") {
        client();
    }
    

}
fn new_msg(snippet: Snippet, state: &mut State, mut stream: &TcpStream) {
    state.log.push(snippet.clone());
    state.render();
    snippet.clone().to_binary(&mut stream).unwrap();
}
fn host() {
    let mut port;
    let mut state = State{log: Vec::new()};
    let mut stream;
    loop {
        loop {
            println!("port to await");
            match input().parse::<u16>() {
                Ok(parsed_port) => {port = parsed_port; break;},
                Err(_) => println!("port must be a u16"),
            }
        }
        match std::net::TcpListener::bind((std::net::Ipv4Addr::UNSPECIFIED, port)) {
            Ok(listener) => {
                match listener.accept() {
                    Ok((tcpstream, _)) => {stream = tcpstream; break},
                    Err(err) => {println!("{err}"); continue}
                }
            },
            Err(err) => {println!("{err}"); continue}
        };
    }
    state.render();
    loop {
        let new_message = Snippet{ text: input()};
        new_msg(new_message, &mut state, &mut stream);
        state.log.push(match Snippet::from_binary(&mut stream) {
            Ok(snippet) => snippet,
            Err(_) => continue,
        });
        state.render();
    }
}


fn client() {
    let mut stream; 
    let mut state = State{log: Vec::new()};
    loop {
        println!("address:port to connect to:");
        let address = input();
        println!("address; '{address}'");
        match TcpStream::connect(address) {
            Ok(connected_stream) => {
                stream = connected_stream;
                break
            },
            Err(err) => println!("{err}"),
        }
    }
    state.render();
    loop {
        let new_message = Snippet{ text: input()};
        new_msg(new_message, &mut state, &mut stream);
        state.log.push(match Snippet::from_binary(&mut stream) {
            Ok(snippet) => snippet,
            Err(_) => continue,
        });
        state.render();
    }
}

    #[test]
    #[should_panic]
    fn no_width() {
        let string: String = "aosneuthanoeuxnasoenuhtaoeuaoteuhsaontehuasontehuaosentuh".to_string();
        display::enclose_text(&string, 0);
    }
    #[test]
    fn three_width() {
        let string = "aosneuthanoeuxnasoenuhtaoeuaoteuhsaontehuasontehuaosentuh".to_string();
        let enclosed_string = display::enclose_text(&string, 3);

        assert!(enclosed_string == "╔═╗\n║ ║\n║a║\n║o║\n║s║\n║n║\n║e║\n║u║\n║t║\n║h║\n║a║\n║n║\n║o║\n║e║\n║u║\n║x║\n║n║\n║a║\n║s║\n║o║\n║e║\n║n║\n║u║\n║h║\n║t║\n║a║\n║o║\n║e║\n║u║\n║a║\n║o║\n║t║\n║e║\n║u║\n║h║\n║s║\n║a║\n║o║\n║n║\n║t║\n║e║\n║h║\n║u║\n║a║\n║s║\n║o║\n║n║\n║t║\n║e║\n║h║\n║u║\n║a║\n║o║\n║s║\n║e║\n║n║\n║t║\n║u║\n║h║\n╚═╝")
    }
    #[test]
    fn many_width() {
        let string = "aosneuthanoeuxnasoenuhtaoeuaoteuhsaontehuasontehuaosentuh";
        let mut big_string = "".to_string();
        for _ in 0..16 {
            big_string.push_str(string) 
        }
        let wrapped_string = display::enclose_text(&big_string, 45);
        println!("{wrapped_string}");
        assert!(format!("\n{}",wrapped_string) == 
"
╔═══════════════════════════════════════════╗
║                                           ║
║aosneuthanoeuxnasoenuhtaoeuaoteuhsaontehuas║
║ontehuaosentuhaosneuthanoeuxnasoenuhtaoeuao║
║teuhsaontehuasontehuaosentuhaosneuthanoeuxn║
║asoenuhtaoeuaoteuhsaontehuasontehuaosentuha║
║osneuthanoeuxnasoenuhtaoeuaoteuhsaontehuaso║
║ntehuaosentuhaosneuthanoeuxnasoenuhtaoeuaot║
║euhsaontehuasontehuaosentuhaosneuthanoeuxna║
║soenuhtaoeuaoteuhsaontehuasontehuaosentuhao║
║sneuthanoeuxnasoenuhtaoeuaoteuhsaontehuason║
║tehuaosentuhaosneuthanoeuxnasoenuhtaoeuaote║
║uhsaontehuasontehuaosentuhaosneuthanoeuxnas║
║oenuhtaoeuaoteuhsaontehuasontehuaosentuhaos║
║neuthanoeuxnasoenuhtaoeuaoteuhsaontehuasont║
║ehuaosentuhaosneuthanoeuxnasoenuhtaoeuaoteu║
║hsaontehuasontehuaosentuhaosneuthanoeuxnaso║
║enuhtaoeuaoteuhsaontehuasontehuaosentuhaosn║
║euthanoeuxnasoenuhtaoeuaoteuhsaontehuasonte║
║huaosentuhaosneuthanoeuxnasoenuhtaoeuaoteuh║
║saontehuasontehuaosentuhaosneuthanoeuxnasoe║
║nuhtaoeuaoteuhsaontehuasontehuaosentuhaosne║
║uthanoeuxnasoenuhtaoeuaoteuhsaontehuasonteh║
║uaosentuh                                  ║
╚═══════════════════════════════════════════╝".to_string())
}
    #[test]
    fn three_width_ipsum() {
        let string = "Lorem ipsum dolor sit amet".to_string();
        let wrapped_string = display::enclose_text(&string, 3);
        println!("{}", wrapped_string);
        println!("╔═╗\n║ ║\n║L║\n║o║\n║r║\n║e║\n║m║\n║ ║\n║i║\n║p║\n║s║\n║u║\n║m║\n║ ║\n║d║\n║o║\n║l║\n║o║\n║r║\n║ ║\n║s║\n║i║\n║t║\n║ ║\n║a║\n║m║\n║e║\n║t║\n╚═╝\n");
        assert!(wrapped_string == "╔═╗
║ ║
║L║
║o║
║r║
║e║
║m║
║ ║
║i║
║p║
║s║
║u║
║m║
║ ║
║d║
║o║
║l║
║o║
║r║
║ ║
║s║
║i║
║t║
║ ║
║a║
║m║
║e║
║t║
╚═╝"
);
    }
    #[test]
    fn many_width_ipsum() {
        let string = "Lorem ipsum dolor sit amet consectetur adipiscing elit. Consectetur adipiscing elit quisque faucibus ex sapien vitae. Ex sapien vitae pellentesque sem placerat in id. Placerat in id cursus mi pretium tellus duis. Pretium tellus duis convallis tempus leo eu aenean.".to_string();
        let wrapped_string = display::enclose_text(&string, 12);
        println!("{wrapped_string}");
        println!("╔══════════╗
║Lorem     ║
║ipsum     ║
║dolor sit ║
║amet      ║
║consectetu║
║r         ║
║adipiscing║
║elit.     ║
║Consectetu║
║r         ║
║adipiscing║
║elit      ║
║quisque   ║
║faucibus  ║
║ex sapien ║
║vitae. Ex ║
║sapien    ║
║vitae     ║
║pellentesq║
║ue        ║
║sem       ║
║placerat  ║
║in id.    ║
║Placerat  ║
║in id     ║
║cursus mi ║
║pretium   ║
║tellus    ║
║duis.     ║
║Pretium   ║
║tellus    ║
║duis      ║
║convallis ║
║tempus leo║
║eu aenean.║
╚══════════╝");
        assert!(wrapped_string == "╔══════════╗
║Lorem     ║
║ipsum     ║
║dolor sit ║
║amet      ║
║consectetu║
║r         ║
║adipiscing║
║elit.     ║
║Consectetu║
║r         ║
║adipiscing║
║elit      ║
║quisque   ║
║faucibus  ║
║ex sapien ║
║vitae. Ex ║
║sapien    ║
║vitae     ║
║pellentesq║
║ue        ║
║sem       ║
║placerat  ║
║in id.    ║
║Placerat  ║
║in id     ║
║cursus mi ║
║pretium   ║
║tellus    ║
║duis.     ║
║Pretium   ║
║tellus    ║
║duis      ║
║convallis ║
║tempus leo║
║eu aenean.║
╚══════════╝")
    }