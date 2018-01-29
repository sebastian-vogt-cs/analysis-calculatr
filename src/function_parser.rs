//this func checks weather or not the function is a correctly entered polynomal function and maps the function in a usable way into a Vec
//the resulting Vec consist of a tuple for each term, the tuple consists of the multiplicant a and the power n (ax^n)
//the parser is basically a deterministic finite state machine, draw it up in a diagramm to understand it.
pub fn parse_function(input:&str) -> Vec<(i8, u32, u32)>{
    let mut state:u8 = 0;
    let function:&str = &input[7..input.len()];
    let mut polynomal_representation:Vec<(i8, u32, u32)> = Vec::new();
    let mut memory:(i8, u32, u32) = (1, 0, 0);

    for c in function.chars() {
        match state {
            0 =>{
                match c {
                    '-' => {state = 6; memory = (-1, 1, 0)},
                    'x' => {state = 2; memory = (memory.0, 1, 1)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {state = 3; memory = (memory.0, c.to_digit(10).unwrap() as u32, 0)},
                    _ => {state = 7; break;},
                }
            }
            1 =>{
                match c {
                    'x' => {state = 2; memory = (memory.0, 1, 1)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {state = 3; memory = (memory.0, c.to_digit(10).unwrap() as u32, 0)},
                    _ => {state = 7; break;},
                }
            }
            2 =>{
                match c {
                    ' ' => {state = 5; memory = (memory.0, memory.1, 1); polynomal_representation.push(memory)},
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' => {state = 4; memory = (memory.0, memory.1, superscript_to_u32(&c))},
                    _ => {state = 7; break;},
                }
            }
            3 =>{
                match c {
                    ' ' => {state = 5; memory = (memory.0, memory.1, 0); polynomal_representation.push(memory)},
                    'x' => {state = 2; memory = (memory.0, memory.1, 1);}
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {state = 3; memory = (memory.0, memory.1*10 + c.to_digit(10).unwrap() as u32, 0)},
                    _ => {state = 7; break;},
                }
            }
            4 =>{
                match c {
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' | '⁰' => {state = 4; memory = (memory.0, memory.1, memory.2*10 + superscript_to_u32(&c))},
                    ' ' => {state = 5; memory = (memory.0, memory.1, memory.2); polynomal_representation.push(memory)},
                    _ => {state = 7; break;},
                }
            }
            5 =>{
                match c {
                    '+' => {state = 6; memory = (1, 0, 0)},
                    '-' => {state = 6; memory = (-1, 0, 0)},
                    _ => {state = 7; break;},
                }
            }
            6 =>{
                match c {
                    ' ' => state = 0,
                    _ => {state = 7; break;},
                }
            }
            _ => {state = 7; break;},
        }
    }
    polynomal_representation.push(memory);
    if (state == 2) | (state == 3) | (state == 4) {
        polynomal_representation
    }else{
        Vec::new()
    }
}

pub fn func_to_string(func:&Vec<(i8, u32, u32)>)->String{
    let mut func_string:String = String::new();
    for i in 0 .. (func.len()) {
        if func[i].0 != 1 {
            func_string.push_str(" - ");
        }else{
            func_string.push_str(" + ");
        }
        func_string.push_str(&func[i].1.to_string());
        func_string.push('x');
        func_string.push_str(&u32_to_superscript(func[i].2));
    }
    func_string
}



//this function converts number like ¹²³³⁴⁵ to normal u32
fn superscript_to_u32(char:&char)->u32{
    match *char as u16{
        185  => 1,
        178  => 2,
        179  => 3,
        8308 => 4,
        8309 => 5,
        8310 => 6,
        8311 => 7,
        8312 => 8,
        8313 => 9,
        8304 => 0,
        _    => 1000, //high number to indicate to myself somthing hase gone wrong
    }
}

fn u32_to_superscript(num:u32)->String{
    let nums:String = num.to_string();
    let mut result:String = String::new();
    for c in nums.chars(){
        match c{
            '1' => result.push('¹'),
            '2' => result.push('²'),
            '3' => result.push('³'),
            '4' => result.push('⁴'),
            '5' => result.push('⁵'),
            '6' => result.push('⁶'),
            '7' => result.push('⁷'),
            '8' => result.push('⁸'),
            '9' => result.push('⁹'),
            '0' => result.push('⁰'),
            _ => result.push_str("error"),
        }
    }
    result
}
