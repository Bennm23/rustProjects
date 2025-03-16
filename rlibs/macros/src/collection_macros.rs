
#[macro_export]
macro_rules! print_iter {
    ($vec:expr) => {

        for item in $vec {
            print!("{}, ", item);
        }
        println!();
    };
}

#[macro_export]
macro_rules! set {
    ($($val:expr),* $(,)?) => {{
        use std::collections::HashSet;

        let mut s = HashSet::new();
        $(s.insert($val);)*
        s
    }};
}