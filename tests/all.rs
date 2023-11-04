use pop::Ptr;

#[test]
fn test_special_traits() {
  fn is_clone<T: Clone>() {}
  fn is_copy<T: Copy>() {}
  fn is_eq<T: Eq>() {}
  fn is_hash<T: core::hash::Hash>() {}
  fn is_ord<T: Ord>() {}
  fn is_partial_eq<T: PartialEq>() {}
  fn is_partial_ord<T: PartialOrd>() {}
  fn is_ref_unwind_safe<T: core::panic::RefUnwindSafe>() {}
  fn is_send<T: Send>() {}
  fn is_sync<T: Sync>() {}
  fn is_unpin<T: Unpin>() {}
  fn is_unwind_safe<T: core::panic::UnwindSafe>() {}

  is_clone::<Ptr>();
  is_copy::<Ptr>();
  is_eq::<Ptr>();
  is_hash::<Ptr>();
  is_ord::<Ptr>();
  is_partial_eq::<Ptr>();
  is_partial_ord::<Ptr>();
  is_ref_unwind_safe::<Ptr>();
  is_send::<Ptr>();
  is_sync::<Ptr>();
  is_unpin::<Ptr>();
  is_unwind_safe::<Ptr>();
}
