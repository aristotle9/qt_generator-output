/// C++ type: <span style='color: green;'>```QJsonDocument::DataValidation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DataValidation {
  /// C++ enum variant: <span style='color: green;'>```Validate = 0```</span>
  Validate = 0,
  /// C++ enum variant: <span style='color: green;'>```BypassValidation = 1```</span>
  BypassValidation = 1,
}

/// C++ type: <span style='color: green;'>```QJsonDocument```</span>
#[repr(C)]
pub struct JsonDocument([u8; ::type_sizes::QT_CORE_JSON_DOCUMENT_JSON_DOCUMENT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for JsonDocument {
  unsafe fn new_uninitialized() -> JsonDocument {
    JsonDocument(::std::mem::uninitialized())
  }
}

impl JsonDocument {
  /// C++ method: <span style='color: green;'>```QJsonArray QJsonDocument::array() const```</span>
  ///
  ///
  pub fn array(&self) -> ::json_array::JsonArray {
    {
      let mut object: ::json_array::JsonArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonDocument_array_to_output(self as *const ::json_document::JsonDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonDocument::fromBinaryData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_binary_data(&::byte_array::ByteArray) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```static QJsonDocument QJsonDocument::fromBinaryData(const QByteArray& data)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_binary_data((&::byte_array::ByteArray, ::json_document::DataValidation)) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```static QJsonDocument QJsonDocument::fromBinaryData(const QByteArray& data, QJsonDocument::DataValidation validation = ?)```</span>
  ///
  ///
  pub fn from_binary_data<Args>(args: Args) -> ::json_document::JsonDocument
    where Args: overloading::JsonDocumentFromBinaryDataArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QJsonDocument QJsonDocument::fromJson(const QByteArray& json)```</span>
  ///
  ///
  pub fn from_json(json: &::byte_array::ByteArray) -> ::json_document::JsonDocument {
    {
      let mut object: ::json_document::JsonDocument =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonDocument_fromJson_to_output_json(json as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QJsonDocument QJsonDocument::fromJson(const QByteArray& json, QJsonParseError* error = ?)```</span>
  ///
  ///
  pub unsafe fn from_json_unsafe(json: &::byte_array::ByteArray,
                                 error: *mut ::json_parse_error::JsonParseError)
                                 -> ::json_document::JsonDocument {
    {
      let mut object: ::json_document::JsonDocument =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QJsonDocument_fromJson_to_output_json_error(json as *const ::byte_array::ByteArray,
                                                                   error,
                                                                   &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonDocument::fromRawData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_raw_data((*const ::libc::c_char, ::libc::c_int)) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```static QJsonDocument QJsonDocument::fromRawData(const char* data, int size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_raw_data((*const ::libc::c_char, ::libc::c_int, ::json_document::DataValidation)) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```static QJsonDocument QJsonDocument::fromRawData(const char* data, int size, QJsonDocument::DataValidation validation = ?)```</span>
  ///
  ///
  pub unsafe fn from_raw_data<Args>(args: Args) -> ::json_document::JsonDocument
    where Args: overloading::JsonDocumentFromRawDataArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QJsonDocument QJsonDocument::fromVariant(const QVariant& variant)```</span>
  ///
  ///
  pub fn from_variant(variant: &::variant::Variant) -> ::json_document::JsonDocument {
    {
      let mut object: ::json_document::JsonDocument =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonDocument_fromVariant_to_output(variant as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonDocument::isArray() const```</span>
  ///
  ///
  pub fn is_array(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonDocument_isArray(self as *const ::json_document::JsonDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonDocument::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonDocument_isEmpty(self as *const ::json_document::JsonDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonDocument::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonDocument_isNull(self as *const ::json_document::JsonDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonDocument::isObject() const```</span>
  ///
  ///
  pub fn is_object(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonDocument_isObject(self as *const ::json_document::JsonDocument) }
  }

  /// C++ method: <span style='color: green;'>```QJsonDocument::QJsonDocument```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonDocument::QJsonDocument()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::json_array::JsonArray) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonDocument::QJsonDocument(const QJsonArray& array)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::json_document::JsonDocument) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonDocument::QJsonDocument(const QJsonDocument& other)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::json_object::JsonObject) -> ::json_document::JsonDocument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonDocument::QJsonDocument(const QJsonObject& object)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::json_document::JsonDocument
    where Args: overloading::JsonDocumentNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QJsonObject QJsonDocument::object() const```</span>
  ///
  ///
  pub fn object(&self) -> ::json_object::JsonObject {
    {
      let mut object: ::json_object::JsonObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonDocument_object_to_output(self as *const ::json_document::JsonDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonDocument& QJsonDocument::operator=(const QJsonDocument& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::json_document::JsonDocument)
                             -> &'l0 mut ::json_document::JsonDocument {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QJsonDocument_operator_assign(self as *mut ::json_document::JsonDocument,
                                                     other as *const ::json_document::JsonDocument)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QJsonDocument::operator==(const QJsonDocument& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::json_document::JsonDocument) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonDocument_operator_eq(self as *const ::json_document::JsonDocument,
                                                 other as *const ::json_document::JsonDocument)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonDocument::operator!=(const QJsonDocument& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::json_document::JsonDocument) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonDocument_operator_neq(self as *const ::json_document::JsonDocument,
                                                  other as *const ::json_document::JsonDocument)
    }
  }

  /// C++ method: <span style='color: green;'>```const char* QJsonDocument::rawData(int* size) const```</span>
  ///
  ///
  pub unsafe fn raw_data(&self, size: *mut ::libc::c_int) -> *const ::libc::c_char {
    ::ffi::qt_core_c_QJsonDocument_rawData(self as *const ::json_document::JsonDocument, size)
  }

  /// C++ method: <span style='color: green;'>```void QJsonDocument::setArray(const QJsonArray& array)```</span>
  ///
  ///
  pub fn set_array(&mut self, array: &::json_array::JsonArray) {
    unsafe {
      ::ffi::qt_core_c_QJsonDocument_setArray(self as *mut ::json_document::JsonDocument,
                                              array as *const ::json_array::JsonArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QJsonDocument::setObject(const QJsonObject& object)```</span>
  ///
  ///
  pub fn set_object(&mut self, object: &::json_object::JsonObject) {
    unsafe {
      ::ffi::qt_core_c_QJsonDocument_setObject(self as *mut ::json_document::JsonDocument,
                                               object as *const ::json_object::JsonObject)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QJsonDocument::toBinaryData() const```</span>
  ///
  ///
  pub fn to_binary_data(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonDocument_toBinaryData_to_output(self as *const ::json_document::JsonDocument,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonDocument::toJson```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_json(&self, ()) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QJsonDocument::toJson() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_json(&self, ::json_document::JsonFormat) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QJsonDocument::toJson(QJsonDocument::JsonFormat format) const```</span>
  ///
  ///
  pub fn to_json<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::JsonDocumentToJsonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVariant QJsonDocument::toVariant() const```</span>
  ///
  ///
  pub fn to_variant(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonDocument_toVariant_to_output(self as *const ::json_document::JsonDocument, &mut object);
      }
      object
    }
  }
}

impl Drop for ::json_document::JsonDocument {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonDocument::~QJsonDocument()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonDocument_destructor(self as *mut ::json_document::JsonDocument) }
  }
}

/// C++ type: <span style='color: green;'>```QJsonDocument::JsonFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum JsonFormat {
  /// C++ enum variant: <span style='color: green;'>```Indented = 0```</span>
  Indented = 0,
  /// C++ enum variant: <span style='color: green;'>```Compact = 1```</span>
  Compact = 1,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [JsonDocument::from_binary_data](../struct.JsonDocument.html#method.from_binary_data) method.
  pub trait JsonDocumentFromBinaryDataArgs {
    fn exec(self) -> ::json_document::JsonDocument;
  }
  impl<'a> JsonDocumentFromBinaryDataArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::json_document::JsonDocument {
      let data = self;
      {
        let mut object: ::json_document::JsonDocument =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_fromBinaryData_to_output_data(data as *const ::byte_array::ByteArray,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonDocumentFromBinaryDataArgs for (&'a ::byte_array::ByteArray, ::json_document::DataValidation) {
    fn exec(self) -> ::json_document::JsonDocument {
      let data = self.0;
      let validation = self.1;
      {
        let mut object: ::json_document::JsonDocument =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_fromBinaryData_to_output_data_validation(data as *const ::byte_array::ByteArray, validation, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonDocument::from_raw_data](../struct.JsonDocument.html#method.from_raw_data) method.
  pub trait JsonDocumentFromRawDataArgs {
    unsafe fn exec(self) -> ::json_document::JsonDocument;
  }
  impl JsonDocumentFromRawDataArgs for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::json_document::JsonDocument {
      let data = self.0;
      let size = self.1;
      {
        let mut object: ::json_document::JsonDocument =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QJsonDocument_fromRawData_to_output_data_size(data, size, &mut object);
        object
      }
    }
  }
  impl JsonDocumentFromRawDataArgs for (*const ::libc::c_char, ::libc::c_int, ::json_document::DataValidation) {
    unsafe fn exec(self) -> ::json_document::JsonDocument {
      let data = self.0;
      let size = self.1;
      let validation = self.2;
      {
        let mut object: ::json_document::JsonDocument =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QJsonDocument_fromRawData_to_output_data_size_validation(data, size, validation, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonDocument::new](../struct.JsonDocument.html#method.new) method.
  pub trait JsonDocumentNewArgs {
    fn exec(self) -> ::json_document::JsonDocument;
  }
  impl<'a> JsonDocumentNewArgs for &'a ::json_array::JsonArray {
    fn exec(self) -> ::json_document::JsonDocument {
      let array = self;
      {
        let mut object: ::json_document::JsonDocument =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_constructor_array(array as *const ::json_array::JsonArray, &mut object);
        }
        object
      }
    }
  }
  impl JsonDocumentNewArgs for () {
    fn exec(self) -> ::json_document::JsonDocument {

      {
        let mut object: ::json_document::JsonDocument =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonDocumentNewArgs for &'a ::json_object::JsonObject {
    fn exec(self) -> ::json_document::JsonDocument {
      let object = self;
      {
        let mut object2: ::json_document::JsonDocument =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_constructor_object(object as *const ::json_object::JsonObject, &mut object2);
        }
        object2
      }
    }
  }
  impl<'a> JsonDocumentNewArgs for &'a ::json_document::JsonDocument {
    fn exec(self) -> ::json_document::JsonDocument {
      let other = self;
      {
        let mut object: ::json_document::JsonDocument =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_constructor_other(other as *const ::json_document::JsonDocument, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonDocument::to_json](../struct.JsonDocument.html#method.to_json) method.
  pub trait JsonDocumentToJsonArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_document::JsonDocument) -> ::byte_array::ByteArray;
  }
  impl<'largs> JsonDocumentToJsonArgs<'largs> for ::json_document::JsonFormat {
    fn exec(self, original_self: &'largs ::json_document::JsonDocument) -> ::byte_array::ByteArray {
      let format = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_toJson_to_output_format(original_self as *const ::json_document::JsonDocument, format, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonDocumentToJsonArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_document::JsonDocument) -> ::byte_array::ByteArray {

      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonDocument_toJson_to_output_no_args(original_self as *const ::json_document::JsonDocument, &mut object);
        }
        object
      }
    }
  }
}
