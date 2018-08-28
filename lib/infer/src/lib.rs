#![feature(infer_outlives_requirements)]

struct Foo<'a, T> {
    bar: &'a T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
