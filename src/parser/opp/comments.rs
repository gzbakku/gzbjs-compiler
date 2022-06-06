

use crate::{Compiler,Opp,Error};

pub fn init(
    app:&mut Compiler,
    start:usize,
    end_limit:usize,
    comment_type:&'static str
)->Result<(Opp,usize,usize),Error>{

    let mut index = start;
    let mut collect = String::new();
    let mut started = false;
    let mut start_at = start;

    loop{

        let char = app.code[index];

        // println!("char : {:?}",char);

        if !started{
            if comment_type == "//"{
                if char == '/'{
                    if app.code.len() > index+1{
                        if app.code[index+1] == '/'{
                            started = true;
                            start_at = index;
                            index += 1;
                        }
                    }
                }
            }
            if comment_type == "/*"{
                if char == '/'{
                    if app.code.len() > index+1{
                        if app.code[index+1] == '*'{
                            started = true;
                            start_at = index;
                            index += 1;
                        }
                    } else {
                        return Err(Error::string(
                            format!("invalid_comment => {:?}",app.location(start_at))
                        ));
                    }
                }
            }
        } else {
            if comment_type == "//"{
                if char == '\n'{
                    return Ok((Opp::Comment(collect),start_at,index));
                } else {
                    collect.push(char);
                }
            }
            if comment_type == "/*"{
                if char == '*'{
                    if app.code.len() > index+1{
                        if app.code[index+1] == '/'{
                            return Ok((Opp::Comment(collect),start_at,index));
                        } else {
                            collect.push(char);
                        }
                    } else {
                        return Err(Error::string(
                            format!("invalid_comment => {:?}",app.location(start_at))
                        ));
                    }
                } else {
                    collect.push(char);
                }
            }
        }

        if index == app.code.len()-1 || index == end_limit{
            break;
        } else {
            index += 1;
        }

    }

    return Err(Error::string(
        format!("comment does not end properly => {:?}",app.location(start_at))
    ));

}