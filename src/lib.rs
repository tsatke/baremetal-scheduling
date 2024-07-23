#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::marker::PhantomData;
use core::mem::swap;
use core::ops::Deref;

mod lfill;

pub use lfill::IntrusiveNode;
use crate::lfill::LockFreeIntrusiveLinkedList;

unsafe trait SchedulingStrategy<T> {
    fn next_task(&self) -> Box<T>;
    fn post_schedule(&self, task: Box<T>);
}

struct Scheduler<T, S: SchedulingStrategy<T>, F: Fn(Box<T>)> {
    strategy: S,
    current: Box<T>,
    finished_sink: F,
}

impl<T, S: SchedulingStrategy<T>, F: Fn(Box<T>)> Scheduler<T, S, F> {
    pub const fn new(strategy: S, current: Box<T>, finished_sink: F) -> Self {
        Self {
            strategy,
            current,
            finished_sink,
        }
    }

    pub fn register_task(&self, task: Box<T>) {
        todo!()
    }

    pub fn reschedule(&mut self) {
        let mut next_task = self.strategy.next_task();
        swap(&mut self.current, &mut next_task);
        self.strategy.post_schedule(next_task);

        todo!("switch")
    }
}

struct RoundRobin<T: IntrusiveNode> {
    q: LockFreeIntrusiveLinkedList<T>,
}

unsafe impl<T: IntrusiveNode> SchedulingStrategy<T> for RoundRobin<T> {
    fn next_task(&self) -> Box<T> {
        todo!()
    }

    fn post_schedule(&self, task: Box<T>) {
        todo!()
    }
}