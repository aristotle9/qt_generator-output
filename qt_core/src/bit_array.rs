/// C++ type: <span style='color: green;'>```QBitArray```</span>
#[repr(C)]
pub struct BitArray([u8; ::type_sizes::QT_CORE_BIT_ARRAY_BIT_ARRAY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for BitArray {
  unsafe fn new_uninitialized() -> BitArray {
    BitArray(::std::mem::uninitialized())
  }
}

impl BitArray {
  /// C++ method: <span style='color: green;'>```bool QBitArray::at(int i) const```</span>
  ///
  ///
  pub fn at(&self, i: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QBitArray_at(self as *const ::bit_array::BitArray, i) }
  }

  /// C++ method: <span style='color: green;'>```void QBitArray::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QBitArray_clear(self as *mut ::bit_array::BitArray) }
  }

  /// C++ method: <span style='color: green;'>```void QBitArray::clearBit(int i)```</span>
  ///
  ///
  pub fn clear_bit(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QBitArray_clearBit(self as *mut ::bit_array::BitArray, i) }
  }

  /// C++ method: <span style='color: green;'>```QBitArray::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QBitArray::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, bool) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QBitArray::count(bool on) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::BitArrayCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QBitArray::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, bool) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QBitArray::fill(bool val)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (bool, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QBitArray::fill(bool val, int size = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn fill(&mut self, (bool, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBitArray::fill(bool val, int first, int last)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::BitArrayFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QBitArray::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBitArray_isEmpty(self as *const ::bit_array::BitArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QBitArray::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBitArray_isNull(self as *const ::bit_array::BitArray) }
  }

  /// C++ method: <span style='color: green;'>```QBitArray::QBitArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::bit_array::BitArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitArray::QBitArray()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::bit_array::BitArray) -> ::bit_array::BitArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitArray::QBitArray(const QBitArray& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::bit_array::BitArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitArray::QBitArray(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, bool)) -> ::bit_array::BitArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitArray::QBitArray(int size, bool val = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::bit_array::BitArray
    where Args: overloading::BitArrayNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QBitArray& QBitArray::operator=(const QBitArray& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::bit_array::BitArray) -> &'l0 mut ::bit_array::BitArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QBitArray_operator_assign(self as *mut ::bit_array::BitArray,
                                                 other as *const ::bit_array::BitArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QBitArray& QBitArray::operator&=(const QBitArray& arg1)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'l0, 'l1>(&'l0 mut self,
                                     arg1: &'l1 ::bit_array::BitArray)
                                     -> &'l0 mut ::bit_array::BitArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QBitArray_operator_bit_and_assign(self as *mut ::bit_array::BitArray,
                                                         arg1 as *const ::bit_array::BitArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QBitArray QBitArray::operator~() const```</span>
  ///
  ///
  pub fn op_bit_not(&self) -> ::bit_array::BitArray {
    {
      let mut object: ::bit_array::BitArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QBitArray_operator_bit_not_to_output(self as *const ::bit_array::BitArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBitArray& QBitArray::operator|=(const QBitArray& arg1)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::bit_array::BitArray) -> &'l0 mut ::bit_array::BitArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QBitArray_operator_bit_or_assign(self as *mut ::bit_array::BitArray,
                                                        arg1 as *const ::bit_array::BitArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QBitArray& QBitArray::operator^=(const QBitArray& arg1)```</span>
  ///
  ///
  pub fn op_bit_xor_assign<'l0, 'l1>(&'l0 mut self,
                                     arg1: &'l1 ::bit_array::BitArray)
                                     -> &'l0 mut ::bit_array::BitArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QBitArray_operator_bit_xor_assign(self as *mut ::bit_array::BitArray,
                                                         arg1 as *const ::bit_array::BitArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QBitArray::operator==(const QBitArray& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::bit_array::BitArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QBitArray_operator_eq(self as *const ::bit_array::BitArray,
                                             other as *const ::bit_array::BitArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QBitArray::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index(&self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QBitArray::operator[](int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index(&self, ::libc::c_uint) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QBitArray::operator[](unsigned int i) const```</span>
  ///
  ///
  pub fn op_index<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::BitArrayOpIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QBitArray::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, ::libc::c_int) -> ::bit_ref::BitRef```<br>
  /// C++ method: <span style='color: green;'>```QBitRef QBitArray::operator[](int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, ::libc::c_uint) -> ::bit_ref::BitRef```<br>
  /// C++ method: <span style='color: green;'>```QBitRef QBitArray::operator[](unsigned int i)```</span>
  ///
  ///
  pub fn op_index_mut<'largs, Args>(&'largs mut self, args: Args) -> ::bit_ref::BitRef
    where Args: overloading::BitArrayOpIndexMutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QBitArray::operator!=(const QBitArray& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::bit_array::BitArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QBitArray_operator_neq(self as *const ::bit_array::BitArray,
                                              other as *const ::bit_array::BitArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBitArray::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QBitArray_resize(self as *mut ::bit_array::BitArray, size) }
  }

  /// C++ method: <span style='color: green;'>```QBitArray::setBit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_bit(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBitArray::setBit(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_bit(&mut self, (::libc::c_int, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBitArray::setBit(int i, bool val)```</span>
  ///
  ///
  pub fn set_bit<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BitArraySetBitArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QBitArray::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QBitArray_size(self as *const ::bit_array::BitArray) }
  }

  /// C++ method: <span style='color: green;'>```void QBitArray::swap(QBitArray& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::bit_array::BitArray) {
    unsafe {
      ::ffi::qt_core_c_QBitArray_swap(self as *mut ::bit_array::BitArray,
                                      other as *mut ::bit_array::BitArray)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QBitArray::testBit(int i) const```</span>
  ///
  ///
  pub fn test_bit(&self, i: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QBitArray_testBit(self as *const ::bit_array::BitArray, i) }
  }

  /// C++ method: <span style='color: green;'>```bool QBitArray::toggleBit(int i)```</span>
  ///
  ///
  pub fn toggle_bit(&mut self, i: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QBitArray_toggleBit(self as *mut ::bit_array::BitArray, i) }
  }

  /// C++ method: <span style='color: green;'>```void QBitArray::truncate(int pos)```</span>
  ///
  ///
  pub fn truncate(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QBitArray_truncate(self as *mut ::bit_array::BitArray, pos) }
  }
}

impl Drop for ::bit_array::BitArray {
  /// C++ method: <span style='color: green;'>```[destructor] void QBitArray::~QBitArray()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QBitArray_destructor(self as *mut ::bit_array::BitArray) }
  }
}

/// C++ method: <span style='color: green;'>```QBitArray operator&(const QBitArray& arg1, const QBitArray& arg2)```</span>
///
///
pub fn op_bit_and(arg1: &::bit_array::BitArray, arg2: &::bit_array::BitArray) -> ::bit_array::BitArray {
  {
    let mut object: ::bit_array::BitArray =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QBitArray_G_operator_bit_and_to_output(arg1 as *const ::bit_array::BitArray,
                                                              arg2 as *const ::bit_array::BitArray,
                                                              &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QBitArray operator|(const QBitArray& arg1, const QBitArray& arg2)```</span>
///
///
pub fn op_bit_or(arg1: &::bit_array::BitArray, arg2: &::bit_array::BitArray) -> ::bit_array::BitArray {
  {
    let mut object: ::bit_array::BitArray =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QBitArray_G_operator_bit_or_to_output(arg1 as *const ::bit_array::BitArray,
                                                             arg2 as *const ::bit_array::BitArray,
                                                             &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QBitArray operator^(const QBitArray& arg1, const QBitArray& arg2)```</span>
///
///
pub fn op_bit_xor(arg1: &::bit_array::BitArray, arg2: &::bit_array::BitArray) -> ::bit_array::BitArray {
  {
    let mut object: ::bit_array::BitArray =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QBitArray_G_operator_bit_xor_to_output(arg1 as *const ::bit_array::BitArray,
                                                              arg2 as *const ::bit_array::BitArray,
                                                              &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QBitArray& arg2)```</span>
///
///
pub fn op_shl(arg1: &::debug::Debug, arg2: &::bit_array::BitArray) -> ::debug::Debug {
  {
    let mut object: ::debug::Debug = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QBitArray_G_operator_shl_to_output(arg1 as *const ::debug::Debug,
                                                          arg2 as *const ::bit_array::BitArray,
                                                          &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QBitArray& value1, QBitArray& value2)```</span>
///
///
pub fn swap(value1: &mut ::bit_array::BitArray, value2: &mut ::bit_array::BitArray) {
  unsafe {
    ::ffi::qt_core_c_QBitArray_G_swap(value1 as *mut ::bit_array::BitArray,
                                      value2 as *mut ::bit_array::BitArray)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [BitArray::count](../struct.BitArray.html#method.count) method.
  pub trait BitArrayCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::bit_array::BitArray) -> ::libc::c_int;
  }
  impl<'largs> BitArrayCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::bit_array::BitArray) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QBitArray_count_no_args(original_self as *const ::bit_array::BitArray) }
    }
  }
  impl<'largs> BitArrayCountArgs<'largs> for bool {
    fn exec(self, original_self: &'largs ::bit_array::BitArray) -> ::libc::c_int {
      let on = self;
      unsafe { ::ffi::qt_core_c_QBitArray_count_on(original_self as *const ::bit_array::BitArray, on) }
    }
  }
  /// This trait represents a set of arguments accepted by [BitArray::fill](../struct.BitArray.html#method.fill) method.
  pub trait BitArrayFillArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> Self::ReturnType;
  }
  impl<'largs> BitArrayFillArgs<'largs> for bool {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> bool {
      let val = self;
      unsafe { ::ffi::qt_core_c_QBitArray_fill_val(original_self as *mut ::bit_array::BitArray, val) }
    }
  }
  impl<'largs> BitArrayFillArgs<'largs> for (bool, ::libc::c_int, ::libc::c_int) {
    type ReturnType = ();
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> () {
      let val = self.0;
      let first = self.1;
      let last = self.2;
      unsafe {
        ::ffi::qt_core_c_QBitArray_fill_val_first_last(original_self as *mut ::bit_array::BitArray,
                                                       val,
                                                       first,
                                                       last)
      }
    }
  }
  impl<'largs> BitArrayFillArgs<'largs> for (bool, ::libc::c_int) {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> bool {
      let val = self.0;
      let size = self.1;
      unsafe { ::ffi::qt_core_c_QBitArray_fill_val_size(original_self as *mut ::bit_array::BitArray, val, size) }
    }
  }
  /// This trait represents a set of arguments accepted by [BitArray::new](../struct.BitArray.html#method.new) method.
  pub trait BitArrayNewArgs {
    fn exec(self) -> ::bit_array::BitArray;
  }
  impl BitArrayNewArgs for () {
    fn exec(self) -> ::bit_array::BitArray {

      {
        let mut object: ::bit_array::BitArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QBitArray_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> BitArrayNewArgs for &'a ::bit_array::BitArray {
    fn exec(self) -> ::bit_array::BitArray {
      let other = self;
      {
        let mut object: ::bit_array::BitArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QBitArray_constructor_other(other as *const ::bit_array::BitArray, &mut object);
        }
        object
      }
    }
  }
  impl BitArrayNewArgs for ::libc::c_int {
    fn exec(self) -> ::bit_array::BitArray {
      let size = self;
      {
        let mut object: ::bit_array::BitArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QBitArray_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl BitArrayNewArgs for (::libc::c_int, bool) {
    fn exec(self) -> ::bit_array::BitArray {
      let size = self.0;
      let val = self.1;
      {
        let mut object: ::bit_array::BitArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QBitArray_constructor_size_val(size, val, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [BitArray::op_index](../struct.BitArray.html#method.op_index) method.
  pub trait BitArrayOpIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::bit_array::BitArray) -> bool;
  }
  impl<'largs> BitArrayOpIndexArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::bit_array::BitArray) -> bool {
      let i = self;
      unsafe { ::ffi::qt_core_c_QBitArray_operator_index_int(original_self as *const ::bit_array::BitArray, i) }
    }
  }
  impl<'largs> BitArrayOpIndexArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs ::bit_array::BitArray) -> bool {
      let i = self;
      unsafe {
        ::ffi::qt_core_c_QBitArray_operator_index_unsigned_int(original_self as *const ::bit_array::BitArray, i)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [BitArray::op_index_mut](../struct.BitArray.html#method.op_index_mut) method.
  pub trait BitArrayOpIndexMutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> ::bit_ref::BitRef;
  }
  impl<'largs> BitArrayOpIndexMutArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> ::bit_ref::BitRef {
      let i = self;
      {
        let mut object: ::bit_ref::BitRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QBitArray_operator_index_to_output_int(original_self as *mut ::bit_array::BitArray,
                                                                  i,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> BitArrayOpIndexMutArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> ::bit_ref::BitRef {
      let i = self;
      {
        let mut object: ::bit_ref::BitRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QBitArray_operator_index_to_output_unsigned_int(original_self as *mut ::bit_array::BitArray, i, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [BitArray::set_bit](../struct.BitArray.html#method.set_bit) method.
  pub trait BitArraySetBitArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> ();
  }
  impl<'largs> BitArraySetBitArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QBitArray_setBit_i(original_self as *mut ::bit_array::BitArray, i) }
    }
  }
  impl<'largs> BitArraySetBitArgs<'largs> for (::libc::c_int, bool) {
    fn exec(self, original_self: &'largs mut ::bit_array::BitArray) -> () {
      let i = self.0;
      let val = self.1;
      unsafe { ::ffi::qt_core_c_QBitArray_setBit_i_val(original_self as *mut ::bit_array::BitArray, i, val) }
    }
  }
}
