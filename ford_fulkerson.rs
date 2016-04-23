use std::io;

fn main() {
    println!("Hello, world!");
    
    	let mut numbers1 = String::new();
        io::stdin().read_line(&mut numbers1).ok().expect("read error");
        
         let numbers1: Vec<i32> = numbers1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        
        let mut nodes = 0;
        let mut edges = 0;
        
        let mut i = 0;
        
    for num in numbers1 {
        if i == 0 {
            nodes  = num;
        }
        
        if i ==1 {
            edges = num;
        }
        
        i = i+1;     
        
    }
    
    let mut numbers3 = String::new();
    io::stdin().read_line(&mut numbers3).ok().expect("read error");
    
        
   //  for num in edges {
        
        	
            
            let numbers3: Vec<i32> = numbers3
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
            
            let mut j =0 ;
            let mut start = 0;
            let mut end = 0;
            let mut value = 0;
            
            for num2 in numbers3 {
                if j == 0 {
                    start = num2;
                }
                if j == 1 {
                    end = num2;
                }if j == 2 {
                    value = num2;
                }
                j = j + 1;
           
           
            }
            
        println!("{} {} {}",start,end,value);
        
   //  }
    
        
        
        println!("{}",nodes);
        println!("{}",edges);
    
}
