pub fn parse_function(input:&str) -> String{

    let mut state:u8 = 0;
    let function:&str = &input[7..input.len()];

    for c in function.chars() {
        match state {
            0 =>{
                match c {
                    '-' => state = 1,
                    'x' => state = 2,
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = 3,
                    _ => {state = 7; break;},
                }
            }
            1 =>{
                match c {
                    'x' => state = 2,
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state = 3,
                    _ => {state = 7; break;},
                }
            }
            2 =>{
                match c {
                    ' ' => state = 5,
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' => state = 4,
                    _ => {state = 7; break;},
                }
            }
            3 =>{
                match c {
                    ' ' => state = 5,
                    'x' => state = 2,
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => state = 3,
                    _ => {state = 7; break;},
                }
            }
            4 =>{
                match c {
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' | '⁰' => state = 4,
                    ' ' => state = 5,
                    _ => {state = 7; break;},
                }
            }
            5 =>{
                match c {
                    '+' | '-' => state = 6,
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
