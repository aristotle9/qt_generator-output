/// C++ type: <span style='color: green;'>```QXmlStreamWriter```</span>
#[repr(C)]
pub struct XmlStreamWriter([u8; ::type_sizes::QT_CORE_XML_STREAM_WRITER_XML_STREAM_WRITER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamWriter {
  unsafe fn new_uninitialized() -> XmlStreamWriter {
    XmlStreamWriter(::std::mem::uninitialized())
  }
}

impl XmlStreamWriter {
  /// C++ method: <span style='color: green;'>```bool QXmlStreamWriter::autoFormatting() const```</span>
  ///
  ///
  pub fn auto_formatting(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_autoFormatting(self as *const ::xml_stream_writer::XmlStreamWriter) }
  }

  /// C++ method: <span style='color: green;'>```int QXmlStreamWriter::autoFormattingIndent() const```</span>
  ///
  ///
  pub fn auto_formatting_indent(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_autoFormattingIndent(self as *const ::xml_stream_writer::XmlStreamWriter)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCodec* QXmlStreamWriter::codec() const```</span>
  ///
  ///
  pub fn codec(&self) -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_codec(self as *const ::xml_stream_writer::XmlStreamWriter) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QXmlStreamWriter::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::io_device::IODevice {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_device(self as *const ::xml_stream_writer::XmlStreamWriter) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamWriter::hasError() const```</span>
  ///
  ///
  pub fn has_error(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_hasError(self as *const ::xml_stream_writer::XmlStreamWriter) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamWriter::QXmlStreamWriter()```</span>
  ///
  ///
  pub fn new() -> ::xml_stream_writer::XmlStreamWriter {
    {
      let mut object: ::xml_stream_writer::XmlStreamWriter =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamWriter_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::QXmlStreamWriter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::byte_array::ByteArray) -> ::xml_stream_writer::XmlStreamWriter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamWriter::QXmlStreamWriter(QByteArray* array)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::io_device::IODevice) -> ::xml_stream_writer::XmlStreamWriter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamWriter::QXmlStreamWriter(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::string::String) -> ::xml_stream_writer::XmlStreamWriter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamWriter::QXmlStreamWriter(QString* string)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::xml_stream_writer::XmlStreamWriter
    where Args: overloading::XmlStreamWriterNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::setAutoFormatting(bool arg1)```</span>
  ///
  ///
  pub fn set_auto_formatting(&mut self, arg1: bool) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_setAutoFormatting(self as *mut ::xml_stream_writer::XmlStreamWriter, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs)```</span>
  ///
  ///
  pub fn set_auto_formatting_indent(&mut self, spaces_or_tabs: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_setAutoFormattingIndent(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                                spaces_or_tabs)
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::setCodec```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_codec(&mut self, *mut ::text_codec::TextCodec) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::setCodec(QTextCodec* codec)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_codec(&mut self, *const ::libc::c_char) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::setCodec(const char* codecName)```</span>
  ///
  ///
  pub unsafe fn set_codec<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterSetCodecArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::io_device::IODevice) {
    ::ffi::qt_core_c_QXmlStreamWriter_setDevice(self as *mut ::xml_stream_writer::XmlStreamWriter, device)
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::writeAttribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_attribute(&mut self, (&::string::String, &::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeAttribute(const QString& namespaceUri, const QString& name, const QString& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_attribute(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeAttribute(const QString& qualifiedName, const QString& value)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn write_attribute(&mut self, &::xml_stream_attribute::XmlStreamAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeAttribute(const QXmlStreamAttribute& attribute)```</span>
  ///
  ///
  pub fn write_attribute<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterWriteAttributeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes& attributes)```</span>
  ///
  ///
  pub fn write_attributes(&mut self, attributes: &::xml_stream_attributes::XmlStreamAttributes) {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeAttributes(self as *mut ::xml_stream_writer::XmlStreamWriter, attributes as *const ::xml_stream_attributes::XmlStreamAttributes) }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeCDATA(const QString& text)```</span>
  ///
  ///
  pub fn write_c_d_a_t_a(&mut self, text: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_writeCDATA(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                   text as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeCharacters(const QString& text)```</span>
  ///
  ///
  pub fn write_characters(&mut self, text: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_writeCharacters(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                        text as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeComment(const QString& text)```</span>
  ///
  ///
  pub fn write_comment(&mut self, text: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_writeComment(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                     text as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader& reader)```</span>
  ///
  ///
  pub fn write_current_token(&mut self, reader: &::xml_stream_reader::XmlStreamReader) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_writeCurrentToken(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                          reader as *const ::xml_stream_reader::XmlStreamReader)
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeDTD(const QString& dtd)```</span>
  ///
  ///
  pub fn write_d_t_d(&mut self, dtd: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_writeDTD(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                 dtd as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeDefaultNamespace(const QString& namespaceUri)```</span>
  ///
  ///
  pub fn write_default_namespace(&mut self, namespace_uri: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_writeDefaultNamespace(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                              namespace_uri as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::writeEmptyElement```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_empty_element(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeEmptyElement(const QString& namespaceUri, const QString& name)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_empty_element(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeEmptyElement(const QString& qualifiedName)```</span>
  ///
  ///
  pub fn write_empty_element<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterWriteEmptyElementArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeEndDocument()```</span>
  ///
  ///
  pub fn write_end_document(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeEndDocument(self as *mut ::xml_stream_writer::XmlStreamWriter) }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeEndElement()```</span>
  ///
  ///
  pub fn write_end_element(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeEndElement(self as *mut ::xml_stream_writer::XmlStreamWriter) }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeEntityReference(const QString& name)```</span>
  ///
  ///
  pub fn write_entity_reference(&mut self, name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamWriter_writeEntityReference(self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                             name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::writeNamespace```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_namespace(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeNamespace(const QString& namespaceUri)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_namespace(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeNamespace(const QString& namespaceUri, const QString& prefix = ?)```</span>
  ///
  ///
  pub fn write_namespace<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterWriteNamespaceArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::writeProcessingInstruction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_processing_instruction(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeProcessingInstruction(const QString& target)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_processing_instruction(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeProcessingInstruction(const QString& target, const QString& data = ?)```</span>
  ///
  ///
  pub fn write_processing_instruction<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterWriteProcessingInstructionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::writeStartDocument```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_start_document(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeStartDocument()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_start_document(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeStartDocument(const QString& version)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn write_start_document(&mut self, (&::string::String, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeStartDocument(const QString& version, bool standalone)```</span>
  ///
  ///
  pub fn write_start_document<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterWriteStartDocumentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::writeStartElement```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_start_element(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeStartElement(const QString& namespaceUri, const QString& name)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_start_element(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeStartElement(const QString& qualifiedName)```</span>
  ///
  ///
  pub fn write_start_element<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterWriteStartElementArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamWriter::writeTextElement```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_text_element(&mut self, (&::string::String, &::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeTextElement(const QString& namespaceUri, const QString& name, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_text_element(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamWriter::writeTextElement(const QString& qualifiedName, const QString& text)```</span>
  ///
  ///
  pub fn write_text_element<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamWriterWriteTextElementArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::xml_stream_writer::XmlStreamWriter {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamWriter::~QXmlStreamWriter()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamWriter_destructor(self as *mut ::xml_stream_writer::XmlStreamWriter) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::new_unsafe](../struct.XmlStreamWriter.html#method.new_unsafe) method.
  pub trait XmlStreamWriterNewUnsafeArgs {
    unsafe fn exec(self) -> ::xml_stream_writer::XmlStreamWriter;
  }
  impl XmlStreamWriterNewUnsafeArgs for *mut ::byte_array::ByteArray {
    unsafe fn exec(self) -> ::xml_stream_writer::XmlStreamWriter {
      let array = self;
      {
        let mut object: ::xml_stream_writer::XmlStreamWriter =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QXmlStreamWriter_constructor_array(array, &mut object);
        object
      }
    }
  }
  impl XmlStreamWriterNewUnsafeArgs for *mut ::io_device::IODevice {
    unsafe fn exec(self) -> ::xml_stream_writer::XmlStreamWriter {
      let device = self;
      {
        let mut object: ::xml_stream_writer::XmlStreamWriter =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QXmlStreamWriter_constructor_device(device, &mut object);
        object
      }
    }
  }
  impl XmlStreamWriterNewUnsafeArgs for *mut ::string::String {
    unsafe fn exec(self) -> ::xml_stream_writer::XmlStreamWriter {
      let string = self;
      {
        let mut object: ::xml_stream_writer::XmlStreamWriter =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QXmlStreamWriter_constructor_string(string, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::set_codec](../struct.XmlStreamWriter.html#method.set_codec) method.
  pub trait XmlStreamWriterSetCodecArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterSetCodecArgs<'largs> for *mut ::text_codec::TextCodec {
    unsafe fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let codec = self;
      ::ffi::qt_core_c_QXmlStreamWriter_setCodec_codec(original_self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                       codec)
    }
  }
  impl<'largs> XmlStreamWriterSetCodecArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let codec_name = self;
      ::ffi::qt_core_c_QXmlStreamWriter_setCodec_codecName(original_self as *mut ::xml_stream_writer::XmlStreamWriter,
                                                           codec_name)
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::write_attribute](../struct.XmlStreamWriter.html#method.write_attribute) method.
  pub trait XmlStreamWriterWriteAttributeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterWriteAttributeArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let attribute = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeAttribute_attribute(original_self as *mut ::xml_stream_writer::XmlStreamWriter, attribute as *const ::xml_stream_attribute::XmlStreamAttribute) }
    }
  }
  impl<'largs> XmlStreamWriterWriteAttributeArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let namespace_uri = self.0;
      let name = self.1;
      let value = self.2;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeAttribute_namespaceUri_name_value(original_self as *mut ::xml_stream_writer::XmlStreamWriter, namespace_uri as *const ::string::String, name as *const ::string::String, value as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamWriterWriteAttributeArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let qualified_name = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeAttribute_qualifiedName_value(original_self as *mut ::xml_stream_writer::XmlStreamWriter, qualified_name as *const ::string::String, value as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::write_empty_element](../struct.XmlStreamWriter.html#method.write_empty_element) method.
  pub trait XmlStreamWriterWriteEmptyElementArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterWriteEmptyElementArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let namespace_uri = self.0;
      let name = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeEmptyElement_namespaceUri_name(original_self as *mut ::xml_stream_writer::XmlStreamWriter, namespace_uri as *const ::string::String, name as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamWriterWriteEmptyElementArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let qualified_name = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeEmptyElement_qualifiedName(original_self as *mut ::xml_stream_writer::XmlStreamWriter, qualified_name as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::write_namespace](../struct.XmlStreamWriter.html#method.write_namespace) method.
  pub trait XmlStreamWriterWriteNamespaceArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterWriteNamespaceArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let namespace_uri = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeNamespace_namespaceUri(original_self as *mut ::xml_stream_writer::XmlStreamWriter, namespace_uri as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamWriterWriteNamespaceArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let namespace_uri = self.0;
      let prefix = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeNamespace_namespaceUri_prefix(original_self as *mut ::xml_stream_writer::XmlStreamWriter, namespace_uri as *const ::string::String, prefix as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::write_processing_instruction](../struct.XmlStreamWriter.html#method.write_processing_instruction) method.
  pub trait XmlStreamWriterWriteProcessingInstructionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterWriteProcessingInstructionArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let target = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeProcessingInstruction_target(original_self as *mut ::xml_stream_writer::XmlStreamWriter, target as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamWriterWriteProcessingInstructionArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let target = self.0;
      let data = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeProcessingInstruction_target_data(original_self as *mut ::xml_stream_writer::XmlStreamWriter, target as *const ::string::String, data as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::write_start_document](../struct.XmlStreamWriter.html#method.write_start_document) method.
  pub trait XmlStreamWriterWriteStartDocumentArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterWriteStartDocumentArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {

      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeStartDocument_no_args(original_self as *mut ::xml_stream_writer::XmlStreamWriter) }
    }
  }
  impl<'largs> XmlStreamWriterWriteStartDocumentArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let version = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeStartDocument_version(original_self as *mut ::xml_stream_writer::XmlStreamWriter, version as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamWriterWriteStartDocumentArgs<'largs> for (&'largs ::string::String, bool) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let version = self.0;
      let standalone = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeStartDocument_version_standalone(original_self as *mut ::xml_stream_writer::XmlStreamWriter, version as *const ::string::String, standalone) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::write_start_element](../struct.XmlStreamWriter.html#method.write_start_element) method.
  pub trait XmlStreamWriterWriteStartElementArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterWriteStartElementArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let namespace_uri = self.0;
      let name = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeStartElement_namespaceUri_name(original_self as *mut ::xml_stream_writer::XmlStreamWriter, namespace_uri as *const ::string::String, name as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamWriterWriteStartElementArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let qualified_name = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeStartElement_qualifiedName(original_self as *mut ::xml_stream_writer::XmlStreamWriter, qualified_name as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamWriter::write_text_element](../struct.XmlStreamWriter.html#method.write_text_element) method.
  pub trait XmlStreamWriterWriteTextElementArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> ();
  }
  impl<'largs> XmlStreamWriterWriteTextElementArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let namespace_uri = self.0;
      let name = self.1;
      let text = self.2;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeTextElement_namespaceUri_name_text(original_self as *mut ::xml_stream_writer::XmlStreamWriter, namespace_uri as *const ::string::String, name as *const ::string::String, text as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamWriterWriteTextElementArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_writer::XmlStreamWriter) -> () {
      let qualified_name = self.0;
      let text = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamWriter_writeTextElement_qualifiedName_text(original_self as *mut ::xml_stream_writer::XmlStreamWriter, qualified_name as *const ::string::String, text as *const ::string::String) }
    }
  }
}
