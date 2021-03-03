mod storage_id;

pub use storage_id::StorageId;

use crate::atomic_refcell::AtomicRefCell;
use crate::unknown_storage::UnknownStorage;
use alloc::boxed::Box;
#[cfg(feature = "thread_local")]
use std::thread::ThreadId;

/// Abstract away `T` from `AtomicRefCell<T>` to be able to store
/// different types in a `HashMap<TypeId, Storage>`.
/// and box the `AtomicRefCell` so it doesn't move when the `HashMap` reallocates
pub(crate) struct Storage(pub(crate) *mut AtomicRefCell<dyn UnknownStorage>);

#[cfg(not(feature = "thread_local"))]
unsafe impl Send for Storage {}

unsafe impl Sync for Storage {}

impl Drop for Storage {
    fn drop(&mut self) {
        // SAFE the pointer came from a `Box` of the same type
        unsafe {
            Box::from_raw(self.0);
        }
    }
}

impl Storage {
    #[inline]
    pub(crate) fn new<T: UnknownStorage + Send + Sync + 'static>(value: T) -> Self {
        Storage(Box::into_raw(Box::new(AtomicRefCell::new(value))))
    }
    #[cfg(feature = "thread_local")]
    #[inline]
    pub(crate) fn new_non_send<T: UnknownStorage + Sync + 'static>(
        value: T,
        thread_id: ThreadId,
    ) -> Self {
        Storage(Box::into_raw(Box::new(AtomicRefCell::new_non_send(
            value, thread_id,
        ))))
    }
    #[cfg(feature = "thread_local")]
    #[inline]
    pub(crate) fn new_non_sync<T: UnknownStorage + Send + 'static>(value: T) -> Self {
        Storage(Box::into_raw(Box::new(AtomicRefCell::new_non_sync(value))))
    }
    #[cfg(feature = "thread_local")]
    #[inline]
    pub(crate) fn new_non_send_sync<T: UnknownStorage + 'static>(
        value: T,
        thread_id: ThreadId,
    ) -> Self {
        Storage(Box::into_raw(Box::new(AtomicRefCell::new_non_send_sync(
            value, thread_id,
        ))))
    }
}

impl core::fmt::Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Ok(storage) = unsafe { &*self.0 }.borrow() {
            f.write_str(&*storage.name())
        } else {
            f.write_str("Could not borrow storage")
        }
    }
}

#[test]
fn delete() {
    use crate::entity_id::EntityId;
    use crate::sparse_set::SparseSet;

    let storage = Storage(Box::into_raw(Box::new(AtomicRefCell::new(SparseSet::<
        &'static str,
    >::new()))));
    let mut entity_id = EntityId::zero();
    entity_id.set_index(5);
    unsafe { &mut *storage.0 }
        .get_mut()
        .as_any_mut()
        .downcast_mut::<SparseSet<&'static str>>()
        .unwrap()
        .insert(entity_id, "test5");
    entity_id.set_index(10);
    unsafe { &mut *storage.0 }
        .get_mut()
        .as_any_mut()
        .downcast_mut::<SparseSet<&'static str>>()
        .unwrap()
        .insert(entity_id, "test10");
    entity_id.set_index(1);
    unsafe { &mut *storage.0 }
        .get_mut()
        .as_any_mut()
        .downcast_mut::<SparseSet<&'static str>>()
        .unwrap()
        .insert(entity_id, "test1");
    entity_id.set_index(5);
    unsafe { &mut *storage.0 }
        .get_mut()
        .as_any_mut()
        .downcast_mut::<SparseSet<&'static str>>()
        .unwrap()
        .delete(entity_id);
    assert_eq!(
        unsafe { &mut *storage.0 }
            .get_mut()
            .as_any_mut()
            .downcast_mut::<SparseSet<&'static str>>()
            .unwrap()
            .private_get(entity_id),
        None
    );
    entity_id.set_index(10);
    assert_eq!(
        unsafe { &mut *storage.0 }
            .get_mut()
            .as_any_mut()
            .downcast_mut::<SparseSet<&'static str>>()
            .unwrap()
            .private_get(entity_id),
        Some(&"test10")
    );
    entity_id.set_index(1);
    assert_eq!(
        unsafe { &mut *storage.0 }
            .get_mut()
            .as_any_mut()
            .downcast_mut::<SparseSet<&'static str>>()
            .unwrap()
            .private_get(entity_id),
        Some(&"test1")
    );
    entity_id.set_index(10);
    unsafe { &mut *storage.0 }
        .get_mut()
        .as_any_mut()
        .downcast_mut::<SparseSet<&'static str>>()
        .unwrap()
        .delete(entity_id);
    entity_id.set_index(1);
    unsafe { &mut *storage.0 }
        .get_mut()
        .as_any_mut()
        .downcast_mut::<SparseSet<&'static str>>()
        .unwrap()
        .delete(entity_id);
    entity_id.set_index(5);
    assert_eq!(
        unsafe { &mut *storage.0 }
            .get_mut()
            .as_any_mut()
            .downcast_mut::<SparseSet<&'static str>>()
            .unwrap()
            .private_get(entity_id),
        None
    );
    entity_id.set_index(10);
    assert_eq!(
        unsafe { &mut *storage.0 }
            .get_mut()
            .as_any_mut()
            .downcast_mut::<SparseSet<&'static str>>()
            .unwrap()
            .private_get(entity_id),
        None
    );
    entity_id.set_index(1);
    assert_eq!(
        unsafe { &mut *storage.0 }
            .get_mut()
            .as_any_mut()
            .downcast_mut::<SparseSet<&'static str>>()
            .unwrap()
            .private_get(entity_id),
        None
    );
}
