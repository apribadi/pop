#![doc = include_str!("../README.md")]
#![no_std]

use core::marker::PhantomData;
use core::ptr::NonNull;

/// TODO

#[allow(non_camel_case_types)]
pub struct ptr<T>(*mut u8, PhantomData<fn(T) -> T>);

impl<T> Clone for ptr<T> {
  #[inline(always)]
  fn clone(&self) -> Self {
    return *self;
  }
}

impl<T> Copy for ptr<T> {
}

impl<T> Eq for ptr<T> {
}

impl<T> core::hash::Hash for ptr<T> {
  #[inline(always)]
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl<T> Ord for ptr<T> {
  #[inline(always)]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    return self.0.cmp(&other.0);
  }
}

impl<T> PartialEq for ptr<T> {
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    return self.0 == other.0;
  }

  #[inline(always)]
  fn ne(&self, other: &Self) -> bool {
    return self.0 != other.0;
  }
}

impl<T> PartialOrd for ptr<T> {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    return self.0.partial_cmp(&other.0);
  }

  #[inline(always)]
  fn lt(&self, other: &Self) -> bool {
    return self.0 < other.0;
  }

  #[inline(always)]
  fn le(&self, other: &Self) -> bool {
    return self.0 <= other.0;
  }

  #[inline(always)]
  fn gt(&self, other: &Self) -> bool {
    return self.0 > other.0;
  }

  #[inline(always)]
  fn ge(&self, other: &Self) -> bool {
    return self.0 >= other.0;
  }
}

unsafe impl<T> Send for ptr<T> {
}

unsafe impl<T> Sync for ptr<T> {
}

impl<T> ptr<T> {
  /// Creates a pointer with the given address and no provenance.

  #[inline(always)]
  pub const fn invalid(addr: usize) -> ptr<T> {
    return ptr(core::ptr::without_provenance_mut(addr), PhantomData);
  }

  /// Creates a pointer with address zero and no provenance.

  #[inline(always)]
  pub const fn null() -> ptr<T> {
    return ptr::invalid(0);
  }

  /// Whether the pointer's address is zero.

  #[inline(always)]
  pub fn is_null(self) -> bool {
    return self.addr() == 0;
  }

  /// Casts the pointer to a different type.

  #[inline(always)]
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

  /// TODO

  #[inline(always)]
  pub fn byte_add<U>(self, n: usize) -> ptr<U> {
    return ptr(self.0.wrapping_add(n) as _, PhantomData);
  }

  /// TODO

  #[inline(always)]
  pub fn byte_sub<U>(self, n: usize) -> ptr<U> {
    return ptr(self.0.wrapping_sub(n) as _, PhantomData);
  }

  /// TODO

