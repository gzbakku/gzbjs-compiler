

use crate::{Compiler,Opp,Error};

pub fn init(app:&mut Compiler,opps:Vec<(Opp,usize,usize)>)->Result<Vec<(Opp,usize,usize)>,Error>{

    let mut build:Vec<(Opp,usize,usize)> = vec![];
    let mut index = 0;

    loop{

        let item = &opps[index];
        let opp = &item.0;

        // println!("opp : {:?}",opp);

        match opp{
            Opp::Question=>{

                //loop until u find valid colon

                let mut colon_count = 0;
                let mut collect:Vec<(Opp,usize,usize)> = vec![];

                if index == 0{
                    return Err(Error::string(
                        format!("why is there a question mark here?? => {:?}",app.location(item.1))
                    ));
                }

                match opps[index-1].0{
                    Opp::Pointer(_)=>{},
                    Opp::PATH(_)=>{},
                    Opp::Statement(_)=>{},
                    Opp::VERIFY(_,_,_)=>{},
                    Opp::NOT(_)=>{},
                    _=>{
                        return Err(Error::string(
                            format!("why is there a question mark here?? => {:?}",app.location(item.1))
                        ));
                    }
                }

                build.remove(build.len()-1);
                collect.push(opps[index-1].clone());

                loop{

                    let local_item = &opps[index];
                    let local_opp = &local_item.0;

                    if local_opp == &Opp::Question{
                        colon_count += 1;
                    }
                    if local_opp == &Opp::Colon{
                        colon_count -= 1;
                    }

                    // println!("local_opp : {:?} {:?}",local_opp,colon_count);

                    if colon_count == 0{
                        collect.push(local_item.clone());
                        collect.push(opps[index+1].clone());
                        index += 1;
                        match build_nested_opps(
                            app,
                            collect
                        ){
                            Ok(v)=>{
                                build.push(v);
                                // collect = vec![];
                                break;
                                // return Ok(build);
                            },
                            Err(_e)=>{
                                return Err(_e);
                            }
                        }
                    } else {
                        collect.push(local_item.clone());
                    }

                    if index >= opps.len()-1{
                        break;
                    } else {
                        index += 1;
                    }

                }

                // return Err(Error::string(
                //     format!("invalid ternary operator => {:?}",app.location(item.1))
                // ));

                // println!("\nis question : {:?}\n",collect);

            },
            _=>{
                build.push(item.clone());
            }
        }

        if index >= opps.len()-1{
            break;
        } else {
            index += 1;
        }

    }

    return Ok(build);

}

fn build_nested_opps(
    app:&mut Compiler,
    c:Vec<(Opp,usize,usize)>
)->Result<(Opp,usize,usize),Error>{

    // println!("\nc : {:?}\n",c);

    if c.len() == 0{
        return Err(Error::str("why is this opp empty"));
    }
    if c.len() == 1{
        return Ok(c[0].clone());
    }

    let start_at = c[0].1;
    let end_at = c[c.len()-1].2;

    let mut index = 0;
    let mut last_colon = 0;
    for i in &c{
        if i.0 == Opp::Colon{
            last_colon = index;
        }
        index += 1;
    }
    index = 0;

    let mut first_question = 0;
    let mut no_question = true;
    for i in &c{
        if i.0 == Opp::Question{
            first_question = index;
            no_question = false;
            break;
        }
        index += 1;
    }

    if no_question{
        return Ok((Opp::Statement(c),start_at,end_at));
    }

    let mut verification = vec![];
    for i in 0..first_question{
        verification.push(c[i].clone());
    }

    let verification_opp;
    if verification.len() == 1{
        verification_opp = verification[0].clone();
    } else {
        verification_opp = (
            Opp::Statement(verification.clone()),
            verification[0].1,
            verification[verification.len()-1].2
        );
    }

    // println!("verification : {:?}",verification);

    let mut first = vec![];
    for i in first_question+1..last_colon{
        first.push(c[i].clone());
    }
    if first_question+1 == last_colon{
        first.push(c[first_question+1].clone());
    }

    // println!("\nfirst : {:?}\n",first);

    let first_opp;
    if first.len() == 1{
        first_opp = first[0].clone();
    } else {
        match build_nested_opps(app,first){
            Ok(v)=>{
                first_opp = v;
            },
            Err(_e)=>{
                return Err(_e);
            }
        }
    }

    // println!("2 start : {:?} end : {:?}",last_colon+1,c.len()-1);

    let mut second = vec![];
    for i in last_colon+1..c.len()-1{
        // println!("2 : {:?}",i);
        second.push(c[i].clone());
    }
    if last_colon+1 == c.len()-1{
        second.push(c[last_colon+1].clone());
    }

    // println!("\nsecond : {:?}\n",second);

    let second_opp;
    if second.len() == 1{
        second_opp = second[0].clone();
    } else {
        match build_nested_opps(app,second){
            Ok(v)=>{
                second_opp = v;
            },
            Err(_e)=>{
                return Err(_e);
            }
        }
    }

    let build = (Opp::TERNARY(
        Box::new(verification_opp.0),
        Box::new(first_opp.0),
        Box::new(second_opp.0),
    ),start_at,end_at);

    // println!("\n ternary : {:?}\n",build);

    return Ok(build);

}