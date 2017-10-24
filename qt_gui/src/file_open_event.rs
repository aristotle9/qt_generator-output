/// C++ type: <span style='color: green;'>```QFileOpenEvent```</span>
#[repr(C)]
pub struct FileOpenEvent(u8);

impl FileOpenEvent {
  /// C++ method: <span style='color: green;'>```QString QFileOpenEvent::file() const```</span>
  ///
  ///
  pub fn file(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFileOpenEvent_file_to_output(self as *const ::file_open_event::FileOpenEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileOpenEvent::QFileOpenEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::file_open_event::FileOpenEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileOpenEvent::QFileOpenEvent(const QString& file)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::url::Url) -> ::cpp_utils::CppBox<::file_open_event::FileOpenEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileOpenEvent::QFileOpenEvent(const QUrl& url)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::file_open_event::FileOpenEvent>
    where Args: overloading::FileOpenEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUrl QFileOpenEvent::url() const```</span>
  ///
  ///
  pub fn url(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFileOpenEvent_url_to_output(self as *const ::file_open_event::FileOpenEvent, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::file_open_event::FileOpenEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QFileOpenEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::file_open_event::FileOpenEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QFileOpenEvent_G_static_cast_QEvent_ptr(self as *mut ::file_open_event::FileOpenEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QFileOpenEvent_G_static_cast_QEvent_ptr(self as *const ::file_open_event::FileOpenEvent as *mut ::file_open_event::FileOpenEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_open_event::FileOpenEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_open_event::FileOpenEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QFileOpenEvent_G_static_cast_QFileOpenEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_open_event::FileOpenEvent {
    let ffi_result = ::ffi::qt_gui_c_QFileOpenEvent_G_static_cast_QFileOpenEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::file_open_event::FileOpenEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QFileOpenEvent_G_static_cast_QEvent_ptr(self as *const ::file_open_event::FileOpenEvent as *mut ::file_open_event::FileOpenEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::file_open_event::FileOpenEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QFileOpenEvent_G_static_cast_QEvent_ptr(self as *mut ::file_open_event::FileOpenEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileOpenEvent::new](../struct.FileOpenEvent.html#method.new) method.
  pub trait FileOpenEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::file_open_event::FileOpenEvent>;
  }
  impl<'a> FileOpenEventNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::file_open_event::FileOpenEvent> {
      let file = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QFileOpenEvent_new_file(file as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> FileOpenEventNewArgs for &'a ::qt_core::url::Url {
    fn exec(self) -> ::cpp_utils::CppBox<::file_open_event::FileOpenEvent> {
      let url = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QFileOpenEvent_new_url(url as *const ::qt_core::url::Url) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
