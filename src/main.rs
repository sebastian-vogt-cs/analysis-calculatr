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
        if result == 1{ //store function
            functions.insert(input[0..1].to_string(), function_parser::parse_function(&input));
            print_output("this function was saved to memory");
        }else if result == 2{ //print function
            if let Some(value) = functions.get(&input[0..1]) { //search for function in memory
                print_output(&function_parser::func_to_string(value));
            }else{
                print_output("this function is not defined");
            }
        }else if result == 3{ //derive function
            if let Some(value) = functions.get(&input[7..input.len()-3]) { //search for function in memory
                print_output(&function_parser::func_to_string(&fmath::derive(value)));
            }else{
                print_output("this function is not defined");
            }
        }

    }

}

//command parser
fn interpret_command(input:&str)->u8{ //returns a "status" (0 = do nothing, 1 = store function, 2 = print function, 3 = derive)
    let command:&str;
    if input.len() > 7{
        command = &input[1..7]; //cut the first letter, so we don't need to define a "define function" command for the entire alphabet (f(x), g(x),...)
    }else{
        command = &input[1..input.len()];
    }
    match command {
        "elp" => {print_output("define a function like that: f(x) = x, only one-character function names are allowed"); 0}, //prints help for help, yelp... (fix in future)
        "(x) = " => 1, //define function and store
        "(x)" => 2, //print function if exists
        "erive " => 3,
        _ => {print_output("command not found"); 0},
    }
}

//function to define all outputs, is a sepeate function to be able to swiftly change
//the style of the output without having to change every single println!
fn print_output(output:&str){
    println!("\n>> {}\n", output);
}
