use std::collections::HashMap;
use crate::{Compiler,Error,Action};

pub mod bracket;
pub mod string;
pub mod comments;

#[derive(Clone,Debug,PartialEq)]
#[allow(non_camel_case_types)]
pub enum Opp{
    None,Property,Comma,EndStatement,
    If,Else,ElseIf,Function,Class,Switch,Let,Var,Const,Question,
    Plus,Minus,Divide,Multiply,
    AngleBracket,CurlyBracket,Bracket,
    Or,And,Equal,IsEqual,StrictIsEqual,NotEqual,StrictNotEqual,
    AttachFunction,Colon,
    This,Constructor,PlusEq,MinusEq,New,MultiplyEq,DivideEq,
    LessThen,MoreThen,MoreThenEq,LessThenEq,
    Return,Break,While,For,Do,
    Null,Undefined,Modulus,
    Async,Await,Increment,Decrement,Exponentiation,Not,Module,
    Comment(String),
    Pointer(String),
    String(String),
    StringBuild(String),
    Num(f64),
    Bool(bool),
    Array(Vec<(Opp,usize,usize)>),
    Statement(Vec<(Opp,usize,usize)>),
    Object(HashMap<String,(Opp,usize,usize)>),
    FunctionCaller(Vec<(u8,String)>),
    FUNCTION_EXECUTER(Box<Opp>,Box<Opp>,Box<Action>),
    FUNCTION(Function),
    STATE(Vec<State>),
    CLASS(Class),
    PATH(Vec<(usize,String)>),
    SECTION(Vec<(Opp,usize,usize)>),
    VERIFY(Box<Opp>,Box<Opp>,Box<Opp>),//match type,first item,second item
    NOT(Box<Opp>),
    // ternary 
    TERNARY(Box<Opp>,Box<Opp>,Box<Opp>),//verification statement,value builders
}

#[derive(Clone,Debug,PartialEq)]
pub struct Class{
    pub name:String,
    pub scope:Box<Opp>
}

#[derive(Clone,Debug,PartialEq)]
pub struct Function{
    pub name:String,
    pub caller:Box<Opp>,
    pub scope:Box<Opp>
}

#[derive(Clone,Debug,PartialEq)]
pub struct State{
    pub index:u16,
    pub check:Box<Opp>,
    pub body:Box<Opp>
}

const CONSTANTS:[&str;78] = [
    "//","/*",//2 53
    "=>",//1 45
    "if","function ","class ","switch ","let ","var ","const ",//7 7
    "*=","/=",//2 56
    "++","--","**","!",//4 76
    "+=","-=",//2 51
    "+","-","/","*",//4 11
    "(","{","[",//3 14
    "||","&&","===","==","=","!==","!=",//7 21
    "'","\"",//2 23
    ".",//1 24
    "for","while",//2 26
    "true","false",//2 28,
    "else if","else",//2 30,
    "function ","class ","switch ","let ","var ","const ",//6 36
    "function\n","class\n","switch\n","let\n","var\n","const\n",//6 42,
    ",",//1 43,
    ";",//1 44,
    ":",//1 46,
    "?",//1 47,
    "this","constructor",//2 51,
    "new",//1 54
    ">=","<=",">","<",//4 60
    "return","break","while","for","do",//5 65
    "null ","undefined ","null\n","undefined\n",//4 69
    "%",//1 70,
    "async","await",//2 72
    "module",//1 77
    "`",//1 78
    //77
];

const CONSTANT_OPERATORS:[&str;38] = [
    "//","/*",//2 26
    "=>",//1 20
    "*=","/=",//2 28
    "++","--","**","!",//4 37
    "+=","-=",//2 24
    "+","-","/","*",//4 4
    "(","{","[",//3 7
    "||","&&","=","==","===","!=","!==",//7 14
    "'","\"",//2 16
    ".",//1 17
    ",",//1 18
    ";",//1 19,
    ":",//1 21
    "?",//1 22
    ">=","<=",">","<",//4 32
    "%",//1 33
    "`",//1 38
    //38
];

