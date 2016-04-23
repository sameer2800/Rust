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
        
        
    
        
        
        println!("{}",nodes);
        println!("{}",edges);
    
}
