/// C++ type: <span style='color: green;'>```QXmlStreamReader::Error```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Error {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```UnexpectedElementError = 1```</span>
  UnexpectedElement = 1,
  /// C++ enum variant: <span style='color: green;'>```CustomError = 2```</span>
  Custom = 2,
  /// C++ enum variant: <span style='color: green;'>```NotWellFormedError = 3```</span>
  NotWellFormed = 3,
  /// C++ enum variant: <span style='color: green;'>```PrematureEndOfDocumentError = 4```</span>
  PrematureEndOfDocument = 4,
}

/// C++ type: <span style='color: green;'>```QXmlStreamReader::ReadElementTextBehaviour```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ReadElementTextBehaviour {
  /// C++ enum variant: <span style='color: green;'>```ErrorOnUnexpectedElement = 0```</span>
  ErrorOnUnexpectedElement = 0,
  /// C++ enum variant: <span style='color: green;'>```IncludeChildElements = 1```</span>
  IncludeChildElements = 1,
  /// C++ enum variant: <span style='color: green;'>```SkipChildElements = 2```</span>
  SkipChildElements = 2,
}

/// C++ type: <span style='color: green;'>```QXmlStreamReader::TokenType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TokenType {
  /// C++ enum variant: <span style='color: green;'>```NoToken = 0```</span>
  NoToken = 0,
  /// C++ enum variant: <span style='color: green;'>```Invalid = 1```</span>
  Invalid = 1,
  /// C++ enum variant: <span style='color: green;'>```StartDocument = 2```</span>
  StartDocument = 2,
  /// C++ enum variant: <span style='color: green;'>```EndDocument = 3```</span>
  EndDocument = 3,
  /// C++ enum variant: <span style='color: green;'>```StartElement = 4```</span>
  StartElement = 4,
  /// C++ enum variant: <span style='color: green;'>```EndElement = 5```</span>
  EndElement = 5,
  /// C++ enum variant: <span style='color: green;'>```Characters = 6```</span>
  Characters = 6,
  /// C++ enum variant: <span style='color: green;'>```Comment = 7```</span>
  Comment = 7,
  /// C++ enum variant: <span style='color: green;'>```DTD = 8```</span>
  DTD = 8,
  /// C++ enum variant: <span style='color: green;'>```EntityReference = 9```</span>
  EntityReference = 9,
  /// C++ enum variant: <span style='color: green;'>```ProcessingInstruction = 10```</span>
  ProcessingInstruction = 10,
}

