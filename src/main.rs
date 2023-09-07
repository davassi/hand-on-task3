

/// F defines the function, T the type
struct FilterCondition<F, T> 
where
    F: Fn(T) -> bool,
{
    condition : F,
}

impl<F, T: Copy> FilterCondition<F, T> 
where
    F: Fn(T) -> bool,
{

    fn new(condition: fn(T) -> bool) -> Self {
        FilterCondition { condition }
    }

    fn is_match(&self, c: &T) -> bool {
        (self.condition)(*c)
    }   
}

fn custom_filter<F : Fn(T,) -> bool, T: Copy>(v : &Vec<T>, predicate: &FilterCondition<F,T>) -> Vec<T> {
    v.iter().filter(|i| predicate.is_match(i)).cloned().collect()
}

fn main() -> () {
    
    let v : Vec<u32> = (0..20).collect();

    let predicate = FilterCondition::new(|x: u32| { x<10 });
    let v : Vec<u32> = custom_filter(&v, &predicate);
    
    println!("The filtered vec: {:?}",v);
}
