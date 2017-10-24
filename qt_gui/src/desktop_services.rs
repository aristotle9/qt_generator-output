/// C++ type: <span style='color: green;'>```QDesktopServices```</span>
#[repr(C)]
pub struct DesktopServices(u8);

impl DesktopServices {
  /// C++ method: <span style='color: green;'>```static bool QDesktopServices::openUrl(const QUrl& url)```</span>
  ///
  ///
  pub fn open_url(url: &::qt_core::url::Url) -> bool {
    unsafe { ::ffi::qt_gui_c_QDesktopServices_openUrl(url as *const ::qt_core::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```static void QDesktopServices::setUrlHandler(const QString& scheme, QObject* receiver, const char* method)```</span>
  ///
  ///
  pub unsafe fn set_url_handler(scheme: &::qt_core::string::String,
                                receiver: *mut ::qt_core::object::Object,
                                method: *const ::libc::c_char) {
    ::ffi::qt_gui_c_QDesktopServices_setUrlHandler(scheme as *const ::qt_core::string::String, receiver, method)
  }

  /// C++ method: <span style='color: green;'>```static void QDesktopServices::unsetUrlHandler(const QString& scheme)```</span>
  ///
  ///
  pub fn unset_url_handler(scheme: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QDesktopServices_unsetUrlHandler(scheme as *const ::qt_core::string::String) }
  }
}

impl ::cpp_utils::CppDeletable for ::desktop_services::DesktopServices {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QDesktopServices_delete
  }
}
