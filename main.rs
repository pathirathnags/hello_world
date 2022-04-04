use colored::Colorize;
use std::env;

/*

	Prints hello world in whatever color
	the user inputs!

*/
fn main()
{

        let args:Vec<String> = env::args().collect();
        if args.len() < 2 {return;}

        //args [0] --> name of the executable
        //args [1+] --> arguments

        if args[1].trim() == "green"{
        println!("{}", "Hello, world!".bright_green());
        }

        else if args[1].trim() == "red"{
        println!("{}", "Hello, world!".bright_red());
        }

        else if args[1].trim() == "yellow"{
        println!("{}", "Hello, world!".bright_yellow());
        }
	
	else if args[1].trim() == "blue"{
        println!("{}", "Hello, world!".blue());
        }

	else if args[1].trim() == "purple"{
        println!("{}", "Hello, world!".purple());
        }

	else if args[1].trim() == "cyan"{
        println!("{}", "Hello, world!".cyan());
        }
}

