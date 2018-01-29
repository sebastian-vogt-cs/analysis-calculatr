pub fn derive(func:&Vec<(i8, u32, u32)>)->Vec<(i8, u32, u32)>{
    let mut new_func:Vec<(i8, u32, u32)> = Vec::new();
    let mut memory:(i8, u32, u32) = (1, 0, 0);
    for &(sign, a, n) in func{
        if n >= 1{
            memory.2 = n - 1;
            memory.1 = a * n;
            memory.0 = sign;
            new_func.push(memory);
        }
    }
    new_func
}
