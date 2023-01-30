fn add (var1: f64, var2: f64) -> f64{
    return var1 + var2
}

fn sub (var1: f64, var2: f64) -> f64{
    return var1 - var2
}

fn mult (var1: f64, var2: f64) -> f64{
    return var1 * var2
}

fn div (var1: f64, var2: f64) -> f64{
    return var1 / var2
}

fn modulo (var1: f64, var2: f64) -> f64{
    return var1 % var2
}

fn exponent (var1: f64, var2: f64) -> f64{
    let mut limit: f64 = 0.0;
    let mut ret = var1;
    while limit < var2{
        ret = ret * var1;
        limit += 1.0;
    }
    return ret;
}

pub fn  select_calcul(var1: f64, var2: f64, sep: &str) -> f64{
    let ret: f64;

    if sep == "+"{
        ret = add(var1, var2);
    } else if sep == "-"{
        ret = sub(var1, var2);
    } else if sep == "*"{
        ret = mult(var1, var2);
    } else if sep == "/"{
        ret = div(var1, var2);
    } else if sep == "%"{
        ret = modulo(var1, var2);
    } else if sep == "^"{
        ret = exponent(var1, var2);
    } else {
        ret = 0.0;
    }
    return ret 
}
