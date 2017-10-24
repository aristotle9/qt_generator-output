/// C++ type: <span style='color: green;'>```QFileSelector```</span>
#[repr(C)]
pub struct FileSelector(u8);

impl FileSelector {
  /// C++ method: <span style='color: green;'>```QStringList QFileSelector::allSelectors() const```</span>
  ///
  ///
  pub fn all_selectors(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileSelector_allSelectors_to_output(self as *const ::file_selector::FileSelector,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileSelector::extraSelectors() const```</span>
  ///
  ///
  pub fn extra_selectors(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileSelector_extraSelectors_to_output(self as *const ::file_selector::FileSelector,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFileSelector::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QFileSelector_metaObject(self as *const ::file_selector::FileSelector) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFileSelector::QFileSelector()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::file_selector::FileSelector> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSelector_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFileSelector::QFileSelector(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::file_selector::FileSelector> {
    let ffi_result = ::ffi::qt_core_c_QFileSelector_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QFileSelector::select```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn select(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QFileSelector::select(const QString& filePath) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn select(&self, &::url::Url) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```QUrl QFileSelector::select(const QUrl& filePath) const```</span>
  ///
  ///
  pub fn select<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::FileSelectorSelectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QFileSelector::setExtraSelectors(const QStringList& list)```</span>
  ///
  ///
  pub fn set_extra_selectors(&mut self, list: &::string_list::StringList) {
    unsafe {
      ::ffi::qt_core_c_QFileSelector_setExtraSelectors(self as *mut ::file_selector::FileSelector,
                                                       list as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileSelector::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFileSelector_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileSelector::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFileSelector_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::file_selector::FileSelector {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QFileSelector_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FileSelector`.
  pub struct Signals<'a>(&'a ::file_selector::FileSelector);
  /// Represents a built-in Qt signal `QFileSelector::objectNameChanged`.
  ///
  /// An object of this type can be created from `FileSelector` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSelector` object.
  pub struct ObjectNameChanged<'a>(&'a ::file_selector::FileSelector);
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
    /// Returns an object representing a built-in Qt signal `QFileSelector::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::file_selector::FileSelector {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::file_selector::FileSelector> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file_selector::FileSelector> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFileSelector_G_dynamic_cast_QFileSelector_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file_selector::FileSelector> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSelector_G_dynamic_cast_QFileSelector_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::file_selector::FileSelector {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFileSelector_G_static_cast_QObject_ptr(self as *mut ::file_selector::FileSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSelector_G_static_cast_QObject_ptr(self as *const ::file_selector::FileSelector as *mut ::file_selector::FileSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_selector::FileSelector> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_selector::FileSelector {
    let ffi_result = ::ffi::qt_core_c_QFileSelector_G_static_cast_QFileSelector_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_selector::FileSelector {
    let ffi_result = ::ffi::qt_core_c_QFileSelector_G_static_cast_QFileSelector_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::file_selector::FileSelector {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSelector_G_static_cast_QObject_ptr(self as *const ::file_selector::FileSelector as *mut ::file_selector::FileSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::file_selector::FileSelector {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFileSelector_G_static_cast_QObject_ptr(self as *mut ::file_selector::FileSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileSelector::select](../struct.FileSelector.html#method.select) method.
  pub trait FileSelectorSelectArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::file_selector::FileSelector) -> Self::ReturnType;
  }
  impl<'largs> FileSelectorSelectArgs<'largs> for &'largs ::string::String {
    type ReturnType = ::string::String;
    fn exec(self, original_self: &'largs ::file_selector::FileSelector) -> ::string::String {
      let file_path = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QFileSelector_select_to_output_QString(original_self as *const ::file_selector::FileSelector, file_path as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileSelectorSelectArgs<'largs> for &'largs ::url::Url {
    type ReturnType = ::url::Url;
    fn exec(self, original_self: &'largs ::file_selector::FileSelector) -> ::url::Url {
      let file_path = self;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QFileSelector_select_to_output_QUrl(original_self as *const ::file_selector::FileSelector,
                                                               file_path as *const ::url::Url,
                                                               &mut object);
        }
        object
      }
    }
  }
}
