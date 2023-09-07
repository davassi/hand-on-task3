

struct FilterCondition<T> {
    condition : fn(T) -> bool,
}

impl<T: Copy> FilterCondition<T> {

    fn new(condition: fn(T) -> bool) -> Self {
        FilterCondition { condition }
    }

    fn is_match(&self, c: &T) -> bool {
        (self.condition)(*c)
    }   
}

fn custom_filter<T: Copy>(v : &Vec<T>, predicate: &FilterCondition<T>) -> Vec<T> {
    v.iter().filter(|i| predicate.is_match(i)).cloned().collect()
}


fn main() -> () {
    
    let v : Vec<u32> = (0..20).collect();
    let v : Vec<u32> = custom_filter(&v, &FilterCondition::new(|x: u32| { x<10 }));
    
    println!("The filtered vec: {:?}",v);


}
