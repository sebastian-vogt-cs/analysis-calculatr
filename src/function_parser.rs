//this func checks weather or not the function is a correctly entered polynomal function and maps the function in a usable way into a Vec
//the resulting Vec consist of a tuple for each term, the tuple consists of the multiplicant a and the power n (ax^n)
//the parser is basically a deterministic finite state machine, draw it up in a diagramm to understand it.
pub fn parse_function(input:&str) -> String{
    let mut state:u8 = 0;
    let function:&str = &input[7..input.len()];
    let mut polynomal_representation:Vec<(i64, u32)> = Vec::new();
    let mut memory:(i64, i64, u32) = (0, 0, 0);

    for c in function.chars() {
        match state {
            0 =>{
                match c {
                    '-' => {state = 1; memory = (-1, 0, 0)},
                    'x' => {state = 2; memory = (1, 1, 0)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {state = 3; memory = (1, c.to_digit(10).unwrap() as i64, 0)},
                    _ => {state = 7; break;},
                }
            }
            1 =>{
                match c {
                    'x' => {state = 2; memory = (memory.0, 1, 0)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {state = 3; memory = (memory.0, c.to_digit(10).unwrap() as i64, 0)},
                    _ => {state = 7; break;},
                }
            }
            2 =>{
                match c {
                    ' ' => {state = 5; memory = (memory.0, memory.1, 1); polynomal_representation.push((memory.0*memory.1, memory.2))},
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' => {state = 4; memory = (memory.0, memory.1, superscript_to_u32(&c))},
                    _ => {state = 7; break;},
                }
            }
            3 =>{
                match c {
                    ' ' => {state = 5; memory = (memory.0, memory.1, 0); polynomal_representation.push((memory.0*memory.1, memory.2))},
                    'x' => state = 2,
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {state = 3; memory = (memory.0, memory.1*10 + c.to_digit(10).unwrap() as i64, 0)},
                    _ => {state = 7; break;},
                }
            }
            4 =>{
                match c {
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' | '⁰' => {state = 4; memory = (memory.0, memory.1, memory.2*10 + superscript_to_u32(&c))},
                    ' ' => {state = 5; memory = (memory.0, memory.1, memory.2); polynomal_representation.push((memory.0*memory.1, memory.2))},
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
    let mut result:String = String::new();
    if (state == 2) | (state == 3) | (state == 4) {
        result = "the parser realized this is a polynomal function".to_string();
    }else{
        result = "this function is either not in the supported notation or this kind of function is not supported yet".to_string();
    }
    result
}

//this function converts number like ¹²³³⁴⁵ to normal u32
fn superscript_to_u32(char:&char)->u32{
    if *char as u16 == 185 {
        1
    }else if *char as u16 == 178 {
        2
    }else if *char as u16 == 179 {
        3
    }else if *char as u16 == 8308 {
        4
    }else if *char as u16 == 8309 {
        5
    }else if *char as u16 == 8310 {
        6
    }else if *char as u16 == 8311 {
        7
    }else if *char as u16 == 8312 {
        8
    }else if *char as u16 == 8313 {
        9
    }else{
        1000 //high number to indicate to myself somthing hase gone wrong
    }
}
