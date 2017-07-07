use base64::{encode};
use curl::easy::{Easy,List};

pub  fn set_headers(headers : &String, auth : &String) -> List
{

    let mut list = List::new();   
    if headers != ""
    {
        if headers.contains(",")
        {
            let splited_line = headers.split(",").collect::<Vec<&str>>();
            
            for line in splited_line
            {
                println!("Adding {}",line);
                list.append(&line).unwrap();
            }
        } 
    }

    if auth != ""
    {
	    let auth_b64 =  encode(&auth);
        let basicauth = "Basic ".to_string() + &auth_b64;
        let bauthheader = "Authorization: ".to_string() + &basicauth;     
        list.append(&bauthheader).unwrap();
    }
    
    list
}


pub fn make_request(url : &String, auth: &String, headers : &String , method : &String) -> u32
{
    let mut easy = Easy::new();
    easy.url(&url).unwrap(); 
    let  head_list = set_headers(headers,auth);
    easy.http_headers(head_list).unwrap();

    if method == "GET"
    {   
        easy.perform().unwrap(); 	
	    return easy.response_code().unwrap()
    }
    else if  method == "POST"
    {
        easy.post(true).unwrap();
        easy.perform().unwrap(); 	
	    return easy.response_code().unwrap()
    }
    else if method == "PUT"
    {
        easy.put(true).unwrap();
        easy.perform().unwrap(); 	
	    return easy.response_code().unwrap()
    }
    else
    {
        panic!("Method not supported, aborting");
    }
}
