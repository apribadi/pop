use core::marker::PhantomData;
use core::ptr::NonNull;

/// TODO

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ptr<T>(*mut u8, PhantomData<fn(T) -> T>);

unsafe impl<T> Send for ptr<T> {
}

unsafe impl<T> Sync for ptr<T> {
}

impl<T> ptr<T> {
  /// Whether the pointer's address is zero.

  #[inline(always)]
  pub fn is_null(self) -> bool {
    return self.addr() == 0;
  }

  /// Creates a pointer with the given address and no provenance.

  #[inline(always)]
  pub const fn invalid(addr: usize) -> ptr<T> {
    return ptr(core::ptr::without_provenance_mut(addr), PhantomData);
  }

  /// Creates a pointer with address zero and no provenance.

  pub const fn null() -> ptr<T> {
    return ptr::invalid(0);
  }

  /// Casts the pointer to a different type.

  pub const fn cast<U>(self) -> ptr<U> {
    return ptr(self.0, PhantomData);
  }

  /// The address of the pointer.

  #[inline(always)]
  pub fn addr(self) -> usize {
    return self.0.addr();
  }

  /// Changes the address of the pointer while keeping the provenance.

  #[inline(always)]
  pub fn with_addr(self, addr: usize) -> ptr<T> {
    return ptr(self.0.with_addr(addr), PhantomData);
  }

  /// Whether the pointer is aligned appropriately for `T`.

  #[inline(always)]
  pub fn is_aligned(self) -> bool {
    return self.addr() & align_of::<T>() - 1 == 0;
  }

  /// Converts into a `*const T`.

  #[inline(always)]
  pub const fn as_const_ptr(self) -> *const T {
    return self.0 as *const T;
  }

  /// Converts into a `*mut T`.

  #[inline(always)]
  pub const fn as_mut_ptr(self) -> *mut T {
    return self.0 as *mut T;
  }

  /// Converts into a `*const [T]`.

  #[inline(always)]
  pub const fn as_slice_const_ptr(self, len: usize) -> *const [T] {
    return core::ptr::slice_from_raw_parts(self.0 as *const T, len);
  }

  /// Converts into a `*mut [T]`.

  #[inline(always)]
  pub const fn as_slice_mut_ptr(self, len: usize) -> *mut [T] {
    return core::ptr::slice_from_raw_parts_mut(self.0 as *mut T, len);
  }

  /// Converts into a `&T`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_ref<'a>(self) -> &'a T {
    return unsafe { &*(self.0 as *const T) };
  }

  /// Converts into a `&mut T`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_mut_ref<'a>(self) -> &'a mut T {
    return unsafe { &mut *(self.0 as *mut T) };
  }

  /// Converts into a `&[T]`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_slice_ref<'a>(self, len: usize) -> &'a [T] {
    return unsafe { &*core::ptr::slice_from_raw_parts(self.0 as *const T, len) };
  }

  /// Converts into a `&mut [T]`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_slice_mut_ref<'a>(self, len: usize) -> &'a mut [T] {
    return unsafe { &mut *core::ptr::slice_from_raw_parts_mut(self.0 as *mut T, len) };
  }

  /// Converts into a `NonNull<T>`.
  ///
  /// # SAFETY
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub const unsafe fn as_non_null(self) -> NonNull<T> {
    return unsafe { NonNull::new_unchecked(self.0 as *mut T) };
  }

  /// Converts into a `NonNull<[T]>`.
  ///
  /// # SAFETY
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub const unsafe fn as_slice_non_null(self, len: usize) -> NonNull<[T]> {
    return unsafe { NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(self.0 as *mut T, len)) };
  }

  /// Reads a value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::read].

  #[inline(always)]
  pub const unsafe fn read(self) -> T {
    return unsafe { core::ptr::read(self.0 as *const T) };
  }

  /// Reads a value without requiring alignment.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::read_unaligned].

  #[inline(always)]
  pub const unsafe fn read_unaligned(self) -> T {
    return unsafe { core::ptr::read_unaligned(self.0 as *const T) };
  }

  /// # SAFETY
  ///
  /// See [core::ptr::read_volatile].

  #[inline(always)]
  pub unsafe fn read_volatile(self) -> T {
    return unsafe { core::ptr::read_volatile(self.0 as *const T) };
  }

  /// Writes a value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write].

  #[inline(always)]
  pub const unsafe fn write(self, value: T) {
    unsafe { core::ptr::write(self.0 as *mut T, value) };
  }

  /// Writes a value without requiring alignment.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write_unaligned].

  #[inline(always)]
  pub const unsafe fn write_unaligned(self, value: T) {
    unsafe { core::ptr::write_unaligned(self.0 as *mut T, value) };
  }

  /// # SAFETY
  ///
  /// See [core::ptr::write_volatile].

  #[inline(always)]
  pub unsafe fn write_volatile(self, value: T) {
    unsafe { core::ptr::write_volatile(self.0 as *mut T, value) };
  }

  /// Replaces the value at `self` with `value`, returning the old value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::replace].

  #[inline(always)]
  pub const unsafe fn replace(self, value: T) -> T {
    return unsafe { core::ptr::replace(self.0 as *mut T, value) };
  }

  /// Drops the pointed-to value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::drop_in_place].

  #[inline(always)]
  pub unsafe fn drop_in_place(self) {
    // WARNING! It is necessary to cast the pointer to `*mut T` so that we
    // invoke the correct drop function.
    unsafe { core::ptr::drop_in_place(self.0 as *mut T) };
  }

  /// Copies `count * size_of::<T>()` bytes from `src` to `self`. The source
  /// and destination regions must not overlap.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::copy_nonoverlapping].

  #[inline(always)]
  pub const unsafe fn copy_from_nonoverlapping(self, src: ptr<T>, count: usize) {
    unsafe { core::ptr::copy_nonoverlapping(src.0 as *const T, self.0 as *mut T, count) };
  }

  /// Writes `count * size_of::<T>()` copies of byte `value` at `x`.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write_bytes].

  #[inline(always)]
  pub unsafe fn write_bytes(self, value: u8, count: usize) {
    unsafe { core::ptr::write_bytes(self.0 as *mut T, value, count) };
  }
}

