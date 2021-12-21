fn hello(){
    let tmp = "hello";
    let tmp2 = "world";
    
    println!("{}",tmp.to_owned()+tmp2);
}

fn main(){
    hello()
}
