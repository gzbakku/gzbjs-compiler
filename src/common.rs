
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Error{
    e:String
}

impl Error{
    pub fn string(e:String)->Error{
        Error{
            e:e
        }
    }
    pub fn str(e:&'static str)->Error{
        Error{
            e:e.to_string()
        }
    }
}