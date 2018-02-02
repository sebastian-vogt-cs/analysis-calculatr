pub fn derive(func:&Vec<(i8, f64, usize)>)->Vec<(i8, f64, usize)>{
    let mut new_func:Vec<(i8, f64, usize)> = Vec::new();
    let mut memory:(i8, f64, usize) = (1, 0.0, 0);
    for &(sign, a, n) in func{
        if n >= 1{
            memory.2 = n - 1;
            memory.1 = a * (n as f64);
            memory.0 = sign;
            new_func.push(memory);
        }
    }
    new_func
}

pub fn get_y_for(x:i16, func:&Vec<(i8, f64, usize)>)->f64{
    let mut result:f64 = 0.0;
    for &(sign, a, n) in func{
        result = result + ((sign as f64) * (a as f64) * (x as f64).powf(n as f64));
    }
    result
}
