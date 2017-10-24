/// C++ type: <span style='color: green;'>```QCollator```</span>
#[repr(C)]
pub struct Collator([u8; ::type_sizes::QT_CORE_COLLATOR_COLLATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Collator {
  unsafe fn new_uninitialized() -> Collator {
    Collator(::std::mem::uninitialized())
  }
}

impl Collator {
  /// C++ method: <span style='color: green;'>```QCollator::compare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn compare(&self, (&::string::String, &::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QCollator::compare(const QString& s1, const QString& s2) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn compare(&self, (&::string_ref::StringRef, &::string_ref::StringRef)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QCollator::compare(const QStringRef& s1, const QStringRef& s2) const```</span>
  ///
  ///
  pub fn compare<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::CollatorCompareArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QCollator::compare(const QChar* s1, int len1, const QChar* s2, int len2) const```</span>
  ///
  ///
  pub unsafe fn compare_unsafe(&self,
                               s1: *const ::char::Char,
                               len1: ::libc::c_int,
                               s2: *const ::char::Char,
                               len2: ::libc::c_int)
                               -> ::libc::c_int {
    ::ffi::qt_core_c_QCollator_compare_QChar_int_QChar_int(self as *const ::collator::Collator, s1, len1, s2, len2)
  }

  /// C++ method: <span style='color: green;'>```bool QCollator::ignorePunctuation() const```</span>
  ///
  ///
  pub fn ignore_punctuation(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCollator_ignorePunctuation(self as *const ::collator::Collator) }
  }

  /// C++ method: <span style='color: green;'>```QLocale QCollator::locale() const```</span>
  ///
  ///
  pub fn locale(&self) -> ::locale::Locale {
    {
      let mut object: ::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCollator_locale_to_output(self as *const ::collator::Collator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCollator::QCollator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::collator::Collator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCollator::QCollator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::collator::Collator) -> ::collator::Collator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCollator::QCollator(const QCollator& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::locale::Locale) -> ::collator::Collator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCollator::QCollator(const QLocale& locale = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::collator::Collator
    where Args: overloading::CollatorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QCollator::numericMode() const```</span>
  ///
  ///
  pub fn numeric_mode(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCollator_numericMode(self as *const ::collator::Collator) }
  }

  /// C++ method: <span style='color: green;'>```QCollator& QCollator::operator=(const QCollator& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::collator::Collator) -> &'l0 mut ::collator::Collator {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QCollator_operator_assign(self as *mut ::collator::Collator,
                                                 arg1 as *const ::collator::Collator)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QCollator::operator()(const QString& s1, const QString& s2) const```</span>
  ///
  ///
  pub fn op_call(&self, s1: &::string::String, s2: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QCollator_operator_call(self as *const ::collator::Collator,
                                               s1 as *const ::string::String,
                                               s2 as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCollator::setCaseSensitivity(Qt::CaseSensitivity cs)```</span>
  ///
  ///
  pub fn set_case_sensitivity(&mut self, cs: &::qt::CaseSensitivity) {
    unsafe {
      ::ffi::qt_core_c_QCollator_setCaseSensitivity(self as *mut ::collator::Collator,
                                                    cs as *const ::qt::CaseSensitivity)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCollator::setIgnorePunctuation(bool on)```</span>
  ///
  ///
  pub fn set_ignore_punctuation(&mut self, on: bool) {
    unsafe { ::ffi::qt_core_c_QCollator_setIgnorePunctuation(self as *mut ::collator::Collator, on) }
  }

  /// C++ method: <span style='color: green;'>```void QCollator::setLocale(const QLocale& locale)```</span>
  ///
  ///
  pub fn set_locale(&mut self, locale: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QCollator_setLocale(self as *mut ::collator::Collator,
                                           locale as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCollator::setNumericMode(bool on)```</span>
  ///
  ///
  pub fn set_numeric_mode(&mut self, on: bool) {
    unsafe { ::ffi::qt_core_c_QCollator_setNumericMode(self as *mut ::collator::Collator, on) }
  }

  /// C++ method: <span style='color: green;'>```QCollatorSortKey QCollator::sortKey(const QString& string) const```</span>
  ///
  ///
  pub fn sort_key(&self, string: &::string::String) -> ::collator_sort_key::CollatorSortKey {
    {
      let mut object: ::collator_sort_key::CollatorSortKey =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCollator_sortKey_to_output(self as *const ::collator::Collator,
                                                     string as *const ::string::String,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QCollator::swap(QCollator& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::collator::Collator) {
    unsafe {
      ::ffi::qt_core_c_QCollator_swap(self as *mut ::collator::Collator,
                                      other as *mut ::collator::Collator)
    }
  }
}

impl Drop for ::collator::Collator {
  /// C++ method: <span style='color: green;'>```[destructor] void QCollator::~QCollator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QCollator_destructor(self as *mut ::collator::Collator) }
  }
}

/// C++ method: <span style='color: green;'>```bool operator<(const QCollatorSortKey& lhs, const QCollatorSortKey& rhs)```</span>
///
///
pub fn op_lt(lhs: &::collator_sort_key::CollatorSortKey, rhs: &::collator_sort_key::CollatorSortKey) -> bool {
  unsafe {
    ::ffi::qt_core_c_QCollator_G_operator_lt(lhs as *const ::collator_sort_key::CollatorSortKey,
                                             rhs as *const ::collator_sort_key::CollatorSortKey)
  }
}

/// C++ method: <span style='color: green;'>```swap```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn swap((&mut ::collator::Collator, &mut ::collator::Collator)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QCollator& value1, QCollator& value2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn swap((&mut ::collator_sort_key::CollatorSortKey, &mut ::collator_sort_key::CollatorSortKey)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QCollatorSortKey& value1, QCollatorSortKey& value2)```</span>
///
///
pub fn swap<Args>(args: Args) -> ()
  where Args: overloading::SwapArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Collator::compare](../struct.Collator.html#method.compare) method.
  pub trait CollatorCompareArgs<'largs> {
    fn exec(self, original_self: &'largs ::collator::Collator) -> ::libc::c_int;
  }
  impl<'largs> CollatorCompareArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::string_ref::StringRef) {
    fn exec(self, original_self: &'largs ::collator::Collator) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QCollator_compare_QStringRef_QStringRef(original_self as *const ::collator::Collator,
                                                                 s1 as *const ::string_ref::StringRef,
                                                                 s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> CollatorCompareArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::collator::Collator) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QCollator_compare_QString_QString(original_self as *const ::collator::Collator,
                                                           s1 as *const ::string::String,
                                                           s2 as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Collator::new](../struct.Collator.html#method.new) method.
  pub trait CollatorNewArgs {
    fn exec(self) -> ::collator::Collator;
  }
  impl<'a> CollatorNewArgs for &'a ::collator::Collator {
    fn exec(self) -> ::collator::Collator {
      let arg1 = self;
      {
        let mut object: ::collator::Collator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCollator_constructor_arg1(arg1 as *const ::collator::Collator, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CollatorNewArgs for &'a ::locale::Locale {
    fn exec(self) -> ::collator::Collator {
      let locale = self;
      {
        let mut object: ::collator::Collator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCollator_constructor_locale(locale as *const ::locale::Locale, &mut object);
        }
        object
      }
    }
  }
  impl CollatorNewArgs for () {
    fn exec(self) -> ::collator::Collator {

      {
        let mut object: ::collator::Collator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCollator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [swap](../fn.swap.html) method.
  pub trait SwapArgs {
    fn exec(self) -> ();
  }
  impl<'a> SwapArgs for (&'a mut ::collator_sort_key::CollatorSortKey, &'a mut ::collator_sort_key::CollatorSortKey) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_core_c_QCollator_G_swap_QCollatorSortKey_QCollatorSortKey(value1 as *mut ::collator_sort_key::CollatorSortKey, value2 as *mut ::collator_sort_key::CollatorSortKey) }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::collator::Collator, &'a mut ::collator::Collator) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QCollator_G_swap_QCollator_QCollator(value1 as *mut ::collator::Collator,
                                                              value2 as *mut ::collator::Collator)
      }
    }
  }
}
