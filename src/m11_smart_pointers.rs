//  Bocked in Coding and Lets Get Rusty

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[test]
    #[allow(dead_code, unused_variables)]
    
    fn tests_box_smart_pointer() {

        let x: Box<i32> = Box::new(50);

        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>
        }

        let nodes: Box<Node> = Box::new(Node {id: 0, next: Some(Box::new(Node {id: 1, next: Some(Box::new(
            Node{id: 2, next: None}))}))});

            dbg!(nodes);

    }

    #[test]
    #[allow(dead_code, unused_variables)]

    fn tests_reference_counter() {

        let mut x: Rc<RefCell<i32>> = Rc::new(RefCell::new(50));
        let y: Rc<RefCell<i32>> = Rc::clone( &x);

        x.replace(70);  

        dbg!(x.borrow());
        dbg!(y.borrow());

        #[derive(Debug)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>
        }

        #[derive(Debug)]
        struct Furniture {
            id: String,
            description: String,
            owner: Weak<House>
        }

        let house1: Rc<House> = Rc::new(House {
            address_number: 123,
            street: "Brockford".to_string(),
            furniture: RefCell::new(vec!()) 
        });

        let table:Rc<Furniture>  =  Rc::new(Furniture{ 
            id: "table".to_string(),
            description: "oak table".to_string(),
            owner: Rc::downgrade(&house1)
        });

        let desk:Rc<Furniture>  =  Rc::new(Furniture{ 
            id: "desk".to_string(),
            description: "oak table".to_string(),
            owner: Rc::downgrade(&house1)
        });


        house1.furniture.borrow_mut().push(Rc::clone(&table));
        house1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house1); 
        dbg!(table);

    }
}