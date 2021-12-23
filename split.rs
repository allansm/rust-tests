fn main(){
    let test = "a;b".split(";");
      
    for n in test{
        print!("{} ",n);
    }

    let test2:Vec<&str> = "a;b;c".split(";").collect();

    println!("");
    print!("{} {} {}",test2[0],test2[1],test2[2]);
}
