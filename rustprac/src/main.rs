fn using_function_as_parameter() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
        let result = operation(x, y);
        println!("Result of operation: {}", result);    
    }

    calculator(1, 2, add);
    calculator(1, 2, |x, y| x + y);
    calculator(1, 2, |x, y| x - y);
    calculator(1, 2, |x, y| x * y);
    calculator(1, 2, |x: f32, y: f32| x / y); 
}


fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }


    
    #[derive(Debug)]
    struct Dog;
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Woof woof".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Meow meow".to_string()
        }
    }
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    
    zoo.push(Box::new(Dog{}));
    zoo.push(Box::new(Cat{}));
    
    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound());
    }
}



fn capture_ownership_modify() {
    let nums = vec![1, 2, 3, 4, 5].into_iter();
    let name = "John".to_string();

    

    let product_through_iterator: Box<dyn FnOnce() -> i32> = Box::new(move || nums.product());
    let result: i32 = product_through_iterator();
    println!("{}", result);  // Output: 120
}



//MOdule 7 closure: 04rust-smart-pointers

//Combining Rc and RefCell
//shred mutable state
use std::rc::Rc;
use std::cell::RefCell;

fn sharing_resource_refcell_count() {
    struct FamilyMember {
        tv: Rc<RefCell<TV>>,
    }

    #[derive(Debug)]
    struct TV {
        channel: String,
    }

    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(RefCell::new(TV{channel:"BBC".to_string()}));
        FamilyMember {
            tv: tv_is_on, 
        }
    }

    let dad = member_start_watch_tv();
    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("TV channel for mom {:?}", mom.tv);

    let mut remote_control = dad.tv.borrow_mut();
    println!("TV channel {:?}", remote_control);

    remote_control.channel = "MTV".to_string();
    println!("TV channel {:?}", remote_control);
    drop(remote_control);
    println!("TV channel for mom {:?}", mom.tv);
}
