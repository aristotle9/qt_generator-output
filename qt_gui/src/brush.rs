/// C++ type: <span style='color: green;'>```QBrush```</span>
#[repr(C)]
pub struct Brush([u8; ::type_sizes::QT_GUI_BRUSH_BRUSH]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Brush {
  unsafe fn new_uninitialized() -> Brush {
    Brush(::std::mem::uninitialized())
  }
}

impl Brush {
  /// C++ method: <span style='color: green;'>```QVariant QBrush::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QBrush_convert_to_QVariant_to_output(self as *const ::brush::Brush, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QColor& QBrush::color() const```</span>
  ///
  ///
  pub fn color<'l0>(&'l0 self) -> &'l0 ::color::Color {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBrush_color(self as *const ::brush::Brush) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QGradient* QBrush::gradient() const```</span>
  ///
  ///
  pub fn gradient(&self) -> *const ::gradient::Gradient {
    unsafe { ::ffi::qt_gui_c_QBrush_gradient(self as *const ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```bool QBrush::isDetached() const```</span>
  ///
  ///
  pub fn is_detached(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QBrush_isDetached(self as *const ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```bool QBrush::isOpaque() const```</span>
  ///
  ///
  pub fn is_opaque(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QBrush_isOpaque(self as *const ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```const QMatrix& QBrush::matrix() const```</span>
  ///
  ///
  pub fn matrix<'l0>(&'l0 self) -> &'l0 ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBrush_matrix(self as *const ::brush::Brush) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QBrush::QBrush```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::BrushStyle) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(Qt::BrushStyle bs)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::GlobalColor) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(Qt::GlobalColor color)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::qt_core::qt::GlobalColor, &::qt_core::qt::BrushStyle)) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(Qt::GlobalColor color, Qt::BrushStyle bs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_core::qt::GlobalColor, &::pixmap::Pixmap)) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(Qt::GlobalColor color, const QPixmap& pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(&::brush::Brush) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(const QBrush& brush)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::color::Color) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((&::color::Color, &::qt_core::qt::BrushStyle)) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(const QColor& color, Qt::BrushStyle bs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((&::color::Color, &::pixmap::Pixmap)) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(const QColor& color, const QPixmap& pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new(&::gradient::Gradient) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(const QGradient& gradient)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn new(&::image::Image) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(const QImage& image)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn new(&::pixmap::Pixmap) -> ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBrush::QBrush(const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::brush::Brush
    where Args: overloading::BrushNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QBrush& QBrush::operator=(const QBrush& brush)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, brush: &'l1 ::brush::Brush) -> &'l0 mut ::brush::Brush {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QBrush_operator_assign(self as *mut ::brush::Brush, brush as *const ::brush::Brush) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QBrush::operator==(const QBrush& b) const```</span>
  ///
  ///
  pub fn op_eq(&self, b: &::brush::Brush) -> bool {
    unsafe { ::ffi::qt_gui_c_QBrush_operator_eq(self as *const ::brush::Brush, b as *const ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```bool QBrush::operator!=(const QBrush& b) const```</span>
  ///
  ///
  pub fn op_neq(&self, b: &::brush::Brush) -> bool {
    unsafe { ::ffi::qt_gui_c_QBrush_operator_neq(self as *const ::brush::Brush, b as *const ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```QBrush::setColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_color(&mut self, &::qt_core::qt::GlobalColor) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBrush::setColor(Qt::GlobalColor color)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_color(&mut self, &::color::Color) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBrush::setColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_color<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BrushSetColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QBrush::setMatrix(const QMatrix& mat)```</span>
  ///
  ///
  pub fn set_matrix(&mut self, mat: &::matrix::Matrix) {
    unsafe { ::ffi::qt_gui_c_QBrush_setMatrix(self as *mut ::brush::Brush, mat as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```void QBrush::setStyle(Qt::BrushStyle arg1)```</span>
  ///
  ///
  pub fn set_style(&mut self, arg1: &::qt_core::qt::BrushStyle) {
    unsafe {
      ::ffi::qt_gui_c_QBrush_setStyle(self as *mut ::brush::Brush,
                                      arg1 as *const ::qt_core::qt::BrushStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBrush::setTexture(const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn set_texture(&mut self, pixmap: &::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_gui_c_QBrush_setTexture(self as *mut ::brush::Brush,
                                        pixmap as *const ::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBrush::setTextureImage(const QImage& image)```</span>
  ///
  ///
  pub fn set_texture_image(&mut self, image: &::image::Image) {
    unsafe { ::ffi::qt_gui_c_QBrush_setTextureImage(self as *mut ::brush::Brush, image as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```void QBrush::setTransform(const QTransform& arg1)```</span>
  ///
  ///
  pub fn set_transform(&mut self, arg1: &::transform::Transform) {
    unsafe {
      ::ffi::qt_gui_c_QBrush_setTransform(self as *mut ::brush::Brush,
                                          arg1 as *const ::transform::Transform)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBrush::swap(QBrush& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::brush::Brush) {
    unsafe { ::ffi::qt_gui_c_QBrush_swap(self as *mut ::brush::Brush, other as *mut ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap QBrush::texture() const```</span>
  ///
  ///
  pub fn texture(&self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBrush_texture_as_ptr(self as *const ::brush::Brush) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QImage QBrush::textureImage() const```</span>
  ///
  ///
  pub fn texture_image(&self) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBrush_textureImage_as_ptr(self as *const ::brush::Brush) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QTransform QBrush::transform() const```</span>
  ///
  ///
  pub fn transform(&self) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QBrush_transform_to_output(self as *const ::brush::Brush, &mut object);
      }
      object
    }
  }
}

impl Drop for ::brush::Brush {
  /// C++ method: <span style='color: green;'>```[destructor] void QBrush::~QBrush()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QBrush_destructor(self as *mut ::brush::Brush) }
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::brush::Brush)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QBrush& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::brush::Brush)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QBrush& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QBrush& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::brush::Brush)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QBrush_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                          arg2 as *mut ::brush::Brush)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QBrush& value1, QBrush& value2)```</span>
///
///
pub fn swap(value1: &mut ::brush::Brush, value2: &mut ::brush::Brush) {
  unsafe { ::ffi::qt_gui_c_QBrush_G_swap(value1 as *mut ::brush::Brush, value2 as *mut ::brush::Brush) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Brush::new](../struct.Brush.html#method.new) method.
  pub trait BrushNewArgs {
    fn exec(self) -> ::brush::Brush;
  }
  impl<'a> BrushNewArgs for &'a ::brush::Brush {
    fn exec(self) -> ::brush::Brush {
      let brush = self;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_QBrush(brush as *const ::brush::Brush, &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for &'a ::color::Color {
    fn exec(self) -> ::brush::Brush {
      let color = self;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_QColor(color as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for (&'a ::color::Color, &'a ::pixmap::Pixmap) {
    fn exec(self) -> ::brush::Brush {
      let color = self.0;
      let pixmap = self.1;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_QColor_QPixmap(color as *const ::color::Color,
                                                            pixmap as *const ::pixmap::Pixmap,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for (&'a ::color::Color, &'a ::qt_core::qt::BrushStyle) {
    fn exec(self) -> ::brush::Brush {
      let color = self.0;
      let bs = self.1;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_QColor_Qt_BrushStyle(color as *const ::color::Color,
                                                                  bs as *const ::qt_core::qt::BrushStyle,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for &'a ::gradient::Gradient {
    fn exec(self) -> ::brush::Brush {
      let gradient = self;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_QGradient(gradient as *const ::gradient::Gradient, &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for &'a ::image::Image {
    fn exec(self) -> ::brush::Brush {
      let image = self;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_QImage(image as *const ::image::Image, &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for &'a ::pixmap::Pixmap {
    fn exec(self) -> ::brush::Brush {
      let pixmap = self;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_QPixmap(pixmap as *const ::pixmap::Pixmap, &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for &'a ::qt_core::qt::BrushStyle {
    fn exec(self) -> ::brush::Brush {
      let bs = self;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_Qt_BrushStyle(bs as *const ::qt_core::qt::BrushStyle, &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for &'a ::qt_core::qt::GlobalColor {
    fn exec(self) -> ::brush::Brush {
      let color = self;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_Qt_GlobalColor(color as *const ::qt_core::qt::GlobalColor, &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for (&'a ::qt_core::qt::GlobalColor, &'a ::pixmap::Pixmap) {
    fn exec(self) -> ::brush::Brush {
      let color = self.0;
      let pixmap = self.1;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_Qt_GlobalColor_QPixmap(color as *const ::qt_core::qt::GlobalColor,
                                                                    pixmap as *const ::pixmap::Pixmap,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'a> BrushNewArgs for (&'a ::qt_core::qt::GlobalColor, &'a ::qt_core::qt::BrushStyle) {
    fn exec(self) -> ::brush::Brush {
      let color = self.0;
      let bs = self.1;
      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_Qt_GlobalColor_Qt_BrushStyle(color as *const ::qt_core::qt::GlobalColor,
                                                                          bs as *const ::qt_core::qt::BrushStyle,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl BrushNewArgs for () {
    fn exec(self) -> ::brush::Brush {

      {
        let mut object: ::brush::Brush =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Brush::set_color](../struct.Brush.html#method.set_color) method.
  pub trait BrushSetColorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::brush::Brush) -> ();
  }
  impl<'largs> BrushSetColorArgs<'largs> for &'largs ::color::Color {
    fn exec(self, original_self: &'largs mut ::brush::Brush) -> () {
      let color = self;
      unsafe {
        ::ffi::qt_gui_c_QBrush_setColor_QColor(original_self as *mut ::brush::Brush,
                                               color as *const ::color::Color)
      }
    }
  }
  impl<'largs> BrushSetColorArgs<'largs> for &'largs ::qt_core::qt::GlobalColor {
    fn exec(self, original_self: &'largs mut ::brush::Brush) -> () {
      let color = self;
      unsafe {
        ::ffi::qt_gui_c_QBrush_setColor_Qt_GlobalColor(original_self as *mut ::brush::Brush,
                                                       color as *const ::qt_core::qt::GlobalColor)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::brush::Brush) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QBrush_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                              arg2 as *const ::brush::Brush)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::brush::Brush) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QBrush_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                          arg2 as *const ::brush::Brush,
                                                          &mut object);
        }
        object
      }
    }
  }
}
