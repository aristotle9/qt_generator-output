/// C++ type: <span style='color: green;'>```QRegularExpressionMatchIterator```</span>
#[repr(C)]
pub struct RegularExpressionMatchIterator([u8; ::type_sizes::QT_CORE_REGULAR_EXPRESSION_MATCH_ITERATOR_REGULAR_EXPRESSION_MATCH_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for RegularExpressionMatchIterator {
  unsafe fn new_uninitialized() -> RegularExpressionMatchIterator {
    RegularExpressionMatchIterator(::std::mem::uninitialized())
  }
}

impl RegularExpressionMatchIterator {
  /// C++ method: <span style='color: green;'>```bool QRegularExpressionMatchIterator::hasNext() const```</span>
  ///
  ///
  pub fn has_next(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatchIterator_hasNext(self as *const ::regular_expression_match_iterator::RegularExpressionMatchIterator) }
  }

  /// C++ method: <span style='color: green;'>```bool QRegularExpressionMatchIterator::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatchIterator_isValid(self as *const ::regular_expression_match_iterator::RegularExpressionMatchIterator) }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator::QRegularExpressionMatchIterator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::regular_expression_match_iterator::RegularExpressionMatchIterator) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator(const QRegularExpressionMatchIterator& iterator)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator
    where Args: overloading::RegularExpressionMatchIteratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpressionMatchIterator::next()```</span>
  ///
  ///
  pub fn next(&mut self) -> ::regular_expression_match::RegularExpressionMatch {
    {
      let mut object: ::regular_expression_match::RegularExpressionMatch =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpressionMatchIterator_next_to_output(self as *mut ::regular_expression_match_iterator::RegularExpressionMatchIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator& QRegularExpressionMatchIterator::operator=(const QRegularExpressionMatchIterator& iterator)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             iterator: &'l1 ::regular_expression_match_iterator::RegularExpressionMatchIterator)
                             -> &'l0 mut ::regular_expression_match_iterator::RegularExpressionMatchIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QRegularExpressionMatchIterator_operator_assign(self as *mut ::regular_expression_match_iterator::RegularExpressionMatchIterator, iterator as *const ::regular_expression_match_iterator::RegularExpressionMatchIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext() const```</span>
  ///
  ///
  pub fn peek_next(&self) -> ::regular_expression_match::RegularExpressionMatch {
    {
      let mut object: ::regular_expression_match::RegularExpressionMatch =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpressionMatchIterator_peekNext_to_output(self as *const ::regular_expression_match_iterator::RegularExpressionMatchIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpression QRegularExpressionMatchIterator::regularExpression() const```</span>
  ///
  ///
  pub fn regular_expression(&self) -> ::regular_expression::RegularExpression {
    {
      let mut object: ::regular_expression::RegularExpression =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpressionMatchIterator_regularExpression_to_output(self as *const ::regular_expression_match_iterator::RegularExpressionMatchIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::regular_expression_match_iterator::RegularExpressionMatchIterator) {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatchIterator_swap(self as *mut ::regular_expression_match_iterator::RegularExpressionMatchIterator, other as *mut ::regular_expression_match_iterator::RegularExpressionMatchIterator) }
  }
}

impl Drop for ::regular_expression_match_iterator::RegularExpressionMatchIterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QRegularExpressionMatchIterator::~QRegularExpressionMatchIterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatchIterator_destructor(self as *mut ::regular_expression_match_iterator::RegularExpressionMatchIterator) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RegularExpressionMatchIterator::new](../struct.RegularExpressionMatchIterator.html#method.new) method.
  pub trait RegularExpressionMatchIteratorNewArgs {
    fn exec(self) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator;
  }
  impl<'a> RegularExpressionMatchIteratorNewArgs for &'a ::regular_expression_match_iterator::RegularExpressionMatchIterator {

  fn exec(self, ) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
    let iterator = self;
    {
let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QRegularExpressionMatchIterator_constructor_iterator(iterator as *const ::regular_expression_match_iterator::RegularExpressionMatchIterator, &mut object); }object
}
  }
}
  impl RegularExpressionMatchIteratorNewArgs for () {
    fn exec(self) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {

      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatchIterator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}
