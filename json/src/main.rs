use json;

fn main(){
    let mut tmp = json::parse(r#"{"msg":"helloworld","x":2}"#).unwrap();

    println!("{}", tmp["msg"]);
    println!("{}", tmp["x"]);
    
    tmp["msg"] = "HelloWorld!!!".into();
    
    tmp["y"] = 5.into();
    tmp["z"] = 8.into();

    println!("{}", json::stringify(tmp));
}
