

use crate::{Compiler,Opp,Error,Action};

pub mod condense;
pub mod mathify;
pub mod equality;
pub mod questionify;
pub mod noteify;
pub mod actionify;

pub fn init(
    app:&mut Compiler,
    mut opps:Vec<(Opp,usize,usize)>
)->Result<Vec<(Action,usize,usize)>,Error>{

    let mut state:bool = false;/*open=1 closed=0*/
    let mut index = 0;
    let mut actions:Vec<(Action,usize,usize)> = vec![];

    // println!("builder init");

    // println!("builder : {:?}\n\n",opps);

    match condense::init(app,opps){
        Ok(v)=>{
            opps = v;
        },
        Err(_e)=>{
            println!("builder condense failed => {:?}",_e);
            return Err(_e);
        }
    }

    if false{
        println!("condensed : {:?}",opps);
        return Err(Error::str("no_error"));
    }

    match noteify::init(app,opps){
        Ok(v)=>{
            opps = v;
        },
        Err(_e)=>{
            println!("builder noteify failed => {:?}",_e);
            return Err(_e);
        }
    }

    if false{
        println!("noteify : {:?}",opps);
        return Err(Error::str("no_error"));
    }

    match mathify::init(app,opps){
        Ok(v)=>{
            opps = v;
        },
        Err(_e)=>{
            println!("builder mathify failed => {:?}",_e);
            return Err(_e);
        }
    }

    // println!("\n\nmathify : {:?}\n\n",opps);

    if false{
        println!("mathify : {:?}",opps);
        return Err(Error::str("no_error"));
    }

    match equality::init(app,opps){
        Ok(v)=>{
            opps = v;
        },
        Err(_e)=>{
            println!("builder equality failed => {:?}",_e);
            return Err(_e);
        }
    }

    if false{
        println!("equality : {:?}",opps);
        return Err(Error::str("no_error"));
    }

    match questionify::init(app,opps){
        Ok(v)=>{
            opps = v;
        },
        Err(_e)=>{
            println!("builder questionify failed => {:?}",_e);
            return Err(_e);
        }
    }

    if false{
        println!("questionify : {:?}",opps);
        return Err(Error::str("no_error"));
    }

    match actionify::init(app,opps){
        Ok(v)=>{
            // opps = v;
            println!("actionify : {:?}",v);

            return Ok(v);

        },
        Err(_e)=>{
            println!("builder actionify failed => {:?}",_e);
            return Err(_e);
        }
    }

    // if true{
    //     println!("actionify : {:?}",opps);
    //     return Err(Error::str("no_error"));
    // }

    return Err(Error::str("no_error"));

}

