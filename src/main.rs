mod window;
mod document;
mod math;
mod io;
mod compiler;
mod parser;
mod common;
mod jsvalue;

pub use compiler::{Compiler,CompilerToDo};
pub use parser::scope::Scope;
pub use common::Error;
pub use jsvalue::{JsValue,Object,Array};
pub use parser::opp::{Opp,Function,State,Class};
pub use parser::action::{Action};

#[tokio::main]
async fn main() {

    // let hold = f64::MAX + 1561561456.36565;

    // println!("max : {:?}",hold);

    start(TEST.to_string()).await;

}

async fn start(code:String){

    let mut hold = Compiler::init(code);

    hold.compile();

}

const TEST:&'static str = r#"

function two(king,ping,ding){

    let k = king;

}

"#;

// !one ? akku === "king" ? king === "akku" ? : true : false : false : false;


/*
// const full:&'static str = r#"

fun(king+1,2+2,"akku is king")["name"]["age"][65+32]

one = 1 > 2 ? 3 + 5 : 6 * 5

// const hold0 = true;
// const hold1 = "akku";
// const hold2 = \'akku\';
// const hold3 = `akku`;
// const hold4 = `${akku}`;
// const hold5 = 100;

// function main(){

//     const test = true;

// }

// "#;
*/
