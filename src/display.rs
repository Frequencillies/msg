use std::process::{Command, Stdio};

pub struct TermSize{
    pub width: usize,
    pub height: usize
}
pub fn term_size() -> TermSize {
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

fn wrap_words(string: &str, width: usize) -> Vec<String> {
    assert!(width > 0, "width must be 1 or greater"); // will run indefinitely otherwise
    // handle every line except the last, which might not be full length
    let wordlist = string.split(' '); // to avoid splitting words with newlines
    let mut current_line = String::with_capacity(width); // each line will have exactly `width` elements
    let mut wrapped: Vec<String> = Vec::new(); // returns each line as a seperate element of the vec
    for word in wordlist {
        let word_length = word.chars().count(); // .len() is byte based, .chars() is still not fully correct, but it's close enough for a program this simple
        // println!("remaining space = {width} - length of '{current_line}', or {}", width - current_line.chars().count());
        let remaining_space = width - current_line.chars().count(); // space left in line to put words
        if word_length > width {
            // println!("word ('{word}') too large, wrapping");
            let empty_slots = remaining_space; // how much padding is needed?
            // println!("empty slots = '{empty_slots}'");
            for _ in 0..empty_slots {current_line.push(' ');}; // pad by that much
            // println!("padded line = '{current_line}'");
            wrapped.push(current_line); // add full line
            current_line = String::with_capacity(width); // reset current line
            // by this point, line is complete, and should be pushed and reset before further use

            for i in 0..(word_length / width) /* every full line */ {
                        let this_range = (i*width)..((i+1)*width);
                        let this_line: &str = &word[this_range];
                        wrapped.push(this_line.to_string());
            }
            let residual: usize = word_length % width;
            if residual > 0 {
                let last_line_start: usize = word_length - residual;
                let word_slice: &str = &word[last_line_start..];
                current_line.push_str(word_slice);
                for _ in 0..(width - residual) {
                    current_line.push(' ');
                }
                wrapped.push(current_line)
            }
            current_line = String::with_capacity(width);
        }
        else if remaining_space == width && word != ""{
            // println!("\nfirst word in this line; '{word}'");
            current_line.push_str(word); // first word on line
            // no padding no the first word, don't want to have all lines start with a space
        }
        else if word_length + 1 <= remaining_space { // if a padding space and the word fit onto the rest of the line
            // + 1 handles the padding space 
            // println!("padding word; '{word}'");
            // println!("current line; '{current_line}'");
            current_line.push(' '); // add spacing
            current_line.push_str(word); // add word
        }
        else if word_length + 1 > remaining_space {
            // word does not fit on this line with a padding space
            // word does fit on the next line, or it would have been caught already
            // as the first if statement catches all words that do not fit on any line
            // println!("\npadding and putting word on next line; '{word}'");

            for _ in 0..remaining_space {current_line.push(' ')}; // pads rest of current line with spaces
            // print!("padded_line; '{current_line}'");
            wrapped.push(current_line); // add full line
            
            current_line = String::with_capacity(width); // reset current line
            current_line.push_str(word); // put word at start of next line
            // println!("next line; '{current_line}'")
        } 
    }
    if current_line != String::new() {
        wrapped.push(current_line);

    }
    wrapped
}

pub fn enclose_text(text: &String, width: usize) -> String {
    assert!(text.len() > 0); // i don't want to handle this
    assert!(width > 2);
    // so im going to make it me when calling the function's problem
    let wrapped_text = wrap_words(&text, width - 2);
    // ╔═╗
    // ║ ║
    // ╚═╝ 
    let straight_line = "═".repeat(width - 2); // top line except corners
    let top_line = format!("╔{straight_line}╗"); //top line with corners
    let mut boxxed_vec = Vec::new();
    boxxed_vec.push(top_line);
    for line in wrapped_text {
        let padding = " ".repeat(width - (2 +line.chars().count()));
        let boxxed_line = format!("║{line}{padding}║");
        boxxed_vec.push(boxxed_line);
    }
    let botttom_line = format!("╚{straight_line}╝");
    boxxed_vec.push(botttom_line);
    let mut vec_iter = boxxed_vec.into_iter();
    let mut boxxed_text = vec_iter.next().unwrap();
    for line in vec_iter {
        boxxed_text = format!("{boxxed_text}\n{line}");
    }
    boxxed_text
}






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
