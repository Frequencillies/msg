use std::fmt::Display;
use std::fmt::write;
use std::io::Stdout;
use std::io::Write;
use std::iter::repeat_n;
use std::process::{Command, Stdio};
use abes_nice_things;
const ESCAPE: char = '\x1b';
#[derive(Clone, Copy,Debug)]
struct DefaultColour {}
impl DefaultColour {
        const SEQUENCE: &'static str = "\x1b[0m"; // ANSI code to reset colour to terminal default
}

impl Display for DefaultColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DefaultColour::SEQUENCE)
    }
}
#[derive(Clone, Copy,Debug)]
struct FourBit {
    // todo
}
impl FourBit {
    fn sequence(self) -> String {
        todo!()
    }
}

#[derive(Clone, Copy,Debug)]
struct EightBit {

}
impl EightBit {
    fn sequence(self) -> String {
        todo!()
    }
}

#[derive(Clone, Copy,Debug)]
struct TrueColour {

}
impl TrueColour {
    fn sequence(self) -> String {
        todo!()
    }
}
#[derive(Clone, Copy,Debug)]
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
            Colour::Default(_) => String::from(DefaultColour::SEQUENCE),
            Colour::FourBit(four_bit) => String::from(four_bit.sequence()),
            Colour::EightBit(eight_bit) => String::from(eight_bit.sequence()),
            Colour::TrueColour(true_colour) => String::from(true_colour.sequence())
        }
    }
}
impl Display for Colour {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}", &self.sequence_string()))
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
fn wrap_string(string: &str, width: usize) -> Vec<String> {
    assert!(width > 0, "width must be 1 or greater"); // will run indefinitely otherwise
    // handle every line except the last, which might not be full length
    let mut remainder = string;
    let mut wrapped: Vec<String> = Vec::new(); // returns each line as a seperate element of the vec, 
    while {
        // only iterate when there's a full line
        let remaining_characters = remainder.chars().count();
        // width is the characters in one line
        // as long as this isn't greater than the remaining characters, loop
        // this is to avoid a bounds panic from ```string.split_at(width)```
        // if the width is equal, remainder is set to an empty string, but no panic

        let fills_line = width <= remaining_characters;
        fills_line // pass boolean to while statement
    } {
        let split_string = string.split_at(width); 
        // split string to seperate next full line
        let line = split_string.0; // assign next full line to a variable
        remainder = split_string.1; // update remainder to exclude next full line

        let remaining_space =  width - line.chars().count(); // always less than width, one character or more is a newline
        let padding = " ".repeat(remaining_space); // pad remaining with spaces
        let long_line = format!("{line}{padding}");
        wrapped.push(long_line) // add to list
    }
    // the last line might not be full, and has no remainder
    // if it has a remainder, the above while loop failed
    // instead of using split, directly assigns remainder to last line, buffers with spaces
    let line_content = string.to_string(); // needs to be a `String`
    let line_length = line_content.chars().count(); // length in characters
    let padding_length = width - line_length; // number of spaces needed at end to fill a full square
    let padding = " ".repeat(padding_length); // the spaces themselves
    let padded_line = format!("{line_content}{padding}");
    wrapped.push(line_content); // put last line back
    return wrapped
}

fn wrap_words(string: &str, width: usize) -> Vec<String> {
    assert!(width > 0, "width must be 1 or greater"); // will run indefinitely otherwise
    // handle every line except the last, which might not be full length
    let wordlist = string.split(' '); // to avoid splitting words with newlines
    let mut current_line = String::with_capacity(width); // each line will have exactly `width` elements
    let mut wrapped: Vec<String> = Vec::new(); // returns each line as a seperate element of the vec
    wrapped.push("".to_string()); // initialize vec with an empty line to avoid out of bound errors
    for word in wordlist {
        let word_length = word.chars().count(); // .len() is byte based, .chars() is still not fully correct, but it's close enough for a program this simple
        let remaining_space = width - current_line.chars().count(); // space left in line to put words
        if word_length > width {
            match remaining_space {
                0 | 1 => {
                    wrapped.push(current_line); // add full line
                    current_line = String::with_capacity(width); // reset current line
                },
                _ => {
                    current_line.push(' '); // space between words
                    let partial_line = ; //
                }
            }
        }
        else if word_length == 0 && word != ""{
            current_line.push_str(word); // first word on line
            // no padding no the first word, don't want to have all lines start with a space
        }
        else if word_length <= remaining_space { // if a padding space and the word fit onto the rest of the line
            current_line.push(' '); // add spacing
            current_line.push_str(word); // add word
        }
        else if word_length > remaining_space {
            // word does not fit on this line
            // word does fit on the next line, or it would have been caught already
            // as the first if statement catches all words that do not fit on any line
            
            todo!(); // pads rest of current line with spaces
            wrapped.push(current_line); // add full line
            current_line = String::with_capacity(width); // reset current line
            if word == "" {
                // word was a space
                // do not start newlines with spaces
                // will only cut off one space per newline
                continue;
            }
            current_line.push_str(word); // add word to newline, at start
        }
    }
    todo!()
}

