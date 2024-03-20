use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
pub struct VectorClock {
    data: HashMap<String, u64>
}

impl VectorClock {
    // create an empty VectorClock. The "inc" method must
    // be called to ensure that the VectorClock is incremented
    // for the specific id. 
    pub fn new() -> VectorClock {
        VectorClock {
            data: HashMap::new()
        }
    }

    // Increments the id position of the VectorClock.
    pub fn inc(mut self, id: &str) -> VectorClock {
        self.inc_nr(id);
        self
    }

    pub fn inc_nr(&mut self, id: &str) {
        self.data
            .entry(id.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    pub fn preceeds(&self, other: &VectorClock) -> bool {
        let mut strict_lt = false;
        for (k, v) in self.data.iter() {
            let ov = match other.data.get(k) {
                Some(v) => v,
                None => &0,
            };
            if v < ov {
                strict_lt = true;
            } else if v > ov {
                return false;
            }
        }
        strict_lt
    }

    pub fn concurrent(&self, other: &VectorClock) -> bool {
        !self.preceeds(other) && !other.preceeds(self)
    }

    // merge two vector clocks, taking the maximum of the two. 
    pub fn merge(&self, other: &VectorClock) -> VectorClock {
        let mut data: HashMap<String, u64> = other.data.clone();
        self.data
            .iter()
            .for_each(|(k, v)| {
                data.entry(k.to_string())
                    .and_modify(|c| {
                        *c = std::cmp::max(*c, *v)
                    });
            });
        VectorClock { data: data }
    }

    // length of current vector clock. 
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// == operator for strict equality evaluation.
// equality iff the intersection of keysets is the keyset
// for both vectors + the values for the keys are equal.
impl PartialEq for VectorClock {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.data
            .iter()
            .all(|(k, v)| {
                match other.data.get(k) {
                    Some(val) => v == val,
                    None => false,
                }
            })
    }
}

// extending PartialEq implementation
impl Eq for VectorClock {}

// TODO: complete Display impl
// for debug purposes + to_string() function
// impl fmt::Display for VectorClock {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "()")
//     }
// }

// implement clone
impl Clone for VectorClock {
    fn clone(&self) -> Self {
        VectorClock { data: self.data.clone() }
    }
}

// TODO: complete tests

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
