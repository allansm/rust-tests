fn x() -> i32{
    return 2;
}

fn y() -> i32{
    return 3;
}

fn z() -> [i32;3]{
    return [3,1,2];
}

fn val(arr:[i32;3],i:usize) -> i32{
    return arr[i];
}

fn main(){ 
    let mut res = x();
    res += y();
    res-= val(z(),1);

    print!("{}",res);
}
