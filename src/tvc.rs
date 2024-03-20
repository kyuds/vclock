use crate::vc;
use std::fmt;
use std::sync::RwLock;

pub struct VectorClock {
    clock: RwLock<vc::VectorClock>
}

impl VectorClock {
    pub fn new() -> VectorClock {
        VectorClock{ clock: RwLock::new(vc::VectorClock::new()) }
    }

    pub fn inc(mut self, id: &str) -> VectorClock {
        self.inc_nr(id);
        self
    }

    pub fn inc_nr(&mut self, id: &str) {
        self.clock.write().unwrap()
            .inc_nr(id);
    }

    pub fn preceeds(&self, other: &VectorClock) -> bool {
        let vc1 = self.clock.read().unwrap();
        let vc2 = other.clock.read().unwrap();
        vc1.preceeds(&vc2)
    }

    pub fn concurrent(&self, other: &VectorClock) -> bool {
        let vc1 = self.clock.read().unwrap();
        let vc2 = other.clock.read().unwrap();
        vc1.concurrent(&vc2)
    }

    pub fn merge(&self, other: &VectorClock) -> VectorClock {
        let vc1 = self.clock.read().unwrap();
        let vc2 = other.clock.read().unwrap();
        let mvc = vc1.merge(&vc2);
        VectorClock { clock: RwLock::new(mvc) }
    }

    pub fn len(&self) -> usize {
        let vc = self.clock.read().unwrap();
        vc.len()
    }
}

impl PartialEq for VectorClock {
    fn eq(&self, other: &Self) -> bool {
        let vc1 = self.clock.read().unwrap();
        let vc2 = other.clock.read().unwrap();
        *vc1 == *vc2
    }
}

impl Eq for VectorClock {}

impl Clone for VectorClock {
    fn clone(&self) -> Self {
        let vc = self.clock.read().unwrap();
        VectorClock{ clock: RwLock::new(vc.clone()) }
    }
}

// tests (copied from vc.rs)
#[test]
fn test_equals_impl() {
    assert!(VectorClock::new() == VectorClock::new());
    assert!(VectorClock::new().inc("A") == VectorClock::new().inc("A"));
    assert!(VectorClock::new().inc("A") != VectorClock::new().inc("B"));
    assert!(VectorClock::new().inc("A") != VectorClock::new().inc("A").inc("A"));
}

#[test]
fn test_clone_impl() {
    let v1 = VectorClock::new().inc("A");
    let v2 = v1.clone();
    assert!(v1 == v2);
    assert!(v1 != v2.inc("B"));
}
