

use crate::{Compiler,Opp,Error,Action};
use crate::parser::action::*;

pub fn init(app:&mut Compiler,opps:Vec<(Opp,usize,usize)>)
->Result<Vec<(Action,usize,usize)>,Error>
{

    println!("\n\nactionify : {:?}\n\n",opps);

    let mut build:Vec<(Action,usize,usize)> = vec![];
    let mut index = 0;
    let mut state:&str;
    // let mut build:Vec<(Opp,usize,usize)> = vec![];

    let statements:Vec<(&'static str,Vec<Vec<&'static str>>)> = vec![
        ("assign",vec![
            vec!["var","let","const"],
            vec!["pointer"],
            vec!["equal"],
            vec![
                "pointer","function","class","statement","string","num",
                "array","object","bool","path","state","verify","not","ternary",
                "function_executer","string_build","section"
            ]
        ]),
        ("reassign",vec![
            vec!["pointer"],
            vec!["equal","pluseq","minuseq","multiplyeq","divideeq"],
            vec![
                "pointer","function","class","statement","string","num",
                "array","object","bool","path","state","verify","not","ternary",
                "function_executer","string_build","section"
            ]
        ]),
        ("function",vec![
            vec!["function"],
        ]),
        ("class",vec![
            vec!["class"],
        ]),
        ("end",vec![
            vec!["end"],
        ]),
        ("state",vec![
            vec!["state"],
        ]),
        ("return",vec![
            vec!["return"],
            vec![
                "pointer","function","class","statement","string","num",
                "array","object","bool","path","state","verify","not","ternary",
                "function_executer","string_build","section"
            ],
        ]),
        ("modifier",vec![
            vec!["pointer","path"],
            vec![
                "increment","decrement","exponentiation",
            ],
        ]),

        ("single",vec![
            vec![
                "pointer","function","class","statement","string","num",
                "array","object","bool","path","state","verify","not","ternary",
                "function_executer","string_build","section"
            ],
        ]),

    ];

    loop{

        let item = &opps[index];
        let opp = &item.0;
        state = opp_type(opp);
        let mut matched = false;

        for case in &statements{

            if case.1[0].contains(&state){

                let mut case_index = 0;
                let mut case_matched = true;
                for case_item in &case.1{
                    if index+case_index >= opps.len(){
                        return Err(Error::string(
                            format!("either u r missing some code here or i m stupid {:?} => {:?}",opp,app.location(item.1)
                        )));
                    }
                    let local_state = opp_type(&opps[index+case_index].0);
                    if !case.1[case_index].contains(&local_state){
                        case_matched = false;
                        break;
                    }
                    case_index += 1;
                }
                
                if case_matched{
                    if case.0 != "end"{
                        let mut collect = vec![];
                        let mut collection_index = index;
                        for _ in &case.1{
                            collect.push(opps[collection_index].clone());
                            collection_index += 1;
                        }
                        // println!("case : {:?} items : {:?}",case.0,collect);
                        build.push(build_action(case.0, collect));
                    }
                    index += case.1.len();
                    matched = true;
                    break;
                }

            }

        }

        if !matched{
            return Err(Error::string(
                format!("i dont know what to do with this code {:?} => {:?}",opp,app.location(item.1)
            )));
        }

        if index >= opps.len()-1{
            break;
        }

    }

    return Ok(build);

}

fn build_action(case:&str,opps:Vec<(Opp,usize,usize)>)->(Action,usize,usize){

    let start_at = opps[0].1;
    let end_at = opps[opps.len()-1].2;

    let build:Action;
    if case == "function"{

        println!("func : {:?}",opps);

        build = Action::None;

    } else if case == "single"{
        build = Action::Point(
            opps[0].0.clone(),
            opps[0].1.clone(),
            opps[0].2.clone()
        );
    } else if case == "assign"{
        build = Action::Assign(Assign{
            assigner_type:opps[0].clone(),
            pointer:opps[1].clone(),
            assigner:opps[2].clone(),
            value:opps[3].clone(),
        });
    } else if case == "reassign"{
        build = Action::Reassign(Reassign{
            pointer:opps[0].clone(),
            assigner:opps[1].clone(),
            value:opps[2].clone(),
        });
    } else {
        build = Action::None;
    }

    return (build,start_at,end_at);

}

fn opp_type(opp:&Opp)->&'static str{

    let state:&'static str;

    match opp{
        Opp::Let=>{state = "let";},
        Opp::Var=>{state = "var";},
        Opp::Const=>{state = "const";},
        Opp::Pointer(_)=>{state = "pointer";},
        Opp::FUNCTION(_)=>{state = "function";},
        Opp::CLASS(_)=>{state = "class";},
        Opp::Statement(_)=>{state = "statement";},
        Opp::SECTION(_)=>{state = "section";},
        Opp::String(_)=>{state = "string";},
        Opp::StringBuild(_)=>{state = "string_build";},
        Opp::Num(_)=>{state = "num";},
        Opp::Array(_)=>{state = "array";},
        Opp::Object(_)=>{state = "object";},
        Opp::Bool(_)=>{state = "bool";},
        Opp::PATH(_)=>{state = "path";},
        Opp::STATE(_)=>{state = "state";},
        Opp::VERIFY(_,_,_)=>{state = "verify";},
        Opp::NOT(_)=>{state = "not";},
        Opp::TERNARY(_,_,_)=>{state = "ternary";},
        Opp::FUNCTION_EXECUTER(_,_,_)=>{state = "function_executer";},
        Opp::EndStatement=>{state = "end";},
        Opp::Equal=>{state = "equal";},
        Opp::PlusEq=>{state = "pluseq";},
        Opp::MinusEq=>{state = "minuseq";},
        Opp::DivideEq=>{state = "divideeq";},
        Opp::MultiplyEq=>{state = "multiplyeq";},
        Opp::Exponentiation=>{state = "exponentiation";},
        Opp::Increment=>{state = "increment";},
        Opp::Decrement=>{state = "decrement";},
        Opp::Return=>{state = "return";}
        _=>{state = "none";}
    }

    return state;

}