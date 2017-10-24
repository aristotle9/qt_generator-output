/// C++ type: <span style='color: green;'>```QTextBlockGroup```</span>
#[repr(C)]
pub struct TextBlockGroup(u8);

impl TextBlockGroup {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextBlockGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QTextBlockGroup_metaObject(self as *const ::text_block_group::TextBlockGroup) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextBlockGroup::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QTextBlockGroup_qt_metacall(self as *mut ::text_block_group::TextBlockGroup,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextBlockGroup::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QTextBlockGroup_qt_metacast(self as *mut ::text_block_group::TextBlockGroup, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QTextBlockGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextBlockGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextBlockGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextBlockGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::DynamicCast<::text_block_group::TextBlockGroup> for ::text_object::TextObject {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_block_group::TextBlockGroup> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextBlockGroup_G_dynamic_cast_QTextBlockGroup_ptr(self as *mut ::text_object::TextObject)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_block_group::TextBlockGroup> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockGroup_G_dynamic_cast_QTextBlockGroup_ptr(self as *const ::text_object::TextObject as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_block_group::TextBlockGroup {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QObject_ptr(self as *mut ::text_block_group::TextBlockGroup)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QObject_ptr(self as *const ::text_block_group::TextBlockGroup as *mut ::text_block_group::TextBlockGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_object::TextObject> for ::text_block_group::TextBlockGroup {
  fn static_cast_mut(&mut self) -> &mut ::text_object::TextObject {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextObject_ptr(self as *mut ::text_block_group::TextBlockGroup)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_object::TextObject {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextObject_ptr(self as *const ::text_block_group::TextBlockGroup as *mut ::text_block_group::TextBlockGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_block_group::TextBlockGroup> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_block_group::TextBlockGroup {
    let ffi_result = ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextBlockGroup_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_block_group::TextBlockGroup {
    let ffi_result = ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextBlockGroup_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_block_group::TextBlockGroup> for ::text_object::TextObject {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_block_group::TextBlockGroup {
    let ffi_result = ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextBlockGroup_ptr_QTextObject(self as *mut ::text_object::TextObject);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_block_group::TextBlockGroup {
    let ffi_result = ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextBlockGroup_ptr_QTextObject(self as *const ::text_object::TextObject as *mut ::text_object::TextObject);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_block_group::TextBlockGroup {
  type Target = ::text_object::TextObject;
  fn deref(&self) -> &::text_object::TextObject {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextObject_ptr(self as *const ::text_block_group::TextBlockGroup as *mut ::text_block_group::TextBlockGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_block_group::TextBlockGroup {
  fn deref_mut(&mut self) -> &mut ::text_object::TextObject {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextBlockGroup_G_static_cast_QTextObject_ptr(self as *mut ::text_block_group::TextBlockGroup)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