impl<T, U: ?Sized> From<*const U> for ptr<T> {
  #[inline(always)]
  fn from(value: *const U) -> ptr<T> {
    return ptr(value as _, PhantomData);
  }
}

impl<T, U: ?Sized> From<*mut U> for ptr<T> {
  #[inline(always)]
  fn from(value: *mut U) -> ptr<T> {
    return ptr(value as _, PhantomData);
  }
}

impl<T, U: ?Sized> From<&U> for ptr<T> {
  #[inline(always)]
  fn from(value: &U) -> ptr<T> {
    return ptr(value as *const U as _, PhantomData);
  }
}

impl<T, U: ?Sized> From<&mut U> for ptr<T> {
  #[inline(always)]
  fn from(value: &mut U) -> ptr<T> {
    return ptr(value as *mut U as _, PhantomData);
  }
}

impl<T, U: ?Sized> From<NonNull<U>> for ptr<T> {
  #[inline(always)]
  fn from(value: NonNull<U>) -> ptr<T> {
    return ptr(value.as_ptr() as _, PhantomData);
  }
}

impl<T, U> From<ptr<T>> for *const U {
  #[inline(always)]
  fn from(value: ptr<T>) -> *const U {
    return value.0 as *const U;
  }
}

impl<T, U> From<ptr<T>> for *mut U {
  #[inline(always)]
  fn from(value: ptr<T>) -> *mut U {
    return value.0 as *mut U;
  }
}
/*

impl core::ops::Add<isize> for ptr {
  type Output = ptr;

  #[inline(always)]
  fn add(self, rhs: isize) -> ptr {
    return ptr(self.0.wrapping_offset(rhs));
  }
}

impl core::ops::AddAssign<isize> for ptr {
  #[inline(always)]
  fn add_assign(&mut self, rhs: isize) {
    *self = *self + rhs;
  }
}

impl core::ops::Add<usize> for ptr {
  type Output = ptr;

  #[inline(always)]
  fn add(self, rhs: usize) -> ptr {
    return ptr(self.0.wrapping_add(rhs));
  }
}

impl core::ops::AddAssign<usize> for ptr {
  #[inline(always)]
  fn add_assign(&mut self, rhs: usize) {
    *self = *self + rhs;
  }
}

impl core::ops::Sub<isize> for ptr {
  type Output = ptr;

  #[inline(always)]
  fn sub(self, rhs: isize) -> ptr {
    return ptr(self.0.wrapping_offset(rhs.wrapping_neg()));
  }
}

impl core::ops::SubAssign<isize> for ptr {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: isize) {
    *self = *self - rhs;
  }
}

impl core::ops::Sub<usize> for ptr {
  type Output = ptr;

  #[inline(always)]
  fn sub(self, rhs: usize) -> ptr {
    return ptr(self.0.wrapping_sub(rhs));
  }
}

impl core::ops::SubAssign<usize> for ptr {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: usize) {
    *self = *self - rhs;
  }
}

impl core::ops::Sub<ptr> for ptr {
  type Output = usize;

  #[inline(always)]
  fn sub(self, rhs: ptr) -> usize {
    return self.addr().wrapping_sub(rhs.addr());
  }
}

impl Default for ptr {
  #[inline(always)]
  fn default() -> ptr {
    return ptr::NULL;
  }
}

impl core::fmt::Pointer for ptr {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    return <*const u8 as core::fmt::Pointer>::fmt(&(self.0 as _), f);
  }
}

impl core::fmt::Debug for ptr {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    return <ptr as core::fmt::Pointer>::fmt(self, f);
  }
}

/// Copies `count * size_of::<T>()` bytes from `src` to `dst`. The source
/// and destination regions must not overlap.
///
/// # SAFETY
///
/// See [core::ptr::copy_nonoverlapping].

#[inline(always)]
pub const unsafe fn copy_nonoverlapping<T>(src: ptr, dst: ptr, count: usize) {
  unsafe { core::ptr::copy_nonoverlapping::<T>(src.0 as _, dst.0 as _, count) };
}

/// Swaps `count * size_of::<T>()` bytes between the regions pointed-to by
/// `x` and `y`.
///
/// # SAFETY
///
/// See [core::ptr::swap_nonoverlapping].

#[inline(always)]
pub unsafe fn swap_nonoverlapping<T>(x: ptr, y: ptr, count: usize) {
  unsafe { core::ptr::swap_nonoverlapping::<T>(x.0 as _, y.0 as _, count) };
}
*/
