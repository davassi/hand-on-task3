

struct FilterCondition {
    field : u32,
}

impl FilterCondition {

    fn is_match(&self, c: &u32) -> bool {
        self.field > *c
    }   
}

fn custom_filter(v : &Vec<u32>, predicate: &FilterCondition) -> Vec<u32> {
    
    v.iter().filter(|i| predicate.is_match(i)).cloned().collect()
}


fn main() -> () {

    let condition  = |x| { x<10 };

    condition(10);
    
    let v : Vec<u32> = (0..20).collect();

    let v : Vec<u32> = custom_filter(&v, &FilterCondition { field : 10 } );
    
    println!("The filtered vec: {:?}",v);


}
