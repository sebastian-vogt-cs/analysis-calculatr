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
