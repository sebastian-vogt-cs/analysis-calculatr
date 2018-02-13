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

pub fn get_y_for(x:f64, func:&Vec<(i8, f64, usize)>)->f64{
    let mut result:f64 = 0.0;
    for &(sign, a, n) in func{
        result = result + ((sign as f64) * (a as f64) * x.powf(n as f64));
    }
    result
}

pub fn get_zeros(func:&Vec<(i8, f64, usize)>)->Vec<f64>{
    let derivative:Vec<(i8, f64, usize)> = derive(func);
    let mut zeros:Vec<f64> = Vec::new();
    let mut border:f64 = 0.0;
    let mut i:usize = 0;
    loop{
        zeros.push(newton_alg(border + 1.0, func));
        if border == newton_alg(zeros[i], &derivative){
            border = 0.0;
            i = 0;
            loop{
                zeros.push(newton_alg(border - 1.0, func));
                if border == newton_alg(zeros[i], &derivative){
                    break;
                }else{
                    border = newton_alg(zeros[i], &derivative);
                    i = i + 1;
                }
            }
            break;
        }else{
            border = newton_alg(zeros[i], &derivative);
            i = i + 1;
        }
    }
    zeros
}

fn newton_alg(x_start:f64, func:&Vec<(i8, f64, usize)>)->f64{
    let mut x:f64 = x_start;
    let derivative:Vec<(i8, f64, usize)> = derive(func);
    for _i in 0..10000{
        x = x - (get_y_for(x, func)/get_y_for(x, &derivative));
    }
    x
}
