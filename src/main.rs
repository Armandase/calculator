use std::io::stdin;
pub mod operator;

//  function to print help ressources
fn check_help(line :&str) -> bool {
    let match1 = "h";

    match line.find(match1){
        None => return false,
        Some(_) =>{
            println!("MRC is a minimalist rust calculator.\n");
            println!("You can use float and theses operators : +, -, *, /, % and ^");
            println!("Usage :");
            println!("   q -> quit the program");
            return true
        }
    }
}

//  parsing function for work without space
fn rm_whitespace(input :&str) -> String{
    input
        .split_whitespace()
        .collect()
}

//  check if the code should be quit
fn check_quit(line :&str) -> bool{
    let match1 = "q";

    match line.find(match1) {
        None => (),
        Some(_) => return true
    }
    false
}

fn main() {
    let mut input = String::new();
    println!("MRC 1.0");
    println!("h for help, q for quit\n");
    loop {
       stdin()
           .read_line(&mut input)
           .expect("Error in standard input"); 
       let mut line = rm_whitespace(&input);
       if check_quit(&mut line){
           break ;
       } else if check_help(&mut line){
            continue ;
       }
       pars_line(&mut line);
        input = String::new();
    }
}

fn pars_line(line :&str){
    let sep: Vec<&str> = line
        .split(['1' , '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'].as_ref())
        .filter(|&x| !x.is_empty())
        .collect();
    let check = sep.clone();
    for byte in check{
        if byte.len() != 1 ||
            (byte != "+" && byte != "-"
             && byte != "*" && byte != "/"
             && byte != "%" && byte != "^")
        {
            return 
        }
    }
    let array = collect_number(line);
    apply_calcul(&array, sep);
}

fn collect_number(line :&str) -> Vec<f64>{
    let value: Vec<&str> = line
        .split(['+' , '-', '*', '/', '%', '^'].as_ref())
        .collect();
    let mut array: Vec<f64> = Vec::new();
    let mut i: usize = 0;
    while i < value.len(){
        array.push(value[i].parse().expect("Parsing error"));
        i += 1;
    }
    return array
}


fn apply_calcul(array :&Vec<f64>, sep :Vec<&str>)
{
    let mut _result :f64 = 0.0;
    let mut new = array.clone();
    let mut op = sep.clone();

    loop{
        let mut prio: usize = 0;
        while prio < op.len(){
            if op[prio] == "*" || op[prio] == "/"
                || op[prio] == "%" || op[prio] == "^"{
               break ; 
            }
            prio += 1;
        }
        if prio == op.len(){
            prio = 0;
        }
        new = if prio != 0{
            let mut cpy = new.clone();
            let tmp = select_calcul(new[prio], new[prio + 1], sep[prio]);
            cpy[prio] = tmp;
            cpy.remove(prio + 1);
            op.remove(prio);
            cpy
        } else {
            let tmp = select_calcul(new[0], new[1], sep[0]);
            let mut cpy = new.clone();
            cpy[0] = tmp;
            cpy.remove(1);
            op.remove(0);
            cpy
        };
        if new.len() == 1{
            break ;
        }
    }
    println!("{}", new[0]);
}

fn  select_calcul(var1: f64, var2: f64, sep: &str) -> f64{
    let ret: f64;

    if sep == "+"{
        ret = operator::add(var1, var2);
    } else if sep == "-"{
        ret = operator::sub(var1, var2);
    } else if sep == "*"{
        ret = operator::mult(var1, var2);
    } else if sep == "/"{
        ret = operator::div(var1, var2);
    } else if sep == "%"{
        ret = operator::modulo(var1, var2);
    } else if sep == "^"{
        ret = operator::exponent(var1, var2);
    } else {
        ret = 0.0;
    }
    return ret 
}