/// C++ type: <span style='color: green;'>```QXmlStreamReader```</span>
#[repr(C)]
pub struct XmlStreamReader([u8; ::type_sizes::QT_CORE_XML_STREAM_READER_XML_STREAM_READER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamReader {
  unsafe fn new_uninitialized() -> XmlStreamReader {
    XmlStreamReader(::std::mem::uninitialized())
  }
}

impl XmlStreamReader {
  /// C++ method: <span style='color: green;'>```QXmlStreamReader::addData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_data(&mut self, &::byte_array::ByteArray) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::addData(const QByteArray& data)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_data(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::addData(const QString& data)```</span>
  ///
  ///
  pub fn add_data<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamReaderAddDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::addData(const char* data)```</span>
  ///
  ///
  pub unsafe fn add_data_unsafe(&mut self, data: *const ::libc::c_char) {
    ::ffi::qt_core_c_QXmlStreamReader_addData_char(self as *mut ::xml_stream_reader::XmlStreamReader, data)
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration& extraNamespaceDeclaraction)```</span>
  ///
  ///
pub fn add_extra_namespace_declaration(&mut self, extra_namespace_declaraction: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_addExtraNamespaceDeclaration(self as *mut ::xml_stream_reader::XmlStreamReader, extra_namespace_declaraction as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::addExtraNamespaceDeclarations(const QVector<QXmlStreamNamespaceDeclaration>& extraNamespaceDeclaractions)```</span>
  ///
  ///
pub fn add_extra_namespace_declarations(&mut self, extra_namespace_declaractions: &::vector::VectorXmlStreamNamespaceDeclaration) {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_addExtraNamespaceDeclarations(self as *mut ::xml_stream_reader::XmlStreamReader, extra_namespace_declaractions as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_atEnd(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttributes QXmlStreamReader::attributes() const```</span>
  ///
  ///
  pub fn attributes(&self) -> ::xml_stream_attributes::XmlStreamAttributes {
    {
      let mut object: ::xml_stream_attributes::XmlStreamAttributes =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_attributes_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QXmlStreamReader::characterOffset() const```</span>
  ///
  ///
  pub fn character_offset(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_characterOffset(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_clear(self as *mut ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QXmlStreamReader::columnNumber() const```</span>
  ///
  ///
  pub fn column_number(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_columnNumber(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QXmlStreamReader::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::io_device::IODevice {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_device(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::documentEncoding() const```</span>
  ///
  ///
  pub fn document_encoding(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_documentEncoding_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::documentVersion() const```</span>
  ///
  ///
  pub fn document_version(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_documentVersion_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::dtdName() const```</span>
  ///
  ///
  pub fn dtd_name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_dtdName_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::dtdPublicId() const```</span>
  ///
  ///
  pub fn dtd_public_id(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_dtdPublicId_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::dtdSystemId() const```</span>
  ///
  ///
  pub fn dtd_system_id(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_dtdSystemId_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration> QXmlStreamReader::entityDeclarations() const```</span>
  ///
  ///
  pub fn entity_declarations(&self) -> ::vector::VectorXmlStreamEntityDeclaration {
    {
      let mut object: ::vector::VectorXmlStreamEntityDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_entityDeclarations_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityResolver* QXmlStreamReader::entityResolver() const```</span>
  ///
  ///
  pub fn entity_resolver(&self) -> *mut ::xml_stream_entity_resolver::XmlStreamEntityResolver {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_entityResolver(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamReader::Error QXmlStreamReader::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::xml_stream_reader::Error {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_error(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```QString QXmlStreamReader::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_errorString_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::hasError() const```</span>
  ///
  ///
  pub fn has_error(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_hasError(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isCDATA() const```</span>
  ///
  ///
  pub fn is_c_d_a_t_a(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isCDATA(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isCharacters() const```</span>
  ///
  ///
  pub fn is_characters(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isCharacters(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isComment() const```</span>
  ///
  ///
  pub fn is_comment(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isComment(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isDTD() const```</span>
  ///
  ///
  pub fn is_d_t_d(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isDTD(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isEndDocument() const```</span>
  ///
  ///
  pub fn is_end_document(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isEndDocument(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isEndElement() const```</span>
  ///
  ///
  pub fn is_end_element(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isEndElement(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isEntityReference() const```</span>
  ///
  ///
  pub fn is_entity_reference(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isEntityReference(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isProcessingInstruction() const```</span>
  ///
  ///
  pub fn is_processing_instruction(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamReader_isProcessingInstruction(self as *const ::xml_stream_reader::XmlStreamReader)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isStandaloneDocument() const```</span>
  ///
  ///
  pub fn is_standalone_document(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamReader_isStandaloneDocument(self as *const ::xml_stream_reader::XmlStreamReader)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isStartDocument() const```</span>
  ///
  ///
  pub fn is_start_document(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isStartDocument(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isStartElement() const```</span>
  ///
  ///
  pub fn is_start_element(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isStartElement(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::isWhitespace() const```</span>
  ///
  ///
  pub fn is_whitespace(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_isWhitespace(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QXmlStreamReader::lineNumber() const```</span>
  ///
  ///
  pub fn line_number(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_lineNumber(self as *const ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_name_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration> QXmlStreamReader::namespaceDeclarations() const```</span>
  ///
  ///
  pub fn namespace_declarations(&self) -> ::vector::VectorXmlStreamNamespaceDeclaration {
    {
      let mut object: ::vector::VectorXmlStreamNamespaceDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_namespaceDeclarations_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::namespaceProcessing() const```</span>
  ///
  ///
  pub fn namespace_processing(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamReader_namespaceProcessing(self as *const ::xml_stream_reader::XmlStreamReader)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::namespaceUri() const```</span>
  ///
  ///
  pub fn namespace_uri(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_namespaceUri_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamReader::QXmlStreamReader```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::xml_stream_reader::XmlStreamReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamReader::QXmlStreamReader()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::xml_stream_reader::XmlStreamReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamReader::QXmlStreamReader(const QByteArray& data)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::xml_stream_reader::XmlStreamReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamReader::QXmlStreamReader(const QString& data)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::xml_stream_reader::XmlStreamReader
    where Args: overloading::XmlStreamReaderNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamReader::QXmlStreamReader```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::io_device::IODevice) -> ::xml_stream_reader::XmlStreamReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamReader::QXmlStreamReader(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::libc::c_char) -> ::xml_stream_reader::XmlStreamReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamReader::QXmlStreamReader(const char* data)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::xml_stream_reader::XmlStreamReader
    where Args: overloading::XmlStreamReaderNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration> QXmlStreamReader::notationDeclarations() const```</span>
  ///
  ///
  pub fn notation_declarations(&self) -> ::vector::VectorXmlStreamNotationDeclaration {
    {
      let mut object: ::vector::VectorXmlStreamNotationDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_notationDeclarations_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::prefix() const```</span>
  ///
  ///
  pub fn prefix(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_prefix_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::processingInstructionData() const```</span>
  ///
  ///
  pub fn processing_instruction_data(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_processingInstructionData_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::processingInstructionTarget() const```</span>
  ///
  ///
  pub fn processing_instruction_target(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_processingInstructionTarget_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::qualifiedName() const```</span>
  ///
  ///
  pub fn qualified_name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_qualifiedName_to_output(self as *const ::xml_stream_reader::XmlStreamReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamReader::raiseError```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn raise_error(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::raiseError()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn raise_error(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::raiseError(const QString& message = ?)```</span>
  ///
  ///
  pub fn raise_error<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamReaderRaiseErrorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamReader::readElementText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn read_element_text(&mut self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QXmlStreamReader::readElementText()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn read_element_text(&mut self, ::xml_stream_reader::ReadElementTextBehaviour) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QXmlStreamReader::readElementText(QXmlStreamReader::ReadElementTextBehaviour behaviour = ?)```</span>
  ///
  ///
  pub fn read_element_text<'largs, Args>(&'largs mut self, args: Args) -> ::string::String
    where Args: overloading::XmlStreamReaderReadElementTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamReader::TokenType QXmlStreamReader::readNext()```</span>
  ///
  ///
  pub fn read_next(&mut self) -> ::xml_stream_reader::TokenType {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_readNext(self as *mut ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamReader::readNextStartElement()```</span>
  ///
  ///
  pub fn read_next_start_element(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_readNextStartElement(self as *mut ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::io_device::IODevice) {
    ::ffi::qt_core_c_QXmlStreamReader_setDevice(self as *mut ::xml_stream_reader::XmlStreamReader, device)
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver* resolver)```</span>
  ///
  ///
  pub unsafe fn set_entity_resolver(&mut self, resolver: *mut ::xml_stream_entity_resolver::XmlStreamEntityResolver) {
    ::ffi::qt_core_c_QXmlStreamReader_setEntityResolver(self as *mut ::xml_stream_reader::XmlStreamReader, resolver)
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::setNamespaceProcessing(bool arg1)```</span>
  ///
  ///
  pub fn set_namespace_processing(&mut self, arg1: bool) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamReader_setNamespaceProcessing(self as *mut ::xml_stream_reader::XmlStreamReader, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamReader::skipCurrentElement()```</span>
  ///
  ///
  pub fn skip_current_element(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_skipCurrentElement(self as *mut ::xml_stream_reader::XmlStreamReader) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamReader::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_text_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QXmlStreamReader::tokenString() const```</span>
  ///
  ///
  pub fn token_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_tokenString_to_output(self as *const ::xml_stream_reader::XmlStreamReader,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamReader::TokenType QXmlStreamReader::tokenType() const```</span>
  ///
  ///
  pub fn token_type(&self) -> ::xml_stream_reader::TokenType {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_tokenType(self as *const ::xml_stream_reader::XmlStreamReader) }
  }
}

impl Drop for ::xml_stream_reader::XmlStreamReader {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamReader::~QXmlStreamReader()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamReader_destructor(self as *mut ::xml_stream_reader::XmlStreamReader) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamReader::add_data](../struct.XmlStreamReader.html#method.add_data) method.
  pub trait XmlStreamReaderAddDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> ();
  }
  impl<'largs> XmlStreamReaderAddDataArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> () {
      let data = self;
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_addData_QByteArray(original_self as *mut ::xml_stream_reader::XmlStreamReader, data as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> XmlStreamReaderAddDataArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> () {
      let data = self;
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_addData_QString(original_self as *mut ::xml_stream_reader::XmlStreamReader,
                                                          data as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamReader::new](../struct.XmlStreamReader.html#method.new) method.
  pub trait XmlStreamReaderNewArgs {
    fn exec(self) -> ::xml_stream_reader::XmlStreamReader;
  }
  impl<'a> XmlStreamReaderNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::xml_stream_reader::XmlStreamReader {
      let data = self;
      {
        let mut object: ::xml_stream_reader::XmlStreamReader =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamReader_constructor_QByteArray(data as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> XmlStreamReaderNewArgs for &'a ::string::String {
    fn exec(self) -> ::xml_stream_reader::XmlStreamReader {
      let data = self;
      {
        let mut object: ::xml_stream_reader::XmlStreamReader =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamReader_constructor_QString(data as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl XmlStreamReaderNewArgs for () {
    fn exec(self) -> ::xml_stream_reader::XmlStreamReader {

      {
        let mut object: ::xml_stream_reader::XmlStreamReader =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamReader_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamReader::new_unsafe](../struct.XmlStreamReader.html#method.new_unsafe) method.
  pub trait XmlStreamReaderNewUnsafeArgs {
    unsafe fn exec(self) -> ::xml_stream_reader::XmlStreamReader;
  }
  impl XmlStreamReaderNewUnsafeArgs for *mut ::io_device::IODevice {
    unsafe fn exec(self) -> ::xml_stream_reader::XmlStreamReader {
      let device = self;
      {
        let mut object: ::xml_stream_reader::XmlStreamReader =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QXmlStreamReader_constructor_QIODevice(device, &mut object);
        object
      }
    }
  }
  impl XmlStreamReaderNewUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::xml_stream_reader::XmlStreamReader {
      let data = self;
      {
        let mut object: ::xml_stream_reader::XmlStreamReader =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QXmlStreamReader_constructor_char(data, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamReader::raise_error](../struct.XmlStreamReader.html#method.raise_error) method.
  pub trait XmlStreamReaderRaiseErrorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> ();
  }
  impl<'largs> XmlStreamReaderRaiseErrorArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> () {
      let message = self;
      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_raiseError_message(original_self as *mut ::xml_stream_reader::XmlStreamReader, message as *const ::string::String)
      }
    }
  }
  impl<'largs> XmlStreamReaderRaiseErrorArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> () {

      unsafe {
        ::ffi::qt_core_c_QXmlStreamReader_raiseError_no_args(original_self as *mut ::xml_stream_reader::XmlStreamReader)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamReader::read_element_text](../struct.XmlStreamReader.html#method.read_element_text) method.
  pub trait XmlStreamReaderReadElementTextArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> ::string::String;
  }
  impl<'largs> XmlStreamReaderReadElementTextArgs<'largs> for ::xml_stream_reader::ReadElementTextBehaviour {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> ::string::String {
      let behaviour = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamReader_readElementText_to_output_behaviour(original_self as *mut ::xml_stream_reader::XmlStreamReader, behaviour, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> XmlStreamReaderReadElementTextArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::xml_stream_reader::XmlStreamReader) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamReader_readElementText_to_output_no_args(original_self as *mut ::xml_stream_reader::XmlStreamReader, &mut object);
        }
        object
      }
    }
  }
}
