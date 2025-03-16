
pub mod fileio;
pub mod socketio;

use std::{thread, time::Duration};

use proc_macros::benchmark;

#[benchmark]
pub fn t1() {
    for _ in 0..5 {
        thread::sleep(Duration::from_micros(200));
    }
}


#[cfg(test)]
mod tests {

    use macros::{print_iter, set};

    use proc_macros::comp;

    use super::*;


    #[test]
    fn attr() {
        t1();
    }

    #[test]
    fn it_works() {
    
        let xs = vec![2, 4, 5, 8, 12];

        let new = comp![x for x in [1, 2, 3]];
        print_iter!(new);

        let new = comp!(x * 2 for x in xs if x > 4 if x < 10);
        print_iter!(new);

        let set = set!(1, 2, 3, 4, 5);

        let new = comp!(x for x in set);
        print_iter!(new);

        let new = comp!(x for x in (0 .. 12).rev());
        print_iter!(new);
    }
}
