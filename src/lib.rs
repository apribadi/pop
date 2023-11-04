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
pub struct Ptr(*const u8);

unsafe impl Send for Ptr { }

unsafe impl Sync for Ptr { }

#[inline(always)]
const fn size_of<T>() -> isize {
  core::mem::size_of::<T>() as isize
}

#[inline(always)]
const fn offset_of_element_at_index<T>(index: isize) -> isize {
  size_of::<T>().wrapping_mul(index)
}

impl Ptr {
  /// Creates a pointer with the given address and no provenance.

  #[inline(always)]
  pub const fn invalid(addr: usize) -> Self {
    // Once the `strict_provenance` feature has been stabilized, this should
    // use the `core::ptr::invalid` function.

    Self(unsafe { core::mem::transmute::<usize, *const u8>(addr) })
  }

  /// An invalid pointer with address zero.

  pub const NULL: Self = Self::invalid(0);

  /// Gets the address of the pointer.

  #[inline(always)]
  pub fn addr(self) -> usize {
    // NB: This must not be a `const` function.
    //
    // Transmuting a pointer into an integer in a const context is undefined
    // behavior.

    // Once the `strict_provenance` feature has been stabilized, this should
    // use the `addr` method on the primitive pointer type.

    unsafe { core::mem::transmute::<*const u8, usize>(self.0) }
  }

  /// Whether the pointer has address zero.

  #[inline(always)]
  pub fn is_null(self) -> bool {
    self.addr() == 0
  }

  #[inline(always)]
  pub const fn from_const_ptr<T: ?Sized>(x: *const T) -> Self {
    Self(x as *const u8)
  }

  #[inline(always)]
  pub const fn from_mut_ptr<T: ?Sized>(x: *mut T) -> Self {
    Self(x as *const u8)
  }

  #[inline(always)]
  pub const fn from_ref<T: ?Sized>(x: &T) -> Self {
    Self::from_const_ptr(x)
  }

  #[inline(always)]
  pub fn from_mut_ref<T: ?Sized>(x: &mut T) -> Self {
    Self::from_mut_ptr(x)
  }

  #[inline(always)]
  pub const fn from_non_null<T: ?Sized>(x: core::ptr::NonNull<T>) -> Self {
    Self::from_mut_ptr(x.as_ptr())
  }

  /// Adds the given byte offset to the pointer's address with wrapping
  /// arithmetic.

  #[inline(always)]
  pub const fn add(self, offset: isize) -> Self {
    Self(self.0.wrapping_offset(offset))
  }

  /// Subtracts the given byte offset from the pointer's address with wrapping
  /// arithmetic.

  #[inline(always)]
  pub const fn sub(self, offset: isize) -> Self {
    Self(self.0.wrapping_offset(offset.wrapping_neg()))
  }

  /// Computes the difference in bytes between the two pointers' addresses with
  /// wrapping arithmetic.

  #[inline(always)]
  pub fn diff(self, offset: Self) -> isize {
    self.addr().wrapping_sub(offset.addr()) as isize
  }

  /// Updates the pointer's address by bitwise and-ing it with the given mask.

  #[inline(always)]
  pub fn mask(self, mask: usize) -> Self {
    Self(self.0.wrapping_sub(self.addr() & ! mask))
  }

  #[inline(always)]
  pub const fn gep<T>(self, index: isize) -> Self {
    self.add(offset_of_element_at_index::<T>(index))
  }

  /// # Safety:
  ///
  /// See `core::ptr::read`.

