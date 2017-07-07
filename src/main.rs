extern crate argparse;
extern crate curl;
extern crate base64;
extern crate threadpool;

mod req;
mod wildcards;
mod args;

use argparse::{ArgumentParser,Store};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    
    let mut auth = "".to_string();
    let mut url = "".to_string();
    let mut arg_file = "".to_string();
    let mut headers = "".to_string();
    let mut workers = 1;
    let mut jobs = 1;
    let mut method = "GET".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Execute http requests to test REST services");
        
        ap.refer(&mut url)
            .add_option(&["-u","--url"],Store,"Url address");
        
        ap.refer(&mut auth)
            .add_option(&["-a","--auth"],Store,"Basic auth");
        
        ap.refer(&mut arg_file)
            .add_option(&["-f","--argument_file"],Store,"Argument csv format file");
        
        ap.refer(&mut workers)
            .add_option(&["-w","--workers"],Store,"Number of workers");
        
        ap.refer(&mut jobs)
            .add_option(&["-j","--jobs"],Store,"Number of jobs");
        
        ap.refer(&mut headers)
            .add_option(&["-c","--headers"],Store,"Headers in format header:value,header:value,...");
        
        ap.refer(&mut method)
            .add_option(&["-m","--method"],Store," Method (GET, POST or PUT) default GET ");

        ap.parse_args_or_exit();
    }


	let pool = ThreadPool::new(workers);
	let (tx, rx) = channel();
    
    if arg_file != ""
    { 
        args::check_argfile_length(&arg_file,jobs as usize);
    }

	for x in 0..jobs {
    	
		let aut = auth.to_owned();
		let mut add = url.to_owned();
        let heads = headers.to_owned();
        let arg_file_copy = arg_file.to_owned();
        let meth = method.to_owned();

		if arg_file != ""
		{
			add = wildcards::replace_wildcards(add,arg_file_copy,x);
		}
		
		let tx = tx.clone();
   		pool.execute(move|| {
        	tx.send(1).unwrap();
			let response = req::make_request(&add,&aut,&heads,&meth);
    		println!("{} with this url : {}  ",response,&add);	 
    	});
	}

    for _ in 0..jobs {
        rx.recv().unwrap();  
    }   
    
}
