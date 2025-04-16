use std::cell::{RefCell, Cell};
use std::ops::Drop;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.track_worker();
        let thread = Thread::new_thread(pid, c, self);
        (pid, thread)
    }
    pub fn track_worker(&self) -> usize {
        let mut states = self.states.borrow_mut();
        states.push(false);
        states.len() - 1

    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow().get(id).map_or(false, |&dropped| dropped)
    }
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if let Some(state) = states.get_mut(id) {
            if *state {
                panic!("{} is already dropped", id);
            }
            *state = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Thread { pid: p, cmd: c, parent: t }
    }

    pub fn skill(self) {}
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}