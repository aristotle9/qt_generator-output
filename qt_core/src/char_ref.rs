/// C++ type: <span style='color: green;'>```QCharRef```</span>
#[repr(C)]
pub struct CharRef([u8; ::type_sizes::QT_CORE_CHAR_REF_CHAR_REF]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for CharRef {
  unsafe fn new_uninitialized() -> CharRef {
    CharRef(::std::mem::uninitialized())
  }
}

impl CharRef {
  /// C++ method: <span style='color: green;'>```QChar QCharRef::operator QChar() const```</span>
  ///
  ///
  pub fn as_q_char(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCharRef_convert_to_QChar_to_output(self as *const ::char_ref::CharRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QCharRef::cell() const```</span>
  ///
  ///
  pub fn cell(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QCharRef_cell(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QCharRef::combiningClass() const```</span>
  ///
  ///
  pub fn combining_class(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QCharRef_combiningClass(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```QString QCharRef::decomposition() const```</span>
  ///
  ///
  pub fn decomposition(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCharRef_decomposition_to_output(self as *const ::char_ref::CharRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QCharRef::digitValue() const```</span>
  ///
  ///
  pub fn digit_value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QCharRef_digitValue(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::hasMirrored() const```</span>
  ///
  ///
  pub fn has_mirrored(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_hasMirrored(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isDigit() const```</span>
  ///
  ///
  pub fn is_digit(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isDigit(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isLetter() const```</span>
  ///
  ///
  pub fn is_letter(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isLetter(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isLetterOrNumber()```</span>
  ///
  ///
  pub fn is_letter_or_number(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isLetterOrNumber(self as *mut ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isLower() const```</span>
  ///
  ///
  pub fn is_lower(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isLower(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isMark() const```</span>
  ///
  ///
  pub fn is_mark(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isMark(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isNull(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isNumber() const```</span>
  ///
  ///
  pub fn is_number(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isNumber(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isPrint() const```</span>
  ///
  ///
  pub fn is_print(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isPrint(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isPunct() const```</span>
  ///
  ///
  pub fn is_punct(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isPunct(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isSpace() const```</span>
  ///
  ///
  pub fn is_space(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isSpace(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isTitleCase() const```</span>
  ///
  ///
  pub fn is_title_case(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isTitleCase(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QCharRef::isUpper() const```</span>
  ///
  ///
  pub fn is_upper(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCharRef_isUpper(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```QChar QCharRef::mirroredChar() const```</span>
  ///
  ///
  pub fn mirrored_char(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCharRef_mirroredChar_to_output(self as *const ::char_ref::CharRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCharRef::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign0(&mut self, &'l1 ::char::Char) -> &'l0 mut ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign0(&mut self, ::libc::c_char) -> &'l0 mut ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_assign0(&mut self, &'l1 ::char_ref::CharRef) -> &'l0 mut ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(const QCharRef& c)```</span>
  ///
  ///
  pub fn op_assign0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::char_ref::CharRef
    where Args: overloading::CharRefOpAssign0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCharRef::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign1(&mut self, ::libc::c_int) -> &'l0 mut ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(int rc)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign1(&mut self, ::libc::c_uchar) -> &'l0 mut ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(unsigned char c)```</span>
  ///
  ///
  pub fn op_assign1<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::char_ref::CharRef
    where Args: overloading::CharRefOpAssign1Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCharRef::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign2(&mut self, ::libc::c_short) -> &'l0 mut ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(short rc)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign2(&mut self, ::libc::c_uint) -> &'l0 mut ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(unsigned int rc)```</span>
  ///
  ///
  pub fn op_assign2<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::char_ref::CharRef
    where Args: overloading::CharRefOpAssign2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCharRef& QCharRef::operator=(unsigned short rc)```</span>
  ///
  ///
  pub fn op_assign3<'l0>(&'l0 mut self, rc: ::libc::c_ushort) -> &'l0 mut ::char_ref::CharRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QCharRef_operator_assign_unsigned_short(self as *mut ::char_ref::CharRef, rc) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```unsigned char QCharRef::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QCharRef_row(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```void QCharRef::setCell(unsigned char cell)```</span>
  ///
  ///
  pub fn set_cell(&mut self, cell: ::libc::c_uchar) {
    unsafe { ::ffi::qt_core_c_QCharRef_setCell(self as *mut ::char_ref::CharRef, cell) }
  }

  /// C++ method: <span style='color: green;'>```void QCharRef::setRow(unsigned char row)```</span>
  ///
  ///
  pub fn set_row(&mut self, row: ::libc::c_uchar) {
    unsafe { ::ffi::qt_core_c_QCharRef_setRow(self as *mut ::char_ref::CharRef, row) }
  }

  /// C++ method: <span style='color: green;'>```char QCharRef::toLatin1() const```</span>
  ///
  ///
  pub fn to_latin1(&self) -> ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QCharRef_toLatin1(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```QChar QCharRef::toLower() const```</span>
  ///
  ///
  pub fn to_lower(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCharRef_toLower_to_output(self as *const ::char_ref::CharRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QChar QCharRef::toTitleCase() const```</span>
  ///
  ///
  pub fn to_title_case(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCharRef_toTitleCase_to_output(self as *const ::char_ref::CharRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QChar QCharRef::toUpper() const```</span>
  ///
  ///
  pub fn to_upper(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCharRef_toUpper_to_output(self as *const ::char_ref::CharRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned short QCharRef::unicode() const```</span>
  ///
  ///
  pub fn unicode(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QCharRef_unicode_const(self as *const ::char_ref::CharRef) }
  }

  /// C++ method: <span style='color: green;'>```unsigned short& QCharRef::unicode()```</span>
  ///
  ///
  pub fn unicode_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_ushort {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCharRef_unicode(self as *mut ::char_ref::CharRef) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::char_ref::CharRef {
  /// C++ method: <span style='color: green;'>```[destructor] void QCharRef::~QCharRef()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QCharRef_destructor(self as *mut ::char_ref::CharRef) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CharRef::op_assign0](../struct.CharRef.html#method.op_assign0) method.
  pub trait CharRefOpAssign0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef;
  }
  impl<'largs> CharRefOpAssign0Args<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QCharRef_operator_assign_QChar(original_self as *mut ::char_ref::CharRef,
                                                        c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> CharRefOpAssign0Args<'largs> for &'largs ::char_ref::CharRef {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QCharRef_operator_assign_QCharRef(original_self as *mut ::char_ref::CharRef,
                                                           c as *const ::char_ref::CharRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> CharRefOpAssign0Args<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef {
      let c = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QCharRef_operator_assign_char(original_self as *mut ::char_ref::CharRef, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [CharRef::op_assign1](../struct.CharRef.html#method.op_assign1) method.
  pub trait CharRefOpAssign1Args<'largs> {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef;
  }
  impl<'largs> CharRefOpAssign1Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef {
      let rc = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QCharRef_operator_assign_int(original_self as *mut ::char_ref::CharRef, rc) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> CharRefOpAssign1Args<'largs> for ::libc::c_uchar {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef {
      let c = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QCharRef_operator_assign_unsigned_char(original_self as *mut ::char_ref::CharRef, c)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [CharRef::op_assign2](../struct.CharRef.html#method.op_assign2) method.
  pub trait CharRefOpAssign2Args<'largs> {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef;
  }
  impl<'largs> CharRefOpAssign2Args<'largs> for ::libc::c_short {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef {
      let rc = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QCharRef_operator_assign_short(original_self as *mut ::char_ref::CharRef, rc) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> CharRefOpAssign2Args<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::char_ref::CharRef) -> &'largs mut ::char_ref::CharRef {
      let rc = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QCharRef_operator_assign_unsigned_int(original_self as *mut ::char_ref::CharRef, rc)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
