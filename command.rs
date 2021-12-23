use std::process::Command;

fn main(){
    let out = Command::new("cmd")
        .args(["/C", "echo helloworld"])
        .output()
        .expect("failed to execute process");
    
    let hello = String::from_utf8_lossy(&out.stdout);
    
    print!("{}",hello);
}
