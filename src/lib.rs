#![doc = include_str!("../README.md")]
#![no_std]

#[derive(
  Clone,
  Copy,
  Eq,
  Hash,
  Ord,
  PartialEq,
  PartialOrd,
)]
#[repr(transparent)]
pub struct ptr(*mut u8);

unsafe impl Send for ptr { }

unsafe impl Sync for ptr { }

impl ptr {
  /// Creates a pointer with the given address and no provenance.

  #[inline(always)]
  pub const fn invalid(addr: usize) -> ptr {
    // Once the `strict_provenance` feature has been stabilized, this should
    // use the `core::ptr::invalid` function.

    ptr(unsafe { core::mem::transmute::<usize, *mut u8>(addr) })
  }

  /// An invalid pointer with address zero.

  pub const NULL: ptr = ptr::invalid(0);

  /// Gets the address of the pointer.

  #[inline(always)]
  pub fn addr(x: impl Into<ptr>) -> usize {
    // NB: This must not be a `const` function.
    //
    // Transmuting a pointer into an integer in a const context is undefined
    // behavior.

    // Once the `strict_provenance` feature has been stabilized, this should
    // use the `addr` method on the primitive pointer type.

    let x = x.into();
    unsafe { core::mem::transmute::<*mut u8, usize>(x.0) }
  }

  /// Whether the pointer has address zero.

  #[inline(always)]
  pub fn is_null(x: impl Into<ptr>) -> bool {
    ptr::addr(x) == 0
  }

  #[inline(always)]
  pub const fn from_const_ptr<T: ?Sized>(x: *const T) -> ptr {
    ptr(x as *mut u8)
  }

  #[inline(always)]
  pub const fn from_mut_ptr<T: ?Sized>(x: *mut T) -> ptr {
    ptr(x as *mut u8)
  }

  #[inline(always)]
  pub const fn from_ref<T: ?Sized>(x: &T) -> ptr {
    ptr::from_const_ptr(x)
  }

  #[inline(always)]
  pub fn from_mut_ref<T: ?Sized>(x: &mut T) -> ptr {
    ptr::from_mut_ptr(x)
  }

  #[inline(always)]
  pub const fn from_non_null<T: ?Sized>(x: core::ptr::NonNull<T>) -> ptr {
    ptr::from_mut_ptr(x.as_ptr())
  }

  /// Adds the given byte offset to the pointer's address with wrapping
  /// arithmetic.

  #[inline(always)]
  pub fn offset(x: impl Into<ptr>, n: isize) -> ptr {
    let x = x.into();
    ptr(x.0.wrapping_offset(n))
  }

  /// Adds the given byte offset to the pointer's address with wrapping
  /// arithmetic.

  #[inline(always)]
  pub fn add(x: impl Into<ptr>, n: usize) -> ptr {
    let x = x.into();
    ptr(x.0.wrapping_add(n))
  }

  /// Subtracts the given byte offset from the pointer's address with wrapping
  /// arithmetic.

  #[inline(always)]
  pub fn sub(x: impl Into<ptr>, n: usize) -> ptr {
    let x = x.into();
    ptr(x.0.wrapping_sub(n))
  }

  /// Computes the difference in bytes between the two pointers' addresses with
  /// wrapping arithmetic.

  #[inline(always)]
  pub fn diff(x: impl Into<ptr>, other: impl Into<ptr>) -> usize {
    ptr::addr(x).wrapping_sub(ptr::addr(other))
  }

  /// Updates the pointer's address by bitwise and-ing it with the given mask.

  #[inline(always)]
  pub fn mask(x: impl Into<ptr>, mask: usize) -> ptr {
    let x = x.into();
    ptr(x.0.wrapping_sub(ptr::addr(x) & ! mask))
  }

  #[inline(always)]
  pub fn gep<T>(x: impl Into<ptr>, index: isize) -> ptr {
    ptr::offset(x, (core::mem::size_of::<T>() as isize).wrapping_mul(index))
  }

  /// # Safety:
  ///
  /// See `core::ptr::read`.