//     } {
//         let split_string = string.split_at(width); 
//         // split string to seperate next full line
//         let line = split_string.0; // assign next full line to a variable
//         remainder = split_string.1; // update remainder to exclude next full line

//         let chars = line.chars(); // get line characters
//         let mut newline = false;
//         for char in chars {
//             // check each character for a newline
//             if char == '\n' {
//                 newline = true
//             }
//         }
//         match newline {
//             false => {
//                 let remaining_space =  width - line.chars().count(); // always less than width, one character or more is a newline
//                 let padding = " ".repeat(remaining_space); // pad remaining with spaces
//                 let long_line = format!("{line}{padding}");
//                 wrapped.push(long_line) // add to list
//             },
//             true => {
//                 let lines = line.split('\n');
//                 for short_line in lines {
//                     let remaining_space =  width - short_line.chars().count(); // always less than width, one character or more is a newline
//                     let padding = " ".repeat(remaining_space); // pad remaining with spaces
//                     let long_line = format!("{short_line}{padding}");
//                     wrapped.push(long_line) // add to list
//                 }
//             },
//         }
//     }
//     // the last line might not be full, and has no remainder
//     // if it has a remainder, the above while loop failed
//     // instead of using split, directly assigns remainder to last line, buffers with spaces
//     let line_content = string.to_string(); // needs to be a `String`
//     let line_length = line_content.chars().count(); // length in characters
//     let padding_length = width - line_length; // number of spaces needed at end to fill a full square
//     let padding = " ".repeat(padding_length); // the spaces themselves
//     let padded_line = format!("{line_content}{padding}");
//     wrapped.push(padded_line); // put last line back
//     return wrapped
// }


fn enclose_text(text: String, width: usize) -> String {
    let mut wrapped_vec = Vec::new();
    // ╔═╗
    // ║ ║
    // ╚═╝ 
    let straight_line = "═".repeat(width - 2); // top line except corners
    let top_line = format!("╔{straight_line}╗");
    wrapped_vec.push(top_line);
    wrapped_vec = [wrapped_vec, wrap_string(&text, width)].concat();
    for (i, _) in wrapped_vec.iter().enumerate() {
        let next_line = format!("║{}║", wrapped_vec);
        wrapped_vec.push(next_line);
    }
    let botttom_line = format!("╚{straight_line}╝");
    wrapped_vec.push(botttom_line);
    for line in wrapped_vec {
        write!(std::io::stdout(), "{}", line);
        todo!()
    };
    todo!()
}







































// fn enclose_text(text: String, width: usize) -> String {
//     let mut lines: Vec<String> = Vec::new();
//     let mut current_line = 0;
//     let mut words = text.split(' ').enumerate().peekable(); // split by spaces for better word wrapping, 
//     let line_width = width - 2; // requires 
//     let mut remaining_space = line_width;
//     let mut words_in_line = 0;
//     while let Some((i, word)) = words.next() {
//         let mut word = word;
//         let mut word_width = word.chars().count();
//         // handles spaces when there's more than one in a row
//         // second space and above gives an iterator of '', resetting when 
//         if word_width == 0 {
//             word = " ";
//             word_width = 1;   
//         }
//         if word == "\n" {
//             current_line += 1;
//             continue 
//         }
//         // checks if any text is on the current line
//         match remaining_space <= line_width {
//             true => {
//                 // check if there's space to fit the word on the current line
//                 if remaining_space >= word_width {
//                     remaining_space -= word_width;
//                     lines[current_line] += word;
//                     // if the next word won't fit on the line, move to the next
//                     match Some(words.peek()) {
//                         Some(_) => todo!(),
//                         None => todo!(),
//                     } 
//                 }
//             },
//             false => todo!(),
//         }
//         // if there's enough space to put the word on the current line, do so. 
//         // to allow for spaces, 
//         if remaining_space >  word_width || (remaining_space >= word_width && remaining_space == line_width) { 
//             remaining_space -= word_width; // update how much space there is for any other words on the line
//             lines[current_line] += word; // adds the word to the line 
//             match &words.peek() {
//                 Some(next_word) => {
//                     let next_size = next_word.0;
//                     if next_size > remaining_space {
//                         current_line +=1;
//                         remaining_space = line_width;
//                 }
//             },
//                 None => todo!(),
//             }
            
