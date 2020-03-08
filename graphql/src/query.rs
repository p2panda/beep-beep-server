use juniper;

use crate::context::Context;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn foo() -> &'static str {
        "bar"
    }
}
