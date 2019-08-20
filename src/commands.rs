
#[macro_use]
use serde_derive::{Deserialize,Serialize};
use std::net::{TcpListener,TcpStream};
use std::io::{Write,Result,Error};
use serde_json;

#[derive(Debug,Serialize, Deserialize)]
pub enum Direction{
    RightToLeft,
    LeftToRight,
}

#[derive(Debug,Serialize, Deserialize,PartialEq,Hash)]
pub enum RobotId{
    Blue{number:i32},
    Yellow{number:i32},
}

#[derive(Debug,Serialize, Deserialize)]
pub struct RobotPosition{
    x:f32,
    y:f32,
    theta:f32,
}

#[derive(Debug,Serialize, Deserialize)]
pub enum Target{
    At(RobotPosition),
    To(RobotId),
    Ball,
}

#[derive(Debug,Serialize, Deserialize)]
pub enum Operation{
    None,
    Move(Target),
    Block(Target),
    Pass(Target),
    Receive(Target),
    Kick(Target),
    //FreeKick,
    Dribble(Target),
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Command{
    direction:Direction,
    my_id:RobotId,    
    my_position:RobotPosition,
    robots:Vec<(RobotId,RobotPosition)>,
    operation:Operation,
}

impl Command {

    fn Send(&self,url:&str){
        //TODO ちゃんと真面目なコードを書くこと
        let json = serde_json::to_vec(&self).unwrap();
        let mut stream = TcpStream::connect(&url).unwrap();
        stream.write(&json).unwrap();
        stream.flush().unwrap();
    }

    fn Receive(url:&str)->Option<Command>{
        //TODO ちゃんと真面目なコードを書くこと
        None
    }



}