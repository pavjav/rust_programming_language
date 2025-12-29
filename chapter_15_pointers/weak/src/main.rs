use std::cell::RefCell;
use std::rc::{Rc, Weak};


/// We create this Node struct to model trees.
/// Children is a RefCell to a vector of Ref Counts to Nodes.
/// We wrap Node in Rc for recursion. So Vec<Rc<Node>> is a vector of references
/// We then wrap this in RefCell for interior mutability of the vector (add/remove refs)
/// parent is a refcell to a Weak<Node>
/// The difference between Weak and Rc is that a weak reference will only increment a weak_count
/// weak_count does not need to be 0 for the Rc to be cleaned up
/// This prevents a reference cycle
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}


fn main() {
    let leaf = Rc::new(
        Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        }
    );
    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade() // Upgrades a Weak to an Rc
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    //leaf is an Rc with 1 strong count and 0 weak count

    // We will now introduce an inner scope to add a parent to this leaf called branch
    {
        let branch = Rc::new(
            Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)])
            }
        );

        println!(
            "branch parent = {:?}",
            branch.parent.borrow().upgrade()
        );

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        // branch has strong 1, weak 0

        // Mutable borrow of leaf's parent
        // Assign it to a Weak of the branch Rc with downgrade
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


        println!(
            "leaf parent = {:?}",
            leaf.parent.borrow().upgrade()
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        // strong count is now 2, but weak count is still 0
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        // branch has strong 1, weak 1
    }
    
 
    // Because our parents are weak references, we can make changes to something in the outer scope
    // Without worrying about memory leaks. Branch can be safely dropped once we leave the inner scope
    // And the parent of leaf will no longer point to it
    println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    // leaf is now strong 1, weak 0
    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );
    // Now leaf parent = None




}
