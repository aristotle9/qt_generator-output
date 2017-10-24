/// C++ type: <span style='color: green;'>```QPair<double, QVariant>```</span>
#[repr(C)]
pub struct PairCDoubleVariant([u8; ::type_sizes::QT_CORE_PAIR_PAIR_C_DOUBLE_VARIANT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairCDoubleVariant {
  unsafe fn new_uninitialized() -> PairCDoubleVariant {
    PairCDoubleVariant(::std::mem::uninitialized())
  }
}

impl PairCDoubleVariant {
  /// C++ method: <span style='color: green;'>```QPair<double, QVariant>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, QVariant>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::libc::c_double, &::variant::Variant)) -> ::pair::PairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, QVariant>::QPair(const double& t1, const QVariant& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairCDoubleVariant
    where Args: overloading::PairCDoubleVariantNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<double, QVariant>::swap(QPair<double, QVariant>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairCDoubleVariant) {
    unsafe {
      ::ffi::qt_core_c_QPair_double_QVariant_swap(self as *mut ::pair::PairCDoubleVariant,
                                                  other as *mut ::pair::PairCDoubleVariant)
    }
  }
}

impl Drop for ::pair::PairCDoubleVariant {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<double, QVariant>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QPair_double_QVariant_destructor(self as *mut ::pair::PairCDoubleVariant) }
  }
}

/// C++ type: <span style='color: green;'>```QPair<QString, QString>```</span>
#[repr(C)]
pub struct PairStringString([u8; ::type_sizes::QT_CORE_PAIR_PAIR_STRING_STRING]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairStringString {
  unsafe fn new_uninitialized() -> PairStringString {
    PairStringString(::std::mem::uninitialized())
  }
}

impl PairStringString {
  /// C++ method: <span style='color: green;'>```QPair<QString, QString>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairStringString```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<QString, QString>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String)) -> ::pair::PairStringString```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<QString, QString>::QPair(const QString& t1, const QString& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairStringString
    where Args: overloading::PairStringStringNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<QString, QString>::swap(QPair<QString, QString>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairStringString) {
    unsafe {
      ::ffi::qt_core_c_QPair_QString_QString_swap(self as *mut ::pair::PairStringString,
                                                  other as *mut ::pair::PairStringString)
    }
  }
}

impl Drop for ::pair::PairStringString {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<QString, QString>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QPair_QString_QString_destructor(self as *mut ::pair::PairStringString) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PairCDoubleVariant::new](../struct.PairCDoubleVariant.html#method.new) method.
  pub trait PairCDoubleVariantNewArgs {
    fn exec(self) -> ::pair::PairCDoubleVariant;
  }
  impl PairCDoubleVariantNewArgs for () {
    fn exec(self) -> ::pair::PairCDoubleVariant {

      {
        let mut object: ::pair::PairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPair_double_QVariant_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairCDoubleVariantNewArgs for (&'a ::libc::c_double, &'a ::variant::Variant) {
    fn exec(self) -> ::pair::PairCDoubleVariant {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPair_double_QVariant_constructor_t1_t2(t1 as *const ::libc::c_double,
                                                                   t2 as *const ::variant::Variant,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PairStringString::new](../struct.PairStringString.html#method.new) method.
  pub trait PairStringStringNewArgs {
    fn exec(self) -> ::pair::PairStringString;
  }
  impl PairStringStringNewArgs for () {
    fn exec(self) -> ::pair::PairStringString {

      {
        let mut object: ::pair::PairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPair_QString_QString_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairStringStringNewArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::pair::PairStringString {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPair_QString_QString_constructor_t1_t2(t1 as *const ::string::String,
                                                                   t2 as *const ::string::String,
                                                                   &mut object);
        }
        object
      }
    }
  }
}
