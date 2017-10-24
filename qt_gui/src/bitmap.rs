/// C++ type: <span style='color: green;'>```QBitmap```</span>
#[repr(C)]
pub struct Bitmap(u8);

impl Bitmap {
  /// C++ method: <span style='color: green;'>```QVariant QBitmap::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QBitmap_convert_to_QVariant_to_output(self as *const ::bitmap::Bitmap, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QBitmap::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QBitmap_clear(self as *mut ::bitmap::Bitmap) }
  }

  /// C++ method: <span style='color: green;'>```QBitmap::fromData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_data((&::qt_core::size::Size, *const ::libc::c_uchar)) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```static QBitmap QBitmap::fromData(const QSize& size, const unsigned char* bits)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_data((&::qt_core::size::Size, *const ::libc::c_uchar, &::image::Format)) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```static QBitmap QBitmap::fromData(const QSize& size, const unsigned char* bits, QImage::Format monoFormat = ?)```</span>
  ///
  ///
  pub unsafe fn from_data<Args>(args: Args) -> ::cpp_utils::CppBox<::bitmap::Bitmap>
    where Args: overloading::BitmapFromDataArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QBitmap::QBitmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitmap::QBitmap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::bitmap::Bitmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitmap::QBitmap(const QBitmap& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::pixmap::Pixmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitmap::QBitmap(const QPixmap& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::size::Size) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitmap::QBitmap(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitmap::QBitmap(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBitmap::QBitmap(int w, int h)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::bitmap::Bitmap>
    where Args: overloading::BitmapNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QBitmap::QBitmap(const QString& fileName, const char* format = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(file_name: &::qt_core::string::String,
                           format: *const ::libc::c_char)
                           -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
    let ffi_result = ::ffi::qt_gui_c_QBitmap_new_QString_char(file_name as *const ::qt_core::string::String, format);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QBitmap::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::bitmap::Bitmap) -> &'l0 mut ::bitmap::Bitmap```<br>
  /// C++ method: <span style='color: green;'>```QBitmap& QBitmap::operator=(const QBitmap& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::pixmap::Pixmap) -> &'l0 mut ::bitmap::Bitmap```<br>
  /// C++ method: <span style='color: green;'>```QBitmap& QBitmap::operator=(const QPixmap& arg1)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::bitmap::Bitmap
    where Args: overloading::BitmapOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QBitmap::swap(QBitmap& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::bitmap::Bitmap) {
    unsafe {
      ::ffi::qt_gui_c_QBitmap_swap(self as *mut ::bitmap::Bitmap,
                                   other as *mut ::bitmap::Bitmap)
    }
  }

  /// C++ method: <span style='color: green;'>```QBitmap::transformed```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn transformed(&self, &::matrix::Matrix) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```QBitmap QBitmap::transformed(const QMatrix& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn transformed(&self, &::transform::Transform) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```QBitmap QBitmap::transformed(const QTransform& matrix) const```</span>
  ///
  ///
  pub fn transformed<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::bitmap::Bitmap>
    where Args: overloading::BitmapTransformedArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::bitmap::Bitmap {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QBitmap_delete
  }
}

/// C++ method: <span style='color: green;'>```void swap(QBitmap& value1, QBitmap& value2)```</span>
///
///
pub fn swap(value1: &mut ::bitmap::Bitmap, value2: &mut ::bitmap::Bitmap) {
  unsafe {
    ::ffi::qt_gui_c_QBitmap_G_swap(value1 as *mut ::bitmap::Bitmap,
                                   value2 as *mut ::bitmap::Bitmap)
  }
}

impl ::cpp_utils::DynamicCast<::bitmap::Bitmap> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::bitmap::Bitmap> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::bitmap::Bitmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::bitmap::Bitmap> for ::pixmap::Pixmap {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::bitmap::Bitmap> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPixmap(self as *mut ::pixmap::Pixmap) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::bitmap::Bitmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPixmap(self as *const ::pixmap::Pixmap as *mut ::pixmap::Pixmap) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::bitmap::Bitmap {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_G_static_cast_QPaintDevice_ptr(self as *mut ::bitmap::Bitmap) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QBitmap_G_static_cast_QPaintDevice_ptr(self as *const ::bitmap::Bitmap as *mut ::bitmap::Bitmap)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::pixmap::Pixmap> for ::bitmap::Bitmap {
  fn static_cast_mut(&mut self) -> &mut ::pixmap::Pixmap {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_G_static_cast_QPixmap_ptr(self as *mut ::bitmap::Bitmap) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::pixmap::Pixmap {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QBitmap_G_static_cast_QPixmap_ptr(self as *const ::bitmap::Bitmap as *mut ::bitmap::Bitmap)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::bitmap::Bitmap> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::bitmap::Bitmap {
    let ffi_result =
      ::ffi::qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::bitmap::Bitmap {
    let ffi_result = ::ffi::qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::bitmap::Bitmap> for ::pixmap::Pixmap {
  unsafe fn static_cast_mut(&mut self) -> &mut ::bitmap::Bitmap {
    let ffi_result = ::ffi::qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPixmap(self as *mut ::pixmap::Pixmap);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::bitmap::Bitmap {
    let ffi_result = ::ffi::qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPixmap(self as *const ::pixmap::Pixmap as *mut ::pixmap::Pixmap);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::bitmap::Bitmap {
  type Target = ::pixmap::Pixmap;
  fn deref(&self) -> &::pixmap::Pixmap {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QBitmap_G_static_cast_QPixmap_ptr(self as *const ::bitmap::Bitmap as *mut ::bitmap::Bitmap)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::bitmap::Bitmap {
  fn deref_mut(&mut self) -> &mut ::pixmap::Pixmap {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_G_static_cast_QPixmap_ptr(self as *mut ::bitmap::Bitmap) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Bitmap::from_data](../struct.Bitmap.html#method.from_data) method.
  pub trait BitmapFromDataArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap>;
  }
  impl<'a> BitmapFromDataArgs for (&'a ::qt_core::size::Size, *const ::libc::c_uchar) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let size = self.0;
      let bits = self.1;
      let ffi_result = ::ffi::qt_gui_c_QBitmap_fromData_as_ptr_size_bits(size as *const ::qt_core::size::Size, bits);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> BitmapFromDataArgs for (&'a ::qt_core::size::Size, *const ::libc::c_uchar, &'a ::image::Format) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let size = self.0;
      let bits = self.1;
      let mono_format = self.2;
      let ffi_result =
        ::ffi::qt_gui_c_QBitmap_fromData_as_ptr_size_bits_monoFormat(size as *const ::qt_core::size::Size,
                                                                     bits,
                                                                     mono_format as *const ::image::Format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Bitmap::new](../struct.Bitmap.html#method.new) method.
  pub trait BitmapNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap>;
  }
  impl<'a> BitmapNewArgs for &'a ::bitmap::Bitmap {
    fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_new_QBitmap(other as *const ::bitmap::Bitmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> BitmapNewArgs for &'a ::pixmap::Pixmap {
    fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_new_QPixmap(arg1 as *const ::pixmap::Pixmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> BitmapNewArgs for &'a ::qt_core::size::Size {
    fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_new_QSize(arg1 as *const ::qt_core::size::Size) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> BitmapNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let file_name = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_new_QString(file_name as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl BitmapNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let w = self.0;
      let h = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_new_int_int(w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl BitmapNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QBitmap_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Bitmap::op_assign](../struct.Bitmap.html#method.op_assign) method.
  pub trait BitmapOpAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::bitmap::Bitmap) -> &'largs mut ::bitmap::Bitmap;
  }
  impl<'largs> BitmapOpAssignArgs<'largs> for &'largs ::pixmap::Pixmap {
    fn exec(self, original_self: &'largs mut ::bitmap::Bitmap) -> &'largs mut ::bitmap::Bitmap {
      let arg1 = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QBitmap_operator_assign_arg1(original_self as *mut ::bitmap::Bitmap,
                                                     arg1 as *const ::pixmap::Pixmap)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> BitmapOpAssignArgs<'largs> for &'largs ::bitmap::Bitmap {
    fn exec(self, original_self: &'largs mut ::bitmap::Bitmap) -> &'largs mut ::bitmap::Bitmap {
      let other = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QBitmap_operator_assign_other(original_self as *mut ::bitmap::Bitmap,
                                                      other as *const ::bitmap::Bitmap)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Bitmap::transformed](../struct.Bitmap.html#method.transformed) method.
  pub trait BitmapTransformedArgs<'largs> {
    fn exec(self, original_self: &'largs ::bitmap::Bitmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap>;
  }
  impl<'largs> BitmapTransformedArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs ::bitmap::Bitmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let arg1 = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QBitmap_transformed_as_ptr_arg1(original_self as *const ::bitmap::Bitmap,
                                                        arg1 as *const ::matrix::Matrix)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> BitmapTransformedArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs ::bitmap::Bitmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let matrix = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QBitmap_transformed_as_ptr_matrix(original_self as *const ::bitmap::Bitmap,
                                                          matrix as *const ::transform::Transform)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
