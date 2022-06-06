

use crate::{Compiler,Opp,Error};

pub fn init(app:&mut Compiler,opps:Vec<(Opp,usize,usize)>)->Result<Vec<(Opp,usize,usize)>,Error>{

    let mut build:Vec<(Opp,usize,usize)> = vec![];
    let mut index = 0;
    

    loop{

        let item = &opps[index];
        let opp = &item.0;
        let mut generate = false;

        if opp == &Opp::Not{
            generate = true;
        } else {
            build.push(item.clone());
        }

        if generate{

            match &opps[index+1].0{
                Opp::Bool(_)=>{},
                Opp::Num(_)=>{},
                Opp::SECTION(_)=>{},
                Opp::Statement(_)=>{},
                Opp::Pointer(_)=>{},
                Opp::PATH(_)=>{},
                _=>{
                    return Err(Error::string(
                        format!("cannot use {:?} with a conditional operator {:?} at => {:?}",opp,opps[index+1].0,app.location(item.1))
                    ));
                }
            }

            // build.remove(build.len()-1);
            build.push((Opp::NOT(
                Box::new(opps[index+1].0.clone())
            ),opps[index].1,opps[index+1].2));
            index += 1;

        }

        if index >= opps.len()-1{
            break;
        } else {
            index += 1;
        }

    }

    return Ok(build);

}