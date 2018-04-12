//this func checks weather or not the function is a correctly entered polynomal function and maps the function in a usable way into a Vec
//the resulting Vec consist of a tuple for each term, the tuple consists of the multiplicant a and the power n (ax^n)
//the parser is basically a deterministic finite state machine, draw it up in a diagramm to understand it.
pub fn parse_function(input:&str) -> (Vec<(f64, isize)>, bool){

    let mut state:u8 = 0;
    let function:&str = &input[7..input.len()];
    let mut polynomal_representation:Vec<(f64, isize)> = Vec::new();
    let mut memory:(f64, isize) = (0.0, 0);
    let mut dezplace:usize = 10;
    let mut is_sign_negative = false;

    for c in function.chars() {
        match state {
            0 =>{
                match c {
                    '-' => {state = 6; memory = (1.0, 0); is_sign_negative = true;},
                    'x' => {state = 2; memory = (1.0, 1)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {state = 3; memory = (c.to_digit(10).unwrap() as f64, 0); dezplace = 10},
                    _ => {state = 8; break;},
                }
            }
            1 =>{
                match c {
                    'x' => {state = 2; memory = (1.0, 1)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {state = 3; memory = (c.to_digit(10).unwrap() as f64, 0)},
                    _ => {state = 8; break;},
                }
            }
            2 =>{
                match c {
                    ' ' => {state = 5; memory = (memory.0, 1); polynomal_representation.push(memory)},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {state = 4; memory = (memory.0, memory.1 * c.to_digit(10).unwrap() as isize)},
                    '-' => memory = (memory.0, -1),
                    _ => {state = 8; break;},
                }
            }
            3 =>{
                match c {
                    ' ' => {
                        state = 5; memory = (memory.0, 0);
                        if is_sign_negative {
                            memory.0 = - memory.0
                        }
                        polynomal_representation.push(memory)
                    },
                    'x' => {state = 2; memory = (memory.0, 1);}
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {state = 3; 
                        memory = (memory.0*10.0 + c.to_digit(10).unwrap() as f64, 0);
                    },
                    '.' => state = 7,
                    _ => {state = 8; break;},
                }
            }
            4 =>{
                match c {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {state = 4; 
                        memory = (memory.0, memory.1*10 + c.to_digit(10).unwrap() as isize);
                    },
                    ' ' => {
                        state = 5; memory = (memory.0, memory.1);
                        if is_sign_negative {
                            memory.0 = - memory.0;
                        }
                        polynomal_representation.push(memory)
                        },
                    _ => {state = 8; break;},
                }
            }
            5 =>{
                match c {
                    '+' => {state = 6; memory = (0.0, 0); is_sign_negative = false;},
                    '-' => {state = 6; memory = (0.0, 0); is_sign_negative = true;},
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
                    ' ' => {
                        state = 5; memory = (memory.0, 0);
                        if is_sign_negative {
                            memory.0 = - memory.0
                        }
                        polynomal_representation.push(memory)
                    },
                    'x' => {state = 2; memory = (memory.0, 1);}
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                        state = 7; 
                        memory = (memory.0 + (c.to_digit(10).unwrap() as f64)/(dezplace as f64), 0); dezplace = dezplace * 10;
                    },
                    _ => {state = 8; break;},
                }
            }
            _ => {state = 8; break;},
        }
    }
    if is_sign_negative {
     memory.0 = - memory.0;
    }
    polynomal_representation.push(memory);
    if (state == 2) | (state == 3) | (state == 4) | (state == 7) {
        (sort(polynomal_representation), true)
    }else{
        (Vec::new(), false)
    }
}


//converts the a function to a string
pub fn func_to_string(func:&Vec<(f64, isize)>)->String{
    let mut func_string:String = String::new();
    for i in 0 .. (func.len()) {
        if func[i].0 < 0.0 {
            func_string.push_str(" - ");
            if func[i].0 != - 1.0 {
                func_string.push_str(&(- &func[i].0).to_string());
            }   
        }else if i != 0 {
            func_string.push_str(" + ");
            if (func[i].0 != 1.0) || (func[i].1 == 0) {
                func_string.push_str(&func[i].0.to_string());
            }
        } else if (func[i].0 != 1.0) || (func[i].1 == 0) {
            func_string.push_str(&func[i].0.to_string());
        }
        if func[i].1 != 0{
            func_string.push('x');
            if func[i].1 != 1{
                func_string.push_str(&isize_to_superscript(func[i].1));
            }
        }
    }
    func_string
}


pub fn fraction_to_string(func: &(Vec<(f64, isize)>, Vec<(f64, isize)>)) -> String{
    let mut func_string: String = String::new();
    func_string.push_str(func_to_string(&func.0).as_str());
    func_string.push_str(" / ");
    func_string.push_str(func_to_string(&func.1).as_str());
    func_string
}


//converts an integer to a String in "superscript" representation (for eg. x²)
fn isize_to_superscript(num:isize)->String{
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
            '-' => result.push('⁻'),
            _ => result.push_str("error"),
        }
    }
    result
}