pub fn init(
    app:&mut Compiler,
    start:usize,
    end:usize
)->Result<(Opp,usize,usize),Error>{

    let mut index = start;
    let mut collect = String::new();
    let mut started = false;
    let mut start = start;
    
    loop{

        if started && app.is_whitespace(index){
            // return Ok((Opp::Pointer(collect),start,index));
            match check_if_int(collect,start,index){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        }

        if started && !app.is_whitespace(index){
            match match_code_to_symbols(
                app,index,end
            ){
                Ok(v)=>{
                    if v.0{
                        // return Ok((Opp::Pointer(collect),start,index-1));
                        match check_if_int(collect,start,index-1){
                            Ok(v)=>{
                                return Ok(v);
                            },
                            Err(_e)=>{
                                return Err(_e);
                            }
                        }
                    }
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
            collect.push(app.code[index]);
        }

        if !started && !app.is_whitespace(index){
            let char = app.code[index];
            for item in CONSTANTS{
                let chars = Compiler::str_to_char_vec(item);
                if chars[0] == char{
                    let mut matched = true;
                    for i in 1..item.len(){
                        // println!("matched : {:?} {:?} {:?}",item,app.code[index+i],chars[i]);
                        if app.code[index+i] != chars[i]{
                            matched = false;
                        }
                    }
                    // println!("final matched : {:?}",matched);
                    if matched{
                        match match_to_item(app,item,index,end){
                            Ok(v)=>{
                                // println!("match_to_item : {:?}",v);
                                return Ok(v);
                            },
                            Err(_e)=>{
                                return Err(_e);
                            }
                        }
                    }
                }
            }
            start = index;
            started = true;
            collect.push(char);
        }

        if index == app.code.len()-1 || index == end{
            // println!("break at : {:?}",collect);
            if collect.len()>0{
                match check_if_int(collect,start,index){
                    Ok(v)=>{
                        // println!("p check_if_int => {:?}",v);
                        return Ok(v);
                    },
                    Err(_e)=>{
                        return Err(_e);
                    }
                }
            }
            break;
        } else {
            index += 1;
        }

    }

    if started{
        return Ok((Opp::Pointer(collect),start,index));
    }

    return Ok((Opp::None,start,index));

}

const NUMS:&'static str = "0123456789";

fn check_if_int(v:String,start:usize,end:usize)->Result<(Opp,usize,usize),Error>{

    // println!("check_if_int : {:?}",v);

    if v == "false"{
        return Ok((Opp::Bool(false),start,end)); 
    }
    if v == "true"{
        return Ok((Opp::Bool(true),start,end)); 
    }

    let mut all_nums = true;
    for i in v.chars(){
        // println!("{:?}",i);
        if !NUMS.contains(i){
            all_nums = false;
            break;
        }
    }

    if all_nums{
        match v.parse::<f64>(){
            Ok(v)=>{
                return Ok((Opp::Num(v),start,end));
            },
            Err(_)=>{
                return Ok((Opp::Pointer(v),start,end));
            }
        }
    }

    return Ok((Opp::Pointer(v),start,end));

}

fn match_code_to_symbols(
    app:&mut Compiler,
    index:usize,
    opp_limit:usize
)->Result<(bool,(Opp,usize,usize)),Error>{

    let char = app.code[index];

    for item in CONSTANT_OPERATORS{
        let chars = Compiler::str_to_char_vec(item);
        if chars[0] == char{
            let mut matched = true;
            for i in 1..item.len(){
                // println!("matched : {:?} {:?} {:?}",item,app.code[index+i],chars[i]);
                if app.code[index+i] != chars[i]{
                    matched = false;
                }
            }
            if matched{
                match match_to_item(app,item,index,opp_limit){
                    Ok(v)=>{
                        // println!("match_to_item : {:?}",v);
                        return Ok((true,v));
                    },
                    Err(_e)=>{
                        return Err(_e);
                    }
                }
            }
        }
    }

    return Ok((false,(Opp::None,index,index)));

}

fn match_to_item(
    app:&mut Compiler,
    item:&str,
    start:usize,
    opp_limit:usize
)->Result<(Opp,usize,usize),Error>{

    // "if","function","class","switch","let","var","const",//7 7
    // "+","-","/","*",//4 11
    // "(","{","[",//3 14
    // "||","&&","=","==","===","!=","!==",//7 21
    // "'","\"",//2 23
    // ".",//1 24

    // println!("match_to_item : {:?}",item);

    let end = start + item.len() - 1;

    //string
    {
        if item == "'"{
            match string::init(app,start,opp_limit,'\''){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        } else 
        if item == "\""{
            match string::init(app,start,opp_limit,'"'){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        } else 
        if item == "`"{
            match string::init(app,start,opp_limit,'`'){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        }
    }

    //comments
    {
        if item == "//"{
            match comments::init(app,start,opp_limit,"//"){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        } else 
        if item == "/*"{
            match comments::init(app,start,opp_limit,"/*"){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        }
    }

    {
        if item == "."{return Ok((Opp::Property,start,end));} else
        if item == ";"{return Ok((Opp::EndStatement,start,end));} else
        if item == ":"{return Ok((Opp::Colon,start,end));} else
        if item == "=>"{return Ok((Opp::AttachFunction,start,end));} else
        if item == "?"{return Ok((Opp::Question,start,end));} else
        if item == "%"{return Ok((Opp::Modulus,start,end));} else
        if item == ","{return Ok((Opp::Comma,start,end));}
    }

    {
        if item == "||"{return Ok((Opp::Or,start,end));} else 
        if item == "&&"{return Ok((Opp::And,start,end));} else 
        if item == "="{return Ok((Opp::Equal,start,end));} else 
        if item == "=="{return Ok((Opp::IsEqual,start,end));} else
        if item == "==="{return Ok((Opp::StrictIsEqual,start,end));} else
        if item == "!="{return Ok((Opp::NotEqual,start,end));} else
        if item == "!=="{return Ok((Opp::StrictNotEqual,start,end));} else
        if item == "+="{return Ok((Opp::PlusEq,start,end));} else
        if item == "-="{return Ok((Opp::MinusEq,start,end));} else
        if item == "*="{return Ok((Opp::MultiplyEq,start,end));} else
        if item == "/="{return Ok((Opp::DivideEq,start,end));} else
        if item == ">"{return Ok((Opp::LessThen,start,end));} else
        if item == "<"{return Ok((Opp::MoreThen,start,end));} else
        if item == ">="{return Ok((Opp::MoreThenEq,start,end));} else
        if item == "<="{return Ok((Opp::LessThenEq,start,end));}
    }

    //brackets
    {
        // if item == "("{return Ok((Opp::AngleBracket,start,end));} else 

        if item == "("{
            match bracket::angle::init(app,start,opp_limit){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        } else 
        if item == "{"{
            match bracket::curlybracket::init(app,start,opp_limit){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        } else
        if item == "["{
            match bracket::bracket::init(app,start,opp_limit){
                Ok(v)=>{
                    return Ok(v);
                },
                Err(_e)=>{
                    return Err(_e);
                }
            }
        }
        // if item == "["{return Ok((Opp::Bracket,start,end));} 
        // if item == "{"{return Ok((Opp::CurlyBracket,start,end));}
    }

    {
        if item == "if"{return Ok((Opp::If,start,end));} else 
        if item == "else if"{return Ok((Opp::ElseIf,start,end));} else 
        if item == "else"{return Ok((Opp::Else,start,end));} else 
        if item.contains("function"){return Ok((Opp::Function,start,end));} else 
        if item.contains("class") {return Ok((Opp::Class,start,end));} else 
        if item.contains("switch"){return Ok((Opp::Switch,start,end));} else 
        if item.contains("let"){return Ok((Opp::Let,start,end));} else 
        if item.contains("var"){return Ok((Opp::Var,start,end));} else 
        if item.contains("constructor"){return Ok((Opp::Constructor,start,end));} else 
        if item.contains("this"){return Ok((Opp::This,start,end));} else 
        if item.contains("const"){return Ok((Opp::Const,start,end));} else 
        if item == "true"{return Ok((Opp::Bool(true),start,end));} else 
        if item == "false"{return Ok((Opp::Bool(false),start,end));} else
        if item == "new"{return Ok((Opp::New,start,end));} else
        if item == "return"{return Ok((Opp::New,start,end));} else 
        if item == "for"{return Ok((Opp::For,start,end));} else
        if item == "while"{return Ok((Opp::While,start,end));} else
        if item == "do"{return Ok((Opp::Do,start,end));} else
        if item == "break"{return Ok((Opp::Break,start,end));} else
        if item == "null"{return Ok((Opp::Null,start,end));} else
        if item == "undefined"{return Ok((Opp::Undefined,start,end));} else
        if item == "module"{return Ok((Opp::Module,start,end));} 
    }

    {
        if item == "-"{return Ok((Opp::Minus,start,end));} else 
        if item == "+"{return Ok((Opp::Plus,start,end));} else 
        if item == "++"{return Ok((Opp::Increment,start,end));} else 
        if item == "--"{return Ok((Opp::Decrement,start,end));} else
        if item == "**"{return Ok((Opp::Exponentiation,start,end));} else 
        if item == "!"{return Ok((Opp::Not,start,end));} else  
        if item == "*"{return Ok((Opp::Multiply,start,end));} else 
        if item == "/"{return Ok((Opp::Divide,start,end));}
    }

    return Ok((Opp::None,start,start));

}