/// C++ type: <span style='color: green;'>```QBitRef```</span>
#[repr(C)]
pub struct BitRef([u8; ::type_sizes::QT_CORE_BIT_REF_BIT_REF]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for BitRef {
  unsafe fn new_uninitialized() -> BitRef {
    BitRef(::std::mem::uninitialized())
  }
}

impl BitRef {
  /// C++ method: <span style='color: green;'>```bool QBitRef::operator bool() const```</span>
  ///
  ///
  pub fn as_bool(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBitRef_convert_to_bool(self as *const ::bit_ref::BitRef) }
  }

  /// C++ method: <span style='color: green;'>```QBitRef::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, bool) -> &'l0 mut ::bit_ref::BitRef```<br>
  /// C++ method: <span style='color: green;'>```QBitRef& QBitRef::operator=(bool val)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::bit_ref::BitRef) -> &'l0 mut ::bit_ref::BitRef```<br>
  /// C++ method: <span style='color: green;'>```QBitRef& QBitRef::operator=(const QBitRef& val)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::bit_ref::BitRef
    where Args: overloading::BitRefOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QBitRef::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBitRef_operator_not(self as *const ::bit_ref::BitRef) }
  }
}

impl Drop for ::bit_ref::BitRef {
  /// C++ method: <span style='color: green;'>```[destructor] void QBitRef::~QBitRef()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QBitRef_destructor(self as *mut ::bit_ref::BitRef) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [BitRef::op_assign](../struct.BitRef.html#method.op_assign) method.
  pub trait BitRefOpAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::bit_ref::BitRef) -> &'largs mut ::bit_ref::BitRef;
  }
  impl<'largs> BitRefOpAssignArgs<'largs> for &'largs ::bit_ref::BitRef {
    fn exec(self, original_self: &'largs mut ::bit_ref::BitRef) -> &'largs mut ::bit_ref::BitRef {
      let val = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QBitRef_operator_assign_QBitRef(original_self as *mut ::bit_ref::BitRef,
                                                         val as *const ::bit_ref::BitRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> BitRefOpAssignArgs<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::bit_ref::BitRef) -> &'largs mut ::bit_ref::BitRef {
      let val = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QBitRef_operator_assign_bool(original_self as *mut ::bit_ref::BitRef, val) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
