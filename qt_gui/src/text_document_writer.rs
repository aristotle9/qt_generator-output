/// C++ type: <span style='color: green;'>```QTextDocumentWriter```</span>
#[repr(C)]
pub struct TextDocumentWriter([u8; ::type_sizes::QT_GUI_TEXT_DOCUMENT_WRITER_TEXT_DOCUMENT_WRITER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextDocumentWriter {
  unsafe fn new_uninitialized() -> TextDocumentWriter {
    TextDocumentWriter(::std::mem::uninitialized())
  }
}

impl TextDocumentWriter {
  /// C++ method: <span style='color: green;'>```QTextCodec* QTextDocumentWriter::codec() const```</span>
  ///
  ///
  pub fn codec(&self) -> *mut ::qt_core::text_codec::TextCodec {
    unsafe { ::ffi::qt_gui_c_QTextDocumentWriter_codec(self as *const ::text_document_writer::TextDocumentWriter) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QTextDocumentWriter::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::qt_core::io_device::IODevice {
    unsafe { ::ffi::qt_gui_c_QTextDocumentWriter_device(self as *const ::text_document_writer::TextDocumentWriter) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextDocumentWriter::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocumentWriter_fileName_to_output(self as *const ::text_document_writer::TextDocumentWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QTextDocumentWriter::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocumentWriter_format_to_output(self as *const ::text_document_writer::TextDocumentWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocumentWriter::QTextDocumentWriter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_document_writer::TextDocumentWriter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentWriter::QTextDocumentWriter()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::text_document_writer::TextDocumentWriter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentWriter::QTextDocumentWriter(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::qt_core::byte_array::ByteArray)) -> ::text_document_writer::TextDocumentWriter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentWriter::QTextDocumentWriter(const QString& fileName, const QByteArray& format = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_document_writer::TextDocumentWriter
    where Args: overloading::TextDocumentWriterNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentWriter::QTextDocumentWriter(QIODevice* device, const QByteArray& format)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(device: *mut ::qt_core::io_device::IODevice,
                           format: &::qt_core::byte_array::ByteArray)
                           -> ::text_document_writer::TextDocumentWriter {
    {
      let mut object: ::text_document_writer::TextDocumentWriter =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextDocumentWriter_constructor_device_format(device, format as *const ::qt_core::byte_array::ByteArray, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocumentWriter::setCodec(QTextCodec* codec)```</span>
  ///
  ///
  pub unsafe fn set_codec(&mut self, codec: *mut ::qt_core::text_codec::TextCodec) {
    ::ffi::qt_gui_c_QTextDocumentWriter_setCodec(self as *mut ::text_document_writer::TextDocumentWriter,
                                                 codec)
  }

  /// C++ method: <span style='color: green;'>```void QTextDocumentWriter::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::qt_core::io_device::IODevice) {
    ::ffi::qt_gui_c_QTextDocumentWriter_setDevice(self as *mut ::text_document_writer::TextDocumentWriter,
                                                  device)
  }

  /// C++ method: <span style='color: green;'>```void QTextDocumentWriter::setFileName(const QString& fileName)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, file_name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocumentWriter_setFileName(self as *mut ::text_document_writer::TextDocumentWriter,
                                                      file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocumentWriter::setFormat(const QByteArray& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocumentWriter_setFormat(self as *mut ::text_document_writer::TextDocumentWriter,
                                                    format as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QTextDocumentWriter::supportedDocumentFormats()```</span>
  ///
  ///
  pub fn supported_document_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocumentWriter_supportedDocumentFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocumentWriter::write(const QTextDocumentFragment& fragment)```</span>
  ///
  ///
  pub fn write(&mut self, fragment: &::text_document_fragment::TextDocumentFragment) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocumentWriter_write_fragment(self as *mut ::text_document_writer::TextDocumentWriter, fragment as *const ::text_document_fragment::TextDocumentFragment) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocumentWriter::write(const QTextDocument* document)```</span>
  ///
  ///
  pub unsafe fn write_unsafe(&mut self, document: *const ::text_document::TextDocument) -> bool {
    ::ffi::qt_gui_c_QTextDocumentWriter_write_document(self as *mut ::text_document_writer::TextDocumentWriter,
                                                       document)
  }
}

impl Drop for ::text_document_writer::TextDocumentWriter {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextDocumentWriter::~QTextDocumentWriter()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextDocumentWriter_destructor(self as *mut ::text_document_writer::TextDocumentWriter) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextDocumentWriter::new](../struct.TextDocumentWriter.html#method.new) method.
  pub trait TextDocumentWriterNewArgs {
    fn exec(self) -> ::text_document_writer::TextDocumentWriter;
  }
  impl<'a> TextDocumentWriterNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::text_document_writer::TextDocumentWriter {
      let file_name = self;
      {
        let mut object: ::text_document_writer::TextDocumentWriter =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentWriter_constructor_fileName(file_name as *const ::qt_core::string::String,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'a> TextDocumentWriterNewArgs for (&'a ::qt_core::string::String, &'a ::qt_core::byte_array::ByteArray) {
    fn exec(self) -> ::text_document_writer::TextDocumentWriter {
      let file_name = self.0;
      let format = self.1;
      {
        let mut object: ::text_document_writer::TextDocumentWriter =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentWriter_constructor_fileName_format(file_name as *const ::qt_core::string::String, format as *const ::qt_core::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl TextDocumentWriterNewArgs for () {
    fn exec(self) -> ::text_document_writer::TextDocumentWriter {

      {
        let mut object: ::text_document_writer::TextDocumentWriter =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentWriter_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}
