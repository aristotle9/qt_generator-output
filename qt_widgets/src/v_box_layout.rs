/// C++ type: <span style='color: green;'>```QVBoxLayout```</span>
#[repr(C)]
pub struct VBoxLayout(u8);

impl VBoxLayout {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QVBoxLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QVBoxLayout_metaObject(self as *const ::v_box_layout::VBoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QVBoxLayout::QVBoxLayout()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::v_box_layout::VBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QVBoxLayout::QVBoxLayout(QWidget* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::v_box_layout::VBoxLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QVBoxLayout_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QVBoxLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QVBoxLayout_qt_metacall(self as *mut ::v_box_layout::VBoxLayout,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QVBoxLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QVBoxLayout_qt_metacast(self as *mut ::v_box_layout::VBoxLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QVBoxLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QVBoxLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QVBoxLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QVBoxLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::v_box_layout::VBoxLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QVBoxLayout_delete
  }
}

impl ::cpp_utils::DynamicCast<::v_box_layout::VBoxLayout> for ::box_layout::BoxLayout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::v_box_layout::VBoxLayout> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QBoxLayout(self as *mut ::box_layout::BoxLayout)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::v_box_layout::VBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QBoxLayout(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::v_box_layout::VBoxLayout> for ::layout::Layout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::v_box_layout::VBoxLayout> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QLayout(self as *mut ::layout::Layout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::v_box_layout::VBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::v_box_layout::VBoxLayout> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::v_box_layout::VBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::v_box_layout::VBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::v_box_layout::VBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QObject_ptr(self as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QObject_ptr(self as *const ::v_box_layout::VBoxLayout as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::box_layout::BoxLayout> for ::v_box_layout::VBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::box_layout::BoxLayout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QBoxLayout_ptr(self as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::box_layout::BoxLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QBoxLayout_ptr(self as *const ::v_box_layout::VBoxLayout as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout::Layout> for ::v_box_layout::VBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QLayout_ptr(self as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QLayout_ptr(self as *const ::v_box_layout::VBoxLayout as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::v_box_layout::VBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QLayoutItem_ptr(self as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QLayoutItem_ptr(self as *const ::v_box_layout::VBoxLayout as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::v_box_layout::VBoxLayout> for ::box_layout::BoxLayout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::v_box_layout::VBoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QBoxLayout(self as *mut ::box_layout::BoxLayout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::v_box_layout::VBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QBoxLayout(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::v_box_layout::VBoxLayout> for ::layout::Layout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::v_box_layout::VBoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QLayout(self as *mut ::layout::Layout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::v_box_layout::VBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::v_box_layout::VBoxLayout> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::v_box_layout::VBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::v_box_layout::VBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::v_box_layout::VBoxLayout> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::v_box_layout::VBoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::v_box_layout::VBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::v_box_layout::VBoxLayout {
  type Target = ::box_layout::BoxLayout;
  fn deref(&self) -> &::box_layout::BoxLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QBoxLayout_ptr(self as *const ::v_box_layout::VBoxLayout as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::v_box_layout::VBoxLayout {
  fn deref_mut(&mut self) -> &mut ::box_layout::BoxLayout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVBoxLayout_G_static_cast_QBoxLayout_ptr(self as *mut ::v_box_layout::VBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
