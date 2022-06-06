

use crate::{Compiler,Opp,Error,Action};


/*
    n=none  p=plus  s=minus 
    d=divide  m=multiple  i=int  t=string 
    r=pointer h=path e=statement x=exponention
*/
const WORKERSTATES:&'static str = "psdmx";

pub fn init(app:&mut Compiler,opps:Vec<(Opp,usize,usize)>)->Result<Vec<(Opp,usize,usize)>,Error>{

    // println!("\n\n mathify : {:?}\n\n",opps);

    let mut build:Vec<(Opp,usize,usize)> = vec![];
    let mut index = 0;
    let mut started = false;
    let mut collect:Vec<(Opp,usize,usize)> = vec![];
    let mut last_state = 'n';
    let mut section_boundary = (0,0);

    loop{

        let item = &opps[index];
        let opp = &item.0;

        match &opp{
            Opp::Plus=>{
                if !started{
                    return Err(Error::string(
                        format!("invalid add opp : {:?}",app.location(item.1))
                    ));
                }
                last_state = 'p';
                collect.push(item.clone());
            },
            Opp::Minus=>{
                if !started{
                    return Err(Error::string(
                        format!("invalid minus opp : {:?}",app.location(item.1))
                    ));
                }
                last_state = 's';
                collect.push(item.clone());
            },
            Opp::Multiply=>{
                if !started{
                    return Err(Error::string(
                        format!("invalid multiply opp : {:?}",app.location(item.1))
                    ));
                }
                last_state = 'm';
                collect.push(item.clone());
            },
            Opp::Divide=>{
                if !started{
                    return Err(Error::string(
                        format!("invalid divide opp : {:?}",app.location(item.1))
                    ));
                }
                last_state = 'd';
                collect.push(item.clone());
            },
            Opp::Exponentiation=>{
                if !started{
                    return Err(Error::string(
                        format!("invalid exponentiation opp : {:?}",app.location(item.1))
                    ));
                }
                last_state = 'x';
                collect.push(item.clone());
            },
            Opp::Num(_)=>{
                if started{
                    if !WORKERSTATES.contains(last_state){
                        return Err(Error::string(
                            format!("invalid number : {:?}",app.location(item.1))
                        ));
                    }
                    section_boundary.1 = item.2;
                } else {
                    if last_state != 'n'{
                        return Err(Error::string(
                            format!("invalid number : {:?}",app.location(item.1))
                        ));
                    }
                    section_boundary.0 = item.1;
                    started = true;
                }
                last_state = 'i';
                collect.push(item.clone());
            },
            Opp::String(_)=>{
                if started{
                    if !WORKERSTATES.contains(last_state){
                        return Err(Error::string(
                            format!("invalid string : {:?}",app.location(item.1))
                        ));
                    }
                    section_boundary.1 = item.2;
                } else {
                    if last_state != 'n'{
                        return Err(Error::string(
                            format!("invalid string : {:?}",app.location(item.1))
                        ));
                    }
                    started = true;
                    section_boundary.0 = item.1;
                }
                last_state = 't';
                collect.push(item.clone());
            },
            Opp::StringBuild(_)=>{
                if started{
                    if !WORKERSTATES.contains(last_state){
                        return Err(Error::string(
                            format!("invalid string : {:?}",app.location(item.1))
                        ));
                    }
                    section_boundary.1 = item.2;
                } else {
                    if last_state != 'n'{
                        return Err(Error::string(
                            format!("invalid string : {:?}",app.location(item.1))
                        ));
                    }
                    started = true;
                    section_boundary.0 = item.1;
                }
                last_state = 't';
                collect.push(item.clone());
            },
            Opp::Pointer(_)=>{
                if started{
                    if !WORKERSTATES.contains(last_state){
                        if started && collect.len() > 0{
                            if collect.len() == 1{
                                build.push(collect[0].clone());
                            } else {
                                build.push((Opp::SECTION(collect),section_boundary.0,section_boundary.1));
                            }
                            collect = vec![];
                        }
                        started = false;
                        last_state = 'n';
                        build.push(item.clone());
                    } else {
                        section_boundary.1 = item.2;
                    }
                } else {
                    if last_state != 'n'{
                        if started && collect.len() > 0{
                            if collect.len() == 1{
                                build.push(collect[0].clone());
                            } else {
                                build.push((Opp::SECTION(collect),section_boundary.0,section_boundary.1));
                            }
                            collect = vec![];
                        }
                        started = false;
                        last_state = 'n';
                        build.push(item.clone());
                    } else {
                        started = true;
                        section_boundary.0 = item.1;
                        last_state = 'r';
                        collect.push(item.clone());
                    }
                }
            },
            Opp::PATH(_)=>{
                /*
                    n=none  p=plus  s=minus 
                    d=divide  m=multiple  i=int  t=string 
                    r=pointer h=path e=statement
                */
                if !started{
                    return Err(Error::string(
                        format!("invalid path : {:?}",app.location(item.1))
                    ));
                }
                if last_state != 'r' && last_state != 'e'{
                    return Err(Error::string(
                        format!("invalid path : {:?}",app.location(item.1))
                    ));  
                }
                section_boundary.1 = item.2;
                last_state = 'h';
                collect.push(item.clone());
            },
            Opp::Statement(_)=>{
                if started{
                    if
                        !WORKERSTATES.contains(last_state) && 
                        (
                            last_state != 'r' &&
                            last_state != 'p' &&
                            last_state != 't'
                        )
                    {
                        return Err(Error::string(
                            format!("invalid statement : {:?}",app.location(item.1))
                        ));
                    }
                    section_boundary.1 = item.2;
                    if last_state == 'r'{//pointer
                        let a:Action;
                        match build_function_executers(app, opp.clone()){
                            Ok(v)=>{a = v;},
                            Err(_e)=>{
                                return Err(_e);
                            }
                        }
                        collect.push((
                            Opp::FUNCTION_EXECUTER(
                                Box::new(collect[collect.len()-1].0.clone()),
                                Box::new(Opp::None),
                                Box::new(a)
                            ),
                            collect[collect.len()-1].1,
                            item.2
                        ));
                        collect.remove(collect.len()-2);
                    } else
                    if last_state == 'h'{//path
                        match collect[collect.len()-2].0{
                            Opp::Pointer(_)=>{},
                            _=>{
                                return Err(Error::string(
                                    format!("path without a pointer is invalid : {:?}",app.location(item.1))
                                ));
                            }
                        }
                        let a:Action;
                        match build_function_executers(app, opp.clone()){
                            Ok(v)=>{a = v;},
                            Err(_e)=>{
                                return Err(_e);
                            }
                        }
                        collect.push((
                            Opp::FUNCTION_EXECUTER(
                                Box::new(collect[collect.len()-2].0.clone()),
                                Box::new(collect[collect.len()-1].0.clone()),
                                Box::new(a)
                            ),
                            collect[collect.len()-2].1,
                            item.2
                        ));
                        collect.remove(collect.len()-3);
                        collect.remove(collect.len()-2);
                    } else {
                        collect.push(item.clone());
                    }
                } else {
                    if last_state != 'n'{
                        return Err(Error::string(
                            format!("invalid statement : {:?}",app.location(item.1))
                        ));
                    }
                    started = true;
                    section_boundary.0 = item.1;
                    collect.push(item.clone());
                }
                last_state = 'e';
            },
            _=>{
                if WORKERSTATES.contains(last_state){
                    return Err(Error::string(
                        format!("data opp and data structure does not match : {:?}",app.location(item.1))
                    ));
                }
                if started && collect.len() > 0{
                    if collect.len() == 1{
                        build.push(collect[0].clone());
                    } else {
                        // println!("\nsection : {:?}\n",collect);
                        build.push((Opp::SECTION(collect),section_boundary.0,section_boundary.1));
                    }
                    collect = vec![];
                }
                started = false;
                last_state = 'n';
                build.push(item.clone());
            }
        }

        if index >= opps.len()-1{
            break;
        } else {
            index += 1;
        }

    }

    // println!("\n\n mathify after : {:?}\n\n",collect);

    if collect.len() > 0{
        build.push((Opp::SECTION(collect),section_boundary.0,section_boundary.1));
        collect = vec![];
    }

    return Ok(build);

}

fn build_function_executers(app:&mut Compiler,statement:Opp)->Result<Action,Error>{

    let pool:Vec<(Opp,usize,usize)>;
    match statement{
        Opp::Statement(v)=>{pool = v;},
        _=>{
            return Err(Error::str("i expected this to be a function executer"));
        }
    }

    let mut collect = vec![];
    let mut local = vec![];
    for i in pool{
        match i.0{
            Opp::Comma=>{
                collect.push(local);
                local = vec![];
            },
            _=>{
                local.push(i);
            }
        }
    }

    if local.len()>0{
        collect.push(local);
    }

    println!("{:?}",collect);

    let mut build = vec![];
    let mut index = 0;
    for i in collect{
        if i.len() > 0{
            match crate::parser::builder::init(app, i){
                Ok(v)=>{
                    println!("statement : {:?}",v);
                    build.push((index,v));
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
            index += 1;
        }
    }

    return Ok(Action::Function_Executer(build));

    // return Err(Error::str("no_error"));

}