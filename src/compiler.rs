use crate::Scope;
use crate::{Opp};
use crate::Error;

pub enum CompilerToDo{
    Declare,Var(usize),Value(usize),Operator,Line
}

pub struct Compiler{
    pub cursor:usize,
    pub code:Vec<char>,
    pub scope:Scope,
    pub active_opp:Opp
}

impl Compiler{
    pub fn init(code:String)->Compiler{
        Compiler{
            cursor:0,
            code:code.chars().collect(),
            scope:Scope::init("main".to_string()),
            active_opp:Opp::None
        }
    }
    pub fn location(&mut self,index:usize)->String{
        return format!("index : {:?}",index);
    }
    pub fn is_whitespace(&mut self,index:usize)->bool{
        if
            self.code[index] != ' ' &&
            self.code[index] != '\r' &&
            self.code[index] != '\n' &&
            self.code[index] != '\t'
        {
            false
        } else {
            true
        }
    }
    pub fn str_to_char_vec(s:&str)->Vec<char>{
        let mut collect = vec![];
        for i in s.chars(){
            collect.push(i);
        }
        return collect;
    }
    pub fn print_code(&mut self,start:usize,end:usize){
        let mut collect = String::new();
        for i in start..=end{
            collect.push(self.code[i]);
        }
        println!("{:?}",collect);
    }
    pub fn compile(&mut self)->Result<(),Error>{
        match crate::parser::scope::init(
            self,"main".to_string(),
            0,
            self.code.len()
        ){
            Ok(v)=>{
                self.scope = v;
                return Ok(());
            },
            Err(_e)=>{
                return Err(_e);
            }
        }
    }
    pub fn find(
        &mut self,
        pool:Vec<Vec<char>>,
        end:usize,
        exit_on_unmatch:bool
    )->Option<(usize,usize,usize)>{
        return find(self,pool,end,exit_on_unmatch);
    }
    pub fn get_value(&mut self,start:usize,end:usize)->String{
        let mut collect = String::new();
        for i in start..end{
            let element = self.code[i];
            if
                element != ' ' &&
                element != '\r' &&
                element != '\n' &&
                element != '\t'
            {
                collect.push(element);
            }
        }
        return collect;
    }
    pub fn find_opp(&mut self,start:usize,end:usize)->Result<(Opp,usize,usize),Error>{
        match crate::parser::opp::init(self,start,end){
            Ok(v)=>{
                return Ok(v);
            },
            Err(_e)=>{
                return Err(_e);
            }
        }
    }
}

//start,end,setIndex
fn find(
    app:&mut Compiler,
    pool:Vec<Vec<char>>,
    end_at:usize,
    exit_on_unmatch:bool
)->Option<(usize,usize,usize)>{

    let mut found:Vec<&Vec<char>> = vec![];
    let mut match_count = 0;
    let mut start:usize = 0;

    loop{

        let mut end;
        let pool = &pool;
        if app.cursor >= end_at{
            // println!("overflow find");
            if exit_on_unmatch{
                return Some((0,0,999));
            } else {
                return None;
            }
        }

        let element = app.code[app.cursor];

        // println!("{:?}",element);

        if found.len() == 0{
            for set in pool.iter(){
                if element == set[0]{
                    found.push(set);
                    match_count = 1;
                    start = app.cursor.clone();
                }
            }
        }

        if found.len() > 0{
            let mut unmatched = vec![];
            let mut found_index = 0;
            for set in found.iter(){
                if set.len() > match_count{
                    if set[match_count] == element{
                        match_count += 1;
                    } else {
                        unmatched.push(found_index);
                    }
                } else {
                    unmatched.push(found_index);
                }
                if set.len() == match_count{
                    end = app.cursor.clone();
                    //find ste in pool
                    let mut find_index = 0;
                    for base in pool{
                        if &base == set{
                            return Some((start,end,find_index));
                        }
                        find_index += 1;
                    }
                }
                found_index += 1;
            }
            unmatched.sort();
            unmatched.reverse();
            for i in unmatched{
                found.remove(i);
            }
        }

        if
            found.len() == 0 && 
            exit_on_unmatch &&
            element != ' ' &&
            element != '\t' &&
            element != '\n' &&
            element != '\r'
        {
            return Some((0,0,999));
        }

        // println!("element : {:?}",element);

        app.cursor += 1;

    }

}



