//this func checks weather or not the function is a correctly entered polynomal function and maps the function in a usable way into a Vec
//the resulting Vec consist of a tuple for each term, the tuple consists of the multiplicant a and the power n (ax^n)
//the parser is basically a deterministic finite state machine, draw it up in a diagramm to understand it.
pub fn parse_function(input:&str) -> (Vec<(bool, f64, usize)>, bool){
    let mut state:u8 = 0;
    let function:&str = &input[7..input.len()];
    let mut polynomal_representation:Vec<(bool, f64, usize)> = Vec::new();
    let mut memory:(bool, f64, usize) = (true, 0.0, 0);
    let mut dezplace:usize = 10;

    for c in function.chars() {
        match state {
            0 =>{
                match c {
                    '-' => {state = 6; memory = (false, 1.0, 0)},
                    'x' => {state = 2; memory = (memory.0, 1.0, 1)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {state = 3; memory = (memory.0, c.to_digit(10).unwrap() as f64, 0); dezplace = 10},
                    _ => {state = 8; break;},
                }
            }
            1 =>{
                match c {
                    'x' => {state = 2; memory = (memory.0, 1.0, 1)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {state = 3; memory = (memory.0, c.to_digit(10).unwrap() as f64, 0)},
                    _ => {state = 8; break;},
                }
            }
            2 =>{
                match c {
                    ' ' => {state = 5; memory = (memory.0, memory.1, 1); polynomal_representation.push(memory)},
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' => {state = 4; memory = (memory.0, memory.1, superscript_to_usize(&c))},
                    _ => {state = 8; break;},
                }
            }
            3 =>{
                match c {
                    ' ' => {state = 5; memory = (memory.0, memory.1, 0); polynomal_representation.push(memory)},
                    'x' => {state = 2; memory = (memory.0, memory.1, 1);}
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {state = 3; 
                        memory = (memory.0, memory.1*10.0 + c.to_digit(10).unwrap() as f64, 0);
                    },
                    '.' => state = 7,
                    _ => {state = 8; break;},
                }
            }
            4 =>{
                match c {
                    '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' | '⁰' => {state = 4; 
                        memory = (memory.0, memory.1, memory.2*10 + superscript_to_usize(&c));
                    },
                    ' ' => {state = 5; memory = (memory.0, memory.1, memory.2); polynomal_representation.push(memory)},
                    _ => {state = 8; break;},
                }
            }
            5 =>{
                match c {
                    '+' => {state = 6; memory = (true, 0.0, 0)},
                    '-' => {state = 6; memory = (false, 0.0, 0)},
                    _ => {state = 8; break;},
                }
            }
            6 =>{
                match c {
                    ' ' => state = 0,
                    _ => {state = 8; break;},
                }
            }
            7 =>{
                match c{
                    ' ' => {state = 5; memory = (memory.0, memory.1, 0); polynomal_representation.push(memory)},
                    'x' => {state = 2; memory = (memory.0, memory.1, 1);}
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                        state = 7; 
                        memory = (memory.0, memory.1 + (c.to_digit(10).unwrap() as f64)/(dezplace as f64), 0); dezplace = dezplace * 10;
                    },
                    _ => {state = 8; break;},
                }
            }
            _ => {state = 8; break;},
        }
    }
    polynomal_representation.push(memory);
    if (state == 2) | (state == 3) | (state == 4) | (state == 7) {
        (sort(polynomal_representation), true)
    }else{
        (Vec::new(), false)
    }
}

pub fn func_to_string(func:&Vec<(bool, f64, usize)>)->String{
    let mut func_string:String = String::new();
    for i in 0 .. (func.len()) {
        if !func[i].0 {
            func_string.push_str(" - ");
        }else if i != 0{
            func_string.push_str(" + ");
        }
        if !((func[i].1 == 1.0) && (func[i].2 != 0)){
            func_string.push_str(&func[i].1.to_string());
        }
        if func[i].2 != 0{
            func_string.push('x');
            if func[i].2 != 1{
                func_string.push_str(&usize_to_superscript(func[i].2));
            }
        }
    }
    func_string
}



//this function converts number like ¹²³³⁴⁵ to normal u32
fn superscript_to_usize(char:&char)->usize{
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

fn usize_to_superscript(num:usize)->String{
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

fn sort(func:Vec<(bool, f64, usize)>)->Vec<(bool, f64, usize)>{
    let mut sorter:Vec<Vec<(bool, f64, usize)>> = Vec::new();
    sorter.push(func.clone());
    loop{
        if func.len() == sorter.len(){
            break;
        }
        let mut new_sorter:Vec<Vec<(bool, f64, usize)>> = Vec::new();
        for vec in sorter{
            if vec.len() == 1{
                new_sorter.push(vec);
                continue;
            }
            let middle = vec[vec.len()/2].2 as usize;
            let mut vec1:Vec<(bool, f64, usize)> = Vec::new();
            let mut vec2:Vec<(bool, f64, usize)> = Vec::new();
            let mut middle_counter = false;
            for (sign, a, n) in vec{
                if n > middle{
                    vec1.push((sign, a, n));
                }else if n < middle{
                    vec2.push((sign, a, n));
                }else if (n == middle) && (vec1.len() == 0) && !middle_counter{
                    vec1.push((sign, a, n));
                    middle_counter = true
                }else{
                    vec2.push((sign, a, n));
                }
            }
            new_sorter.push(vec1);
            new_sorter.push(vec2);
        }
        sorter = new_sorter;
    }
    let mut result:Vec<(bool, f64, usize)> = Vec::new();
    for vec in sorter{
        for (sign, a, n) in vec{
            result.push((sign, a, n));
        }
    }
    merge(result)
}

fn merge(func:Vec<(bool, f64, usize)>)->Vec<(bool, f64, usize)>{
    let mut merged:Vec<(bool, f64, usize)> = Vec::new();
    for (sign, a, n) in func{
        if merged.len() == 0{
            merged.push((sign, a, n));
            continue;
        }
        let index = merged.len()-1;
        if n == merged[index].2{
            if (!sign) && (!merged[index].0){
                merged[index] = (false, a + merged[index].1, n);
            }else if (sign) && (merged[index].0){
                merged[index] = (true, a + merged[index].1, n);
            }else{
                let mut new_a = if (sign && merged[index].0) || (!sign && !merged[index].0){
                    a * merged[index].1
                }else{
                    - a * merged[index].1
                };
                let mut new_sign = true;
                if new_a < 0.0{
                    new_sign = false;
                    new_a = new_a + ((-2.0) * new_a);
                }else if new_a == 0.0{
                    merged.remove(index);
                    continue;
                }
                merged[index] = (new_sign, new_a, n);
            }
        }else{
            merged.push((sign, a, n));
        }
    }
    merged
}

pub fn get_f64_from_string(input:&str)->f64{
    let mut x:f64 = 0.0;
    let mut state:u8 = 0;
    let mut dezplace:usize = 10;
        for c in input.chars(){
            match state{
                0 => {
                    match c{
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {x = x*10.0 + c.to_digit(10).unwrap() as f64; state = 0},
                        '.' => state = 1,
                        _ => {x = 0.0; break},
                    }
                }
                1 => {
                    match c{
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {x = x + (c.to_digit(10).unwrap() as f64)/(dezplace as f64); dezplace = dezplace * 10},
                        _ => {x = 0.0; break},
                    }
                }
                _ =>{x = 0.0; break}
            }
        }
    x
}
