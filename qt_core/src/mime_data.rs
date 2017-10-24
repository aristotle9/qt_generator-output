/// C++ type: <span style='color: green;'>```QMimeData```</span>
#[repr(C)]
pub struct MimeData(u8);

impl MimeData {
  /// C++ method: <span style='color: green;'>```void QMimeData::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QMimeData_clear(self as *mut ::mime_data::MimeData) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QMimeData::colorData() const```</span>
  ///
  ///
  pub fn color_data(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeData_colorData_to_output(self as *const ::mime_data::MimeData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QMimeData::data(const QString& mimetype) const```</span>
  ///
  ///
  pub fn data(&self, mimetype: &::string::String) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeData_data_to_output(self as *const ::mime_data::MimeData,
                                                  mimetype as *const ::string::String,
                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QMimeData::formats() const```</span>
  ///
  ///
  pub fn formats(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeData_formats_to_output(self as *const ::mime_data::MimeData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeData::hasColor() const```</span>
  ///
  ///
  pub fn has_color(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMimeData_hasColor(self as *const ::mime_data::MimeData) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QMimeData::hasFormat(const QString& mimetype) const```</span>
  ///
  ///
  pub fn has_format(&self, mimetype: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMimeData_hasFormat(self as *const ::mime_data::MimeData,
                                           mimetype as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeData::hasHtml() const```</span>
  ///
  ///
  pub fn has_html(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMimeData_hasHtml(self as *const ::mime_data::MimeData) }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeData::hasImage() const```</span>
  ///
  ///
  pub fn has_image(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMimeData_hasImage(self as *const ::mime_data::MimeData) }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeData::hasText() const```</span>
  ///
  ///
  pub fn has_text(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMimeData_hasText(self as *const ::mime_data::MimeData) }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeData::hasUrls() const```</span>
  ///
  ///
  pub fn has_urls(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMimeData_hasUrls(self as *const ::mime_data::MimeData) }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeData::html() const```</span>
  ///
  ///
  pub fn html(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeData_html_to_output(self as *const ::mime_data::MimeData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QMimeData::imageData() const```</span>
  ///
  ///
  pub fn image_data(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeData_imageData_to_output(self as *const ::mime_data::MimeData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMimeData::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMimeData_metaObject(self as *const ::mime_data::MimeData) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMimeData::QMimeData()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::mime_data::MimeData> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMimeData_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QMimeData::removeFormat(const QString& mimetype)```</span>
  ///
  ///
  pub fn remove_format(&mut self, mimetype: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QMimeData_removeFormat(self as *mut ::mime_data::MimeData,
                                              mimetype as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMimeData::setColorData(const QVariant& color)```</span>
  ///
  ///
  pub fn set_color_data(&mut self, color: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QMimeData_setColorData(self as *mut ::mime_data::MimeData,
                                              color as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMimeData::setData(const QString& mimetype, const QByteArray& data)```</span>
  ///
  ///
  pub fn set_data(&mut self, mimetype: &::string::String, data: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QMimeData_setData(self as *mut ::mime_data::MimeData,
                                         mimetype as *const ::string::String,
                                         data as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMimeData::setHtml(const QString& html)```</span>
  ///
  ///
  pub fn set_html(&mut self, html: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QMimeData_setHtml(self as *mut ::mime_data::MimeData,
                                         html as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMimeData::setImageData(const QVariant& image)```</span>
  ///
  ///
  pub fn set_image_data(&mut self, image: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QMimeData_setImageData(self as *mut ::mime_data::MimeData,
                                              image as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMimeData::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QMimeData_setText(self as *mut ::mime_data::MimeData,
                                         text as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMimeData::setUrls(const QList<QUrl>& urls)```</span>
  ///
  ///
  pub fn set_urls(&mut self, urls: &::list::ListUrl) {
    unsafe {
      ::ffi::qt_core_c_QMimeData_setUrls(self as *mut ::mime_data::MimeData,
                                         urls as *const ::list::ListUrl)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeData::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeData_text_to_output(self as *const ::mime_data::MimeData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMimeData::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMimeData_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMimeData::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMimeData_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl> QMimeData::urls() const```</span>
  ///
  ///
  pub fn urls(&self) -> ::list::ListUrl {
    {
      let mut object: ::list::ListUrl =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeData_urls_to_output(self as *const ::mime_data::MimeData, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::mime_data::MimeData {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QMimeData_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MimeData`.
  pub struct Signals<'a>(&'a ::mime_data::MimeData);
  /// Represents a built-in Qt signal `QMimeData::objectNameChanged`.
  ///
  /// An object of this type can be created from `MimeData` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MimeData` object.
  pub struct ObjectNameChanged<'a>(&'a ::mime_data::MimeData);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QMimeData::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::mime_data::MimeData {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::mime_data::MimeData> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mime_data::MimeData> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMimeData_G_dynamic_cast_QMimeData_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mime_data::MimeData> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMimeData_G_dynamic_cast_QMimeData_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::mime_data::MimeData {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QMimeData_G_static_cast_QObject_ptr(self as *mut ::mime_data::MimeData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMimeData_G_static_cast_QObject_ptr(self as *const ::mime_data::MimeData as *mut ::mime_data::MimeData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mime_data::MimeData> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mime_data::MimeData {
    let ffi_result = ::ffi::qt_core_c_QMimeData_G_static_cast_QMimeData_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mime_data::MimeData {
    let ffi_result = ::ffi::qt_core_c_QMimeData_G_static_cast_QMimeData_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mime_data::MimeData {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMimeData_G_static_cast_QObject_ptr(self as *const ::mime_data::MimeData as *mut ::mime_data::MimeData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mime_data::MimeData {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QMimeData_G_static_cast_QObject_ptr(self as *mut ::mime_data::MimeData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
