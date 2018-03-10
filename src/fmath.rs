static ACCURACY:f64 = 100000.0;


//derive a function
pub fn derive(func:&Vec<(bool, f64, usize)>)->Vec<(bool, f64, usize)>{

    //variables for the new function and the memory that safes the latest monomial when iterating through the monomials
    let mut new_func:Vec<(bool, f64, usize)> = Vec::new();
    let mut memory:(bool, f64, usize) = (true, 0.0, 0);

    //iterate through the monomials and derive them seperatly
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


//get y value for a specified x value for a specified function
pub fn get_y_for(x:f64, func:&Vec<(bool, f64, usize)>)->f64{
    let mut result:f64 = 0.0;
    for &(sign, a, n) in func{
        result = if sign {
            result + (a * x.powf(n as f64))
        }else{
            result - (a * x.powf(n as f64))
        };
    }
    result
}


//calculate zeros of a function via newton algorithm
pub fn get_zeros(func:&Vec<(bool, f64, usize)>)->Vec<f64>{

    //this variable stores the zeros
    let mut values:Vec<f64> = Vec::new();

    //if it is a linear function run newton_alg and return the value
    if func[0].2 == 1{
        values.push(round(newton_alg(0.0, func)));
        return values;

    //else get the extrema (zeros of derivative -> recursion until derivative is x¹).
    }else{
        
        //if two neighboring extrema have different signs search for a zeros in between via newton. Loop through all the extrema for that.
        let extrema = get_zeros(&derive(func));
        let last_extreme:f64 = extrema[extrema.len() - 1];
        let mut memory:f64 = extrema[0] - 1.0;
        for x in extrema{
            if (get_y_for(x, func).is_sign_positive() && !(get_y_for(memory, func).is_sign_positive())) || (!(get_y_for(x, func).is_sign_positive()) && get_y_for(memory, func).is_sign_positive()) || (get_y_for(memory, func) == 0.0) || (get_y_for(x, func) == 0.0){
                let new_zero = round(newton_alg((x + memory) / 2.0, func));

                //check if the new zero isn't already there
                if ((values.len() >= 1) && (new_zero != values[values.len()-1])) || (values.len() == 0){
                    values.push(new_zero);
                }
            }
            memory = x;
        }

        //redo that process for the space after the last extreme
        let new_zero = round(newton_alg(last_extreme + 1.0, func));
        if ((values.len() >= 1) && (new_zero != values[values.len()-1])) || (values.len() == 0){
                    values.push(new_zero);
         }
    }
    values
}


//implementation of the newton algorithm for calculating zeros. Google if you want to know how the maths work
fn newton_alg(x_start:f64, func:&Vec<(bool, f64, usize)>)->f64{
    let mut x:f64 = x_start;
    let derivative:Vec<(bool, f64, usize)> = derive(func);
    for _i in 0..(((func[0].2 as f64).powf(2.0))/func[0].1.sqrt()) as usize{
        x = x - (get_y_for(x, func)/get_y_for(x, &derivative));
    }
    x
}


//round f64 to ACCURACY, which is a static global variable
fn round(num:f64)->f64{
    if (((1.0/ACCURACY) > num) && (num.is_sign_positive())) || ((-(1.0/ACCURACY) < num) && (!num.is_sign_positive())){
        0.0
    }else{
        (num * ACCURACY).round()/ACCURACY
    }
}
