use coerce_derive::message;

trait Message {
    type Result;
}

#[message((u64, u64))]
struct M1 {
    pub user: String,
}

#[message(bool)]
struct M2;

#[message(M2)]
enum M3 {
    A,
    B,
}

fn assert_impl_message<T, R>()
where
    T: Message<Result = R>,
{
}

#[test]
fn it_works() {
    assert_impl_message::<M1, (u64, u64)>();
    assert_impl_message::<M2, bool>();
    assert_impl_message::<M3, M2>();
}
