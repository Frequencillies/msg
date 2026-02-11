// use std::fmt::Display;
// use std::process::{Command};
pub mod display;

// struct Snippet {
//     text: String
// }

// impl Display for Snippet {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
//         let width = display::term_size().width;
//         let block = display::enclose_text(&self.text, width);
//         write!(f, "{}", block)
//     }
// }

// struct State {
//     log: Vec<Snippet>,
// }

// fn run(command: String) {
//     Command::new(command).output();
// }

// impl State {
//     fn render(self) { 
//         run(String::from("clear")); // clear terminal
//         for msg in self.log {
//             // display all messages
//             print!("{msg}") 
//         }
//     }
// }
fn main() {
    // let mut state = State {
    //     log: Vec::new()
    // };
    // if std::env::args().any(|arg| arg == "host") {
    //     host(conversation);
    // } 
    // else if std::env::args().any(|arg| arg == "client") {
    //     client(conversation);
    // }
    

}

// fn host(conversation: Vec<Snippet>) {
//     todo!()
// }

// fn client(conversation: Vec<Snippet>) {
//     todo!()
// }

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
        println!("{enclosed_string}");
        println!("╔═╗
║a║
║o║
║s║
║n║
║e║
║u║
║t║
║h║
║a║
║n║
║o║
║e║
║u║
║x║
║n║
║a║
║s║
║o║
║e║
║n║
║u║
║h║
║t║
║a║
║o║
║e║
║u║
║a║
║o║
║t║
║e║
║u║
║h║
║s║
║a║
║o║
║n║
║t║
║e║
║h║
║u║
║a║
║s║
║o║
║n║
║t║
║e║
║h║
║u║
║a║
║o║
║s║
║e║
║n║
║t║
║u║
║h║
╚═╝");
        assert!(enclosed_string == 
"╔═╗
║a║
║o║
║s║
║n║
║e║
║u║
║t║
║h║
║a║
║n║
║o║
║e║
║u║
║x║
║n║
║a║
║s║
║o║
║e║
║n║
║u║
║h║
║t║
║a║
║o║
║e║
║u║
║a║
║o║
║t║
║e║
║u║
║h║
║s║
║a║
║o║
║n║
║t║
║e║
║h║
║u║
║a║
║s║
║o║
║n║
║t║
║e║
║h║
║u║
║a║
║o║
║s║
║e║
║n║
║t║
║u║
║h║
╚═╝")
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
        println!("╔═══════════════════════════════════════════╗
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
╚═══════════════════════════════════════════╝");
        assert!(format!("\n{}",wrapped_string) == 
"
╔═══════════════════════════════════════════╗
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
╚═══════════════════════════════════════════╝".to_string());
    }