  #[inline(always)]
  pub fn byte_diff<U>(self, from: ptr<U>) -> usize {
    return self.addr().wrapping_sub(from.addr());
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

  /// Swaps `count * size_of::<T>()` bytes between the regions pointed-to by
  /// `self` and `with`.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::swap_nonoverlapping].

  #[inline(always)]
  pub const unsafe fn swap_nonoverlapping(self, with: ptr<T>, count: usize) {
    unsafe { core::ptr::swap_nonoverlapping(self.0 as *mut T, with.0 as *mut T, count) };
  }

  /// Writes `count * size_of::<T>()` copies of byte `value` at `x`.
  ///
  /// # SAFETY
  ///
  /// See [core::ptr::write_bytes].

  #[inline(always)]
  pub const unsafe fn write_bytes(self, value: u8, count: usize) {
    unsafe { core::ptr::write_bytes(self.0 as *mut T, value, count) };
  }
}

impl<T> From<*const T> for ptr<T> {
  #[inline(always)]
  fn from(value: *const T) -> ptr<T> {
    return ptr(value as _, PhantomData);
  }
}

impl<T> From<*mut T> for ptr<T> {
  #[inline(always)]
  fn from(value: *mut T) -> ptr<T> {
    return ptr(value as _, PhantomData);
  }
}

impl<T> From<&T> for ptr<T> {
  #[inline(always)]
  fn from(value: &T) -> ptr<T> {
    return ptr(value as *const T as _, PhantomData);
  }
}

impl<T> From<&mut T> for ptr<T> {
  #[inline(always)]
  fn from(value: &mut T) -> ptr<T> {
    return ptr(value as *mut T as _, PhantomData);
  }
}

impl<T> From<NonNull<T>> for ptr<T> {
  #[inline(always)]
  fn from(value: NonNull<T>) -> ptr<T> {
    return ptr(value.as_ptr() as _, PhantomData);
  }
}

impl<T> From<*const [T]> for ptr<T> {
  #[inline(always)]
  fn from(value: *const [T]) -> ptr<T> {
    return ptr(value as _, PhantomData);
  }
}

impl<T> From<*mut [T]> for ptr<T> {
  #[inline(always)]
  fn from(value: *mut [T]) -> ptr<T> {
    return ptr(value as _, PhantomData);
  }
}

impl<T> From<&[T]> for ptr<T> {
  #[inline(always)]
  fn from(value: &[T]) -> ptr<T> {
    return ptr(value as *const [T] as _, PhantomData);
  }
}

impl<T> From<&mut [T]> for ptr<T> {
  #[inline(always)]
  fn from(value: &mut [T]) -> ptr<T> {
    return ptr(value as *mut [T] as _, PhantomData);
  }
}

impl<T> From<NonNull<[T]>> for ptr<T> {
  #[inline(always)]
  fn from(value: NonNull<[T]>) -> ptr<T> {
    return ptr(value.as_ptr() as _, PhantomData);
  }
}

impl<T> From<ptr<T>> for *const T {
  #[inline(always)]
  fn from(value: ptr<T>) -> *const T {
    return value.0 as *const T;
  }
}

impl<T> From<ptr<T>> for *mut T {
  #[inline(always)]
  fn from(value: ptr<T>) -> *mut T {
    return value.0 as *mut T;
  }
}

impl<T> core::ops::Add<isize> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn add(self, rhs: isize) -> ptr<T> {
    return ptr((self.0 as *mut T).wrapping_offset(rhs) as _, PhantomData);
  }
}

impl<T> core::ops::Add<usize> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn add(self, rhs: usize) -> ptr<T> {
    return ptr((self.0 as *mut T).wrapping_add(rhs) as _, PhantomData);
  }
}

impl<T> core::ops::Add<i32> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn add(self, rhs: i32) -> ptr<T> {
    return self + rhs as isize;
  }
}

impl<T> core::ops::Add<u32> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn add(self, rhs: u32) -> ptr<T> {
    return self + rhs as usize;
  }
}

impl<T> core::ops::Add<i64> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn add(self, rhs: i64) -> ptr<T> {
    return self + rhs as isize;
  }
}

impl<T> core::ops::Add<u64> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn add(self, rhs: u64) -> ptr<T> {
    return self + rhs as usize;
  }
}

impl<T> core::ops::Sub<isize> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn sub(self, rhs: isize) -> ptr<T> {
    return ptr((self.0 as *mut T).wrapping_offset(rhs.wrapping_neg()) as _, PhantomData);
  }
}

impl<T> core::ops::Sub<usize> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn sub(self, rhs: usize) -> ptr<T> {
    return ptr((self.0 as *mut T).wrapping_sub(rhs) as _, PhantomData);
  }
}

impl<T> core::ops::Sub<i32> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn sub(self, rhs: i32) -> ptr<T> {
    return self - rhs as isize;
  }
}

impl<T> core::ops::Sub<u32> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn sub(self, rhs: u32) -> ptr<T> {
    return self - rhs as usize;
  }
}

impl<T> core::ops::Sub<i64> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn sub(self, rhs: i64) -> ptr<T> {
    return self - rhs as isize;
  }
}

impl<T> core::ops::Sub<u64> for ptr<T> {
  type Output = ptr<T>;

  #[inline(always)]
  fn sub(self, rhs: u64) -> ptr<T> {
    return self - rhs as usize;
  }
}

impl<T> core::ops::Sub<ptr<T>> for ptr<T> {
  type Output = usize;

  #[inline(always)]
  fn sub(self, rhs: ptr<T>) -> usize {
    return self.addr().wrapping_sub(rhs.addr()) / size_of::<T>();
  }
}

impl<T> core::ops::AddAssign<isize> for ptr<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: isize) {
    *self = *self + rhs;
  }
}