//         }
//         else if remaining_space == line_width {
//             for i in 0..(word.len()/words_in_line) {
//                 let end_of_next_line = line_width*(i+1);
//                 if end_of_next_line > word.len() - line_width*i {
//                     remaining_space -= word.len();
//                     words_in_line += word.len();
//                     lines[current_line] += word;
//                 }
//                 lines[current_line] = String::from(&word[line_width*i..usize::min(line_width*(i+1), word.len())]);
//                 // lines[current_line] = String::from(&word[0..line_width])
//                 // lines[current_line] = String::from(&word[0..line_width]);
//                 // current_line += 1;
//                 // if word.len()    <=  line_width * 2{
//                 //     lines[current_line] = String::from(&word[line_width..word.len()])
//             }
//         }
//     }
//     todo!()
// }
// #[cfg(test)]
// mod tests {
//     use crate::display::wrap_string;

//     #[test]
//     #[should_panic]
//     fn no_width() {
//         let string = "aosneuthanoeuxnasoenuhtaoeuaoteuhsaontehuasontehuaosentuh";
//         wrap_string(&string, 0);
//     }
//     #[test]
//     fn one_width() {
//         let string = "aosneuthanoeuxnasoenuhtaoeuaoteuhsaontehuasontehuaosentuh";
//         let mut big_string = "".to_string();
//         for _ in 0..16 {
//             big_string.push_str(string) 
//         }
//         let wrapped_string = wrap_string(&big_string, 43);
//         assert!(wrapped_string == vec!["aosneuthanoeuxnasoenuhtaoeuaoteuhsaontehuas",
//                                         "ontehuaosentuhaosneuthanoeuxnasoenuhtaoeuao",
//                                         "teuhsaontehuasontehuaosentuhaosneuthanoeuxn",
//                                         "asoenuhtaoeuaoteuhsaontehuasontehuaosentuha",
//                                         "osneuthanoeuxnasoenuhtaoeuaoteuhsaontehuaso",
//                                         "ntehuaosentuhaosneuthanoeuxnasoenuhtaoeuaot",
//                                         "euhsaontehuasontehuaosentuhaosneuthanoeuxna",
//                                         "soenuhtaoeuaoteuhsaontehuasontehuaosentuhao",
//                                         "sneuthanoeuxnasoenuhtaoeuaoteuhsaontehuason",
//                                         "tehuaosentuhaosneuthanoeuxnasoenuhtaoeuaote",
//                                         "uhsaontehuasontehuaosentuhaosneuthanoeuxnas",
//                                         "oenuhtaoeuaoteuhsaontehuasontehuaosentuhaos",
//                                         "neuthanoeuxnasoenuhtaoeuaoteuhsaontehuasont",
//                                         "ehuaosentuhaosneuthanoeuxnasoenuhtaoeuaoteu",
//                                         "hsaontehuasontehuaosentuhaosneuthanoeuxnaso",
//                                         "enuhtaoeuaoteuhsaontehuasontehuaosentuhaosn",
//                                         "euthanoeuxnasoenuhtaoeuaoteuhsaontehuasonte",
//                                         "huaosentuhaosneuthanoeuxnasoenuhtaoeuaoteuh",
//                                         "saontehuasontehuaosentuhaosneuthanoeuxnasoe",
//                                         "nuhtaoeuaoteuhsaontehuasontehuaosentuhaosne",
//                                         "uthanoeuxnasoenuhtaoeuaoteuhsaontehuasonteh",
//                                         "uaosentuh"]);
//         todo!()
//     }
//     // fn it_works() {
//     //     let result = add(2, 2);
//     //     assert_eq!(result, 4);
//     // }
// }
