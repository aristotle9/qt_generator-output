/// C++ type: <span style='color: green;'>```QFileIconProvider```</span>
#[repr(C)]
pub struct FileIconProvider(u8);

impl FileIconProvider {
  /// C++ method: <span style='color: green;'>```QFileIconProvider::icon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn icon(&self, ::file_icon_provider::IconType) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```virtual QIcon QFileIconProvider::icon(QFileIconProvider::IconType type) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn icon(&self, &::qt_core::file_info::FileInfo) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```virtual QIcon QFileIconProvider::icon(const QFileInfo& info) const```</span>
  ///
  ///
  pub fn icon<'largs, Args>(&'largs self, args: Args) -> ::qt_gui::icon::Icon
    where Args: overloading::FileIconProviderIconArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QFileIconProvider::QFileIconProvider()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::file_icon_provider::FileIconProvider> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileIconProvider_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QFileIconProvider::Option> QFileIconProvider::options() const```</span>
  ///
  ///
  pub fn options(&self) -> ::qt_core::flags::Flags<::file_icon_provider::Option> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileIconProvider_options(self as *const ::file_icon_provider::FileIconProvider) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```void QFileIconProvider::setOptions(QFlags<QFileIconProvider::Option> options)```</span>
  ///
  ///
  pub fn set_options(&mut self, options: ::qt_core::flags::Flags<::file_icon_provider::Option>) {
    unsafe {
      ::ffi::qt_widgets_c_QFileIconProvider_setOptions(self as *mut ::file_icon_provider::FileIconProvider,
                                                       options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QFileIconProvider::type(const QFileInfo& info) const```</span>
  ///
  ///
  pub fn type_(&self, info: &::qt_core::file_info::FileInfo) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileIconProvider_type_to_output(self as *const ::file_icon_provider::FileIconProvider,
                                                             info as *const ::qt_core::file_info::FileInfo,
                                                             &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::file_icon_provider::FileIconProvider {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QFileIconProvider_delete
  }
}

/// C++ type: <span style='color: green;'>```QFileIconProvider::IconType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum IconType {
  /// C++ enum variant: <span style='color: green;'>```Computer = 0```</span>
  Computer = 0,
  /// C++ enum variant: <span style='color: green;'>```Desktop = 1```</span>
  Desktop = 1,
  /// C++ enum variant: <span style='color: green;'>```Trashcan = 2```</span>
  Trashcan = 2,
  /// C++ enum variant: <span style='color: green;'>```Network = 3```</span>
  Network = 3,
  /// C++ enum variant: <span style='color: green;'>```Drive = 4```</span>
  Drive = 4,
  /// C++ enum variant: <span style='color: green;'>```Folder = 5```</span>
  Folder = 5,
  /// C++ enum variant: <span style='color: green;'>```File = 6```</span>
  File = 6,
}

/// C++ type: <span style='color: green;'>```QFileIconProvider::Option```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Option {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```DontUseCustomDirectoryIcons = 1```</span>
  DontUseCustomDirectoryIcons = 1,
}

impl ::qt_core::flags::FlaggableEnum for Option {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Option"
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileIconProvider::icon](../struct.FileIconProvider.html#method.icon) method.
  pub trait FileIconProviderIconArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_icon_provider::FileIconProvider) -> ::qt_gui::icon::Icon;
  }
  impl<'largs> FileIconProviderIconArgs<'largs> for &'largs ::qt_core::file_info::FileInfo {
    fn exec(self, original_self: &'largs ::file_icon_provider::FileIconProvider) -> ::qt_gui::icon::Icon {
      let info = self;
      {
        let mut object: ::qt_gui::icon::Icon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileIconProvider_icon_to_output_info(original_self as *const ::file_icon_provider::FileIconProvider, info as *const ::qt_core::file_info::FileInfo, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileIconProviderIconArgs<'largs> for ::file_icon_provider::IconType {
    fn exec(self, original_self: &'largs ::file_icon_provider::FileIconProvider) -> ::qt_gui::icon::Icon {
      let type_ = self;
      {
        let mut object: ::qt_gui::icon::Icon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileIconProvider_icon_to_output_type(original_self as *const ::file_icon_provider::FileIconProvider, type_, &mut object);
        }
        object
      }
    }
  }
}
