mod bible;
mod search;
mod utils;

use patina::Patina;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("\nUsage:\n{} -s <book> [chapter] [verse]", args[0]);
        eprintln!("{} -q \"book chapter:verse\"\n", args[0]);
        return;
    }
    let patina = Patina::new();
    if args[1] == "-s" {
        let book = &args[2].to_lowercase().replace(" ", "-");
        let chapter = if args.len() > 3 {
            Some(args[3].parse::<u32>().ok().unwrap())
        } else {
            None
        };
        let verse = if args.len() > 4 {
            Some(args[4].parse::<u32>().ok().unwrap())
        } else {
            None
        };
        if let Some(result) = patina.search_by_reference(book, chapter, verse) {
            println!("{}", result);
        }
    } else if args[1] == "-q" {
        let query = &args[2];
        if let Some(result) = patina.search_by_text(query) {
            println!("{}", result);
        }
    }
}
