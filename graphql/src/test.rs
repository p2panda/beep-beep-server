pub fn test() {
    let schema = Schema::parse("type User { id: ID! }").unwrap();

    let mut registry: juniper::Registry = juniper::Registry::new(FnvHashMap::default());
    let meta = Input::meta(&(), &mut registry);
}
