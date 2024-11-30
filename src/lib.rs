#![doc = include_str!("../README.md")]
#![no_std]

/// A pointer type without anything extra.

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ptr(*mut u8);

unsafe impl Send for ptr { }

unsafe impl Sync for ptr { }

impl ptr {
  /// A pointer with address zero and no provenance.

  pub const NULL: ptr = ptr::invalid(0);

  /// Whether the pointer's address is zero.

  #[inline(always)]
  pub fn is_null(self) -> bool {
    self.addr() == 0
  }

  /// Creates a pointer with the given address and no provenance.

  #[inline(always)]
  pub const fn invalid(addr: usize) -> ptr {
    ptr(unsafe { core::mem::transmute::<usize, *mut u8>(addr) })
  }

  /// The address of the pointer.

  #[inline(always)]
  pub fn addr(self) -> usize {
    // NB: This must NOT be `const`. Transmuting a pointer into an integer in a
    // const context is undefined behavior.

    unsafe { core::mem::transmute::<*mut u8, usize>(self.0) }
  }

  /// Changes the address of the pointer while keeping the provenance.

  #[inline(always)]
  pub fn with_addr(self, addr: usize) -> ptr {
    ptr(self.0.wrapping_add(addr.wrapping_sub(self.addr())))
  }

  /// Given a pointer to an array of `T`s, computes the pointer to the `i`th
  /// element.

  #[inline(always)]
  pub fn index<T>(self, i: usize) -> ptr {
    ptr(self.0.wrapping_add(i.wrapping_mul(size_of::<T>())))
  }

  /// Whether the pointer is aligned appropriately for `T`.

  #[inline(always)]
  pub fn is_aligned<T>(self) -> bool {
    self.addr() & align_of::<T>() - 1 == 0
  }

  /// Converts into a `*const T`.

  #[inline(always)]
  pub const fn as_const_ptr<T>(self) -> *const T {
    self.0 as _
  }

  /// Converts into a `*mut T`.

  #[inline(always)]
  pub const fn as_mut_ptr<T>(self) -> *mut T {
    self.0 as _
  }

  /// Converts into a `*const [T]`.

  #[inline(always)]
  pub const fn as_slice_const_ptr<T>(self, len: usize) -> *const [T] {
    core::ptr::slice_from_raw_parts(self.0 as _, len)
  }

  /// Converts into a `*mut [T]`.

  #[inline(always)]
  pub const fn as_slice_mut_ptr<T>(self, len: usize) -> *mut [T] {
    core::ptr::slice_from_raw_parts(self.0 as _, len) as _
  }

