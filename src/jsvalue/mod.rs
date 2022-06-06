use std::collections::HashMap;

trait JSTrait{
    fn toString(&self)->String;
}

#[derive(Clone,Debug)]
pub enum JsValue{
    String(String),
    Int(u64),
    Float(f64),
    Object(Object),
    Array(Array),
    Scope(u32),
    Bool(bool)
}

#[derive(Clone,Debug)]
pub struct Array{
    map:Vec<JsValue>
}

#[derive(Clone,Debug)]
pub struct Object{
    map:HashMap<String,JsValue>
}