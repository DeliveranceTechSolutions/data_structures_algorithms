#[derive(Debug, Clone)]
pub struct Core<T> {
    stack: Vec<T>
}

pub trait StackActions<T> {
    fn peek(&self) -> T;
    fn pop(&mut self) -> T;
    fn push(&mut self, value: T);
    fn empty(&self) -> bool;
}

impl<T: Default> StackActions<T> for Core<T> where T:Copy {
    fn peek(&self) -> T {
        match self.stack.last() {
            Some(t) => *t,
            None => T::default(),
        }
    }

    fn pop(&mut self) -> T {
        match self.stack.pop() {
            Some(t) => t,
            None => T::default(), 
        }
    }

    fn push(&mut self, value: T) {
        self.stack.push(value)
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }

}

pub(crate) fn new<T>() -> Core<T> { 
    Core { stack: vec![] }
}