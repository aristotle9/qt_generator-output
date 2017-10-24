/// C++ type: <span style='color: green;'>```QStackedLayout```</span>
#[repr(C)]
pub struct StackedLayout(u8);

impl StackedLayout {
  /// C++ method: <span style='color: green;'>```virtual void QStackedLayout::addItem(QLayoutItem* item)```</span>
  ///
  ///
  pub unsafe fn add_item(&mut self, item: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QStackedLayout_addItem(self as *mut ::stacked_layout::StackedLayout, item)
  }

  /// C++ method: <span style='color: green;'>```int QStackedLayout::addWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn add_widget(&mut self, w: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStackedLayout_addWidget(self as *mut ::stacked_layout::StackedLayout, w)
  }

  /// C++ method: <span style='color: green;'>```virtual int QStackedLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_count(self as *const ::stacked_layout::StackedLayout) }
  }

  /// C++ method: <span style='color: green;'>```int QStackedLayout::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_currentIndex(self as *const ::stacked_layout::StackedLayout) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QStackedLayout::currentWidget() const```</span>
  ///
  ///
  pub fn current_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_currentWidget(self as *const ::stacked_layout::StackedLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QStackedLayout::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_hasHeightForWidth(self as *const ::stacked_layout::StackedLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QStackedLayout::heightForWidth(int width) const```</span>
  ///
  ///
  pub fn height_for_width(&self, width: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_heightForWidth(self as *const ::stacked_layout::StackedLayout, width) }
  }

  /// C++ method: <span style='color: green;'>```int QStackedLayout::insertWidget(int index, QWidget* w)```</span>
  ///
  ///
  pub unsafe fn insert_widget(&mut self, index: ::libc::c_int, w: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStackedLayout_insertWidget(self as *mut ::stacked_layout::StackedLayout, index, w)
  }

  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QStackedLayout::itemAt(int arg1) const```</span>
  ///
  ///
  pub fn item_at(&self, arg1: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_itemAt(self as *const ::stacked_layout::StackedLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStackedLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_metaObject(self as *const ::stacked_layout::StackedLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QStackedLayout::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStackedLayout_minimumSize_to_output(self as *const ::stacked_layout::StackedLayout,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStackedLayout::QStackedLayout()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::stacked_layout::StackedLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QStackedLayout::QStackedLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::layout::Layout) -> ::cpp_utils::CppBox<::stacked_layout::StackedLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStackedLayout::QStackedLayout(QLayout* parentLayout)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::stacked_layout::StackedLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStackedLayout::QStackedLayout(QWidget* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::stacked_layout::StackedLayout>
    where Args: overloading::StackedLayoutNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QStackedLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStackedLayout_qt_metacall(self as *mut ::stacked_layout::StackedLayout,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QStackedLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QStackedLayout_qt_metacast(self as *mut ::stacked_layout::StackedLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QStackedLayout::setCurrentIndex(int index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_setCurrentIndex(self as *mut ::stacked_layout::StackedLayout, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QStackedLayout::setCurrentWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn set_current_widget(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QStackedLayout_setCurrentWidget(self as *mut ::stacked_layout::StackedLayout, w)
  }

  /// C++ method: <span style='color: green;'>```virtual void QStackedLayout::setGeometry(const QRect& rect)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, rect: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QStackedLayout_setGeometry(self as *mut ::stacked_layout::StackedLayout,
                                                     rect as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStackedLayout::setStackingMode(QStackedLayout::StackingMode stackingMode)```</span>
  ///
  ///
  pub fn set_stacking_mode(&mut self, stacking_mode: ::stacked_layout::StackingMode) {
    unsafe {
      ::ffi::qt_widgets_c_QStackedLayout_setStackingMode(self as *mut ::stacked_layout::StackedLayout, stacking_mode)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QStackedLayout::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStackedLayout_sizeHint_to_output(self as *const ::stacked_layout::StackedLayout,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStackedLayout::StackingMode QStackedLayout::stackingMode() const```</span>
  ///
  ///
  pub fn stacking_mode(&self) -> ::stacked_layout::StackingMode {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_stackingMode(self as *const ::stacked_layout::StackedLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QStackedLayout::takeAt(int arg1)```</span>
  ///
  ///
  pub fn take_at(&mut self, arg1: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_takeAt(self as *mut ::stacked_layout::StackedLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```static QString QStackedLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStackedLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStackedLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStackedLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QStackedLayout::widget(int arg1) const```</span>
  ///
  ///
  pub fn widget(&self, arg1: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QStackedLayout_widget(self as *const ::stacked_layout::StackedLayout, arg1) }
  }
}

impl ::cpp_utils::CppDeletable for ::stacked_layout::StackedLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStackedLayout_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StackedLayout`.
  pub struct Signals<'a>(&'a ::stacked_layout::StackedLayout);
  /// Represents a built-in Qt signal `QStackedLayout::currentChanged`.
  ///
  /// An object of this type can be created from `StackedLayout` with `object.signals().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedLayout` object.
  pub struct CurrentChanged<'a>(&'a ::stacked_layout::StackedLayout);
  impl<'a> ::qt_core::connection::Receiver for CurrentChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentChanged<'a> {}
  /// Represents a built-in Qt signal `QStackedLayout::widgetRemoved`.
  ///
  /// An object of this type can be created from `StackedLayout` with `object.signals().widget_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedLayout` object.
  pub struct WidgetRemoved<'a>(&'a ::stacked_layout::StackedLayout);
  impl<'a> ::qt_core::connection::Receiver for WidgetRemoved<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widgetRemoved(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidgetRemoved<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QStackedLayout::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStackedLayout::widgetRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn widget_removed(&self) -> WidgetRemoved {
      WidgetRemoved(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StackedLayout`.
  pub struct Slots<'a>(&'a ::stacked_layout::StackedLayout);
  /// Represents a built-in Qt slot `QStackedLayout::setCurrentIndex`.
  ///
  /// An object of this type can be created from `StackedLayout` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedLayout` object.
  pub struct SetCurrentIndex<'a>(&'a ::stacked_layout::StackedLayout);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QStackedLayout::setCurrentWidget`.
  ///
  /// An object of this type can be created from `StackedLayout` with `object.slots().set_current_widget()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedLayout` object.
  pub struct SetCurrentWidget<'a>(&'a ::stacked_layout::StackedLayout);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentWidget<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentWidget(QWidget*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QStackedLayout::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStackedLayout::setCurrentWidget`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_widget(&self) -> SetCurrentWidget {
      SetCurrentWidget(self.0)
    }
  }
  impl ::stacked_layout::StackedLayout {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QStackedLayout::StackingMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StackingMode {
  /// C++ enum variant: <span style='color: green;'>```StackOne = 0```</span>
  One = 0,
  /// C++ enum variant: <span style='color: green;'>```StackAll = 1```</span>
  All = 1,
}

impl ::cpp_utils::DynamicCast<::stacked_layout::StackedLayout> for ::layout::Layout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::stacked_layout::StackedLayout> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayout(self as *mut ::layout::Layout)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::stacked_layout::StackedLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::stacked_layout::StackedLayout> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::stacked_layout::StackedLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::stacked_layout::StackedLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::stacked_layout::StackedLayout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QObject_ptr(self as *mut ::stacked_layout::StackedLayout)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QObject_ptr(self as *const ::stacked_layout::StackedLayout as *mut ::stacked_layout::StackedLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout::Layout> for ::stacked_layout::StackedLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QLayout_ptr(self as *mut ::stacked_layout::StackedLayout)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QLayout_ptr(self as *const ::stacked_layout::StackedLayout as *mut ::stacked_layout::StackedLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::stacked_layout::StackedLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QLayoutItem_ptr(self as *mut ::stacked_layout::StackedLayout)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QLayoutItem_ptr(self as *const ::stacked_layout::StackedLayout as *mut ::stacked_layout::StackedLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stacked_layout::StackedLayout> for ::layout::Layout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stacked_layout::StackedLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayout(self as *mut ::layout::Layout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stacked_layout::StackedLayout {
    let ffi_result = ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stacked_layout::StackedLayout> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stacked_layout::StackedLayout {
    let ffi_result = ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stacked_layout::StackedLayout {
    let ffi_result = ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stacked_layout::StackedLayout> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stacked_layout::StackedLayout {
    let ffi_result = ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stacked_layout::StackedLayout {
    let ffi_result = ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::stacked_layout::StackedLayout {
  type Target = ::layout::Layout;
  fn deref(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QLayout_ptr(self as *const ::stacked_layout::StackedLayout as *mut ::stacked_layout::StackedLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::stacked_layout::StackedLayout {
  fn deref_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedLayout_G_static_cast_QLayout_ptr(self as *mut ::stacked_layout::StackedLayout)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StackedLayout::new_unsafe](../struct.StackedLayout.html#method.new_unsafe) method.
  pub trait StackedLayoutNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::stacked_layout::StackedLayout>;
  }
  impl StackedLayoutNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::stacked_layout::StackedLayout> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QStackedLayout_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl StackedLayoutNewUnsafeArgs for *mut ::layout::Layout {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::stacked_layout::StackedLayout> {
      let parent_layout = self;
      let ffi_result = ::ffi::qt_widgets_c_QStackedLayout_new_parentLayout(parent_layout);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
