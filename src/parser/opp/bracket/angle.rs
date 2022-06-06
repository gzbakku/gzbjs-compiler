
use crate::{Opp,Compiler,Error};


pub fn init(app:&mut Compiler,start:usize,end:usize)->Result<(Opp,usize,usize),Error>{

    let mut index = start;

    let mut char_count = 0;

    loop{

        let char = app.code[index];

        if char == '('{
            char_count += 1;
        }

        if char == ')'{
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

        // println!("angle char : {:?}",char);

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
                // println!("angle : {:?} {:?}",v,end);
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

    return Ok((Opp::Statement(collect),base_start-1,end+1));

}

/*

//function callers
    {
        let mut is_function_callers = true;
        let mut last:char = 'n';
        for item in collect.iter(){
            match item.0{
                Opp::Pointer(_)=>{
                    if last != 'n' && last != 'c'{
                        is_function_callers = false;break;
                    } else { last = 'p';}
                },
                Opp::Comma=>{
                    if last != 'p'{
                        is_function_callers = false;break;
                    } else { last = 'c';}
                },
                _=>{
                    is_function_callers = false;break;
                }
            }
        }
        if is_function_callers{
            if last == 'c'{
                is_function_callers = false;
            }
        }
        if is_function_callers{
            let mut clean = vec![];
            let mut hold = true;
            for item in collect{
                if hold{
                    clean.push(item);
                    hold = false;
                } else {
                    hold = true;
                }
            }
            return Ok((Opp::FunctionBinders(clean),base_start-1,end+1));
        }
    }

    //function executers
    {
        let mut is_function_executers = true;
        let mut last:char = 'n';
        for item in collect.iter(){
            match &item.0{
                Opp::Comma=>{
                    if last != 'p'{
                        is_function_executers = false;break;
                    } else { last = 'c';}
                },
                _v=>{
                    if last != 'n' && last != 'c'{
                        is_function_executers = false;break;
                    } else { last = 'p';}
                }
            }
        }
        if is_function_executers{
            if last == 'c'{
                is_function_executers = false;
            }
        }
        if is_function_executers{
            let mut clean = vec![];
            let mut hold = true;
            for item in collect{
                if hold{
                    clean.push(item);
                    hold = false;
                } else {
                    hold = true;
                }
            }
            return Ok((Opp::FunctionExecuters(clean),base_start-1,end+1));
        }
    }

*/