  #[inline(always)]
  pub unsafe fn read<T>(self) -> T {
    let x = self.as_const_ptr::<T>();
    unsafe { core::ptr::read(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::read_unaligned`.

  #[inline(always)]
  pub unsafe fn read_unaligned<T>(self) -> T {
    let x = self.as_const_ptr::<T>();
    unsafe { core::ptr::read_unaligned(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::read_volatile`.

  #[inline(always)]
  pub unsafe fn read_volatile<T>(self) -> T {
    let x = self.as_const_ptr::<T>();
    unsafe { core::ptr::read_volatile(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::write`.

  #[inline(always)]
  pub unsafe fn write<T>(self, value: T) {
    let x = self.as_mut_ptr::<T>();
    unsafe { core::ptr::write(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::write_unaligned`.

  #[inline(always)]
  pub unsafe fn write_unaligned<T>(self, value: T) {
    let x = self.as_mut_ptr::<T>();
    unsafe { core::ptr::write_unaligned(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::write_volatile`.

  #[inline(always)]
  pub unsafe fn write_volatile<T>(self, value: T) {
    let x = self.as_mut_ptr::<T>();
    unsafe { core::ptr::write_volatile(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::replace`.

  #[inline(always)]
  pub unsafe fn replace<T>(self, value: T) -> T {
    let x = self.as_mut_ptr::<T>();
    unsafe { core::ptr::replace(x, value) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::drop_in_place`.

  #[inline(always)]
  pub unsafe fn drop_in_place<T>(self) {
    let x = self.as_mut_ptr::<T>();
    unsafe { core::ptr::drop_in_place(x) }
  }

  /// # Safety:
  ///
  /// See `core::ptr::copy_nonoverlapping`.

  #[inline(always)]
  pub unsafe fn copy_nonoverlapping<T>(src: Self, dst: Self, count: usize) {
    let src = src.as_const_ptr();
    let dst = dst.as_mut_ptr();
    unsafe { core::ptr::copy_nonoverlapping::<T>(src, dst, count) };
  }

  /// # Safety:
  ///
  /// See `core::ptr::swap_nonoverlapping`.

  #[inline(always)]
  pub unsafe fn swap_nonoverlapping<T>(x: Self, y: Self, count: usize) {
    let x = x.as_mut_ptr();
    let y = y.as_mut_ptr();
    unsafe { core::ptr::swap_nonoverlapping::<T>(x, y, count) };
  }

  #[inline(always)]
  pub const fn as_const_ptr<T>(self) -> *const T {
    self.0 as *const T
  }

  #[inline(always)]
  pub const fn as_mut_ptr<T>(self) -> *mut T {
    self.0 as *mut T
  }

  #[inline(always)]
  pub const fn as_slice_const_ptr<T>(self, len: usize) -> *const [T] {
    core::ptr::slice_from_raw_parts(self.as_const_ptr(), len)
  }

  #[inline(always)]
  pub const fn as_slice_mut_ptr<T>(self, len: usize) -> *mut [T] {
    self.as_slice_const_ptr::<T>(len) as *mut [T]
  }

  #[inline(always)]
  pub const unsafe fn as_ref<'a, T>(self) -> &'a T {
    let x = self.as_const_ptr();
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_mut_ref<'a, T>(self) -> &'a mut T {
    let x = self.as_mut_ptr();
    unsafe { &mut *x }
  }

  #[inline(always)]
  pub const unsafe fn as_slice_ref<'a, T>(self, len: usize) -> &'a [T] {
    let x = self.as_slice_const_ptr(len);
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_slice_mut_ref<'a, T>(self, len: usize) -> &'a mut [T] {
    let x = self.as_slice_mut_ptr(len);
    unsafe { &mut *x }
  }

  /// # Safety:
  ///
  /// The pointer must not have address zero.

  #[inline(always)]
  pub const unsafe fn as_non_null<T>(self) -> core::ptr::NonNull<T> {
    unsafe { core::ptr::NonNull::new_unchecked(self.as_mut_ptr()) }
  }

  #[inline(always)]
  pub fn as_option_non_null<T>(self) -> Option<core::ptr::NonNull<T>> {
    core::ptr::NonNull::new(self.as_mut_ptr())
  }
}

impl<T: ?Sized> From<*const T> for Ptr {
  #[inline(always)]
  fn from(value: *const T) -> Self {
    Self::from_const_ptr(value)
  }
}

impl<T: ?Sized> From<*mut T> for Ptr {
  #[inline(always)]
  fn from(value: *mut T) -> Self {
    Self::from_mut_ptr(value)
  }
}

impl<T: ?Sized> From<&T> for Ptr {
  #[inline(always)]
  fn from(value: &T) -> Self {
    Self::from_ref(value)
  }
}

impl<T: ?Sized> From<&mut T> for Ptr {
  #[inline(always)]
  fn from(value: &mut T) -> Self {
    Self::from_mut_ref(value)
  }
}

impl<T: ?Sized> From<core::ptr::NonNull<T>> for Ptr {
  #[inline(always)]
  fn from(value: core::ptr::NonNull<T>) -> Self {
    Self::from_non_null(value)
  }
}

impl<T> From<Ptr> for *const T {
  #[inline(always)]
  fn from(value: Ptr) -> *const T {
    value.as_const_ptr()
  }
}

impl<T> From<Ptr> for *mut T {
  #[inline(always)]
  fn from(value: Ptr) -> *mut T {
    value.as_mut_ptr()
  }
}

impl core::ops::Add<isize> for Ptr {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: isize) -> Self::Output {
    self.add(rhs)
  }
}

impl core::ops::AddAssign<isize> for Ptr {
  #[inline(always)]
  fn add_assign(&mut self, rhs: isize) {
    *self = *self + rhs;
  }
}

impl core::ops::Sub<isize> for Ptr {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: isize) -> Self::Output {
    self.sub(rhs)
  }
}

impl core::ops::SubAssign<isize> for Ptr {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: isize) {
    *self = *self - rhs;
  }
}

impl core::ops::Sub<Ptr> for Ptr {
  type Output = isize;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.diff(rhs)
  }
}

impl core::ops::BitAnd<usize> for Ptr {
  type Output = Self;

  #[inline(always)]
  fn bitand(self, rhs: usize) -> Self::Output {
    self.mask(rhs)
  }
}

impl core::ops::BitAndAssign<usize> for Ptr {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: usize) {
    *self = *self & rhs;
  }
}

impl core::fmt::Debug for Ptr {
  fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(out, "0x{:01$x}", self.addr(), (usize::BITS / 4) as usize)
  }
}
