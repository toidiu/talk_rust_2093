
struct Foo<'a, T>
where
    T: 'a,
{
    bar: &'a T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
