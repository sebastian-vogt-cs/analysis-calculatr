use std::io;
use std::io::Write;
use std::collections::HashMap;
mod function_parser;

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
        //sned input to the command parser
        if interpret_command(&input) == 1{
            functions.insert(input[0..1].to_string(), function_parser::parse_function(&input));
            print_output("this function was saved to memory");
        }else if interpret_command(&input) == 2{
            if let Some(value) = functions.get(&input[0..1]) {
                print_output(&function_parser::func_to_string(value));
            }else{
                print_output("this function is not defined");
            }
        }

    }

}

//command parser
fn interpret_command(input:&str)->u8{ //returns a "status" (0 = do nothing, 1 = store function, 2 = print function)
    let mut command:&str;
    if input.len() > 7{
        command = &input[1..7]; //cut the first letter, so we don't need to define a "define function" command for the entire alphabet (f(x), g(x),...)
    }else{
        command = &input[1..input.len()];
    }
    match command {
        "elp" => {print_output("define a function like that: f(x) = x, only one-character function names are allowed"); 0}, //prints help for help, yelp... (fix in future)
        "(x) = " => 1, //define function and store
        "(x)" => 2, //print function if exists
        _ => {print_output("command not found"); 0},
    }
}

//function to define all outputs, is a sepeate function to be able to swiftly change
//the style of the output without having to change every single println!
fn print_output(output:&str){
    println!("\n>> {}\n", output);
}
