// Cell<T: Clone> is not safe to .get()
// so this is a little shorthand for cell.borrow().clone()
//
// TODO: use UnsafeCell<> for release
//       should be safe if it didnt panic in dev

use std::cell::RefCell;

#[derive(Clone)]
pub struct CloneCell<T: Clone>(RefCell<T>);

impl<T: Clone> CloneCell<T> {
    pub fn get(&self) -> T {
        self.0.borrow().clone()
    }

    pub fn set(&self, value: T) {
        *self.0.borrow_mut() = value;
    }
}
