pub fn derive(func:&Vec<(bool, f64, usize)>)->Vec<(bool, f64, usize)>{
    let mut new_func:Vec<(bool, f64, usize)> = Vec::new();
    let mut memory:(bool, f64, usize) = (true, 0.0, 0);
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

pub fn get_y_for(x:f64, func:&Vec<(bool, f64, usize)>)->f64{
    let mut result:f64 = 0.0;
    for &(sign, a, n) in func{
        result = if sign {
            result + ((a as f64) * x.powf(n as f64))
        }else{
            result - ((a as f64) * x.powf(n as f64))
        };
    }
    result
}

pub fn get_zeros(func:&Vec<(bool, f64, usize)>)->Vec<f64>{
    let mut values:Vec<f64> = Vec::new();
    if func[0].2 == 1{
        values.push(newton_alg(0.0, func));
        return values;
    }else{
        let extrema = get_zeros(&derive(func));
        let last_extreme:f64 = extrema[extrema.len() - 1];
        for x in extrema{
            values.push(newton_alg(x - 1.0, func));
        }
        values.push(newton_alg(last_extreme + 1.0, func));
    }
    values
}

fn newton_alg(x_start:f64, func:&Vec<(bool, f64, usize)>)->f64{
    let mut x:f64 = x_start;
    let derivative:Vec<(bool, f64, usize)> = derive(func);
    for _i in 0..10000{
        x = x - (get_y_for(x, func)/get_y_for(x, &derivative));
    }
    x
}
