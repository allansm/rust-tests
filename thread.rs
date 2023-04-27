use std::thread;

fn main(){ 
    thread::spawn(|| { 
        let mut n = 0;
        
        loop{
            println!("A: {}", n);
            
            n+=1;

            if n % 2 == 0{
                std::thread::sleep(std::time::Duration::from_millis(990));
            }else{
                std::thread::sleep(std::time::Duration::from_millis(1002));
            }
        }
    });

    thread::spawn(|| { 
        let mut n = 0;
        
        loop{
            println!("B: {}", n);

            n+=2;

            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    });

    loop{
        println!("");

        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
} 