//merge sort algorithm to sort monomials from highest to lowest by degree
fn sort(func:Vec<(f64, isize)>)->Vec<(f64, isize)>{
    let mut sorter:Vec<Vec<(f64, isize)>> = Vec::new();
    sorter.push(func.clone());
    loop{
        if func.len() == sorter.len(){
            break;
        }
        let mut new_sorter:Vec<Vec<(f64, isize)>> = Vec::new();
        for vec in sorter{
            if vec.len() == 1{
                new_sorter.push(vec);
                continue;
            }
            let middle = vec[vec.len()/2].1 as isize;
            let mut vec1:Vec<(f64, isize)> = Vec::new();
            let mut vec2:Vec<(f64, isize)> = Vec::new();
            let mut middle_counter = false;
            for (a, n) in vec{
                if n > middle{
                    vec1.push((a, n));
                }else if n < middle{
                    vec2.push((a, n));
                }else if (n == middle) && (vec1.len() == 0) && !middle_counter{
                    vec1.push((a, n));
                    middle_counter = true
                }else{
                    vec2.push((a, n));
                }
            }
            new_sorter.push(vec1);
            new_sorter.push(vec2);
        }
        sorter = new_sorter;
    }
    let mut result:Vec<(f64, isize)> = Vec::new();
    for vec in sorter{
        for (a, n) in vec{
            result.push((a, n));
        }
    }
    merge(result)
}


//merge together monomials like eg. 5x² + 5x² = 10x². Called at the end of merge
fn merge(func:Vec<(f64, isize)>)->Vec<(f64, isize)>{
    let mut merged:Vec<(f64, isize)> = Vec::new();
    for (a, n) in func{
        if merged.len() == 0{
            merged.push((a, n));
            continue;
        }
        let index = merged.len()-1;
        if n == merged[index].1{
            merged[index] = (a + merged[index].0, n);
        }else{
            merged.push((a, n));
        }
    }
    merged
}


//state machine to transform a String to a f64 if possible
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


pub fn into_fraction_representation(func:&Vec<(f64, isize)>) -> (Vec<(f64, isize)>, Vec<(f64, isize)>) {
    let mut negative:Vec<(f64, isize)> = Vec::new();
    let mut positive:Vec<(f64, isize)> = Vec::new();
    let mut numerator:Vec<(f64, isize)> = Vec::new();
    let mut denominator:Vec<(f64, isize)> = Vec::new();
    for &(a, n) in func {
        if n < 0 {
            negative.push((a, -n));
        }else {
            positive.push((a, n));
        }
    }
    let highest_expo = negative[negative.len() - 1].1;
    let highest_expo_a = negative[negative.len() - 1].0;
    denominator.push(negative[negative.len() - 1]);
    for (a, n) in negative {
        let new_a = highest_expo_a / a;
        let new_n = highest_expo - n;
        if new_a != 0.0 {
            numerator.push((new_a, new_n));
        }
    }
    for (a, n) in positive {
        let new_n = n + highest_expo;
        let new_a = a * highest_expo_a;
        if new_a != 0.0 {
            numerator.push((new_a, new_n));
        }
    }
    if numerator.len() == 0 {
        numerator.push((1.0, 0));
    }
    numerator = sort(numerator);
    denominator = sort(denominator);
    (numerator, denominator)
}
