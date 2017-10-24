/// C++ type: <span style='color: green;'>```QMetaClassInfo```</span>
#[repr(C)]
pub struct MetaClassInfo([u8; ::type_sizes::QT_CORE_META_CLASS_INFO_META_CLASS_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MetaClassInfo {
  unsafe fn new_uninitialized() -> MetaClassInfo {
    MetaClassInfo(::std::mem::uninitialized())
  }
}

impl MetaClassInfo {
  /// C++ method: <span style='color: green;'>```const QMetaObject* QMetaClassInfo::enclosingMetaObject() const```</span>
  ///
  ///
  pub fn enclosing_meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMetaClassInfo_enclosingMetaObject(self as *const ::meta_class_info::MetaClassInfo) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaClassInfo::name() const```</span>
  ///
  ///
  pub fn name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaClassInfo_name(self as *const ::meta_class_info::MetaClassInfo) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMetaClassInfo::QMetaClassInfo()```</span>
  ///
  ///
  pub fn new() -> ::meta_class_info::MetaClassInfo {
    {
      let mut object: ::meta_class_info::MetaClassInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaClassInfo_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaClassInfo::value() const```</span>
  ///
  ///
  pub fn value(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaClassInfo_value(self as *const ::meta_class_info::MetaClassInfo) }
  }
}

impl Drop for ::meta_class_info::MetaClassInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QMetaClassInfo::~QMetaClassInfo()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMetaClassInfo_destructor(self as *mut ::meta_class_info::MetaClassInfo) }
  }
}
