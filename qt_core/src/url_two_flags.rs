/// C++ type: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>```</span>
#[repr(C)]
pub struct UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption([u8; ::type_sizes::QT_CORE_URL_TWO_FLAGS_URL_TWO_FLAGS_URL_URL_FORMATTING_OPTION_URL_COMPONENT_FORMATTING_OPTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
  unsafe fn new_uninitialized() -> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption(::std::mem::uninitialized())
  }
}

impl UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
  /// C++ method: <span style='color: green;'>```int QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator int() const```</span>
  ///
  ///
  pub fn as_int(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_convert_to_int(self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator QFlags< QUrl::ComponentFormattingOption >() const```</span>
  ///
  ///
  pub fn as_q_flags_q_url_component_formatting_option(&self) -> ::flags::Flags<::url::ComponentFormattingOption> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_convert_to_QFlags_QUrl_ComponentFormattingOption(self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::QUrlTwoFlags```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(::flags::Flags<::url::ComponentFormattingOption>) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::QUrlTwoFlags(QFlags<QUrl::ComponentFormattingOption> f)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::url::ComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::QUrlTwoFlags(QUrl::ComponentFormattingOption f)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::url::UrlFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::QUrlTwoFlags(QUrl::UrlFormattingOption f)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_and(&self, ::url::ComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&(QUrl::ComponentFormattingOption f) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_and(&self, ::url::UrlFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&(QUrl::UrlFormattingOption f) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_bit_and(&self, ::libc::c_int) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&(int mask) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_bit_and(&self, ::libc::c_uint) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&(unsigned int mask) const```</span>
  ///
  ///
  pub fn op_bit_and<'largs, Args>(&'largs self,
                                  args: Args)
                                  -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_and_assign(&mut self, ::libc::c_int) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&=(int mask)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_and_assign(&mut self, ::libc::c_uint) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator&=(unsigned int mask)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'largs, Args>
    (&'largs mut self,
     args: Args)
     -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator~() const```</span>
  ///
  ///
  pub fn op_bit_not(&self) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    {
      let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_not_to_output(self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_or(&self, ::url::ComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|(QUrl::ComponentFormattingOption f) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_or(&self, ::url::UrlFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|(QUrl::UrlFormattingOption f) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_bit_or(&self, &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> f) const```</span>
  ///
  ///
  pub fn op_bit_or<'largs, Args>(&'largs self,
                                 args: Args)
                                 -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_or_assign(&mut self, ::url::ComponentFormattingOption) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|=(QUrl::ComponentFormattingOption f)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_or_assign(&mut self, ::url::UrlFormattingOption) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|=(QUrl::UrlFormattingOption f)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_bit_or_assign(&mut self, &'l1 ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator|=(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> f)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'largs, Args>
    (&'largs mut self,
     args: Args)
     -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_xor(&self, ::url::ComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^(QUrl::ComponentFormattingOption f) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_xor(&self, ::url::UrlFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^(QUrl::UrlFormattingOption f) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_bit_xor(&self, &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> f) const```</span>
  ///
  ///
  pub fn op_bit_xor<'largs, Args>(&'largs self,
                                  args: Args)
                                  -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_xor_assign(&mut self, ::url::ComponentFormattingOption) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^=(QUrl::ComponentFormattingOption f)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_xor_assign(&mut self, ::url::UrlFormattingOption) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^=(QUrl::UrlFormattingOption f)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_bit_xor_assign(&mut self, &'l1 ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'l0 mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption```<br>
  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>& QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator^=(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> f)```</span>
  ///
  ///
  pub fn op_bit_xor_assign<'largs, Args>
    (&'largs mut self,
     args: Args)
     -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_not(self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) }
  }

  /// C++ method: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::testFlag```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn test_flag(&self, ::url::ComponentFormattingOption) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::testFlag(QUrl::ComponentFormattingOption f) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn test_flag(&self, ::url::UrlFormattingOption) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::testFlag(QUrl::UrlFormattingOption f) const```</span>
  ///
  ///
  pub fn test_flag<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionTestFlagArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
  /// C++ method: <span style='color: green;'>```[destructor] void QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption>::~QUrlTwoFlags()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_destructor(self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::new](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.new) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionNewArgs {
    fn exec(self) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption;
  }
  impl UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionNewArgs for ::flags::Flags<::url::ComponentFormattingOption> {

  fn exec(self, ) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_constructor_QFlags_QUrl_ComponentFormattingOption(f.to_int() as ::libc::c_uint, &mut object); }object
}
  }
}
  impl UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionNewArgs for ::url::ComponentFormattingOption {
    fn exec(self) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
      let f = self;
      {
        let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_constructor_QUrl_ComponentFormattingOption(f, &mut object);
        }
        object
      }
    }
  }
  impl UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionNewArgs for ::url::UrlFormattingOption {
    fn exec(self) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
      let f = self;
      {
        let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_constructor_QUrl_UrlFormattingOption(f, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::op_bit_and](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.op_bit_and) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndArgs<'largs>
     {
    fn exec(self,
            original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
            -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption;
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndArgs<'largs> for ::url::ComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_QUrl_ComponentFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f, &mut object); }object
}
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndArgs<'largs> for ::url::UrlFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_QUrl_UrlFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f, &mut object); }object
}
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
            -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
      let mask = self;
      {
        let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_int(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, mask, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndArgs<'largs> for ::libc::c_uint {
    fn exec(self,
            original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
            -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
      let mask = self;
      {
        let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_unsigned_int(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, mask, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::op_bit_and_assign](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.op_bit_and_assign) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndAssignArgs<'largs>
     {
    fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption;
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndAssignArgs<'largs> for ::libc::c_int {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let mask = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_assign_int(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, mask) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitAndAssignArgs<'largs> for ::libc::c_uint {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let mask = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_assign_unsigned_int(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, mask) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::op_bit_or](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.op_bit_or) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrArgs<'largs>
     {
    fn exec(self,
            original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
            -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption;
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_to_output_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object); }object
}
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrArgs<'largs> for ::url::ComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_to_output_QUrl_ComponentFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f, &mut object); }object
}
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrArgs<'largs> for ::url::UrlFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_to_output_QUrl_UrlFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::op_bit_or_assign](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.op_bit_or_assign) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrAssignArgs<'largs>
     {
    fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption;
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrAssignArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_assign_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrAssignArgs<'largs> for ::url::ComponentFormattingOption {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_assign_QUrl_ComponentFormattingOption(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitOrAssignArgs<'largs> for ::url::UrlFormattingOption {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_assign_QUrl_UrlFormattingOption(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::op_bit_xor](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.op_bit_xor) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorArgs<'largs>
     {
    fn exec(self,
            original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
            -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption;
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_to_output_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object); }object
}
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorArgs<'largs> for ::url::ComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_to_output_QUrl_ComponentFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f, &mut object); }object
}
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorArgs<'largs> for ::url::UrlFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    {
let mut object: ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_to_output_QUrl_UrlFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::op_bit_xor_assign](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.op_bit_xor_assign) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorAssignArgs<'largs>
     {
    fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption;
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorAssignArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_assign_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorAssignArgs<'largs> for ::url::ComponentFormattingOption {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_assign_QUrl_ComponentFormattingOption(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionOpBitXorAssignArgs<'largs> for ::url::UrlFormattingOption {

  fn exec(self, original_self: &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> &'largs mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {
    let f = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_assign_QUrl_UrlFormattingOption(original_self as *mut ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption::test_flag](../struct.UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption.html#method.test_flag) method.
  pub trait UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionTestFlagArgs<'largs>
     {
    fn exec(self,
            original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
            -> bool;
  }
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionTestFlagArgs<'largs> for ::url::ComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> bool {
    let f = self;
    unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_testFlag_QUrl_ComponentFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f) }
  }
}
  impl<'largs> UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOptionTestFlagArgs<'largs> for ::url::UrlFormattingOption {

  fn exec(self, original_self: &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> bool {
    let f = self;
    unsafe { ::ffi::qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_testFlag_QUrl_UrlFormattingOption(original_self as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, f) }
  }
}
}
