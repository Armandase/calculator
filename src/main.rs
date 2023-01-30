pub mod operator;
pub mod utils;

// loop while 

fn main() {
    print!("\x1b[100mMRC\x1b[0m");
    println!(": -> h for help, q for quit\n");
    loop {
        let mut rl = rustyline::Editor::<()>::new();
        let readline = rl.readline(">> ");
        let input = match readline {
            Ok(line) => line,
            Err(_) => String::new(),
        };
        if input.len() == 0 {
            return ;
        }
        let mut line = utils::rm_whitespace(&input);
        if utils::check_quit(&mut line){
            break ;
        } else if utils::check_help(&mut line){
            continue ;
        }
        pars_and_calcul(&mut line);
    }
}

fn pars_and_calcul(line :&str){
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
            println!("You can only use digits and operators.");
            return 
        }
    }
    let array = collect_number(line);
    if array.len() == 1{
        return ;
    }
    apply_calcul(&array, sep);
}

fn collect_number(line :&str) -> Vec<f64>{
    let value: Vec<&str> = line
        .split(['+' , '-', '*', '/', '%', '^'].as_ref())
        .collect();
    let mut array: Vec<f64> = Vec::new();
    let mut i: usize = 0;
    while i < value.len(){
        let tmp = value[i].parse();
        match tmp {
            Ok(x) => array.push(x),
            Err(_e) => eprintln!("Operator at begin or end of line")
        }
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
            let tmp = operator::select_calcul(new[prio], new[prio + 1], op[prio]);
            cpy[prio] = tmp;
            cpy.remove(prio + 1);
            op.remove(prio);
            cpy
        } else {
            let tmp = operator::select_calcul(new[0], new[1], op[0]);
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

