use std::sync::Mutex;

use anymap2::SendSyncAnyMap;
use lazy_static::lazy_static;

pub fn dynamic_registry<T>() -> &'static inventory::Registry<T> {
    lazy_static! {
        static ref REGISTRIES: Mutex<SendSyncAnyMap> = Mutex::new(SendSyncAnyMap::new());
    }

    *REGISTRIES
        .lock()
        .unwrap()
        .entry::<&'static inventory::Registry<T>>()
        .or_insert_with(|| Box::leak(Box::new(inventory::Registry::new())))
}
