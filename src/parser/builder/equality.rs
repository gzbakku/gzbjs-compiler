

use crate::{Compiler,Opp,Error};

pub fn init(app:&mut Compiler,opps:Vec<(Opp,usize,usize)>)->Result<Vec<(Opp,usize,usize)>,Error>{

    // println!("\n\n equality : {:?}",opps);

    let mut build:Vec<(Opp,usize,usize)> = vec![];
    let mut index = 0;

    loop{

        let item = &opps[index];
        let opp = &item.0;
        let mut generate = false;

        if
            opp == &Opp::IsEqual || opp == &Opp::StrictIsEqual ||
            opp == &Opp::NotEqual || opp == &Opp::StrictNotEqual
        {
            generate = true;
        } else {
            build.push(item.clone());
        }

        if generate{

            match &opps[index-1].0{
                Opp::Bool(_)=>{},
                Opp::String(_)=>{},
                Opp::Num(_)=>{},
                Opp::Object(_)=>{},
                Opp::Array(_)=>{},
                Opp::SECTION(_)=>{},
                Opp::Statement(_)=>{},
                Opp::Pointer(_)=>{},
                Opp::PATH(_)=>{},
                _=>{
                    return Err(Error::string(
                        format!("cannot use {:?} with a conditional operator {:?} at => {:?}",opp,opps[index-1].0,app.location(item.1))
                    ));
                }
            }

            match &opps[index+1].0{
                Opp::Bool(_)=>{},
                Opp::String(_)=>{},
                Opp::Num(_)=>{},
                Opp::Object(_)=>{},
                Opp::Array(_)=>{},
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

            build.remove(build.len()-1);
            build.push((Opp::VERIFY(
                Box::new(opp.clone()),
                Box::new(opps[index-1].0.clone()),
                Box::new(opps[index+1].0.clone())
            ),opps[index-1].1,opps[index+1].2));
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