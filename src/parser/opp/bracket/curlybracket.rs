

use std::collections::HashMap;
use crate::{Opp,Compiler,Error};


pub fn init(app:&mut Compiler,start:usize,end:usize)->Result<(Opp,usize,usize),Error>{

    let mut index = start;
    let mut char_count = 0;

    loop{

        let char = app.code[index];

        if char == '{'{
            char_count += 1;
        }

        if char == '}'{
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

    if collect.len() == 0{
        return Ok((Opp::Object(HashMap::new()),start,end));
    }

    /*
        starting = t
        comma = m
        colon = l
        name = n
        value = v
    */

    let mut is_valid_object = true;
    let mut object_started = false;
    let mut build:HashMap<String,(Opp,usize,usize)> = HashMap::new();
    let mut name:String = String::new();
    let mut last:char = 't';
    let mut statement_builder = vec![];
    let mut hold_item = collect[0].clone();
    for item in collect.iter(){
        hold_item = item.clone();
        // println!("last : {:?}",last);
        match &item.0{
            Opp::Colon=>{
                // println!("is Colon : {:?}",last);
                if last != 'n'{is_valid_object = false;break;}
                last = 'l';
            },
            Opp::Comma=>{
                // println!("is Comma : {:?}",last);
                if last != 'v'{is_valid_object = false;break;}
                last = 'm';
                build.insert(name.clone(),(Opp::Statement(statement_builder),item.1,item.2));
                statement_builder = vec![];
                name = String::new();
            },
            Opp::String(v)=>{
                // println!("is String : {:?}",last);
                if !object_started{
                    if last != 't'{is_valid_object = false;break;}
                    last = 'n';
                    object_started = true;
                    name = v.to_string();
                } else {
                    if last != 'm' && last != 'l' && last != 'v'{
                        is_valid_object = false;break;
                    }
                    if last == 'm'{last = 'n';name = v.to_string();} else
                    if last == 'l' || last == 'v'{
                        last = 'v';
                        // build.insert(name.clone(),(Opp::String(v.to_string()),item.1,item.2));
                        statement_builder.push((Opp::String(v.to_string()),item.1,item.2));
                    }
                }
            },
            Opp::Pointer(v)=>{
                // println!("is Pointer : {:?}",last);
                if !object_started{
                    if last != 't'{is_valid_object = false;break;}
                    last = 'n';
                    object_started = true;
                    name = v.to_string();
                } else {
                    if last != 'm' && last != 'l' && last != 'v'{
                        is_valid_object = false;break;
                    }
                    if last == 'm'{last = 'n';name = v.to_string();} else
                    if last == 'l' || last == 'v'{
                        last = 'v';
                        // build.insert(name.clone(),(Opp::Pointer(v.to_string()),item.1,item.2));
                        statement_builder.push((Opp::Pointer(v.to_string()),item.1,item.2));
                    }
                }
            },
            _v=>{
                // println!("is Random : {:?}",last);
                if last != 'l' && last != 'v'{is_valid_object = false;break;}
                last = 'v';
                // build.insert(name.clone(),_v.clone());
                // build.insert(name.clone(),(_v.clone(),item.1,item.2));
                statement_builder.push((_v.clone(),item.1,item.2));
            }
        }
    }

    // println!("is_valid_object : {:?}",is_valid_object);

    if is_valid_object{
        if name.len() > 0 && statement_builder.len() > 0{
            build.insert(
                name.clone(),
                (Opp::Statement(statement_builder),hold_item.1,hold_item.2)
            );
        }
    }

    if is_valid_object{
        return Ok((Opp::Object(build),base_start-1,end+1));
    } else {
        return Ok((Opp::Statement(collect),base_start-1,end+1));
    }

}