impl<T> core::ops::AddAssign<usize> for ptr<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: usize) {
    *self = *self + rhs;
  }
}

impl<T> core::ops::AddAssign<i32> for ptr<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: i32) {
    *self = *self + rhs;
  }
}

impl<T> core::ops::AddAssign<u32> for ptr<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: u32) {
    *self = *self + rhs;
  }
}

impl<T> core::ops::AddAssign<i64> for ptr<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: i64) {
    *self = *self + rhs;
  }
}

impl<T> core::ops::AddAssign<u64> for ptr<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: u64) {
    *self = *self + rhs;
  }
}

impl<T> core::ops::SubAssign<isize> for ptr<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: isize) {
    *self = *self - rhs;
  }
}

impl<T> core::ops::SubAssign<usize> for ptr<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: usize) {
    *self = *self - rhs;
  }
}

impl<T> core::ops::SubAssign<i32> for ptr<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: i32) {
    *self = *self - rhs;
  }
}

impl<T> core::ops::SubAssign<u32> for ptr<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: u32) {
    *self = *self - rhs;
  }
}

impl<T> core::ops::SubAssign<i64> for ptr<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: i64) {
    *self = *self - rhs;
  }
}

impl<T> core::ops::SubAssign<u64> for ptr<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: u64) {
    *self = *self - rhs;
  }
}

impl<T> Default for ptr<T> {
  #[inline(always)]
  fn default() -> ptr<T> {
    return ptr::null();
  }
}

impl<T> core::fmt::Pointer for ptr<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    return <*mut u8 as core::fmt::Pointer>::fmt(&self.0, f);
  }
}

impl<T> core::fmt::Debug for ptr<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    return <*mut u8 as core::fmt::Pointer>::fmt(&self.0, f);
  }
}

#[cfg(feature = "alloc")]
pub mod global {
  //! TODO

  extern crate alloc;

  use core::alloc::Layout;
  use super::ptr;

  /// Polyfill for `never_type`.

  pub enum Never {
  }

  /// Allocates memory with the global allocator.
  ///
  /// On failure, calls [`alloc::alloc::handle_alloc_error`] and does not return.
  ///
  /// # SAFETY
  ///
  /// See [alloc::alloc::GlobalAlloc::alloc].

  pub unsafe fn alloc<T>(layout: Layout) -> Result<ptr<T>, Never> {
    let x = unsafe { alloc::alloc::alloc(layout) };
    let x = ptr::from(x).cast();

    if x.is_null() {
      match alloc::alloc::handle_alloc_error(layout) {
      }
    }

    return Ok(x);
  }

  /// Allocates zero-initialized memory with the global allocator.
  ///
  /// On failure, calls [`alloc::alloc::handle_alloc_error`] and does not return.
  ///
  /// # SAFETY
  ///
  /// See [alloc::alloc::GlobalAlloc::alloc_zeroed].

  pub unsafe fn alloc_zeroed<T>(layout: Layout) -> Result<ptr<T>, Never> {
    let x = unsafe { alloc::alloc::alloc_zeroed(layout) };
    let x = ptr::from(x).cast();

    if x.is_null() {
      match alloc::alloc::handle_alloc_error(layout) {
      }
    }

    return Ok(x);
  }

  /// Deallocates memory with the global allocator.
  ///
  /// # SAFETY
  ///
  /// See [alloc::alloc::GlobalAlloc::dealloc].

  pub unsafe fn dealloc<T>(x: ptr<T>, layout: Layout) {
    unsafe { alloc::alloc::dealloc(x.cast().as_mut_ptr(), layout) };
  }

  /// Reallocates memory with the global allocator.
  ///
  /// # SAFETY
  ///
  /// See [alloc::alloc::GlobalAlloc::realloc].

  pub unsafe fn realloc<T>(x: ptr<T>, layout: Layout, new_size: usize) -> Result<ptr<T>, Never> {
    let x = unsafe { alloc::alloc::realloc(x.cast().as_mut_ptr(), layout, new_size) };
    let x = ptr::from(x).cast();

    if x.is_null() {
      match alloc::alloc::handle_alloc_error(layout) {
      }
    }

    return Ok(x);
  }
}
