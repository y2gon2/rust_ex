// fn main() {
    
// }

// struct NativeRc<T> {
//     reference_count: usize,
//     inner_value: T,
// }

// impl Clone for NativeRc<T> {
//    fn clone(&self) -> Self {       //  read-only reference to self
//        self.reference_count += 1;  // so the reference count can’t be updated!
//    }
// }
// -> 일반적인 clone() 으로는 reference count 를 올려서 사용한다는 정의를 구현 할수 없다.


// How? -> interior mutibility

use std::cell::Cell;

fn foo(cell: &Cell<u32>) {
    let value = cell.get();
    cell.set(value * 2);
}

fn main() {
    let cell = Cell::new(0);
    let value = cell.get();
    let new_value = cell.get() + 1;
    foo(&cell);
    cell.set(new_value); // oops, we clobbered the work done by foo
}