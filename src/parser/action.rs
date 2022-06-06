

use crate::Opp;

#[derive(Clone,Debug,PartialEq)]
#[allow(non_camel_case_types)]
pub enum Action{
    None,Assign(Assign),Reassign(Reassign),
    Function_Executer(Vec<(usize,Vec<(Action,usize,usize)>)>),
    Point(Opp,usize,usize)
}

#[derive(Clone,Debug,PartialEq)]
pub struct Assign{
    pub assigner_type:(Opp,usize,usize),
    pub pointer:(Opp,usize,usize),
    pub assigner:(Opp,usize,usize),
    pub value:(Opp,usize,usize),
}

#[derive(Clone,Debug,PartialEq)]
pub struct Reassign{
    pub pointer:(Opp,usize,usize),
    pub assigner:(Opp,usize,usize),
    pub value:(Opp,usize,usize),
}

#[derive(Clone,Debug)]
pub struct Function{
    pub name:String,
    pub caller:Vec<(usize,String)>,
    pub body:Box<Action>
}

#[derive(Clone,Debug)]
pub struct Class{
    pub class:(Opp,usize,usize)
}


