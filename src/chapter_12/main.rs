use std::env;
use std::fmt::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;
// Finish this.
// write to standard error 
// allow to be matched for case insensative with a falg
struct Config{
    query : String,
    filename:String,
}
impl Config{
    fn new(query:String,filename:String) -> Self{
        Self{query,filename}
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        eprintln!("Usage ./main query filename");
        process::exit(1);
    }

    let config_info = parse_config(&args);
    if let  Err(e) = run_grep(config_info){
        eprintln!("program ran into error: {}", e);
        process::exit(1);
    }
    // Io Project
}
fn run_grep(conif: Config) ->  Result<String,std::io::Error>{
    println!("searching for {} in {}",conif.query,conif.filename);
    let contents = file_contents(conif.filename)?;
    let matched_lines = search_file(contents, conif.query.clone());
    display_grep(matched_lines);
    Ok("Grep completed successfully".to_string())
}
fn search_file(file_contents:String,query:String) -> Vec<(i32,String)>{
    let mut  content_map:Vec<(i32,String)> = Vec::new();
    for  (idx, line) in file_contents.lines().enumerate(){
        if line.contains(&query){
            content_map.push((idx as i32,line.to_string()));
        }
    }
    content_map
}
fn display_grep(matches:Vec<(i32,String)>){
    for (idx,line) in matches{
        println!("{idx}: {line} ")
    }
}
fn parse_config(args: &[String]) -> Config{
    let query = &args[1];
    let filename = &args[2];
    Config::new(query.clone(),filename.clone())
    //(query,filename)

}
fn file_contents(filename: String) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}