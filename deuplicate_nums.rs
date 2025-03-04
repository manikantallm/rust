use std::collections::HashSet;
fn main(){
    let _num = vec![1,2,3,4,5,6,3,2,1,10,9];
    let (unique_set,duplicate_set)=duplicate_nums(&_num);
    println!("The unique set of numbers are {:?} and the Duplicate set is {:?}",unique_set,duplicate_set);
}

fn duplicate_nums(_nums_array:&Vec<u32>)->(HashSet<u32>,HashSet<u32>){
    let mut set :HashSet<u32> = HashSet::new();
    let mut dup_set :HashSet<u32>= HashSet::new();
    println!("The numbers are {:?}",_nums_array);
    for i in _nums_array{
        if set.contains(i){
            dup_set.insert(*i);
        }else{
          set.insert(*i);  
        }
    }
    (set,dup_set)
}