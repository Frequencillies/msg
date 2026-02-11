use std::fmt::Display;
use std::fmt::write;
use std::process::{Command, Stdio};
mod display;

const ESCAPE: char = '\x1b';
#[derive(Debug)]
struct DefaultColour {}
impl DefaultColour {
        const SEQUENCE: &'static str = "\x1b[0m"; // ANSI code to reset colour to terminal default
}

impl Display for DefaultColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DefaultColour::SEQUENCE)
    }
}
#[derive(Debug)]
struct FourBit {

}
impl FourBit {
    fn sequence(self) -> String {
        todo!()
    }
}

#[derive(Debug)]
struct EightBit {

}
impl EightBit {
    fn sequence(self) -> String {
        todo!()
    }
}

#[derive(Debug)]
struct TrueColour {

}
impl TrueColour {
    fn sequence(self) -> String {
        todo!()
    }
}
#[derive(Debug)]
enum Colour {
    None,
    Default(DefaultColour),
    FourBit(FourBit),
    EightBit(EightBit),
    TrueColour(TrueColour)
}
// env variable for color type: COLORTERM
impl Colour {
    fn sequence_string(self) -> String {
        match self {
            Colour::None => String::new(),
            Colour::Default(default_color) => String::from(default_color.sequence()),
            Colour::FourBit(four_bit) => String::from(four_bit.sequence()),
            Colour::EightBit(eight_bit) => String::from(eight_bit.sequence()),
            Colour::TrueColour(true_colour) => String::from(true_colour.sequence())
        }
    }
}
impl Display for Colour {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}", &str::from(self.sequence_string())))
    }
}
struct User {
    uid: u64,
    name: String,
    // colour: Colour,  // maybe later
    config: std::path::PathBuf
}
struct Snippet {
    author: User,
    message: String,
    // colour: Colour // see above
}
struct TermSize{
    width: usize,
    height: usize
}
fn term_size() -> TermSize {
    let stty_size = Command::new("stty")
        .arg("size")
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .unwrap()
        .wait_with_output()
        .expect("stty failed");
    let string = String::from_utf8(stty_size.stdout).unwrap();
    let size: Vec<&str> = string.split_whitespace().collect();
    let width = size[0].parse::<usize>().unwrap();
    let height = size[1].parse::<usize>().unwrap();
    TermSize { width, height }
}
fn wrap_string(string: String, width: usize, mut prepend: Vec<String>) -> Vec<String> {
    // let max_lines = string.len().div_ceil(width); // 
    let mut un_wrapped = string;
    // handle every line until the last, which might not be full length
    while width < un_wrapped.chars().count() {
        // get the next line's worth of the string
        let (line, remainder) = un_wrapped.split_at(width);
        // append the line
        prepend.push(line.to_string());
        un_wrapped = String::from(remainder);
    }
    // the last line might not be full, so it must be handled carefully
    let last_line = un_wrapped[0..un_wrapped.chars().count()].to_string();
    prepend.push(last_line);
    return prepend
}

fn enclose_text(text: String, width: usize) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = 0;
    let mut words = text.split(' ').enumerate().peekable(); // split by spaces for better word wrapping, 
    let line_width = width - 2;
    let mut remaining_space = line_width;
    let mut words_in_line = 0;
    while let Some((i, word)) = words.next() {
        let mut word = word;
        let mut word_width = word.chars().count();
        // handles spaces when there's more than one in a row
        // second space and above gives an iterator of '', resetting when 
        if word_width == 0 {
            word = " ";
            word_width = 1;   
        }
        if word == "\n" {
            current_line += 1;
            continue 
        }
        // checks if any text is on the current line
        match remaining_space == line_width {
            true => {
                // check if there's space to fit the word on the current line
                if remaining_space >= word_width {
                    remaining_space -= word_width;
                    lines[current_line] += word;
                    // if the next word won't fit on the line, move to the next
                    match Some(words.peek()) {
                        Some(_) => todo!(),
                        None => todo!(),
                    } 
                }
            },
            false => todo!(),
        }
                // if there's enough space to put the word on the current line, do so. 
        // to allow for spaces, 
        if remaining_space >  word_width || (remaining_space >= word_width && remaining_space == line_width) { 
            remaining_space -= word_width; // update how much space there is for any other words on the line
            lines[current_line] += word; // adds the word to the line 
            match &words.peek() {
                Some(next_word) => {
                    let next_size = next_word.0;
                    if next_size > remaining_space {
                        current_line +=1;
                        remaining_space = line_width;
                }
            },
                None => todo!(),
            }
            
        }
        else if remaining_space == line_width {
            for i in 0..(word.len()/words_in_line) {
                let end_of_next_line = line_width*(i+1);
                if end_of_next_line > word.len() - line_width*i {
                    remaining_space -= word.len();
                    words_in_line += word.len();
                    lines[current_line] += word;
                }
                lines[current_line] = String::from(&word[line_width*i..usize::min(line_width*(i+1), word.len())]);
                // lines[current_line] = String::from(&word[0..line_width])
                // lines[current_line] = String::from(&word[0..line_width]);
                // current_line += 1;
                // if word.len()    <=  line_width * 2{
                //     lines[current_line] = String::from(&word[line_width..word.len()])
            }
        }
    }
    todo!()
}
impl Display for Snippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        let width = term_size().width;
        let block = 
        write!(f, "{}", block)
    }
}

struct State {
    log: Vec<Snippet>,
    active: [Vec<Snippet>; 2] //this may be changed if expanded to more users
}
fn run(command: String) {
    Command::new(command).output();
}
impl State {
    fn render(self) -> String { 
        run(String::from("clear"));
        for i in self.log {
            println!("{i}")
        }
        return  todo!()
    }
}
fn main() {
    let mut conversation = Vec::new();
    let user;
    if std::env::args().any(|arg| arg == "host") {
        host(conversation);
    } 
    else if std::env::args().any(|arg| arg == "client") {
        client(conversation);
    }
    if std::env::args().any(|arg| arg == "user") {
        let user = todo!(); 
    }
    else {
        println!("Provide a username");
        loop {
            let input = abes_nice_things::input().to_lowercase();
            todo!()
        }
    }
}

fn host(conversation: Vec<Snippet>) {
    todo!()
}

fn client(conversation: Vec<Snippet>) {
    todo!()
}
