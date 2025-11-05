//! Tests

use pop::ptr;

#[test]
fn test_special_traits() {
  fn is_clone<T: Clone>() {}
  fn is_copy<T: Copy>() {}
  fn is_eq<T: Eq>() {}
  fn is_hash<T: std::hash::Hash>() {}
  fn is_ord<T: Ord>() {}
  fn is_partial_eq<T: PartialEq>() {}
  fn is_partial_ord<T: PartialOrd>() {}
  fn is_ref_unwind_safe<T: std::panic::RefUnwindSafe>() {}
  fn is_send<T: Send>() {}
  fn is_sync<T: Sync>() {}
  fn is_unpin<T: Unpin>() {}
  fn is_unwind_safe<T: std::panic::UnwindSafe>() {}

  type T = (f64, &'static mut f64, *mut f64, core::cell::Cell<f64>);

  is_clone::<ptr<T>>();
  is_copy::<ptr<T>>();
  is_eq::<ptr<T>>();
  is_hash::<ptr<T>>();
  is_ord::<ptr<T>>();
  is_partial_eq::<ptr<T>>();
  is_partial_ord::<ptr<T>>();
  is_ref_unwind_safe::<ptr<T>>();
  is_send::<ptr<T>>();
  is_sync::<ptr<T>>();
  is_unpin::<ptr<T>>();
  is_unwind_safe::<ptr<T>>();
}
