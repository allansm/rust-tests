fn main(){
    let mut array:[i32;3] = [0;3]; 

    array[0] = 5;
    array[1] = 2;
    array[2] = 7;

    println!("{}",array[0]+array[1]);
    
    let x = [2,0,1];

    println!("{} {} {}\n",x[0],x[1],x[2]);
    
    for n in array.iter(){
        println!("{}",n);
    }
}
