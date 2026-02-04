mod shadowing;
mod types;
mod functions;
mod flow;




fn main(){
    types::run();
    shadowing::run();
    let y=functions::five();
    println!("five : {}",y);
    flow::run();
    


}

