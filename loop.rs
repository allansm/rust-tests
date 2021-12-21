fn main(){
    let mut n = 0;
    
    while n < 10{
        println!("{}",n);
        n+=1;
    }

    loop{
        if n == 0{
            break;
        }else{
            println!("{}",n);
            n-=1;
        }
    }
    
    println!("");

    for n in 0..3{
        println!("{}",n);
    }
}
