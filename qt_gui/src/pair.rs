/// C++ type: <span style='color: green;'>```QPair<double, QColor>```</span>
#[repr(C)]
pub struct PairCDoubleColor([u8; ::type_sizes::QT_GUI_PAIR_PAIR_C_DOUBLE_COLOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairCDoubleColor {
  unsafe fn new_uninitialized() -> PairCDoubleColor {
    PairCDoubleColor(::std::mem::uninitialized())
  }
}

impl PairCDoubleColor {
  /// C++ method: <span style='color: green;'>```QPair<double, QColor>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, QColor>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::libc::c_double, &::color::Color)) -> ::pair::PairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, QColor>::QPair(const double& t1, const QColor& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairCDoubleColor
    where Args: overloading::PairCDoubleColorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<double, QColor>::swap(QPair<double, QColor>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairCDoubleColor) {
    unsafe {
      ::ffi::qt_gui_c_QPair_double_QColor_swap(self as *mut ::pair::PairCDoubleColor,
                                               other as *mut ::pair::PairCDoubleColor)
    }
  }
}

impl Drop for ::pair::PairCDoubleColor {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<double, QColor>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPair_double_QColor_destructor(self as *mut ::pair::PairCDoubleColor) }
  }
}

/// C++ type: <span style='color: green;'>```QPair<float, float>```</span>
#[repr(C)]
pub struct PairCFloatCFloat([u8; ::type_sizes::QT_GUI_PAIR_PAIR_C_FLOAT_C_FLOAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairCFloatCFloat {
  unsafe fn new_uninitialized() -> PairCFloatCFloat {
    PairCFloatCFloat(::std::mem::uninitialized())
  }
}

impl PairCFloatCFloat {
  /// C++ method: <span style='color: green;'>```QPair<float, float>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairCFloatCFloat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<float, float>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::libc::c_float, &::libc::c_float)) -> ::pair::PairCFloatCFloat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<float, float>::QPair(const float& t1, const float& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairCFloatCFloat
    where Args: overloading::PairCFloatCFloatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<float, float>::swap(QPair<float, float>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairCFloatCFloat) {
    unsafe {
      ::ffi::qt_gui_c_QPair_float_float_swap(self as *mut ::pair::PairCFloatCFloat,
                                             other as *mut ::pair::PairCFloatCFloat)
    }
  }
}

impl Drop for ::pair::PairCFloatCFloat {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<float, float>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPair_float_float_destructor(self as *mut ::pair::PairCFloatCFloat) }
  }
}

/// C++ type: <span style='color: green;'>```QPair<int, int>```</span>
#[repr(C)]
pub struct PairCIntCInt([u8; ::type_sizes::QT_GUI_PAIR_PAIR_C_INT_C_INT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairCIntCInt {
  unsafe fn new_uninitialized() -> PairCIntCInt {
    PairCIntCInt(::std::mem::uninitialized())
  }
}

impl PairCIntCInt {
  /// C++ method: <span style='color: green;'>```QPair<int, int>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairCIntCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<int, int>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::libc::c_int, &::libc::c_int)) -> ::pair::PairCIntCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<int, int>::QPair(const int& t1, const int& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairCIntCInt
    where Args: overloading::PairCIntCIntNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<int, int>::swap(QPair<int, int>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairCIntCInt) {
    unsafe {
      ::ffi::qt_gui_c_QPair_int_int_swap(self as *mut ::pair::PairCIntCInt,
                                         other as *mut ::pair::PairCIntCInt)
    }
  }
}

impl Drop for ::pair::PairCIntCInt {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<int, int>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPair_int_int_destructor(self as *mut ::pair::PairCIntCInt) }
  }
}

