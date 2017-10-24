/// C++ type: <span style='color: green;'>```QTranslator```</span>
#[repr(C)]
pub struct Translator(u8);

impl Translator {
  /// C++ method: <span style='color: green;'>```virtual bool QTranslator::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTranslator_isEmpty(self as *const ::translator::Translator) }
  }

  /// C++ method: <span style='color: green;'>```QTranslator::load```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn load(&mut self, (&::locale::Locale, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QLocale& locale, const QString& filename)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn load(&mut self, (&::locale::Locale, &::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QLocale& locale, const QString& filename, const QString& prefix = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn load(&mut self, (&::locale::Locale, &::string::String, &::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QLocale& locale, const QString& filename, const QString& prefix = ?, const QString& directory = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn load(&mut self, (&::locale::Locale, &::string::String, &::string::String, &::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QLocale& locale, const QString& filename, const QString& prefix = ?, const QString& directory = ?, const QString& suffix = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn load(&mut self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QString& filename)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn load(&mut self, (&::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QString& filename, const QString& directory = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn load(&mut self, (&::string::String, &::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QString& filename, const QString& directory = ?, const QString& search_delimiters = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn load(&mut self, (&::string::String, &::string::String, &::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const QString& filename, const QString& directory = ?, const QString& search_delimiters = ?, const QString& suffix = ?)```</span>
  ///
  ///
  pub fn load<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::TranslatorLoadArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTranslator::load```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn load_unsafe(&mut self, (*const ::libc::c_uchar, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const unsigned char* data, int len)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn load_unsafe(&mut self, (*const ::libc::c_uchar, ::libc::c_int, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTranslator::load(const unsigned char* data, int len, const QString& directory = ?)```</span>
  ///
  ///
  pub unsafe fn load_unsafe<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::TranslatorLoadUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTranslator::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QTranslator_metaObject(self as *const ::translator::Translator) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTranslator::QTranslator()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::translator::Translator> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTranslator_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTranslator::QTranslator(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::translator::Translator> {
    let ffi_result = ::ffi::qt_core_c_QTranslator_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```static QString QTranslator::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTranslator_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTranslator::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTranslator_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTranslator::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&self, (*const ::libc::c_char, *const ::libc::c_char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```virtual QString QTranslator::translate(const char* context, const char* sourceText) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&self, (*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```virtual QString QTranslator::translate(const char* context, const char* sourceText, const char* disambiguation = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn translate(&self, (*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```virtual QString QTranslator::translate(const char* context, const char* sourceText, const char* disambiguation = ?, int n = ?) const```</span>
  ///
  ///
  pub unsafe fn translate<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::TranslatorTranslateArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::translator::Translator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTranslator_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Translator`.
  pub struct Signals<'a>(&'a ::translator::Translator);
  /// Represents a built-in Qt signal `QTranslator::objectNameChanged`.
  ///
  /// An object of this type can be created from `Translator` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Translator` object.
  pub struct ObjectNameChanged<'a>(&'a ::translator::Translator);
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
    /// Returns an object representing a built-in Qt signal `QTranslator::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::translator::Translator {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::translator::Translator> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::translator::Translator> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTranslator_G_dynamic_cast_QTranslator_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::translator::Translator> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTranslator_G_dynamic_cast_QTranslator_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::translator::Translator {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTranslator_G_static_cast_QObject_ptr(self as *mut ::translator::Translator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTranslator_G_static_cast_QObject_ptr(self as *const ::translator::Translator as *mut ::translator::Translator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::translator::Translator> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::translator::Translator {
    let ffi_result = ::ffi::qt_core_c_QTranslator_G_static_cast_QTranslator_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::translator::Translator {
    let ffi_result = ::ffi::qt_core_c_QTranslator_G_static_cast_QTranslator_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::translator::Translator {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTranslator_G_static_cast_QObject_ptr(self as *const ::translator::Translator as *mut ::translator::Translator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::translator::Translator {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTranslator_G_static_cast_QObject_ptr(self as *mut ::translator::Translator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Translator::load](../struct.Translator.html#method.load) method.
  pub trait TranslatorLoadArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool;
  }
  impl<'largs> TranslatorLoadArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let filename = self;
      unsafe {
        ::ffi::qt_core_c_QTranslator_load_filename(original_self as *mut ::translator::Translator,
                                                   filename as *const ::string::String)
      }
    }
  }
  impl<'largs> TranslatorLoadArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let filename = self.0;
      let directory = self.1;
      unsafe {
        ::ffi::qt_core_c_QTranslator_load_filename_directory(original_self as *mut ::translator::Translator,
                                                             filename as *const ::string::String,
                                                             directory as *const ::string::String)
      }
    }
  }
  impl<'largs> TranslatorLoadArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let filename = self.0;
      let directory = self.1;
      let search_delimiters = self.2;
      unsafe { ::ffi::qt_core_c_QTranslator_load_filename_directory_search_delimiters(original_self as *mut ::translator::Translator, filename as *const ::string::String, directory as *const ::string::String, search_delimiters as *const ::string::String) }
    }
  }
  impl<'largs> TranslatorLoadArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let filename = self.0;
      let directory = self.1;
      let search_delimiters = self.2;
      let suffix = self.3;
      unsafe { ::ffi::qt_core_c_QTranslator_load_filename_directory_search_delimiters_suffix(original_self as *mut ::translator::Translator, filename as *const ::string::String, directory as *const ::string::String, search_delimiters as *const ::string::String, suffix as *const ::string::String) }
    }
  }
  impl<'largs> TranslatorLoadArgs<'largs> for (&'largs ::locale::Locale, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let locale = self.0;
      let filename = self.1;
      unsafe {
        ::ffi::qt_core_c_QTranslator_load_locale_filename(original_self as *mut ::translator::Translator,
                                                          locale as *const ::locale::Locale,
                                                          filename as *const ::string::String)
      }
    }
  }
  impl<'largs> TranslatorLoadArgs<'largs>
    for (&'largs ::locale::Locale, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let locale = self.0;
      let filename = self.1;
      let prefix = self.2;
      unsafe {
        ::ffi::qt_core_c_QTranslator_load_locale_filename_prefix(original_self as *mut ::translator::Translator,
                                                                 locale as *const ::locale::Locale,
                                                                 filename as *const ::string::String,
                                                                 prefix as *const ::string::String)
      }
    }
  }
  impl<'largs> TranslatorLoadArgs<'largs>
    for (&'largs ::locale::Locale, &'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let locale = self.0;
      let filename = self.1;
      let prefix = self.2;
      let directory = self.3;
      unsafe { ::ffi::qt_core_c_QTranslator_load_locale_filename_prefix_directory(original_self as *mut ::translator::Translator, locale as *const ::locale::Locale, filename as *const ::string::String, prefix as *const ::string::String, directory as *const ::string::String) }
    }
  }
  impl<'largs> TranslatorLoadArgs<'largs>
    for (&'largs ::locale::Locale,
                                                   &'largs ::string::String,
                                                   &'largs ::string::String,
                                                   &'largs ::string::String,
                                                   &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let locale = self.0;
      let filename = self.1;
      let prefix = self.2;
      let directory = self.3;
      let suffix = self.4;
      unsafe { ::ffi::qt_core_c_QTranslator_load_locale_filename_prefix_directory_suffix(original_self as *mut ::translator::Translator, locale as *const ::locale::Locale, filename as *const ::string::String, prefix as *const ::string::String, directory as *const ::string::String, suffix as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [Translator::load_unsafe](../struct.Translator.html#method.load_unsafe) method.
  pub trait TranslatorLoadUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool;
  }
  impl<'largs> TranslatorLoadUnsafeArgs<'largs> for (*const ::libc::c_uchar, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let data = self.0;
      let len = self.1;
      ::ffi::qt_core_c_QTranslator_load_data_len(original_self as *mut ::translator::Translator, data, len)
    }
  }
  impl<'largs> TranslatorLoadUnsafeArgs<'largs> for (*const ::libc::c_uchar, ::libc::c_int, &'largs ::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::translator::Translator) -> bool {
      let data = self.0;
      let len = self.1;
      let directory = self.2;
      ::ffi::qt_core_c_QTranslator_load_data_len_directory(original_self as *mut ::translator::Translator,
                                                           data,
                                                           len,
                                                           directory as *const ::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [Translator::translate](../struct.Translator.html#method.translate) method.
  pub trait TranslatorTranslateArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::translator::Translator) -> ::string::String;
  }
  impl<'largs> TranslatorTranslateArgs<'largs> for (*const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::translator::Translator) -> ::string::String {
      let context = self.0;
      let source_text = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTranslator_translate_to_output_context_sourceText(original_self as *const ::translator::Translator, context, source_text, &mut object);
        object
      }
    }
  }
  impl<'largs> TranslatorTranslateArgs<'largs> for (*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::translator::Translator) -> ::string::String {
      let context = self.0;
      let source_text = self.1;
      let disambiguation = self.2;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTranslator_translate_to_output_context_sourceText_disambiguation(original_self as *const ::translator::Translator, context, source_text, disambiguation, &mut object);
        object
      }
    }
  }
  impl<'largs> TranslatorTranslateArgs<'largs>
    for (*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::translator::Translator) -> ::string::String {
      let context = self.0;
      let source_text = self.1;
      let disambiguation = self.2;
      let n = self.3;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTranslator_translate_to_output_context_sourceText_disambiguation_n(original_self as *const ::translator::Translator, context, source_text, disambiguation, n, &mut object);
        object
      }
    }
  }
}
