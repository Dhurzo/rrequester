use std::fs::File;
use std::io::{BufReader,BufRead};


pub fn check_argfile_length(arg_file : &String , index : usize)
{
    println!("Checking argfile length, this can take some time....");

    let file = match File::open(&arg_file)
    {
        Ok(file) => file,
        Err(err) => panic!("No such arg file".to_string() +&err.to_string()),
    };

    let count_buff = BufReader::new(file);
    
    let file_lines = count_buff.lines().count();
    
    if file_lines < index 
    {
        panic!("Your arg file must have at least the same words number than
               iteration number");
    }
    
    println!("The argfile has a correct length...RUNNING!!");
}

pub fn get_argfile_line(arg_file :String , iter:i32) -> String
{
    let rfile = match File::open(&arg_file)
    {
        Ok(file) => file,
        Err(err) => panic!("No such argfile".to_string() + &err.to_string()),
    };

    let read_buff = BufReader::new(rfile);
    read_buff.lines().nth(iter as usize).unwrap().unwrap() //HANDLE THIS
}
