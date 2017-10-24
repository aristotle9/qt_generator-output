/// C++ type: <span style='color: green;'>```QByteRef```</span>
#[repr(C)]
pub struct ByteRef([u8; ::type_sizes::QT_CORE_BYTE_REF_BYTE_REF]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ByteRef {
  unsafe fn new_uninitialized() -> ByteRef {
    ByteRef(::std::mem::uninitialized())
  }
}

impl ByteRef {
  /// C++ method: <span style='color: green;'>```char QByteRef::operator char() const```</span>
  ///
  ///
  pub fn as_char(&self) -> ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteRef_convert_to_char(self as *const ::byte_ref::ByteRef) }
  }

  /// C++ method: <span style='color: green;'>```QByteRef::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, ::libc::c_char) -> &'l0 mut ::byte_ref::ByteRef```<br>
  /// C++ method: <span style='color: green;'>```QByteRef& QByteRef::operator=(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::byte_ref::ByteRef) -> &'l0 mut ::byte_ref::ByteRef```<br>
  /// C++ method: <span style='color: green;'>```QByteRef& QByteRef::operator=(const QByteRef& c)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_ref::ByteRef
    where Args: overloading::ByteRefOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QByteRef::operator==(char c) const```</span>
  ///
  ///
  pub fn op_eq(&self, c: ::libc::c_char) -> bool {
    unsafe { ::ffi::qt_core_c_QByteRef_operator_eq(self as *const ::byte_ref::ByteRef, c) }
  }

  /// C++ method: <span style='color: green;'>```bool QByteRef::operator>=(char c) const```</span>
  ///
  ///
  pub fn op_ge(&self, c: ::libc::c_char) -> bool {
    unsafe { ::ffi::qt_core_c_QByteRef_operator_ge(self as *const ::byte_ref::ByteRef, c) }
  }

  /// C++ method: <span style='color: green;'>```bool QByteRef::operator>(char c) const```</span>
  ///
  ///
  pub fn op_gt(&self, c: ::libc::c_char) -> bool {
    unsafe { ::ffi::qt_core_c_QByteRef_operator_gt(self as *const ::byte_ref::ByteRef, c) }
  }

  /// C++ method: <span style='color: green;'>```bool QByteRef::operator<=(char c) const```</span>
  ///
  ///
  pub fn op_le(&self, c: ::libc::c_char) -> bool {
    unsafe { ::ffi::qt_core_c_QByteRef_operator_le(self as *const ::byte_ref::ByteRef, c) }
  }

  /// C++ method: <span style='color: green;'>```bool QByteRef::operator<(char c) const```</span>
  ///
  ///
  pub fn op_lt(&self, c: ::libc::c_char) -> bool {
    unsafe { ::ffi::qt_core_c_QByteRef_operator_lt(self as *const ::byte_ref::ByteRef, c) }
  }

  /// C++ method: <span style='color: green;'>```bool QByteRef::operator!=(char c) const```</span>
  ///
  ///
  pub fn op_neq(&self, c: ::libc::c_char) -> bool {
    unsafe { ::ffi::qt_core_c_QByteRef_operator_neq(self as *const ::byte_ref::ByteRef, c) }
  }
}

impl Drop for ::byte_ref::ByteRef {
  /// C++ method: <span style='color: green;'>```[destructor] void QByteRef::~QByteRef()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QByteRef_destructor(self as *mut ::byte_ref::ByteRef) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ByteRef::op_assign](../struct.ByteRef.html#method.op_assign) method.
  pub trait ByteRefOpAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_ref::ByteRef) -> &'largs mut ::byte_ref::ByteRef;
  }
  impl<'largs> ByteRefOpAssignArgs<'largs> for &'largs ::byte_ref::ByteRef {
    fn exec(self, original_self: &'largs mut ::byte_ref::ByteRef) -> &'largs mut ::byte_ref::ByteRef {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteRef_operator_assign_QByteRef(original_self as *mut ::byte_ref::ByteRef,
                                                           c as *const ::byte_ref::ByteRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteRefOpAssignArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::byte_ref::ByteRef) -> &'largs mut ::byte_ref::ByteRef {
      let c = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteRef_operator_assign_char(original_self as *mut ::byte_ref::ByteRef, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
