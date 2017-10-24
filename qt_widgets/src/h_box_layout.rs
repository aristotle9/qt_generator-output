/// C++ type: <span style='color: green;'>```QHBoxLayout```</span>
#[repr(C)]
pub struct HBoxLayout(u8);

impl HBoxLayout {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QHBoxLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QHBoxLayout_metaObject(self as *const ::h_box_layout::HBoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QHBoxLayout::QHBoxLayout()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::h_box_layout::HBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QHBoxLayout::QHBoxLayout(QWidget* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::h_box_layout::HBoxLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QHBoxLayout_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QHBoxLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QHBoxLayout_qt_metacall(self as *mut ::h_box_layout::HBoxLayout,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QHBoxLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QHBoxLayout_qt_metacast(self as *mut ::h_box_layout::HBoxLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QHBoxLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QHBoxLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QHBoxLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QHBoxLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::h_box_layout::HBoxLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QHBoxLayout_delete
  }
}

impl ::cpp_utils::DynamicCast<::h_box_layout::HBoxLayout> for ::box_layout::BoxLayout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::h_box_layout::HBoxLayout> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QBoxLayout(self as *mut ::box_layout::BoxLayout)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::h_box_layout::HBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QBoxLayout(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::h_box_layout::HBoxLayout> for ::layout::Layout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::h_box_layout::HBoxLayout> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QLayout(self as *mut ::layout::Layout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::h_box_layout::HBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::h_box_layout::HBoxLayout> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::h_box_layout::HBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::h_box_layout::HBoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::h_box_layout::HBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QObject_ptr(self as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QObject_ptr(self as *const ::h_box_layout::HBoxLayout as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::box_layout::BoxLayout> for ::h_box_layout::HBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::box_layout::BoxLayout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QBoxLayout_ptr(self as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::box_layout::BoxLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QBoxLayout_ptr(self as *const ::h_box_layout::HBoxLayout as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout::Layout> for ::h_box_layout::HBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QLayout_ptr(self as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QLayout_ptr(self as *const ::h_box_layout::HBoxLayout as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::h_box_layout::HBoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QLayoutItem_ptr(self as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QLayoutItem_ptr(self as *const ::h_box_layout::HBoxLayout as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::h_box_layout::HBoxLayout> for ::box_layout::BoxLayout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::h_box_layout::HBoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QBoxLayout(self as *mut ::box_layout::BoxLayout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::h_box_layout::HBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QBoxLayout(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::h_box_layout::HBoxLayout> for ::layout::Layout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::h_box_layout::HBoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QLayout(self as *mut ::layout::Layout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::h_box_layout::HBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::h_box_layout::HBoxLayout> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::h_box_layout::HBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::h_box_layout::HBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::h_box_layout::HBoxLayout> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::h_box_layout::HBoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::h_box_layout::HBoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::h_box_layout::HBoxLayout {
  type Target = ::box_layout::BoxLayout;
  fn deref(&self) -> &::box_layout::BoxLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QBoxLayout_ptr(self as *const ::h_box_layout::HBoxLayout as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::h_box_layout::HBoxLayout {
  fn deref_mut(&mut self) -> &mut ::box_layout::BoxLayout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHBoxLayout_G_static_cast_QBoxLayout_ptr(self as *mut ::h_box_layout::HBoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
