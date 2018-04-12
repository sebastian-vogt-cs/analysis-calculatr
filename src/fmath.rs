static ACCURACY:f64 = 100000.0;


//derive a function
pub fn derive(func:&Vec<(f64, isize)>)->Vec<(f64, isize)>{

    //variables for the new function and the memory that safes the latest monomial when iterating through the monomials
    let mut new_func:Vec<(f64, isize)> = Vec::new();
    let mut memory:(f64, isize) = (0.0, 0);

    //iterate through the monomials and derive them seperatly
    for &(a, n) in func{
        memory.1 = n - 1;
        memory.0 = a * (n as f64);
        new_func.push(memory);
    }
    new_func
}


//get y value for a specified x value for a specified function
pub fn get_y_for(x:f64, func:&Vec<(f64, isize)>)->f64{
    let mut result:f64 = 0.0;
    for &(a, n) in func{
        result = result + (a * x.powf(n as f64));
    }
    result
}


//calculate zeros of a function via newton algorithm
pub fn get_zeros(func:&Vec<(f64, isize)>)->Vec<f64>{

    //this variable stores the zeros
    let mut values:Vec<f64> = Vec::new();

    //if it is a linear function run newton_alg and return the value
    if func[0].1 == 1{
        values.push(round(newton_alg(0.0, func)));
        return values;

    //else get the extrema (zeros of derivative -> recursion until derivative is x¹).
    }else{
        
        //if two neighboring extrema have different signs search for a zeros in between via newton. Loop through all the extrema for that.
        let extrema = get_zeros(&derive(func));
        let last_extreme:f64 = extrema[extrema.len() - 1];
        let mut memory:f64 = extrema[0];
        let new_zero = round(newton_alg(memory - 1.0, func));
        values.push(new_zero);
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
fn newton_alg(x_start:f64, func:&Vec<(f64, isize)>)->f64{
    let mut x:f64 = x_start;
    let derivative:Vec<(f64, isize)> = derive(func);
    let mut i = 0;
    let mut x_max = 0.0;
    let mut x_min = 0.0;
    loop{
        if get_y_for(round(x), func) == 0.0{
            break
        }
        x = x - (get_y_for(x, func)/get_y_for(x, &derivative));
        i += 1;
        if i == 1000{
            if x > x_max{
                x_max = x;
            }else if x < x_min{
                x_min = x;
            }
        }else if i == 1100{
            x = (x_max + x_min)/2.0;
            return x
        }
    }
    x
}


//round f64 to ACCURACY, which is a static global variable
fn round(num:f64)->f64{
    (num * ACCURACY).round()/ACCURACY
}