use crate::{Compiler,Opp};
use crate::Error;

pub struct Scope{
    pub name:String,
    // pub actions:Vec<Action>,
    // pub children:Vec<Scope>
}

impl Scope{
    pub fn init(scope_name:String)->Scope{
        Scope{
            name:scope_name,
            // actions:vec![],
            // children:vec![]
        }
    }
    // pub fn add(&mut self,action:Action){
    //     self.actions.push(action)
    // }
    // pub fn child(&mut self,scope:Scope){
    //     self.children.push(scope)
    // }
}

pub fn init(app:&mut Compiler,_scope_name:String,start:usize,end:usize)->Result<Scope,Error>{

    let mut s = start;
    // let mut e = end;

    let mut collect:Vec<(Opp,usize,usize)> = vec![];

    loop{
        match crate::parser::opp::init(app,s,end){
            Ok(v)=>{

                // println!("opp : {:?} {:?}",v,app.code.len());
                // app.print_code(v.1, v.2);

                s = v.2 + 1;

                if v.2 == app.code.len()-1 || v.1 > end{
                    break;
                }

                collect.push(v);

            },
            Err(_e)=>{
                println!("opp failed : {:?}",_e);
                return Err(_e);
            }
        }
    }

    // println!("scope : {:?}",collect);

    match crate::parser::builder::init(app, collect){
        Ok(_)=>{},
        Err(_e)=>{
            return Err(_e);
        }
    }

    Err(Error::str("no_error"))

}