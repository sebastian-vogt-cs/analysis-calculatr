use std::io;
use std::io::Write;
use std::collections::HashMap;
mod function_parser;
mod fmath;

fn main() {
    
    println!("Welcome to the analysis-calculatr, type help for help");
    let mut functions = HashMap::new(); //here all the functions the user enter are stored in
    //this infinite loop provides the command-line interface of the application
    loop{

        //take user input
        print!("> ");
        io::stdout().flush().ok().expect("error");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("error");
        input.pop();

        //send input to the command parser and execute the command
        let result:u8 = interpret_command(&input);
        if result == 10{
            print_output("define a function like that: f(x) = x, only one-character function names are allowed");
        }else if result == 30{ //store function
            let response:(Vec<(bool, f64, usize)>, bool) = function_parser::parse_function(&input);
            if response.1{
                functions.insert(input[0..1].to_string(), response.0);
                print_output("this function was saved to memory");
            }else{
                print_output("this function is not in supported notation");
            }
        }else if result == 20{ //print function
            if let Some(value) = functions.get(&input[0..1]) { //search for function in memory
                print_output(&function_parser::func_to_string(value));
            }else{
                print_output("this function is not defined");
            }
        }else if result == 60{ //derive function
            if let Some(value) = functions.get(&input[7..input.len()-3]) { //search for function in memory
                print_output(&function_parser::func_to_string(&fmath::derive(value)));
            }else{
                print_output("this function is not defined");
            }
        }else if result == 40{ //get value
            if let Some(value) = functions.get(&input[0..1]) { //search for function in memory
                let x:f64 = function_parser::get_f64_from_string(&input[2..input.len()-1]);
                print_output(&fmath::get_y_for(x, &value).to_string());
            }else{
                print_output("this function is not defined");
            }
        }else if result == 70{
            if let Some(value) = functions.get(&input[6..input.len()-3]) { //search for function in memory
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
        }else if result == 100{
            print_output("command not found");
        }

    }

}

fn interpret_command(input:&str)->u8{//what the returns mean: 10 = print help, 20 = print function, 30 = store function, 40 = get value, 60 = derive, 70 = zeros
    let mut state:u8 = 0;
    if (input.len() > 10) && (&input[0..6] == "derive"){
        state = 60;
    }else if (input.len() > 9) && (&input[0..5] == "zeros"){
        state = 70;
    }else if &input[0..input.len()] == "help"{
        state = 10;
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
    println!("\n>> {}\n", output);
}
