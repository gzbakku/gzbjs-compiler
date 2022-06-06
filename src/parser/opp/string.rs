

use crate::{Compiler,Opp,Error};

pub fn init(
    app:&mut Compiler,
    start:usize,
    end_limit:usize,
    string_type:char
)->Result<(Opp,usize,usize),Error>{

    let mut index = start;
    let mut collect = String::new();
    let mut started = false;
    let mut start_at = start;

    loop{

        let char = app.code[index];

        if started{
            if char == string_type{
                if app.code[index-1] != '\\'{
                    if string_type == '`'{
                        return Ok((Opp::StringBuild(collect),start_at,index));
                    } else {
                        return Ok((Opp::String(collect),start_at,index));
                    }
                }
            }
            collect.push(char);
        }

        if char == string_type{
            started = true;
            start_at = index;
        }

        if index == app.code.len()-1 || index == end_limit{
            break;
        } else {
            index += 1;
        }

    }

    return Err(Error::string(
        format!("string does not end properly => {:?}",app.location(start))
    ));

}