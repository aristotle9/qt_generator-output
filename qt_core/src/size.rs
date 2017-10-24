/// C++ type: <span style='color: green;'>```QSize```</span>
#[repr(C)]
pub struct Size([u8; ::type_sizes::QT_CORE_SIZE_SIZE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Size {
  unsafe fn new_uninitialized() -> Size {
    Size(::std::mem::uninitialized())
  }
}

impl Size {
  /// C++ method: <span style='color: green;'>```QSize QSize::boundedTo(const QSize& arg1) const```</span>
  ///
  ///
  pub fn bounded_to(&self, arg1: &::size::Size) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSize_boundedTo_to_output(self as *const ::size::Size,
                                                   arg1 as *const ::size::Size,
                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QSize::expandedTo(const QSize& arg1) const```</span>
  ///
  ///
  pub fn expanded_to(&self, arg1: &::size::Size) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSize_expandedTo_to_output(self as *const ::size::Size,
                                                    arg1 as *const ::size::Size,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSize::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSize_height(self as *const ::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```bool QSize::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSize_isEmpty(self as *const ::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```bool QSize::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSize_isNull(self as *const ::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```bool QSize::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSize_isValid(self as *const ::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```QSize::QSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::size::Size```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSize::QSize()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::size::Size```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSize::QSize(int w, int h)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::size::Size
    where Args: overloading::SizeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSize& QSize::operator+=(const QSize& arg1)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::size::Size) -> &'l0 mut ::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSize_operator_add_assign(self as *mut ::size::Size, arg1 as *const ::size::Size) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QSize::operator/=(double c)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, c: ::libc::c_double) -> &'l0 mut ::size::Size {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSize_operator_div_assign(self as *mut ::size::Size, c) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QSize::operator*=(double c)```</span>
  ///
  ///
  pub fn op_mul_assign<'l0>(&'l0 mut self, c: ::libc::c_double) -> &'l0 mut ::size::Size {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSize_operator_mul_assign(self as *mut ::size::Size, c) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QSize::operator-=(const QSize& arg1)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::size::Size) -> &'l0 mut ::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSize_operator_sub_assign(self as *mut ::size::Size, arg1 as *const ::size::Size) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QSize::rheight()```</span>
  ///
  ///
  pub fn rheight<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSize_rheight(self as *mut ::size::Size) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QSize::rwidth()```</span>
  ///
  ///
  pub fn rwidth<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSize_rwidth(self as *mut ::size::Size) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize::scale```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scale(&mut self, (&::size::Size, &::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSize::scale(const QSize& s, Qt::AspectRatioMode mode)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scale(&mut self, (::libc::c_int, ::libc::c_int, &::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSize::scale(int w, int h, Qt::AspectRatioMode mode)```</span>
  ///
  ///
  pub fn scale<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SizeScaleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSize::scaled```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled(&self, (&::size::Size, &::qt::AspectRatioMode)) -> ::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QSize::scaled(const QSize& s, Qt::AspectRatioMode mode) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_int, ::libc::c_int, &::qt::AspectRatioMode)) -> ::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QSize::scaled(int w, int h, Qt::AspectRatioMode mode) const```</span>
  ///
  ///
  pub fn scaled<'largs, Args>(&'largs self, args: Args) -> ::size::Size
    where Args: overloading::SizeScaledArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSize::setHeight(int h)```</span>
  ///
  ///
  pub fn set_height(&mut self, h: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QSize_setHeight(self as *mut ::size::Size, h) }
  }

  /// C++ method: <span style='color: green;'>```void QSize::setWidth(int w)```</span>
  ///
  ///
  pub fn set_width(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QSize_setWidth(self as *mut ::size::Size, w) }
  }

  /// C++ method: <span style='color: green;'>```void QSize::transpose()```</span>
  ///
  ///
  pub fn transpose(&mut self) {
    unsafe { ::ffi::qt_core_c_QSize_transpose(self as *mut ::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```QSize QSize::transposed() const```</span>
  ///
  ///
  pub fn transposed(&self) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSize_transposed_to_output(self as *const ::size::Size, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSize::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSize_width(self as *const ::size::Size) }
  }
}

impl Drop for ::size::Size {
  /// C++ method: <span style='color: green;'>```[destructor] void QSize::~QSize()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSize_destructor(self as *mut ::size::Size) }
  }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::size::Size)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QSize& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::size_f::SizeF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QSizeF& arg2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::size::Size)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QSize& arg2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::size_f::SizeF)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QSizeF& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator>>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::size::Size)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QSize& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::size_f::SizeF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QSizeF& arg2)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Size::new](../struct.Size.html#method.new) method.
  pub trait SizeNewArgs {
    fn exec(self) -> ::size::Size;
  }
  impl SizeNewArgs for () {
    fn exec(self) -> ::size::Size {

      {
        let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSize_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl SizeNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::size::Size {
      let w = self.0;
      let h = self.1;
      {
        let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSize_constructor_w_h(w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Size::scale](../struct.Size.html#method.scale) method.
  pub trait SizeScaleArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::size::Size) -> ();
  }
  impl<'largs> SizeScaleArgs<'largs> for (&'largs ::size::Size, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs mut ::size::Size) -> () {
      let s = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QSize_scale_s_mode(original_self as *mut ::size::Size,
                                            s as *const ::size::Size,
                                            mode as *const ::qt::AspectRatioMode)
      }
    }
  }
  impl<'largs> SizeScaleArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs mut ::size::Size) -> () {
      let w = self.0;
      let h = self.1;
      let mode = self.2;
      unsafe {
        ::ffi::qt_core_c_QSize_scale_w_h_mode(original_self as *mut ::size::Size,
                                              w,
                                              h,
                                              mode as *const ::qt::AspectRatioMode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Size::scaled](../struct.Size.html#method.scaled) method.
  pub trait SizeScaledArgs<'largs> {
    fn exec(self, original_self: &'largs ::size::Size) -> ::size::Size;
  }
  impl<'largs> SizeScaledArgs<'largs> for (&'largs ::size::Size, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::size::Size) -> ::size::Size {
      let s = self.0;
      let mode = self.1;
      {
        let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSize_scaled_to_output_s_mode(original_self as *const ::size::Size,
                                                         s as *const ::size::Size,
                                                         mode as *const ::qt::AspectRatioMode,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> SizeScaledArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::size::Size) -> ::size::Size {
      let w = self.0;
      let h = self.1;
      let mode = self.2;
      {
        let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSize_scaled_to_output_w_h_mode(original_self as *const ::size::Size,
                                                           w,
                                                           h,
                                                           mode as *const ::qt::AspectRatioMode,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::size::Size) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QSize_G_operator_shl_QDataStream_QSize(arg1 as *mut ::data_stream::DataStream,
                                                                arg2 as *const ::size::Size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::size_f::SizeF) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QSize_G_operator_shl_QDataStream_QSizeF(arg1 as *mut ::data_stream::DataStream,
                                                                   arg2 as *const ::size_f::SizeF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::size::Size) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSize_G_operator_shl_to_output_QDebug_QSize(arg1 as *const ::debug::Debug,
                                                                       arg2 as *const ::size::Size,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::size_f::SizeF) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSize_G_operator_shl_to_output_QDebug_QSizeF(arg1 as *const ::debug::Debug,
                                                                        arg2 as *const ::size_f::SizeF,
                                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shr](../fn.op_shr.html) method.
  pub trait OpShrArgs<'largs> {
    fn exec(self) -> &'largs mut ::data_stream::DataStream;
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::size::Size) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QSize_G_operator_shr_QDataStream_QSize(arg1 as *mut ::data_stream::DataStream,
                                                                arg2 as *mut ::size::Size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::size_f::SizeF) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QSize_G_operator_shr_QDataStream_QSizeF(arg1 as *mut ::data_stream::DataStream,
                                                                   arg2 as *mut ::size_f::SizeF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
