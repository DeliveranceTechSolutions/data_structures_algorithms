use core::fmt;
use std::{fmt::Debug, default};


#[derive(Clone)]
pub struct Core<T> {
    queue: Vec<T>,
    length: u32,
}



impl<T> fmt::Debug for Core<T> where T:Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        void::<T>();
        f.debug_struct("Core").field("queue", &self.queue).field("length", &self.length).finish()
    }
}

pub trait QueueActions<T> {
    fn first(&self) -> T;
    fn last(&self) -> T;
    fn insert(&mut self, value: T, index: u32);
    fn remove(&mut self, index: u32) -> T;
    fn push(&mut self, value: T);
    fn pop(&mut self, value: T) -> T;
    fn empty(&self) -> bool;
}

impl<T: Default> QueueActions<T> for Core<T> where T:Copy {
    fn first(&self) -> T {
        self.queue[0 as usize]
    }

    fn last(&self) -> T {
        self.queue[self.length as usize]
    }

    fn insert(&mut self, value: T, index: u32) {
        let length = self.length;
        let mut new: Vec<T> = Vec::new();
        for (i, d) in self.queue.iter().copied().enumerate() {
            if i == index as usize {
                new.push(value);
                continue
            }

            new.push(d);
        }

        self.queue = new;
    }

    fn remove(&mut self, index: u32) -> T {
        let mut new: Vec<T> = Vec::new();
        let mut removed: T = T::default();
        for (i, d) in self.queue.iter().copied().enumerate() {
            if i == index as usize {
                removed = d;
                continue
            }

            new.push(d);
        }

        self.queue = new;
        return removed
    }

    fn push(&mut self, value: T) {  
        self.queue.push(value)
        // let mut new_queue = vec![T::default(); self.length as usize + 1];
        // for (i, d) in self.queue.iter().copied().enumerate() {
        //     new_queue[i] = d
        // }

        // new_queue[self.length as usize] = value;
        // self.queue = new_queue.to_vec()
    }

    fn pop(&mut self, value: T) -> T {
        let popped = self.queue[self.length as usize];
        self.queue = self.queue[0..self.length as usize - 1].to_vec();
        return popped
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

pub trait QueueReader<T> {
    fn view(&self);
}

impl<T: Default> QueueReader<T> for Core<T> where T:Debug {
    fn view(&self) {
        println!("Starting Queue View");
        let reader = &self.queue;
        for data in reader.into_iter() {
            println!("element: {:?}", data)
        }
        println!("Ending Queue View");
    }
}

pub(crate) fn new<T>() -> Core<T> {
    return Core { 
        queue: Vec::new() as Vec<T>, 
        length: 0
    }
}

fn void<T>() {}