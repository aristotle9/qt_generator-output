/// C++ type: <span style='color: green;'>```QAccessibleObject```</span>
#[repr(C)]
pub struct AccessibleObject(u8);

impl AccessibleObject {
  /// C++ method: <span style='color: green;'>```virtual QAccessibleInterface* QAccessibleObject::childAt(int x, int y) const```</span>
  ///
  ///
  pub fn child_at(&self, x: ::libc::c_int, y: ::libc::c_int) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleObject_childAt(self as *const ::accessible_object::AccessibleObject, x, y) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAccessibleObject::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleObject_isValid(self as *const ::accessible_object::AccessibleObject) }
  }

  /// C++ method: <span style='color: green;'>```virtual QObject* QAccessibleObject::object() const```</span>
  ///
  ///
  pub fn object(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QAccessibleObject_object(self as *const ::accessible_object::AccessibleObject) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QAccessibleObject::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleObject_rect_to_output(self as *const ::accessible_object::AccessibleObject,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAccessibleObject::setText(QAccessible::Text t, const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, t: &::accessible::Text, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleObject_setText(self as *mut ::accessible_object::AccessibleObject,
                                                t as *const ::accessible::Text,
                                                text as *const ::qt_core::string::String)
    }
  }
}

impl ::cpp_utils::DynamicCast<::accessible_object::AccessibleObject> for ::accessible_interface::AccessibleInterface {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_object::AccessibleObject> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleObject_G_dynamic_cast_QAccessibleObject_ptr(self as *mut ::accessible_interface::AccessibleInterface) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_object::AccessibleObject> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleObject_G_dynamic_cast_QAccessibleObject_ptr(self as *const ::accessible_interface::AccessibleInterface as *mut ::accessible_interface::AccessibleInterface) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::accessible_interface::AccessibleInterface> for ::accessible_object::AccessibleObject {
  fn static_cast_mut(&mut self) -> &mut ::accessible_interface::AccessibleInterface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleInterface_ptr(self as *mut ::accessible_object::AccessibleObject) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::accessible_interface::AccessibleInterface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleInterface_ptr(self as *const ::accessible_object::AccessibleObject as *mut ::accessible_object::AccessibleObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::accessible_object::AccessibleObject> for ::accessible_interface::AccessibleInterface {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_object::AccessibleObject {
let ffi_result = ::ffi::qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleObject_ptr(self as *mut ::accessible_interface::AccessibleInterface);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_object::AccessibleObject {
let ffi_result = ::ffi::qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleObject_ptr(self as *const ::accessible_interface::AccessibleInterface as *mut ::accessible_interface::AccessibleInterface);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_object::AccessibleObject {
  type Target = ::accessible_interface::AccessibleInterface;
  fn deref(&self) -> &::accessible_interface::AccessibleInterface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleInterface_ptr(self as *const ::accessible_object::AccessibleObject as *mut ::accessible_object::AccessibleObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_object::AccessibleObject {
  fn deref_mut(&mut self) -> &mut ::accessible_interface::AccessibleInterface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleInterface_ptr(self as *mut ::accessible_object::AccessibleObject) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
