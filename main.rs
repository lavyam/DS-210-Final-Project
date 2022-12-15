
use std::collections::HashSet;
use std::collections::BinaryHeap;


mod read;
#[warn(unused_assignments)]

fn main() {
    let edges = read::read_file("facebook_combined.txt");
    let mut users:Vec<&usize>= Vec::new();
    let mut _leave:Vec<usize> =Vec::new();

    for (u,_v) in &edges{
        if users.contains(&u){
            _leave.push(*u);
        }else{
            users.push(&*u);
        }
    }

    let max_value =  *users.iter().max().unwrap();
    let mut friends:Vec<Vec<usize>>=Vec::new();

    for i in 0..=*max_value{
        let mut z=Vec::new();
        for (u,v) in &edges{
            if (*u as usize)==i{
                z.push(*v);
            }
        } 
        friends.push(z);
        
    }
    let x:usize=*users[users.len()-1];
    let y:usize=friends.len()-1;

    assert_eq!(y,x,"test 1 does not work");

    let mut max_mutual=Vec::new();
    let mut index=Vec::new();
    
    for i in 0..(friends.len()-1){
        let mut v=Vec::new();
        for j in (i+1)..(friends.len()){
            let a= friends[i].iter().collect::<HashSet<_>>();
            let b=friends[j].iter().collect::<HashSet<_>>();
            let c = a.intersection(&b).collect::<Vec<_>>();
            v.push(c.len());
            
            
        }
        
        let max_value =  *v.iter().max().unwrap();
        max_mutual.push(max_value);
        let ind = v.iter().position(|&r| r == max_value).unwrap();
        index.push(ind+i+1);
        
        
    }

    assert_eq!(max_mutual.len(),friends.len()-1,"test 2 does not work");

    let mut pq=BinaryHeap::new(); 

    for i in 0..max_mutual.len(){
        pq.push(max_mutual[i]);
    }   

    assert_eq!(pq.len(),max_mutual.len(),"test 3 does not work"); 

    let a1=pq.pop().unwrap();
    let a2=pq.pop().unwrap();
    let a3=pq.pop().unwrap();
    let a4=pq.pop().unwrap();
    let a5=pq.pop().unwrap();

    let top_five=vec![a1,a2,a3,a4,a5];

    let mut not_repeat=Vec::new();
    println!("TOP FIVE MAXIMUM MUTUAL FRIENDS");
    for i in top_five{
        let mut l=max_mutual.iter().position(|&r| r == i).unwrap();
        let mut z=index[l];
        if not_repeat.contains(&l){
            let next= &max_mutual[l+1..];
            let nl=next.iter().position(|&r| r == i).unwrap();
            
            l=nl+l+1;
            z=index[l];
            
            
        }
        let n=max_mutual[l];

        assert_eq!(i,n,"test 4 does not run");
        not_repeat.push(l);
        
        println!("User {} has {} mutual friends with User {}",l,i,z )
    }
}