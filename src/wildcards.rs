use args;

pub fn replace_some_wildcards(line : String, url : String) -> String
{
        let mut final_url = String::from("");
        let splited_line = line.split(",").collect::<Vec<&str>>();
        let splited_url = url.split("[*]").collect::<Vec<&str>>();
        
        let mut index = 0; 
        for split in splited_url
        {   
               final_url.push_str(split);
               if index < splited_line.len()
               {
                    final_url.push_str(splited_line[index]);
                    index = index + 1;
               }  
        }

        final_url
}

pub fn replace_wildcards(url : String, arg_file : String , iter : i32) -> String
{
    let mut final_url = String::from(""); 
    let line = args::get_argfile_line(arg_file,iter);
    if line.contains(",")
    {
        final_url = replace_some_wildcards(line,url);
    }
    else
    {
        final_url = url.replace("[*]",&line);
    }
    final_url
}
