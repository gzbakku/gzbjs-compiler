
use crate::{Opp,Compiler,Error};


pub fn init(app:&mut Compiler,start:usize,end:usize)->Result<(Opp,usize,usize),Error>{

    let mut index = start;
    let mut char_count = 0;

    loop{

        let char = app.code[index];
        
        if char == '['{
            char_count += 1;
        }

        if char == ']'{
            char_count -= 1;
            if char_count == 0{
                match build_statement(
                    app,start+1,index-1
                ){
                    Ok(v)=>{
                        return Ok(v);
                    },
                    Err(_e)=>{
                        return Err(_e);
                    }
                }
            }
        }

        if index == app.code.len()-1 || index == end{
            break;
        } else {
            index += 1;
        }

    }

    return Err(Error::string(format!("invalid_angle_bracket_at => {}",app.location(start))));

}

fn build_statement(
    app:&mut Compiler,
    mut start:usize,
    end:usize
)->Result<(Opp,usize,usize),Error>{

    let base_start = start;
    let mut collect:Vec<(Opp,usize,usize)> = vec![];

    loop{
        match crate::parser::opp::init(app,start,end){
            Ok(v)=>{
                if v.1 > end{
                    break;
                }
                start = v.2 + 1;
                collect.push(v);
            },
            Err(_e)=>{
                break;
            }
        }
    }

    /*
        start = t
        comma = c
        value = v
    */

    let mut is_valid_array = true;
    let mut array_started = false;
    let mut build:Vec<(Opp,usize,usize)> = Vec::new();
    let mut last:char = 't';
    for item in collect.iter(){
        match &item.0{
            Opp::Comma=>{
                if last != 'v'{is_valid_array = false;break;}
                last = 'c';
            },
            _v=>{
                if !array_started{
                    if last != 't'{is_valid_array = false;break;}
                    array_started = true;
                } else {
                    if last != 'c'{is_valid_array = false;break;}
                }
                last = 'v';
                build.push((_v.clone(),item.1,item.2));
            }
        }
    }

    if is_valid_array{
        return Ok((Opp::Array(build),base_start-1,end+1));
    } else {
        return Err(Error::string(
            format!("invalid array => {:?}",app.location(base_start-1))
        ));
    }

}

