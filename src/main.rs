extern crate termion;
use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::collections::HashMap;
mod function_parser;
mod fmath;


fn main() {
    
    println!("Welcome to the analysis-calculatr, type help for help");
    
    //here all the functions the user enter are stored in
    let mut functions = HashMap::new();

    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    //this infinite loop provides the command-line interface of the application
    loop{

        //take user input
        //io::stdout().flush().ok().expect("error");
        //io::stdin().read_line(&mut input).ok().expect("error");
        //input.pop();

        let (input, should_break) = get_input();
        if should_break{
            break
        }

        //send input to the command parser
        let result:u8 = interpret_command(&input);

        //execute the command: print help
        if result == 10{
            print_output("define a function like that: f(x) = x, only one-character function names are allowed");

        //store function
        }else if result == 30{
            let response:(Vec<(bool, f64, usize)>, bool) = function_parser::parse_function(&input);
            if response.1{
                functions.insert(input[0..1].to_string(), response.0);
                print_output("this function was saved to memory");
                write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
                for (name, func) in &functions{
                    write!(stdout, "{}(x) = {}, ", name, function_parser::func_to_string(func)).unwrap();
                }
                write!(stdout, "{}", termion::cursor::Hide).unwrap();
            }else{
                print_output("this function is not in supported notation");
            }

         //print function
        }else if result == 20{
            if let Some(value) = functions.get(&input[0..1]) { //search for function in memory
                print_output(&function_parser::func_to_string(value));
            }else{
                print_output("this function is not defined");
            }

        //derive function
        }else if result == 60{
            if let Some(value) = functions.get(&input[7..input.len()-3]) { //search for function in memory
                print_output(&function_parser::func_to_string(&fmath::derive(value)));
            }else{
                print_output("this function is not defined");
            }

        //get value
        }else if result == 40{
            //search for function in memory
            if let Some(value) = functions.get(&input[0..1]) {
                let x:f64 = function_parser::get_f64_from_string(&input[2..input.len()-1]);
                print_output(&fmath::get_y_for(x, &value).to_string());
            }else{
                print_output("this function is not defined");
            }

        //calculate zeros
        }else if result == 70{
            //search for function in memory
            if let Some(value) = functions.get(&input[6..input.len()-3]) {
                let zeros:Vec<f64> = fmath::get_zeros(value);
                let mut result:String = "".to_string();
                for zero in zeros{
                    result.push_str(&zero.to_string());
                    result.push_str(&" ");
                }
                print_output(&result);
            }else{
                print_output("this function is not defined");
            }
            
        //error
        }else if result == 100{
            print_output("command not found");
        }
    }
}


//interprets the entered command
//what the returns mean: 10 = print help, 20 = print function, 30 = store function, 40 = get value, 60 = derive, 70 = zeros
fn interpret_command(input:&str)->u8{
    //state variable for state machine
    let mut state:u8 = 0;
    //check if it's one of the easyer to detect commands
    if (input.len() > 10) && (&input[0..6] == "derive"){
        state = 60;
    }else if (input.len() > 9) && (&input[0..5] == "zeros"){
        state = 70;
    }else if &input[0..input.len()] == "help"{
        state = 10;
    //state machine to detect more complex commands
    }else{
        for c in input.chars(){
            match state{
                0 => {
                    match c{
                        'q'|'w'|'e'|'r'|'t'|'z'|'u'|'i'|'o'|'p'|'a'|'s'|'f'|'g'|'j'|'k'|'l'|'y'|'c'|'v'|'b'|'n'|'m'|'h'|'d' => state = 11,
                        _ => {state = 100; break;},
                    }
                },
                11 => {
                    match c{
                        '(' => state = 12,
                        _ => {state = 100; break;},
                    }
                },
                12 => {
                    match c{
                        '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => state = 31,
                        'x' => state = 13,
                        _ => {state = 100; break;},
                    }
                },
                13 => {
                    match c{
                        ')' => state = 20,
                        _ => {state = 100; break;},
                    }
                },
                20 => {
                    match c{
                        ' ' => state = 21,
                        _ => {state = 100; break;},
                    }
                },
                21 => {
                    match c{
                        '=' => state = 22,
                        _ => {state = 100; break;},
                    }
                },
                22 => {
                    match c{
                        ' ' => state = 30,
                        _ => {state = 100; break;},
                    }
                },
                30 => {
                    match c{
                        '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|' '|'+'|'-'|'x' | '.' => state = 30,
                        _ => {state = 100; break;},
                    }
                },
                31 => {
                    match c{
                        '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' | '.' => state = 31,
                        ')' => state = 40,
                        _ => {state = 100; break;},
                    }
                },
                40 => {
                    match c{
                        _ => {state = 100; break;},
                    }
                },
                _ => break,
            }
        }
    }
    state
}


