/// C++ type: <span style='color: green;'>```QSpacerItem```</span>
#[repr(C)]
pub struct SpacerItem(u8);

impl SpacerItem {
  /// C++ method: <span style='color: green;'>```QSpacerItem::changeSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn change_size(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSpacerItem::changeSize(int w, int h)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn change_size(&mut self, (::libc::c_int, ::libc::c_int, &::size_policy::Policy)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSpacerItem::changeSize(int w, int h, QSizePolicy::Policy hData = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn change_size(&mut self, (::libc::c_int, ::libc::c_int, &::size_policy::Policy, &::size_policy::Policy)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSpacerItem::changeSize(int w, int h, QSizePolicy::Policy hData = ?, QSizePolicy::Policy vData = ?)```</span>
  ///
  ///
  pub fn change_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SpacerItemChangeSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QRect QSpacerItem::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_geometry_to_output(self as *const ::spacer_item::SpacerItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QSpacerItem::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSpacerItem_isEmpty(self as *const ::spacer_item::SpacerItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSpacerItem::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_maximumSize_to_output(self as *const ::spacer_item::SpacerItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSpacerItem::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_minimumSize_to_output(self as *const ::spacer_item::SpacerItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSpacerItem::QSpacerItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSpacerItem::QSpacerItem(int w, int h)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, &::size_policy::Policy)) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSpacerItem::QSpacerItem(int w, int h, QSizePolicy::Policy hData = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, &::size_policy::Policy, &::size_policy::Policy)) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSpacerItem::QSpacerItem(int w, int h, QSizePolicy::Policy hData = ?, QSizePolicy::Policy vData = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem>
    where Args: overloading::SpacerItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual void QSpacerItem::setGeometry(const QRect& arg1)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, arg1: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QSpacerItem_setGeometry(self as *mut ::spacer_item::SpacerItem,
                                                  arg1 as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSpacerItem::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_sizeHint_to_output(self as *const ::spacer_item::SpacerItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizePolicy QSpacerItem::sizePolicy() const```</span>
  ///
  ///
  pub fn size_policy(&self) -> ::size_policy::SizePolicy {
    {
      let mut object: ::size_policy::SizePolicy =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_sizePolicy_to_output(self as *const ::spacer_item::SpacerItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSpacerItem* QSpacerItem::spacerItem()```</span>
  ///
  ///
  pub fn spacer_item(&mut self) -> *mut ::spacer_item::SpacerItem {
    unsafe { ::ffi::qt_widgets_c_QSpacerItem_spacerItem(self as *mut ::spacer_item::SpacerItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::spacer_item::SpacerItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSpacerItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::spacer_item::SpacerItem> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::spacer_item::SpacerItem> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpacerItem_G_dynamic_cast_QSpacerItem_ptr(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::spacer_item::SpacerItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpacerItem_G_dynamic_cast_QSpacerItem_ptr(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::spacer_item::SpacerItem {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpacerItem_G_static_cast_QLayoutItem_ptr(self as *mut ::spacer_item::SpacerItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpacerItem_G_static_cast_QLayoutItem_ptr(self as *const ::spacer_item::SpacerItem as *mut ::spacer_item::SpacerItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spacer_item::SpacerItem> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spacer_item::SpacerItem {
    let ffi_result =
      ::ffi::qt_widgets_c_QSpacerItem_G_static_cast_QSpacerItem_ptr(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spacer_item::SpacerItem {
    let ffi_result = ::ffi::qt_widgets_c_QSpacerItem_G_static_cast_QSpacerItem_ptr(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::spacer_item::SpacerItem {
  type Target = ::layout_item::LayoutItem;
  fn deref(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpacerItem_G_static_cast_QLayoutItem_ptr(self as *const ::spacer_item::SpacerItem as *mut ::spacer_item::SpacerItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::spacer_item::SpacerItem {
  fn deref_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpacerItem_G_static_cast_QLayoutItem_ptr(self as *mut ::spacer_item::SpacerItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SpacerItem::change_size](../struct.SpacerItem.html#method.change_size) method.
  pub trait SpacerItemChangeSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::spacer_item::SpacerItem) -> ();
  }
  impl<'largs> SpacerItemChangeSizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::spacer_item::SpacerItem) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_widgets_c_QSpacerItem_changeSize_w_h(original_self as *mut ::spacer_item::SpacerItem, w, h) }
    }
  }
  impl<'largs> SpacerItemChangeSizeArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::size_policy::Policy) {
    fn exec(self, original_self: &'largs mut ::spacer_item::SpacerItem) -> () {
      let w = self.0;
      let h = self.1;
      let h_data = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_changeSize_w_h_hData(original_self as *mut ::spacer_item::SpacerItem,
                                                             w,
                                                             h,
                                                             h_data as *const ::size_policy::Policy)
      }
    }
  }
  impl<'largs> SpacerItemChangeSizeArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::size_policy::Policy, &'largs ::size_policy::Policy) {
    fn exec(self, original_self: &'largs mut ::spacer_item::SpacerItem) -> () {
      let w = self.0;
      let h = self.1;
      let h_data = self.2;
      let v_data = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_changeSize_w_h_hData_vData(original_self as *mut ::spacer_item::SpacerItem,
                                                                   w,
                                                                   h,
                                                                   h_data as *const ::size_policy::Policy,
                                                                   v_data as *const ::size_policy::Policy)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SpacerItem::new](../struct.SpacerItem.html#method.new) method.
  pub trait SpacerItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem>;
  }
  impl SpacerItemNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem> {
      let w = self.0;
      let h = self.1;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpacerItem_new_w_h(w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SpacerItemNewArgs for (::libc::c_int, ::libc::c_int, &'a ::size_policy::Policy) {
    fn exec(self) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem> {
      let w = self.0;
      let h = self.1;
      let h_data = self.2;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QSpacerItem_new_w_h_hData(w, h, h_data as *const ::size_policy::Policy) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SpacerItemNewArgs for (::libc::c_int, ::libc::c_int, &'a ::size_policy::Policy, &'a ::size_policy::Policy) {
    fn exec(self) -> ::cpp_utils::CppBox<::spacer_item::SpacerItem> {
      let w = self.0;
      let h = self.1;
      let h_data = self.2;
      let v_data = self.3;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QSpacerItem_new_w_h_hData_vData(w,
                                                            h,
                                                            h_data as *const ::size_policy::Policy,
                                                            v_data as *const ::size_policy::Policy)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
