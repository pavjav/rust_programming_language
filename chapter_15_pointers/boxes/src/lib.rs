pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where
    T: Messenger
{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl <'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(
        messenger: &'a T,
        max: usize
    ) -> LimitTracker<'a,T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value (&mut self, value:usize) {
        self.value = value;
        let pct_of_max = self.value as f64 / self.max as f64;
        if pct_of_max >= 1.0 {
            self.messenger.send("Error. Over quota.")
        }
        else if pct_of_max >= 0.9 {
            self.messenger.send("You're at 90% usage.")
        }
        else if pct_of_max >= 0.75 {
            self.messenger.send("You're at 75% usage.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // We wrap our sent messages into a RefCell
    // Because we need to implement Messenger for Mock
    // And because send() takes its immutable self as an argument
    // We cannot make Mock mutable just for this test
    // So instead we use the borrow_mut() method of the RefCell
    // And call push on the Mutable reference to the vector
    // This way we do not have to rewrite the library
    struct Mock {
        sent_messages: RefCell<Vec<String>>
    }

    impl Mock {
        fn new() -> Mock {
            Mock{ sent_messages: RefCell::new(vec![]) }
        }
    }


    impl Messenger for Mock {
        fn send(&self, message: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(message));
        }
    }


}


// In summary, there are a few scenarios where RefCell is really necessary, but working around limitations
// with mock objects tends to be a common use for it.
// One thing to be aware of is, we cannot have multiple borrow_mut() instances for the same object.
// We will get a runtime error