  #[inline(always)]
  pub unsafe fn read<T>(x: impl Into<ptr>) -> T {
    let x = ptr::as_const_ptr::<T>(x);
    unsafe { core::ptr::read(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::read_unaligned`.

  #[inline(always)]
  pub unsafe fn read_unaligned<T>(x: impl Into<ptr>) -> T {
    let x = ptr::as_const_ptr::<T>(x);
    unsafe { core::ptr::read_unaligned(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::read_volatile`.

  #[inline(always)]
  pub unsafe fn read_volatile<T>(x: impl Into<ptr>) -> T {
    let x = ptr::as_const_ptr::<T>(x);
    unsafe { core::ptr::read_volatile(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::write`.

  #[inline(always)]
  pub unsafe fn write<T>(x: impl Into<ptr>, value: T) {
    let x = ptr::as_mut_ptr::<T>(x);
    unsafe { core::ptr::write(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::write_unaligned`.

  #[inline(always)]
  pub unsafe fn write_unaligned<T>(x: impl Into<ptr>, value: T) {
    let x = ptr::as_mut_ptr::<T>(x);
    unsafe { core::ptr::write_unaligned(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::write_volatile`.

  #[inline(always)]
  pub unsafe fn write_volatile<T>(x: impl Into<ptr>, value: T) {
    let x = ptr::as_mut_ptr::<T>(x);
    unsafe { core::ptr::write_volatile(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::replace`.

  #[inline(always)]
  pub unsafe fn replace<T>(x: impl Into<ptr>, value: T) -> T {
    let x = ptr::as_mut_ptr::<T>(x);
    unsafe { core::ptr::replace(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::drop_in_place`.

  #[inline(always)]
  pub unsafe fn drop_in_place<T>(x: impl Into<ptr>) {
    let x = ptr::as_mut_ptr::<T>(x);
    unsafe { core::ptr::drop_in_place(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::copy_nonoverlapping`.

  #[inline(always)]
  pub unsafe fn copy_nonoverlapping<T>(src: impl Into<ptr>, dst: impl Into<ptr>, count: usize) {
    let src = ptr::as_const_ptr(src);
    let dst = ptr::as_mut_ptr(dst);
    unsafe { core::ptr::copy_nonoverlapping::<T>(src, dst, count) };
  }

  /// # Safety:
  ///
  /// See `core::ptr::swap_nonoverlapping`.

  #[inline(always)]
  pub unsafe fn swap_nonoverlapping<T>(x: impl Into<ptr>, y: impl Into<ptr>, count: usize) {
    let x = ptr::as_mut_ptr(x);
    let y = ptr::as_mut_ptr(y);
    unsafe { core::ptr::swap_nonoverlapping::<T>(x, y, count) };
  }

  #[inline(always)]
  pub fn as_const_ptr<T>(x: impl Into<ptr>) -> *const T {
    let x = x.into();
    x.0 as *const T
  }

  #[inline(always)]
  pub fn as_mut_ptr<T>(x: impl Into<ptr>) -> *mut T {
    let x = x.into();
    x.0 as *mut T
  }

  #[inline(always)]
  pub fn as_slice_const_ptr<T>(x: impl Into<ptr>, len: usize) -> *const [T] {
    core::ptr::slice_from_raw_parts(ptr::as_const_ptr(x), len)
  }

  #[inline(always)]
  pub fn as_slice_mut_ptr<T>(x: impl Into<ptr>, len: usize) -> *mut [T] {
    core::ptr::slice_from_raw_parts_mut(ptr::as_mut_ptr(x), len)
  }

  #[inline(always)]
  pub unsafe fn as_ref<'a, T>(x: impl Into<ptr>) -> &'a T {
    let x = ptr::as_const_ptr(x);
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_mut_ref<'a, T>(x: impl Into<ptr>) -> &'a mut T {
    let x = ptr::as_mut_ptr(x);
    unsafe { &mut *x }
  }

  #[inline(always)]
  pub unsafe fn as_slice_ref<'a, T>(x: impl Into<ptr>, len: usize) -> &'a [T] {
    let x = ptr::as_slice_const_ptr(x, len);
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_slice_mut_ref<'a, T>(x: impl Into<ptr>, len: usize) -> &'a mut [T] {
    let x = ptr::as_slice_mut_ptr(x, len);
    unsafe { &mut *x }
  }

  /// # Safety:
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub unsafe fn as_non_null<T>(x: impl Into<ptr>) -> core::ptr::NonNull<T> {
    unsafe { core::ptr::NonNull::new_unchecked(ptr::as_mut_ptr(x)) }
  }
}

impl<T: ?Sized> From<*const T> for ptr {
  #[inline(always)]
  fn from(value: *const T) -> ptr {
    ptr::from_const_ptr(value)
  }
}

impl<T: ?Sized> From<*mut T> for ptr {
  #[inline(always)]
  fn from(value: *mut T) -> ptr {
    ptr::from_mut_ptr(value)
  }
}

impl<T: ?Sized> From<&T> for ptr {
  #[inline(always)]
  fn from(value: &T) -> ptr {
    ptr::from_ref(value)
  }
}

impl<T: ?Sized> From<&mut T> for ptr {
  #[inline(always)]
  fn from(value: &mut T) -> ptr {
    ptr::from_mut_ref(value)
  }
}

impl<T: ?Sized> From<core::ptr::NonNull<T>> for ptr {
  #[inline(always)]
  fn from(value: core::ptr::NonNull<T>) -> ptr {
    ptr::from_non_null(value)
  }
}

impl<T> From<ptr> for *const T {
  #[inline(always)]
  fn from(value: ptr) -> *const T {
    ptr::as_const_ptr(value)
  }
}

impl<T> From<ptr> for *mut T {
  #[inline(always)]
  fn from(value: ptr) -> *mut T {
    ptr::as_mut_ptr(value)
  }
}

impl core::ops::Add<isize> for ptr {
  type Output = ptr;

  #[inline(always)]
  fn add(self, rhs: isize) -> ptr {
    ptr::offset(self, rhs)
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
    ptr::add(self, rhs)
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
    ptr::offset(self, rhs.wrapping_neg())
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
    ptr::sub(self, rhs)
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
    ptr::diff(self, rhs)
  }
}

impl core::ops::BitAnd<usize> for ptr {
  type Output = ptr;

  #[inline(always)]
  fn bitand(self, rhs: usize) -> ptr {
    ptr::mask(self, rhs)
  }
}

impl core::ops::BitAndAssign<usize> for ptr {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: usize) {
    *self = *self & rhs;
  }
}

impl core::fmt::Debug for ptr {
  fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(out, "0x{:01$x}", ptr::addr(*self), (usize::BITS / 4) as usize)
  }
}