/// C++ type: <span style='color: green;'>```QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>```</span>
#[repr(C)]
pub struct PairOpenglTextureFilterOpenglTextureFilter([u8; ::type_sizes::QT_GUI_PAIR_PAIR_OPENGL_TEXTURE_FILTER_OPENGL_TEXTURE_FILTER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairOpenglTextureFilterOpenglTextureFilter {
  unsafe fn new_uninitialized() -> PairOpenglTextureFilterOpenglTextureFilter {
    PairOpenglTextureFilterOpenglTextureFilter(::std::mem::uninitialized())
  }
}

impl PairOpenglTextureFilterOpenglTextureFilter {
  /// C++ method: <span style='color: green;'>```QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairOpenglTextureFilterOpenglTextureFilter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::opengl_texture::Filter, &::opengl_texture::Filter)) -> ::pair::PairOpenglTextureFilterOpenglTextureFilter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>::QPair(const QOpenGLTexture::Filter& t1, const QOpenGLTexture::Filter& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairOpenglTextureFilterOpenglTextureFilter
    where Args: overloading::PairOpenglTextureFilterOpenglTextureFilterNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>::swap(QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairOpenglTextureFilterOpenglTextureFilter) {
    unsafe { ::ffi::qt_gui_c_QPair_QOpenGLTexture_Filter_QOpenGLTexture_Filter_swap(self as *mut ::pair::PairOpenglTextureFilterOpenglTextureFilter, other as *mut ::pair::PairOpenglTextureFilterOpenglTextureFilter) }
  }
}

impl Drop for ::pair::PairOpenglTextureFilterOpenglTextureFilter {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPair_QOpenGLTexture_Filter_QOpenGLTexture_Filter_destructor(self as *mut ::pair::PairOpenglTextureFilterOpenglTextureFilter) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PairCDoubleColor::new](../struct.PairCDoubleColor.html#method.new) method.
  pub trait PairCDoubleColorNewArgs {
    fn exec(self) -> ::pair::PairCDoubleColor;
  }
  impl PairCDoubleColorNewArgs for () {
    fn exec(self) -> ::pair::PairCDoubleColor {

      {
        let mut object: ::pair::PairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_double_QColor_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairCDoubleColorNewArgs for (&'a ::libc::c_double, &'a ::color::Color) {
    fn exec(self) -> ::pair::PairCDoubleColor {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_double_QColor_constructor_t1_t2(t1 as *const ::libc::c_double,
                                                                t2 as *const ::color::Color,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PairCFloatCFloat::new](../struct.PairCFloatCFloat.html#method.new) method.
  pub trait PairCFloatCFloatNewArgs {
    fn exec(self) -> ::pair::PairCFloatCFloat;
  }
  impl PairCFloatCFloatNewArgs for () {
    fn exec(self) -> ::pair::PairCFloatCFloat {

      {
        let mut object: ::pair::PairCFloatCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_float_float_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairCFloatCFloatNewArgs for (&'a ::libc::c_float, &'a ::libc::c_float) {
    fn exec(self) -> ::pair::PairCFloatCFloat {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairCFloatCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_float_float_constructor_t1_t2(t1 as *const ::libc::c_float,
                                                              t2 as *const ::libc::c_float,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PairCIntCInt::new](../struct.PairCIntCInt.html#method.new) method.
  pub trait PairCIntCIntNewArgs {
    fn exec(self) -> ::pair::PairCIntCInt;
  }
  impl PairCIntCIntNewArgs for () {
    fn exec(self) -> ::pair::PairCIntCInt {

      {
        let mut object: ::pair::PairCIntCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_int_int_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairCIntCIntNewArgs for (&'a ::libc::c_int, &'a ::libc::c_int) {
    fn exec(self) -> ::pair::PairCIntCInt {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairCIntCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_int_int_constructor_t1_t2(t1 as *const ::libc::c_int,
                                                          t2 as *const ::libc::c_int,
                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PairOpenglTextureFilterOpenglTextureFilter::new](../struct.PairOpenglTextureFilterOpenglTextureFilter.html#method.new) method.
  pub trait PairOpenglTextureFilterOpenglTextureFilterNewArgs {
    fn exec(self) -> ::pair::PairOpenglTextureFilterOpenglTextureFilter;
  }
  impl PairOpenglTextureFilterOpenglTextureFilterNewArgs for () {
    fn exec(self) -> ::pair::PairOpenglTextureFilterOpenglTextureFilter {

      {
        let mut object: ::pair::PairOpenglTextureFilterOpenglTextureFilter =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_QOpenGLTexture_Filter_QOpenGLTexture_Filter_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairOpenglTextureFilterOpenglTextureFilterNewArgs
    for (&'a ::opengl_texture::Filter, &'a ::opengl_texture::Filter) {
    fn exec(self) -> ::pair::PairOpenglTextureFilterOpenglTextureFilter {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairOpenglTextureFilterOpenglTextureFilter =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPair_QOpenGLTexture_Filter_QOpenGLTexture_Filter_constructor_t1_t2(t1 as *const ::opengl_texture::Filter, t2 as *const ::opengl_texture::Filter, &mut object);
        }
        object
      }
    }
  }
}