  /// Converts into a `&T`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_ref<'a, T>(self) -> &'a T {
    &*(self.0 as *const _)
  }

  /// Converts into a `&mut T`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub unsafe fn as_mut_ref<'a, T>(self) -> &'a mut T {
    &mut *(self.0 as *mut _)
  }

  /// Converts into a `&[T]`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_slice_ref<'a, T>(self, len: usize) -> &'a [T] {
    &*core::ptr::slice_from_raw_parts(self.0 as _, len)
  }

  /// Converts into a `&mut [T]`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub unsafe fn as_slice_mut_ref<'a, T>(self, len: usize) -> &'a mut [T] {
    &mut *core::ptr::slice_from_raw_parts_mut(self.0 as _, len)
  }

  /// Converts into a `NonNull<T>`.
  ///
  /// # SAFETY
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub const unsafe fn as_non_null<T>(self) -> core::ptr::NonNull<T> {
    core::ptr::NonNull::new_unchecked(self.0 as _)
  }

  /// Converts into a `NonNull<[T]>`.
  ///
  /// # SAFETY
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub const unsafe fn as_slice_non_null<T>(self, len: usize) -> core::ptr::NonNull<[T]> {
    core::ptr::NonNull::new_unchecked(core::ptr::slice_from_raw_parts(self.0 as _, len) as _)
  }

  /// Reads a value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::read].

  #[inline(always)]
  pub const unsafe fn read<T>(x: ptr) -> T {
    core::ptr::read(x.0 as _)
  }

  /// Reads a value without requiring alignment.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::read_unaligned].

  #[inline(always)]
  pub const unsafe fn read_unaligned<T>(x: ptr) -> T {
    core::ptr::read_unaligned(x.0 as _)
  }

  /// # SAFETY
  ///
  /// See [core::ptr::read_volatile].

  #[inline(always)]
  pub unsafe fn read_volatile<T>(x: ptr) -> T {
    core::ptr::read_volatile(x.0 as _)
  }

  /// Writes a value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write].

  #[inline(always)]
  pub unsafe fn write<T>(x: ptr, value: T) {
    core::ptr::write(x.0 as _, value)
  }

  /// Writes a value without requiring alignment.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write_unaligned].

  #[inline(always)]
  pub unsafe fn write_unaligned<T>(x: ptr, value: T) {
    core::ptr::write_unaligned(x.0 as _, value)
  }

  /// # SAFETY
  ///
  /// See [core::ptr::write_volatile].

  #[inline(always)]
  pub unsafe fn write_volatile<T>(x: ptr, value: T) {
    core::ptr::write_volatile(x.0 as _, value)
  }

  /// Reads a value and writes another value in its place.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::replace].

  #[inline(always)]
  pub unsafe fn replace<T>(x: ptr, value: T) -> T {
    core::ptr::replace(x.0 as _, value)
  }

  /// Drops the pointed-to value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::drop_in_place].

  #[inline(always)]
  pub unsafe fn drop_in_place<T: ?Sized>(x: ptr) {
    core::ptr::drop_in_place(x.0 as _)
  }

  /// Copies `count * size_of::<T>()` bytes from `src` to `dst`.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::copy_nonoverlapping].

  #[inline(always)]
  pub unsafe fn copy_nonoverlapping<T>(src: ptr, dst: ptr, count: usize) {
    core::ptr::copy_nonoverlapping::<T>(src.0 as _, dst.0 as _, count) ;
  }

  /// Swaps `count * size_of::<T>()` bytes between the regions pointed-to by
  /// `x` and `y`.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::swap_nonoverlapping].

  #[inline(always)]
  pub unsafe fn swap_nonoverlapping<T>(x: ptr, y: ptr, count: usize) {
    core::ptr::swap_nonoverlapping::<T>(x.0 as _, y.0 as _, count) ;
  }
}

impl Default for ptr {
  #[inline(always)]
  fn default() -> ptr {
    ptr::NULL
  }
}

impl<T: ?Sized> From<*const T> for ptr {
  #[inline(always)]
  fn from(value: *const T) -> ptr {
    ptr(value as _)
  }
}

impl<T: ?Sized> From<*mut T> for ptr {
  #[inline(always)]
  fn from(value: *mut T) -> ptr {
    ptr(value as _)
  }
}

impl<T: ?Sized> From<&T> for ptr {
  #[inline(always)]
  fn from(value: &T) -> ptr {
    ptr(value as *const _ as _)
  }
}

impl<T: ?Sized> From<&mut T> for ptr {
  #[inline(always)]
  fn from(value: &mut T) -> ptr {
    ptr(value as *mut _ as _)
  }
}

impl<T: ?Sized> From<core::ptr::NonNull<T>> for ptr {
  #[inline(always)]
  fn from(value: core::ptr::NonNull<T>) -> ptr {
    ptr(value.as_ptr() as _)
  }
}

impl<T> From<ptr> for *const T {
  #[inline(always)]
  fn from(value: ptr) -> *const T {
    value.0 as _
  }
}

impl<T> From<ptr> for *mut T {
  #[inline(always)]
  fn from(value: ptr) -> *mut T {
    value.0 as _
  }
}

impl core::ops::Add<isize> for ptr {
  type Output = ptr;

  #[inline(always)]
  fn add(self, rhs: isize) -> ptr {
    ptr(self.0.wrapping_offset(rhs))
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
    ptr(self.0.wrapping_add(rhs))
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
    ptr(self.0.wrapping_offset(rhs.wrapping_neg()))
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
    ptr(self.0.wrapping_sub(rhs))
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
    self.addr().wrapping_sub(rhs.addr())
  }
}

impl core::fmt::Pointer for ptr {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <*const u8 as core::fmt::Pointer>::fmt(&(self.0 as _), f)
  }
}

impl core::fmt::Debug for ptr {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <ptr as core::fmt::Pointer>::fmt(self, f)
  }
}
