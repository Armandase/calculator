//  function to print help ressources
pub fn check_help(line :&str) -> bool {
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
pub fn rm_whitespace(input :&str) -> String{
    input
        .split_whitespace()
        .collect()
}

//  check if the code should be quit
//       (q or  line)
pub fn check_quit(line :&str) -> bool{
    let match1 = "q";

    match line.find(match1) {
        None => (),
        Some(_) => return true
    }
    false
}
