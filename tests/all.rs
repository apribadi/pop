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

  is_clone::<ptr>();
  is_copy::<ptr>();
  is_eq::<ptr>();
  is_hash::<ptr>();
  is_ord::<ptr>();
  is_partial_eq::<ptr>();
  is_partial_ord::<ptr>();
  is_ref_unwind_safe::<ptr>();
  is_send::<ptr>();
  is_sync::<ptr>();
  is_unpin::<ptr>();
  is_unwind_safe::<ptr>();
}
