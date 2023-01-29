pub fn add (var1: f64, var2: f64) -> f64{
    return var1 + var2
}

pub fn sub (var1: f64, var2: f64) -> f64{
    return var1 - var2
}

pub fn mult (var1: f64, var2: f64) -> f64{
    return var1 * var2
}

pub fn div (var1: f64, var2: f64) -> f64{
    return var1 / var2
}

pub fn modulo (var1: f64, var2: f64) -> f64{
    return var1 % var2
}

pub fn exponent (var1: f64, var2: f64) -> f64{
    let mut limit: f64 = 0.0;
    let mut ret = var1;
    while limit < var2{
        ret = ret * var1;
        limit += 1.0;
    }
    return ret;
}
