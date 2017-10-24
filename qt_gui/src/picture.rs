/// C++ type: <span style='color: green;'>```QPicture```</span>
#[repr(C)]
pub struct Picture(u8);

impl Picture {
  /// C++ method: <span style='color: green;'>```QRect QPicture::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPicture_boundingRect_to_output(self as *const ::picture::Picture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const char* QPicture::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_gui_c_QPicture_data(self as *const ::picture::Picture) }
  }

  /// C++ method: <span style='color: green;'>```void QPicture::detach()```</span>
  ///
  ///
  pub fn detach(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPicture_detach(self as *mut ::picture::Picture) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPicture::devType() const```</span>
  ///
  ///
  pub fn dev_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPicture_devType(self as *const ::picture::Picture) }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QPicture::inputFormatList()```</span>
  ///
  ///
  pub fn input_format_list() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPicture_inputFormatList_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QPicture::inputFormats()```</span>
  ///
  ///
  pub fn input_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPicture_inputFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPicture::isDetached() const```</span>
  ///
  ///
  pub fn is_detached(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPicture_isDetached(self as *const ::picture::Picture) }
  }

  /// C++ method: <span style='color: green;'>```bool QPicture::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPicture_isNull(self as *const ::picture::Picture) }
  }

  /// C++ method: <span style='color: green;'>```bool QPicture::load(const QString& fileName)```</span>
  ///
  ///
  pub fn load(&mut self, file_name: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPicture_load_fileName(self as *mut ::picture::Picture,
                                             file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QPicture::load```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn load_unsafe(&mut self, *mut ::qt_core::io_device::IODevice) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPicture::load(QIODevice* dev)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn load_unsafe(&mut self, (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPicture::load(QIODevice* dev, const char* format = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn load_unsafe(&mut self, (&::qt_core::string::String, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPicture::load(const QString& fileName, const char* format = ?)```</span>
  ///
  ///
  pub unsafe fn load_unsafe<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::PictureLoadUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPicture::QPicture```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::picture::Picture>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPicture::QPicture()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::picture::Picture) -> ::cpp_utils::CppBox<::picture::Picture>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPicture::QPicture(const QPicture& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::picture::Picture>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPicture::QPicture(int formatVersion = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::picture::Picture>
    where Args: overloading::PictureNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPicture& QPicture::operator=(const QPicture& p)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, p: &'l1 ::picture::Picture) -> &'l0 mut ::picture::Picture {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPicture_operator_assign(self as *mut ::picture::Picture,
                                               p as *const ::picture::Picture)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QStringList QPicture::outputFormatList()```</span>
  ///
  ///
  pub fn output_format_list() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPicture_outputFormatList_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QPicture::outputFormats()```</span>
  ///
  ///
  pub fn output_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPicture_outputFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPaintEngine* QPicture::paintEngine() const```</span>
  ///
  ///
  pub fn paint_engine(&self) -> *mut ::paint_engine::PaintEngine {
    unsafe { ::ffi::qt_gui_c_QPicture_paintEngine(self as *const ::picture::Picture) }
  }

  /// C++ method: <span style='color: green;'>```static const char* QPicture::pictureFormat(const QString& fileName)```</span>
  ///
  ///
  pub fn picture_format(file_name: &::qt_core::string::String) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_gui_c_QPicture_pictureFormat(file_name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QPicture::play(QPainter* p)```</span>
  ///
  ///
  pub unsafe fn play(&mut self, p: *mut ::painter::Painter) -> bool {
    ::ffi::qt_gui_c_QPicture_play(self as *mut ::picture::Picture, p)
  }

  /// C++ method: <span style='color: green;'>```bool QPicture::save(const QString& fileName)```</span>
  ///
  ///
  pub fn save(&mut self, file_name: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPicture_save_fileName(self as *mut ::picture::Picture,
                                             file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QPicture::save```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn save_unsafe(&mut self, *mut ::qt_core::io_device::IODevice) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPicture::save(QIODevice* dev)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn save_unsafe(&mut self, (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPicture::save(QIODevice* dev, const char* format = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn save_unsafe(&mut self, (&::qt_core::string::String, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPicture::save(const QString& fileName, const char* format = ?)```</span>
  ///
  ///
  pub unsafe fn save_unsafe<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::PictureSaveUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPicture::setBoundingRect(const QRect& r)```</span>
  ///
  ///
  pub fn set_bounding_rect(&mut self, r: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QPicture_setBoundingRect(self as *mut ::picture::Picture,
                                               r as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QPicture::setData(const char* data, unsigned int size)```</span>
  ///
  ///
  pub unsafe fn set_data(&mut self, data: *const ::libc::c_char, size: ::libc::c_uint) {
    ::ffi::qt_gui_c_QPicture_setData(self as *mut ::picture::Picture, data, size)
  }

  /// C++ method: <span style='color: green;'>```unsigned int QPicture::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QPicture_size(self as *const ::picture::Picture) }
  }

  /// C++ method: <span style='color: green;'>```void QPicture::swap(QPicture& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::picture::Picture) {
    unsafe {
      ::ffi::qt_gui_c_QPicture_swap(self as *mut ::picture::Picture,
                                    other as *mut ::picture::Picture)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::picture::Picture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPicture_delete
  }
}

/// C++ method: <span style='color: green;'>```void swap(QPicture& value1, QPicture& value2)```</span>
///
///
pub fn swap(value1: &mut ::picture::Picture, value2: &mut ::picture::Picture) {
  unsafe {
    ::ffi::qt_gui_c_QPicture_G_swap(value1 as *mut ::picture::Picture,
                                    value2 as *mut ::picture::Picture)
  }
}

impl ::cpp_utils::DynamicCast<::picture::Picture> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::picture::Picture> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPicture_G_dynamic_cast_QPicture_ptr(self as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::picture::Picture> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPicture_G_dynamic_cast_QPicture_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::picture::Picture {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPicture_G_static_cast_QPaintDevice_ptr(self as *mut ::picture::Picture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPicture_G_static_cast_QPaintDevice_ptr(self as *const ::picture::Picture as *mut ::picture::Picture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::picture::Picture> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::picture::Picture {
    let ffi_result = ::ffi::qt_gui_c_QPicture_G_static_cast_QPicture_ptr(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::picture::Picture {
    let ffi_result = ::ffi::qt_gui_c_QPicture_G_static_cast_QPicture_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::picture::Picture {
  type Target = ::paint_device::PaintDevice;
  fn deref(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPicture_G_static_cast_QPaintDevice_ptr(self as *const ::picture::Picture as *mut ::picture::Picture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::picture::Picture {
  fn deref_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPicture_G_static_cast_QPaintDevice_ptr(self as *mut ::picture::Picture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Picture::load_unsafe](../struct.Picture.html#method.load_unsafe) method.
  pub trait PictureLoadUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool;
  }
  impl<'largs> PictureLoadUnsafeArgs<'largs> for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool {
      let dev = self;
      ::ffi::qt_gui_c_QPicture_load_dev(original_self as *mut ::picture::Picture, dev)
    }
  }
  impl<'largs> PictureLoadUnsafeArgs<'largs> for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool {
      let dev = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QPicture_load_dev_format(original_self as *mut ::picture::Picture, dev, format)
    }
  }
  impl<'largs> PictureLoadUnsafeArgs<'largs> for (&'largs ::qt_core::string::String, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool {
      let file_name = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QPicture_load_fileName_format(original_self as *mut ::picture::Picture,
                                                    file_name as *const ::qt_core::string::String,
                                                    format)
    }
  }
  /// This trait represents a set of arguments accepted by [Picture::new](../struct.Picture.html#method.new) method.
  pub trait PictureNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::picture::Picture>;
  }
  impl<'a> PictureNewArgs for &'a ::picture::Picture {
    fn exec(self) -> ::cpp_utils::CppBox<::picture::Picture> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPicture_new_arg1(arg1 as *const ::picture::Picture) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PictureNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::picture::Picture> {
      let format_version = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPicture_new_formatVersion(format_version) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PictureNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::picture::Picture> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QPicture_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Picture::save_unsafe](../struct.Picture.html#method.save_unsafe) method.
  pub trait PictureSaveUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool;
  }
  impl<'largs> PictureSaveUnsafeArgs<'largs> for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool {
      let dev = self;
      ::ffi::qt_gui_c_QPicture_save_dev(original_self as *mut ::picture::Picture, dev)
    }
  }
  impl<'largs> PictureSaveUnsafeArgs<'largs> for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool {
      let dev = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QPicture_save_dev_format(original_self as *mut ::picture::Picture, dev, format)
    }
  }
  impl<'largs> PictureSaveUnsafeArgs<'largs> for (&'largs ::qt_core::string::String, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::picture::Picture) -> bool {
      let file_name = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QPicture_save_fileName_format(original_self as *mut ::picture::Picture,
                                                    file_name as *const ::qt_core::string::String,
                                                    format)
    }
  }
}
