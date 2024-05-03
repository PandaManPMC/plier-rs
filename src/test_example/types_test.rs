use plier_rs::types;use std::env;
#[cfg(test)]
mod types_test {
    use super::*;

    #[test]
    fn type_of_string() {
        let p = env::current_dir();
        let t = types::type_of_string(&p);
        println!("{:?}", t);
    }

}