//function to define all outputs, is a sepeate function to be able to swiftly change
//the style of the output without having to change every single println!
fn print_output(output:&str){
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}{}{}{}>> {}{}{}", termion::cursor::Goto(1, 30), termion::clear::CurrentLine,  color::Fg(color::Blue), style::Bold, output, color::Fg(color::Reset), style::Reset).unwrap();
    stdout.flush().unwrap();
}

fn get_input()->(String, bool){
    let mut break_afterwards:bool = false;
    let mut input = String::new();
     // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    //Clear the current line.
    write!(stdout, "{}{}> ", termion::cursor::Goto(1, 30), termion::clear::CurrentLine).unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    let mut cursor_pos:u16 = 3;

    for c in stdin.keys() {

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => {break_afterwards = true; break},
            Key::Char(c)   => {
                print!("{}", c);
                match c{
                    '¹' => input.insert(cursor_pos as usize - 3, '1'),
                    '²' => input.insert(cursor_pos as usize - 3, '2'),
                    '³' => input.insert(cursor_pos as usize - 3, '3'),
                    '⁴' => input.insert(cursor_pos as usize - 3, '4'),
                    '⁵' => input.insert(cursor_pos as usize - 3, '5'),
                    '⁶' => input.insert(cursor_pos as usize - 3, '6'),
                    '⁷' => input.insert(cursor_pos as usize - 3, '7'),
                    '⁸' => input.insert(cursor_pos as usize - 3, '8'),
                    '⁹' => input.insert(cursor_pos as usize - 3, '9'),
                    '⁰' => input.insert(cursor_pos as usize - 3, '0'),
                    _ => input.insert(cursor_pos as usize - 3, c),
                }
                write!(stdout, "{}{}> {}", termion::cursor::Goto(1, 30), termion::clear::CurrentLine, string_with_superscript(&input)).unwrap();
                cursor_pos = cursor_pos + 1;
                write!(stdout, "{}", termion::cursor::Goto(cursor_pos, 30)).unwrap();
            },
            Key::Alt(c)    => print!("Alt-{}", c),
            Key::Ctrl(c)   => print!("Ctrl-{}", c),
            Key::Left      => {
                if cursor_pos > 3{
                    print!("{}", termion::cursor::Left(1));
                    cursor_pos = cursor_pos - 1;
                }
            },
            Key::Right     => {
                if cursor_pos < input.len() as u16 + 3{
                    print!("{}", termion::cursor::Right(1));
                    cursor_pos = cursor_pos + 1;
                }
            },
            Key::Backspace => {
                if cursor_pos > 3{
                    input.remove(cursor_pos as usize - 4);
                    let old_pos = cursor_pos;
                    while cursor_pos > 0{
                        write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                        cursor_pos = cursor_pos - 1;
                    }
                    write!(stdout, "{}> {}", termion::clear::CurrentLine, string_with_superscript(&input)).unwrap();
                    write!(stdout, "{}", termion::cursor::Goto(1, 30)).unwrap();
                    write!(stdout, "{}", termion::cursor::Goto(old_pos - 1, 30)).unwrap();
                    cursor_pos = old_pos - 1;
                }
            },
            Key::Down      => break,
            _              => print!("Other"),
        }

        // Flush again.
        stdout.flush().unwrap();
    }
    (string_with_superscript(&input), break_afterwards)
}

fn string_with_superscript(input:&String)->String{
    let mut last_char = '.';
    let mut output:String = String::new();
    for c in input.as_str().chars(){
        match c{
            '1' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('¹'),
                    _ => output.push('1'),
                }
                last_char = c;
            },
            '2' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('²'),
                    _ => output.push('2'),
                }
                last_char = c;
            },
            '3' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('³'),
                    _ => output.push('3'),
                }
                last_char = c;
            },
            '4' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('⁴'),
                    _ => output.push('4'),
                }
                last_char = c;
            },
            '5' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('⁵'),
                    _ => output.push('5'),
                }
                last_char = c;
            },
            '6' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('⁶'),
                    _ => output.push('6'),
                }
                last_char = c;
            },
            '7' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('⁷'),
                    _ => output.push('7'),
                }
                last_char = c;
            },
            '8' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('⁸'),
                    _ => output.push('8'),
                }
                last_char = c;
            },
            '9' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('⁹'),
                    _ => output.push('9'),
                }
                last_char = c;
            },
            '0' => {
                match last_char{
                    'x'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'⁰'|'X' => output.push('⁰'),
                    _ => output.push('0'),
                }
                last_char = c;
            },
            _ => {
                output.push(c);
                last_char = c;
            },
        }
    }
    output
}