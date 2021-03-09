use crate::async_di::container::Container;
use std::collections::HashMap;

fn ensure_send_sync<T>(_value: T) where T: Send, T: Sync, {}

#[test]
fn test_resolver_send_sync() {
    let container = Container::new(HashMap::new(), HashMap::new());
    ensure_send_sync(&container);
    ensure_send_sync(container);
}