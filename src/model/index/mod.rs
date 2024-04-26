use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Student{
    pub name:String,
    pub age:i32,
    pub address:String,
}