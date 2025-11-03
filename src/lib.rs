#![doc = include_str!("../README.md")]
#![no_std]

use core::ptr::NonNull;

/// A pointer type without anything extra.

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ptr(*mut u8);

unsafe impl Send for ptr {
}

unsafe impl Sync for ptr {
}

impl ptr {
  /// A pointer with address zero and no provenance.

  pub const NULL: ptr = ptr::invalid(0);

  /// Whether the pointer's address is zero.

  #[inline(always)]
  pub fn is_null(self) -> bool {
    return self.addr() == 0;
  }

  /// Creates a pointer with the given address and no provenance.

  #[inline(always)]
  pub const fn invalid(addr: usize) -> ptr {
    return ptr(core::ptr::without_provenance_mut(addr));
  }

  /// The address of the pointer.

  #[inline(always)]
  pub fn addr(self) -> usize {
    return self.0.addr();
  }

  /// Changes the address of the pointer while keeping the provenance.

  #[inline(always)]
  pub fn with_addr(self, addr: usize) -> ptr {
    return ptr(self.0.with_addr(addr));
  }

  /// Given a pointer to an array of `T`s, computes the pointer to the `i`th
  /// element.

  #[inline(always)]
  pub const fn index<T>(self, i: usize) -> ptr {
    return ptr(self.0.wrapping_add(i.wrapping_mul(size_of::<T>())));
  }

  /// Whether the pointer is aligned appropriately for `T`.

  #[inline(always)]
  pub fn is_aligned<T>(self) -> bool {
    return self.addr() & align_of::<T>() - 1 == 0;
  }

  /// Converts into a `*const T`.

  #[inline(always)]
  pub const fn as_const_ptr<T>(self) -> *const T {
    return self.0 as _;
  }

  /// Converts into a `*mut T`.

  #[inline(always)]
  pub const fn as_mut_ptr<T>(self) -> *mut T {
    return self.0 as _;
  }

  /// Converts into a `*const [T]`.

  #[inline(always)]
  pub const fn as_slice_const_ptr<T>(self, len: usize) -> *const [T] {
    return core::ptr::slice_from_raw_parts(self.0 as _, len);
  }

  /// Converts into a `*mut [T]`.

  #[inline(always)]
  pub const fn as_slice_mut_ptr<T>(self, len: usize) -> *mut [T] {
    return core::ptr::slice_from_raw_parts_mut(self.0 as _, len);
  }

  /// Converts into a `&T`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_ref<'a, T>(self) -> &'a T {
    return unsafe { &*(self.0 as *const _) };
  }

  /// Converts into a `&mut T`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_mut_ref<'a, T>(self) -> &'a mut T {
    return unsafe { &mut *(self.0 as *mut _) };
  }

  /// Converts into a `&[T]`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_slice_ref<'a, T>(self, len: usize) -> &'a [T] {
    return unsafe { &*core::ptr::slice_from_raw_parts(self.0 as _, len) };
  }

  /// Converts into a `&mut [T]`.
  ///
  /// SAFETY
  ///
  /// The reference must be be valid for the lifetime.

  #[inline(always)]
  pub const unsafe fn as_slice_mut_ref<'a, T>(self, len: usize) -> &'a mut [T] {
    return unsafe { &mut *core::ptr::slice_from_raw_parts_mut(self.0 as _, len) };
  }

  /// Converts into a `NonNull<T>`.
  ///
  /// # SAFETY
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub const unsafe fn as_non_null<T>(self) -> NonNull<T> {
    return unsafe { NonNull::new_unchecked(self.0 as _) };
  }

  /// Converts into a `NonNull<[T]>`.
  ///
  /// # SAFETY
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub const unsafe fn as_slice_non_null<T>(self, len: usize) -> NonNull<[T]> {
    return unsafe { NonNull::new_unchecked(core::ptr::slice_from_raw_parts(self.0 as _, len) as _) };
  }

  /// Reads a value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::read].

  #[inline(always)]
  pub const unsafe fn read<T>(self) -> T {
    return unsafe { core::ptr::read(self.0 as _) };
  }

  /// Reads a value without requiring alignment.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::read_unaligned].

  #[inline(always)]
  pub const unsafe fn read_unaligned<T>(self) -> T {
    return unsafe { core::ptr::read_unaligned(self.0 as _) };
  }

  /// # SAFETY
  ///
  /// See [core::ptr::read_volatile].

  #[inline(always)]
  pub unsafe fn read_volatile<T>(self) -> T {
    return unsafe { core::ptr::read_volatile(self.0 as _) };
  }

  /// Writes a value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write].

  #[inline(always)]
  pub const unsafe fn write<T>(self, value: T) {
    unsafe { core::ptr::write(self.0 as _, value) };
  }

  /// Writes a value without requiring alignment.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write_unaligned].

  #[inline(always)]
  pub const unsafe fn write_unaligned<T>(self, value: T) {
    unsafe { core::ptr::write_unaligned(self.0 as _, value) };
  }

  /// # SAFETY
  ///
  /// See [core::ptr::write_volatile].

  #[inline(always)]
  pub unsafe fn write_volatile<T>(self, value: T) {
    unsafe { core::ptr::write_volatile(self.0 as _, value) };
  }

  /// Drops the pointed-to value.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::drop_in_place].

  #[inline(always)]
  pub unsafe fn drop_in_place<T: ?Sized>(self) {
    unsafe { core::ptr::drop_in_place(self.0 as _) };
  }

  /// Copies `count * size_of::<T>()` bytes from `src` to `self`. The source
  /// and destination regions must not overlap.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::copy_nonoverlapping].

  #[inline(always)]
  pub const unsafe fn copy_from_nonoverlapping<T>(self, src: ptr, count: usize) {
    unsafe { core::ptr::copy_nonoverlapping::<T>(src.0 as _, self.0 as _, count) };
  }

  /// Writes `count * size_of::<T>()` copies of byte `value` at `x`.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write_bytes].

  #[inline(always)]
  pub unsafe fn write_bytes<T>(self, value: u8, count: usize) {
    unsafe { core::ptr::write_bytes::<T>(self.0 as _, value, count) };
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
}

impl<T: ?Sized> From<*const T> for ptr {
  #[inline(always)]
  fn from(value: *const T) -> ptr {
    return ptr(value as _);
  }
}

impl<T: ?Sized> From<*mut T> for ptr {
  #[inline(always)]
  fn from(value: *mut T) -> ptr {
    return ptr(value as _);
  }
}

impl<T: ?Sized> From<&T> for ptr {
  #[inline(always)]
  fn from(value: &T) -> ptr {
    return ptr(value as *const _ as _);
  }
}

impl<T: ?Sized> From<&mut T> for ptr {
  #[inline(always)]
  fn from(value: &mut T) -> ptr {
    return ptr(value as *mut _ as _);
  }
}

impl<T: ?Sized> From<NonNull<T>> for ptr {
  #[inline(always)]
  fn from(value: NonNull<T>) -> ptr {
    return ptr(value.as_ptr() as _);
  }
}

impl<T> From<ptr> for *const T {
  #[inline(always)]
  fn from(value: ptr) -> *const T {
    return value.0 as _;
  }
}

impl<T> From<ptr> for *mut T {
  #[inline(always)]
  fn from(value: ptr) -> *mut T {
    return value.0 as _;
  }
}

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
