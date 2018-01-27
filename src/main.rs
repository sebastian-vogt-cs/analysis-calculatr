use std::io;
use std::io::Write;
mod function_parser;

fn main() {

    println!("Welcome to the analysis-calculatr, type help for help");

    //this infinite loop provides the command-line interface of the application
    loop{

        //take user input
        print!("> ");
        io::stdout().flush().ok().expect("error");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("error");
        input.pop();
        //sned input to the command parser
        parse_command(&input);

    }

}

//command parser
fn parse_command(input:&str){
    let command:&str;
    if input.len() > 6 {
        command = &input[1..7];
    }else{
        command = input;
    }
    match command {
        "help" => print_output("define a function like that: f(x) = x, only
            one-character function names are allowed"), //prints help
        "(x) = " => print_output(function_parser::parse_function(input).as_str()), //sends input to define_function func
        _ => print_output("command not found"),
    }
}

//function to define all outputs, is a sepeate function to be able to swiftly change
//the style of the output without having to change every single println!
fn print_output(output:&str){
    println!("\n>> {}\n", output);
}
