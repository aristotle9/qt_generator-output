/// C++ type: <span style='color: green;'>```QPdfWriter```</span>
#[repr(C)]
pub struct PdfWriter(u8);

impl PdfWriter {
  /// C++ method: <span style='color: green;'>```QString QPdfWriter::creator() const```</span>
  ///
  ///
  pub fn creator(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPdfWriter_creator_to_output(self as *const ::pdf_writer::PdfWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPdfWriter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QPdfWriter_metaObject(self as *const ::pdf_writer::PdfWriter) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPdfWriter::QPdfWriter(const QString& filename)```</span>
  ///
  ///
  pub fn new(filename: &::qt_core::string::String) -> ::cpp_utils::CppBox<::pdf_writer::PdfWriter> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPdfWriter_new_filename(filename as *const ::qt_core::string::String) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QPdfWriter::newPage()```</span>
  ///
  ///
  pub fn new_page(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPdfWriter_newPage(self as *mut ::pdf_writer::PdfWriter) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPdfWriter::QPdfWriter(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(device: *mut ::qt_core::io_device::IODevice)
                           -> ::cpp_utils::CppBox<::pdf_writer::PdfWriter> {
    let ffi_result = ::ffi::qt_gui_c_QPdfWriter_new_device(device);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QPdfWriter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QPdfWriter_qt_metacall(self as *mut ::pdf_writer::PdfWriter,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPdfWriter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QPdfWriter_qt_metacast(self as *mut ::pdf_writer::PdfWriter, arg1)
  }

  /// C++ method: <span style='color: green;'>```int QPdfWriter::resolution() const```</span>
  ///
  ///
  pub fn resolution(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPdfWriter_resolution(self as *const ::pdf_writer::PdfWriter) }
  }

  /// C++ method: <span style='color: green;'>```void QPdfWriter::setCreator(const QString& creator)```</span>
  ///
  ///
  pub fn set_creator(&mut self, creator: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QPdfWriter_setCreator(self as *mut ::pdf_writer::PdfWriter,
                                            creator as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QPdfWriter::setMargins(const QPagedPaintDevice::Margins& m)```</span>
  ///
  ///
  pub fn set_margins(&mut self, m: &::paged_paint_device::Margins) {
    unsafe {
      ::ffi::qt_gui_c_QPdfWriter_setMargins(self as *mut ::pdf_writer::PdfWriter,
                                            m as *const ::paged_paint_device::Margins)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QPdfWriter::setPageSize(QPagedPaintDevice::PageSize size)```</span>
  ///
  ///
  pub fn set_page_size(&mut self, size: ::paged_paint_device::PageSize) {
    unsafe { ::ffi::qt_gui_c_QPdfWriter_setPageSize(self as *mut ::pdf_writer::PdfWriter, size) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QPdfWriter::setPageSizeMM(const QSizeF& size)```</span>
  ///
  ///
  pub fn set_page_size_m_m(&mut self, size: &::qt_core::size_f::SizeF) {
    unsafe {
      ::ffi::qt_gui_c_QPdfWriter_setPageSizeMM(self as *mut ::pdf_writer::PdfWriter,
                                               size as *const ::qt_core::size_f::SizeF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPdfWriter::setResolution(int resolution)```</span>
  ///
  ///
  pub fn set_resolution(&mut self, resolution: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QPdfWriter_setResolution(self as *mut ::pdf_writer::PdfWriter, resolution) }
  }

  /// C++ method: <span style='color: green;'>```void QPdfWriter::setTitle(const QString& title)```</span>
  ///
  ///
  pub fn set_title(&mut self, title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QPdfWriter_setTitle(self as *mut ::pdf_writer::PdfWriter,
                                          title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QPdfWriter::title() const```</span>
  ///
  ///
  pub fn title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPdfWriter_title_to_output(self as *const ::pdf_writer::PdfWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPdfWriter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPdfWriter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPdfWriter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPdfWriter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::pdf_writer::PdfWriter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPdfWriter_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PdfWriter`.
  pub struct Signals<'a>(&'a ::pdf_writer::PdfWriter);
  /// Represents a built-in Qt signal `QPdfWriter::objectNameChanged`.
  ///
  /// An object of this type can be created from `PdfWriter` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PdfWriter` object.
  pub struct ObjectNameChanged<'a>(&'a ::pdf_writer::PdfWriter);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPdfWriter::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::pdf_writer::PdfWriter {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::pdf_writer::PdfWriter> for ::paged_paint_device::PagedPaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pdf_writer::PdfWriter> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPagedPaintDevice(self as *mut ::paged_paint_device::PagedPaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pdf_writer::PdfWriter> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPagedPaintDevice(self as *const ::paged_paint_device::PagedPaintDevice as *mut ::paged_paint_device::PagedPaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::pdf_writer::PdfWriter> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pdf_writer::PdfWriter> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pdf_writer::PdfWriter> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::pdf_writer::PdfWriter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QObject_ptr(self as *mut ::pdf_writer::PdfWriter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QObject_ptr(self as *const ::pdf_writer::PdfWriter as *mut ::pdf_writer::PdfWriter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::paged_paint_device::PagedPaintDevice> for ::pdf_writer::PdfWriter {
  fn static_cast_mut(&mut self) -> &mut ::paged_paint_device::PagedPaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPagedPaintDevice_ptr(self as *mut ::pdf_writer::PdfWriter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paged_paint_device::PagedPaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPagedPaintDevice_ptr(self as *const ::pdf_writer::PdfWriter as *mut ::pdf_writer::PdfWriter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::pdf_writer::PdfWriter {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPaintDevice_ptr(self as *mut ::pdf_writer::PdfWriter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPaintDevice_ptr(self as *const ::pdf_writer::PdfWriter as *mut ::pdf_writer::PdfWriter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pdf_writer::PdfWriter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pdf_writer::PdfWriter {
    let ffi_result =
      ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pdf_writer::PdfWriter {
    let ffi_result = ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pdf_writer::PdfWriter> for ::paged_paint_device::PagedPaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pdf_writer::PdfWriter {
    let ffi_result = ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPagedPaintDevice(self as *mut ::paged_paint_device::PagedPaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pdf_writer::PdfWriter {
    let ffi_result = ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPagedPaintDevice(self as *const ::paged_paint_device::PagedPaintDevice as *mut ::paged_paint_device::PagedPaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pdf_writer::PdfWriter> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pdf_writer::PdfWriter {
    let ffi_result =
      ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pdf_writer::PdfWriter {
    let ffi_result = ::ffi::qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}
