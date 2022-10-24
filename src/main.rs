use std::env;
use std::fs::File;
use std::process::exit;

mod token;
mod sections;
mod evaluate;


fn main() {
    let args: Vec<String> = env::args().collect();

    // program arguements
    let mut input_filen = String::new();
    let mut output_filen = String::from("a.out");



    // make sure arguments given
    if args.len() < 1 {
        println!("No path provided");
        exit(0);
    }


    let mut index = 1;
    while index <= args.len() {
        // -h and -v exit the program immediately
        let arg = args.get(index).unwrap();

        // -h and -v
        if arg == "-h" { // print out help message
        }

        else if arg == "-v" { // print program version
        }

        else if arg == "-i" {
            input_filen = String::from(args.get(index += 1));
        }

        else if arg == "-o" {
            output_filen = String::from(args.get(index += 1));
        }

        // first arg is the file name
        else if index == 1 {
            input_filen = String::from(args.get(index += 1));
        }

        else {
            println!("Unknown arguement {}", args.get(index).unwrap())
        }
    }


    // read in source file
    let file = File.open(input_filen).expect(format!("Error: Could not find file {input_filen}"));
    let mut file_buff = String::new();
    file.read_to_string(&mut file_buff)?;

    // find all sections
    let mut section_vec = Vec::new();



}
