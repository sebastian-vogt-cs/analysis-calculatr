static ACCURACY:f64 = 100000.0;

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
            result + (a * x.powf(n as f64))
        }else{
            result - (a * x.powf(n as f64))
        };
    }
    round(result)
}

pub fn get_zeros(func:&Vec<(bool, f64, usize)>)->Vec<f64>{
    let mut values:Vec<f64> = Vec::new();
    if func[0].2 == 1{
        values.push(newton_alg(0.0, func));
        return values;
    }else{
        let extrema = get_zeros(&derive(func));
        let last_extreme:f64 = extrema[extrema.len() - 1];
        let mut memory:f64 = extrema[0] - 100.0;
        for x in extrema{
            let new_zero = newton_alg((x + memory) / 2.0, func);
            memory = x;
            if ((values.len() >= 1) && (new_zero != values[values.len()-1]) && (((new_zero - values[values.len()-1]) > (1.0/ACCURACY)) || ((values[values.len()-1] - new_zero) > (1.0/ACCURACY)))) || (values.len() == 0){
                values.push(new_zero);
            }
        }
        values.push(newton_alg(last_extreme + 0.4, func));
    }
    values
}

fn newton_alg(x_start:f64, func:&Vec<(bool, f64, usize)>)->f64{
    let mut x:f64 = x_start;
    let derivative:Vec<(bool, f64, usize)> = derive(func);
    for _i in 0..10000{
        x = x - (get_y_for(x, func)/get_y_for(x, &derivative));
    }
    round(x)
}

fn round(num:f64)->f64{
    (num * ACCURACY).round()/ACCURACY
}
