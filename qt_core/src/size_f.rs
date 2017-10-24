/// C++ type: <span style='color: green;'>```QSizeF```</span>
#[repr(C)]
pub struct SizeF([u8; ::type_sizes::QT_CORE_SIZE_F_SIZE_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SizeF {
  unsafe fn new_uninitialized() -> SizeF {
    SizeF(::std::mem::uninitialized())
  }
}

impl SizeF {
  /// C++ method: <span style='color: green;'>```QSizeF QSizeF::boundedTo(const QSizeF& arg1) const```</span>
  ///
  ///
  pub fn bounded_to(&self, arg1: &::size_f::SizeF) -> ::size_f::SizeF {
    {
      let mut object: ::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSizeF_boundedTo_to_output(self as *const ::size_f::SizeF,
                                                    arg1 as *const ::size_f::SizeF,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QSizeF::expandedTo(const QSizeF& arg1) const```</span>
  ///
  ///
  pub fn expanded_to(&self, arg1: &::size_f::SizeF) -> ::size_f::SizeF {
    {
      let mut object: ::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSizeF_expandedTo_to_output(self as *const ::size_f::SizeF,
                                                     arg1 as *const ::size_f::SizeF,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QSizeF::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QSizeF_height(self as *const ::size_f::SizeF) }
  }

  /// C++ method: <span style='color: green;'>```bool QSizeF::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSizeF_isEmpty(self as *const ::size_f::SizeF) }
  }

  /// C++ method: <span style='color: green;'>```bool QSizeF::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSizeF_isNull(self as *const ::size_f::SizeF) }
  }

  /// C++ method: <span style='color: green;'>```bool QSizeF::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSizeF_isValid(self as *const ::size_f::SizeF) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF::QSizeF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSizeF::QSizeF()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::size::Size) -> ::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSizeF::QSizeF(const QSize& sz)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double)) -> ::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSizeF::QSizeF(double w, double h)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::size_f::SizeF
    where Args: overloading::SizeFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSizeF& QSizeF::operator+=(const QSizeF& arg1)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::size_f::SizeF) -> &'l0 mut ::size_f::SizeF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QSizeF_operator_add_assign(self as *mut ::size_f::SizeF, arg1 as *const ::size_f::SizeF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSizeF& QSizeF::operator/=(double c)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, c: ::libc::c_double) -> &'l0 mut ::size_f::SizeF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSizeF_operator_div_assign(self as *mut ::size_f::SizeF, c) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSizeF& QSizeF::operator*=(double c)```</span>
  ///
  ///
  pub fn op_mul_assign<'l0>(&'l0 mut self, c: ::libc::c_double) -> &'l0 mut ::size_f::SizeF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSizeF_operator_mul_assign(self as *mut ::size_f::SizeF, c) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSizeF& QSizeF::operator-=(const QSizeF& arg1)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::size_f::SizeF) -> &'l0 mut ::size_f::SizeF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QSizeF_operator_sub_assign(self as *mut ::size_f::SizeF, arg1 as *const ::size_f::SizeF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QSizeF::rheight()```</span>
  ///
  ///
  pub fn rheight<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSizeF_rheight(self as *mut ::size_f::SizeF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QSizeF::rwidth()```</span>
  ///
  ///
  pub fn rwidth<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSizeF_rwidth(self as *mut ::size_f::SizeF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSizeF::scale```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scale(&mut self, (&::size_f::SizeF, &::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSizeF::scale(const QSizeF& s, Qt::AspectRatioMode mode)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scale(&mut self, (::libc::c_double, ::libc::c_double, &::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSizeF::scale(double w, double h, Qt::AspectRatioMode mode)```</span>
  ///
  ///
  pub fn scale<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SizeFScaleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSizeF::scaled```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled(&self, (&::size_f::SizeF, &::qt::AspectRatioMode)) -> ::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```QSizeF QSizeF::scaled(const QSizeF& s, Qt::AspectRatioMode mode) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_double, ::libc::c_double, &::qt::AspectRatioMode)) -> ::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```QSizeF QSizeF::scaled(double w, double h, Qt::AspectRatioMode mode) const```</span>
  ///
  ///
  pub fn scaled<'largs, Args>(&'largs self, args: Args) -> ::size_f::SizeF
    where Args: overloading::SizeFScaledArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSizeF::setHeight(double h)```</span>
  ///
  ///
  pub fn set_height(&mut self, h: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QSizeF_setHeight(self as *mut ::size_f::SizeF, h) }
  }

  /// C++ method: <span style='color: green;'>```void QSizeF::setWidth(double w)```</span>
  ///
  ///
  pub fn set_width(&mut self, w: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QSizeF_setWidth(self as *mut ::size_f::SizeF, w) }
  }

  /// C++ method: <span style='color: green;'>```QSize QSizeF::toSize() const```</span>
  ///
  ///
  pub fn to_size(&self) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSizeF_toSize_to_output(self as *const ::size_f::SizeF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QSizeF::transpose()```</span>
  ///
  ///
  pub fn transpose(&mut self) {
    unsafe { ::ffi::qt_core_c_QSizeF_transpose(self as *mut ::size_f::SizeF) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QSizeF::transposed() const```</span>
  ///
  ///
  pub fn transposed(&self) -> ::size_f::SizeF {
    {
      let mut object: ::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSizeF_transposed_to_output(self as *const ::size_f::SizeF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QSizeF::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QSizeF_width(self as *const ::size_f::SizeF) }
  }
}

impl Drop for ::size_f::SizeF {
  /// C++ method: <span style='color: green;'>```[destructor] void QSizeF::~QSizeF()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSizeF_destructor(self as *mut ::size_f::SizeF) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SizeF::new](../struct.SizeF.html#method.new) method.
  pub trait SizeFNewArgs {
    fn exec(self) -> ::size_f::SizeF;
  }
  impl SizeFNewArgs for () {
    fn exec(self) -> ::size_f::SizeF {

      {
        let mut object: ::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSizeF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SizeFNewArgs for &'a ::size::Size {
    fn exec(self) -> ::size_f::SizeF {
      let sz = self;
      {
        let mut object: ::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSizeF_constructor_sz(sz as *const ::size::Size, &mut object);
        }
        object
      }
    }
  }
  impl SizeFNewArgs for (::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::size_f::SizeF {
      let w = self.0;
      let h = self.1;
      {
        let mut object: ::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSizeF_constructor_w_h(w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SizeF::scale](../struct.SizeF.html#method.scale) method.
  pub trait SizeFScaleArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::size_f::SizeF) -> ();
  }
  impl<'largs> SizeFScaleArgs<'largs> for (&'largs ::size_f::SizeF, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs mut ::size_f::SizeF) -> () {
      let s = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QSizeF_scale_s_mode(original_self as *mut ::size_f::SizeF,
                                             s as *const ::size_f::SizeF,
                                             mode as *const ::qt::AspectRatioMode)
      }
    }
  }
  impl<'largs> SizeFScaleArgs<'largs> for (::libc::c_double, ::libc::c_double, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs mut ::size_f::SizeF) -> () {
      let w = self.0;
      let h = self.1;
      let mode = self.2;
      unsafe {
        ::ffi::qt_core_c_QSizeF_scale_w_h_mode(original_self as *mut ::size_f::SizeF,
                                               w,
                                               h,
                                               mode as *const ::qt::AspectRatioMode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SizeF::scaled](../struct.SizeF.html#method.scaled) method.
  pub trait SizeFScaledArgs<'largs> {
    fn exec(self, original_self: &'largs ::size_f::SizeF) -> ::size_f::SizeF;
  }
  impl<'largs> SizeFScaledArgs<'largs> for (&'largs ::size_f::SizeF, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::size_f::SizeF) -> ::size_f::SizeF {
      let s = self.0;
      let mode = self.1;
      {
        let mut object: ::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSizeF_scaled_to_output_s_mode(original_self as *const ::size_f::SizeF,
                                                          s as *const ::size_f::SizeF,
                                                          mode as *const ::qt::AspectRatioMode,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> SizeFScaledArgs<'largs> for (::libc::c_double, ::libc::c_double, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::size_f::SizeF) -> ::size_f::SizeF {
      let w = self.0;
      let h = self.1;
      let mode = self.2;
      {
        let mut object: ::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSizeF_scaled_to_output_w_h_mode(original_self as *const ::size_f::SizeF,
                                                            w,
                                                            h,
                                                            mode as *const ::qt::AspectRatioMode,
                                                            &mut object);
        }
        object
      }
    }
  }
}
