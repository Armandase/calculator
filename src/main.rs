use std::io::stdin;

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
       split_apply_calcul(&mut line);
        input = String::new();
    }
}

fn split_apply_calcul(line :&str){
    let value: Vec<&str> = line
        .split(['+' , '-', '*', '/', '%', '^'].as_ref())
        .collect();
    let sep: Vec<&str> = line
        .split(['1' , '2', '3', '4', '5', '6', '7', '8', '9', '0'].as_ref())
        .filter(|&x| !x.is_empty())
        .collect();
    for byte in sep{
        if byte.len() != 1 ||
            (byte[0] != b'+' && byte[0] != b'-'
             && byte[0] != b'*' && byte[0] != b'/'
             && byte[0] != b'%' && byte[0] != b'^')
        {
            println!("")
        }
    }
}
