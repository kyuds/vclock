# vclock
Rust Vector Clock

### API
Refer to `src/vc.rs`.

### Why Two Modules?
The module `vc` is for the basic vector clock and the module `tvc` is for the thread-safe wrapper with in-built read-write locks for the vector clock. While it is indeed possible to implement your own locks for vector clocks, we provided a generic one to use out of the box. Note that there is a warning on using both in the same project. It will be very confusing. The APIs for both modules are identical.

### To-Do
- [X] Implement Vector Clock
- [X] Implement RwLocks for VC
- [ ] Finish Writing Tests
- [ ] Implement Display Trait
