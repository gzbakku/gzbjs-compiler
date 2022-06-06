
use crate::{Compiler,Opp,Error,Function,State,Class};


pub fn init(app:&mut Compiler,opps:Vec<(Opp,usize,usize)>)->Result<Vec<(Opp,usize,usize)>,Error>{

    // println!("\n\ncondense before : {:?}\n\n",opps);

    let mut build:Vec<(Opp,usize,usize)> = vec![];
    let mut index = 0;

    loop{

        let item = &opps[index];
        let opp = &item.0;

        match opp{
            Opp::Function=>{
                //check if previous is equal

                //check if next is name
                let name:String;
                let mut caller:Opp = Opp::None;
                let mut state;//name = 1 caller = 2 scope = 3
                let start_at:usize;

                if index+1 >= opps.len(){
                    return Err(Error::string(
                        format!("expected function name => {:?}",app.location(opps[index+1].1))
                    ));
                } else {index += 1;}

                match &opps[index].0{
                    Opp::Pointer(v)=>{
                        name = v.to_string();
                        state = 2;
                    },
                    Opp::Statement(v)=>{
                        name = String::new();
                        match build_function_caller(v){
                            Ok(v)=>{
                                caller = v;
                                state = 3;
                            },
                            Err(_e)=>{
                                return Err(Error::string(
                                    format!("invalid function caller => {:?} at => {}",_e,app.location(opps[index].1))
                                ));
                            }
                        }
                    },
                    _v=>{
                        return Err(Error::string(
                            format!("expected function name or caller => {:?}",app.location(opps[index].1))
                        ));
                    },
                }
                start_at = opps[index+1].1;
                if opps.len() <= index + 1{
                    return Err(Error::string(
                        format!("invalid function => {:?}",app.location(opps[index+1].1))
                    ));
                } else {index = index + 1;}

                if state == 2{
                    match &opps[index].0{
                        Opp::Statement(v)=>{
                            match build_function_caller(v){
                                Ok(v)=>{
                                    caller = v;
                                    state = 3;
                                },
                                Err(_e)=>{
                                    return Err(Error::string(
                                        format!("invalid function caller => {:?} at => {:?}",_e,app.location(opps[index].1))
                                    ));
                                }
                            }
                        },
                        _v=>{
                            return Err(Error::string(
                                format!("expected function caller => {:?}",app.location(opps[index].1))
                            ));
                        },
                    }
                }
                if opps.len() <= index + 1{
                    return Err(Error::string(
                        format!("invalid function => {:?}",app.location(opps[index+1].1))
                    ));
                } else {index = index + 1;}

                if state == 3{
                    match &opps[index].0{
                        Opp::Statement(v)=>{
                            build.push((Opp::FUNCTION(Function{
                                name:name,
                                caller:Box::new(caller),
                                scope:Box::new(Opp::Statement(v.to_vec()))
                            }),start_at,opps[index].2));
                        },
                        _v=>{
                            return Err(Error::string(
                                format!("expected function body => {:?}",app.location(opps[index].1))
                            ));
                        },
                    }
                }
                
                //check if next is statement and build function caller
                //check if next is statement and build function scope
            },
            Opp::AttachFunction=>{

                let caller:Opp;
                let start_at:usize;

                //check last opp
                if index == 0{
                    return Err(Error::string(
                        format!("invalid function attach at => {:?}",app.location(
                            opps[index].1
                        ))
                    ));
                }
                match &opps[index-1].0{
                    Opp::Statement(v)=>{
                        match build_function_caller(&v){
                            Ok(v)=>{
                                caller = v;
                            },
                            Err(_e)=>{
                                return Err(Error::string(
                                    format!("invalid function caller => {:?} at => {}",_e,app.location(opps[index-1].1))
                                ));
                            }
                        }
                    },
                    _=>{
                        return Err(Error::string(
                            format!("expected function caller => {:?}",app.location(
                                opps[index-1].1
                            ))
                        ));
                    }
                }
                start_at = opps[index-1].1;

                match &opps[index+1].0{
                    Opp::Statement(v)=>{
                        match build[build.len()-1].0{
                            Opp::Statement(_)=>{
                                build.remove(build.len()-1);
                            },
                            _=>{}
                        }
                        build.push((Opp::FUNCTION(Function{
                            name:String::new(),
                            caller:Box::new(caller),
                            scope:Box::new(Opp::Statement(v.clone()))
                        }),start_at,opps[index+1].2));
                        index = index+1;
                    },
                    _=>{
                        return Err(Error::string(
                            format!("expected function body => {:?}",app.location(
                                opps[index+1].1
                            ))
                        ));
                    }
                }
                

            }
            Opp::If=>{

                let mut states = vec![];
                let mut state = 'l';//start=l if=i elseif=f else=e statement=s
                let mut check:Opp = Opp::None;
                let mut state_index = 0;
                let mut start_at = 0;

                loop{

                    let local_item = &opps[index];
                    let local_opp = &local_item.0;

                    // println!("local_opp : {:?} {:?}",local_opp,index);

                    match local_opp{
                        Opp::If=>{
                            if state != 'l' && state != 'e' && state != 'b'{
                                break;
                            }
                            if state == 'l'{
                                start_at = local_item.1;
                            }
                            state = 'i';
                        },
                        Opp::ElseIf=>{
                            if state != 'b'{
                                break;
                            }
                            state = 'f';
                        },
                        Opp::Else=>{
                            if state != 'b'{
                                break;
                            }
                            state = 'e';
                        },
                        Opp::Statement(v)=>{
                            if state != 'e' && state != 'i' && state != 'c' && state != 'f'{
                                break;
                            } else
                            if state == 'i' || state == 'f'{
                                check = Opp::Statement(v.clone());
                                state = 'c';
                            } else
                            if state == 'c'{
                                states.push(State{
                                    index:state_index,
                                    check:Box::new(check.clone()),
                                    body:Box::new(Opp::Statement(v.clone()))
                                });
                                state_index += 1;
                                state = 'b';
                            } else 
                            if state == 'e'{
                                states.push(State{
                                    index:state_index,
                                    check:Box::new(Opp::None),
                                    body:Box::new(Opp::Statement(v.clone()))
                                });
                                // index += 1;
                                break;
                            }
                        }
                        _=>{
                            // index = index - 1;
                            break;
                        }
                    }

                    if index >= opps.len()-1{
                        break;
                    } else {
                        index += 1;
                    }

                }

                // println!("if states {:?}",states);

                build.push((Opp::STATE(states),start_at,opps[index-1].2));

            },
            Opp::Class=>{

                let start_at = item.1;

                if index+1 >= opps.len(){
                    return Err(Error::string(
                        format!("expected class name => {:?}",app.location(opps[index+1].1))
                    ));
                } else {index += 1;}

                let name:String;
                match &opps[index].0{
                    Opp::Pointer(v)=>{
                        name = v.to_string();
                    },
                    _=>{
                        return Err(Error::string(
                            format!("expected class name => {:?}",app.location(opps[index].1))
                        ));
                    }
                }
                if index+1 >= opps.len(){
                    return Err(Error::string(
                        format!("expected class name => {:?}",app.location(opps[index+1].1))
                    ));
                } else {index += 1;}

                match &opps[index].0{
                    Opp::Statement(v)=>{
                        build.push((Opp::CLASS(Class{
                            name:name,
                            scope:Box::new(Opp::Statement(v.clone())),
                        }),start_at,opps[index].2));
                    },
                    _=>{
                        return Err(Error::string(
                            format!("expected class name => {:?}",app.location(opps[index+1].1))
                        ));
                    }
                }

            },
            Opp::Array(a)=>{

                let mut last_was_pointer = false;
                let len = a.len();
                match &opps[index-1].0{
                    Opp::Pointer(_)=>{last_was_pointer = true;},
                    Opp::Statement(_)=>{last_was_pointer = true;},
                    _=>{}
                }
                if !last_was_pointer || len != 1{
                    build.push(item.clone());
                } else {

                    let mut last_is_prop = false;
                    let mut collect:Vec<(usize,String)> = vec![];
                    let mut path_index = 0;
                    let start_at = item.1;
                    let mut end_at = 0;
                    let mut last_prop_boundary:(usize,usize) = (0,0);

                    loop{

                        let local_item = &opps[index];
                        let local_opp = &local_item.0;

                        match local_opp{
                            Opp::Array(b)=>{
                                if b.len() != 1{
                                    break;
                                }
                                // if last_is_prop{
                                //     break;
                                // }
                                last_is_prop = false;
                                match &b[0].0{
                                    Opp::Pointer(v)=>{
                                        collect.push((path_index,v.clone()));
                                        end_at = b[0].2;
                                    },
                                    Opp::String(v)=>{
                                        collect.push((path_index,v.clone()));
                                        end_at = b[0].2;
                                    },
                                    _=>{
                                        return Err(Error::string(
                                            format!("expected a array path => {:?}",app.location(local_item.1))
                                        ));
                                    }
                                }
                            },
                            Opp::Pointer(v)=>{
                                if !last_is_prop{
                                    break;
                                }
                                last_is_prop = false;
                                collect.push((path_index,v.clone()));
                                end_at = local_item.2;
                            }
                            Opp::Property=>{
                                last_is_prop = true;
                                last_prop_boundary = (local_item.1,local_item.2);
                            },
                            _=>{
                                break;
                            }
                        }

                        if index >= opps.len()-1{
                            break;
                        } else {
                            index += 1;
                            path_index += 1;
                        }

                    }

                    //add path to code
                    if collect.len() > 0{
                        build.push((Opp::PATH(collect),start_at,end_at));
                    }

                    //if last was prop add it back to code
                    if last_is_prop{
                        build.push((
                            Opp::Property,
                            last_prop_boundary.0,
                            last_prop_boundary.1
                        ));
                    }

                    //bring index to break on element
                    index -= 1;

                }

            },
            Opp::Comment(_)=>{},
            _=>{
                build.push(item.clone());
            },
        }

        if index >= opps.len()-1{
            break;
        } else {
            index += 1;
        }

    }

    return Ok(build);

    // return Err(Error::str("no_error"));

}

fn build_function_caller(pool:&Vec<(Opp,usize,usize)>)->Result<Opp,Error>{

    let mut hold:Vec<(u8,String)> = vec![];

    let mut last = 'l';
    let mut pointer_index = 0;
    for item in pool{
        let opp = &item.0;
        match opp{
            Opp::Comma=>{
                if last != 'p'{
                    return Err(Error::string(
                        format!("expected a comma")
                    ));
                } else {
                    last = 'c';
                }
            },
            Opp::Pointer(v)=>{
                if last != 'l' && last != 'c'{
                    return Err(Error::string(
                        format!("expected a pointer")
                    ));
                }
                hold.push((pointer_index,v.to_string()));
                last = 'p';
                pointer_index += 1;
            },
            _=>{
                return Err(Error::string(
                    format!("expected a pointer name or comma")
                ));
            }
        }
    }

    return Ok(Opp::FunctionCaller(hold));

    // return Err(Error::str("no_error"));

}