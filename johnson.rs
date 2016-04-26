//rustc  new.rs -A warnings
use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Johnson {
    x: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct que_node {
    cost: u32,
    vertex: u32,
}


impl Ord for que_node {
    fn cmp(&self, other: &que_node) -> Ordering {
        // Notice that the we flip the ordering here
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for que_node {
    fn partial_cmp(&self, other: &que_node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



const  INF: u32 = 999999999;

fn Johnson(start:u32, nodes: u32, edges: u32, adj_matrix: Vec<Vec<u32>> )   {
        
    let mut distance = Vec::new();  
    
    for i in 0..nodes {
        distance.push(INF);
    }

    let mut heap = BinaryHeap::new();
    

    distance[start as usize] = 0;
    
    heap.push(que_node { cost: 0, vertex: start });
    
    
     while let Some(que_node { cost , vertex })  = heap.pop() {
        
        if cost > distance[vertex as usize] { 
            continue; 
        }

        for i in 0..nodes {
        
            if(i != vertex){
        
                let mut weigh =  adj_matrix[vertex as usize][i as usize];    
                 let next = que_node { cost: cost + weigh, vertex: i };
                 
                 if next.cost < distance[next.vertex as usize] {
                     heap.push(next);
                     distance[next.vertex as usize] = next.cost;
                 }
                
            }
        }
    /*
        for new_edge in adj_matrix[vertex] {
            let next = que_node { cost: cost + new_edge.cost, vertex: new_edge.vertex };

            // If so, add it to the frontier and continue
            if next.cost < distance[next.vertex] {
                heap.push(next);
                // Relaxation, we have now found a better way
                distance[next.vertex] = next.cost;
            }
        }
        
    */

     }
    
}


fn main() {

    let mut read = String::new();
    io::stdin().read_line(&mut read).ok().expect("read error");
        
      let read1: Vec<usize> = read
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    

    let mut nodes: usize = read1[0];
    let mut edges: usize = read1[1];
    let mut start: usize = read1[2];
    
     println!("Debugging at level 0  : {} {} ",nodes,edges );
  
    let mut adj_list: Vec<Vec<u32>> = Vec::new();    
    let mut dim = nodes;
   // let mut adj_matrix: Vec<Vec<i32>> = (0..dim).map(|_| Vec::with_capacity(dim)).collect();
    
    
    // initilizing vector of vectors, adj matrix with value 0.
    
    for i in 0..nodes {
        
        let mut temp = Vec::new();
        for j in 0..nodes {
            temp.push(0);
        }
        adj_list.push(temp);
        
    }
    
    
    
    
    println!(" Debugging at level 1  : {}",adj_list[0][0]);
    println!(" Debugging at level 1  : {}",adj_list[nodes-1][nodes-1]);


    // Reading the edges
      
    for i in 0..edges {
        let mut read = String::new();
        
        io::stdin().read_line(&mut read).ok().expect("read error");
        
         let read1: Vec<u32> = read
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        
        let mut a = read1[0];
        let mut b = read1[1];
        let mut c = read1[2];
        adj_list[a as usize][b as usize] = c;
        
    }
    
    println!("Debugging at level 2  :{} ", adj_list[0][0]);
    
    
    
     
}    
    
        
