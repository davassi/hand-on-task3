
use std::marker::{self, PhantomData};

/// F defines the function, T the type
struct FilterCondition<F, T> 
where
    F: Fn(T) -> bool,
    T: Copy
{
    condition : F,
    _m : marker::PhantomData<T>,
}

impl<F, T: Copy> FilterCondition<F, T> 
where
    F: Fn(T) -> bool,
{
    fn new(condition: F) -> Self {
        FilterCondition { condition , _m : PhantomData }
    }

    fn is_match(&self, c: &T) -> bool {
        (self.condition)(*c)
    }   
}

fn custom_filter<F : Fn(T,) -> bool, T: Copy>(v : &Vec<T>, predicate: &FilterCondition<F,T>) -> Vec<T> {
    v.iter().filter(|i| predicate.is_match(i)).cloned().collect()
}

fn main() -> () {

    let predicate = FilterCondition::new(|x: i32| { x<0 });
    let v : Vec<i32> = custom_filter(&(-10..10).collect(), &predicate);
    
    println!("A i32 filtered vec: {:?}",v);

    let chars : Vec<char> = vec!['a','A','b','B','c','C','d','D']; 
    let predicate = FilterCondition::new(|x: char| { x.is_uppercase() });
    let v : Vec<char> = custom_filter(&chars, &predicate);
    
    println!("A char filtered vec: {:?}",v);
}
