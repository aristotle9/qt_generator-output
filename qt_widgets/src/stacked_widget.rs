/// C++ type: <span style='color: green;'>```QStackedWidget```</span>
#[repr(C)]
pub struct StackedWidget(u8);

impl StackedWidget {
  /// C++ method: <span style='color: green;'>```int QStackedWidget::addWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn add_widget(&mut self, w: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStackedWidget_addWidget(self as *mut ::stacked_widget::StackedWidget, w)
  }

  /// C++ method: <span style='color: green;'>```int QStackedWidget::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStackedWidget_count(self as *const ::stacked_widget::StackedWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QStackedWidget::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStackedWidget_currentIndex(self as *const ::stacked_widget::StackedWidget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QStackedWidget::currentWidget() const```</span>
  ///
  ///
  pub fn current_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QStackedWidget_currentWidget(self as *const ::stacked_widget::StackedWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QStackedWidget::indexOf(QWidget* arg1) const```</span>
  ///
  ///
  pub unsafe fn index_of(&self, arg1: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStackedWidget_indexOf(self as *const ::stacked_widget::StackedWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```int QStackedWidget::insertWidget(int index, QWidget* w)```</span>
  ///
  ///
  pub unsafe fn insert_widget(&mut self, index: ::libc::c_int, w: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStackedWidget_insertWidget(self as *mut ::stacked_widget::StackedWidget, index, w)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStackedWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QStackedWidget_metaObject(self as *const ::stacked_widget::StackedWidget) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStackedWidget::QStackedWidget()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::stacked_widget::StackedWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStackedWidget::QStackedWidget(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::stacked_widget::StackedWidget> {
    let ffi_result = ::ffi::qt_widgets_c_QStackedWidget_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QStackedWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStackedWidget_qt_metacall(self as *mut ::stacked_widget::StackedWidget,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QStackedWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QStackedWidget_qt_metacast(self as *mut ::stacked_widget::StackedWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QStackedWidget::removeWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn remove_widget(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QStackedWidget_removeWidget(self as *mut ::stacked_widget::StackedWidget, w)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QStackedWidget::setCurrentIndex(int index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStackedWidget_setCurrentIndex(self as *mut ::stacked_widget::StackedWidget, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QStackedWidget::setCurrentWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn set_current_widget(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QStackedWidget_setCurrentWidget(self as *mut ::stacked_widget::StackedWidget, w)
  }

  /// C++ method: <span style='color: green;'>```static QString QStackedWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStackedWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStackedWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStackedWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QStackedWidget::widget(int arg1) const```</span>
  ///
  ///
  pub fn widget(&self, arg1: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QStackedWidget_widget(self as *const ::stacked_widget::StackedWidget, arg1) }
  }
}

impl ::cpp_utils::CppDeletable for ::stacked_widget::StackedWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStackedWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StackedWidget`.
  pub struct Signals<'a>(&'a ::stacked_widget::StackedWidget);
  /// Represents a built-in Qt signal `QStackedWidget::widgetRemoved`.
  ///
  /// An object of this type can be created from `StackedWidget` with `object.signals().widget_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedWidget` object.
  pub struct WidgetRemoved<'a>(&'a ::stacked_widget::StackedWidget);
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
  /// Represents a built-in Qt signal `QStackedWidget::currentChanged`.
  ///
  /// An object of this type can be created from `StackedWidget` with `object.signals().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedWidget` object.
  pub struct CurrentChanged<'a>(&'a ::stacked_widget::StackedWidget);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QStackedWidget::widgetRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn widget_removed(&self) -> WidgetRemoved {
      WidgetRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStackedWidget::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StackedWidget`.
  pub struct Slots<'a>(&'a ::stacked_widget::StackedWidget);
  /// Represents a built-in Qt slot `QStackedWidget::setCurrentWidget`.
  ///
  /// An object of this type can be created from `StackedWidget` with `object.slots().set_current_widget()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedWidget` object.
  pub struct SetCurrentWidget<'a>(&'a ::stacked_widget::StackedWidget);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentWidget<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentWidget(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QStackedWidget::setCurrentIndex`.
  ///
  /// An object of this type can be created from `StackedWidget` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StackedWidget` object.
  pub struct SetCurrentIndex<'a>(&'a ::stacked_widget::StackedWidget);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QStackedWidget::setCurrentWidget`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_widget(&self) -> SetCurrentWidget {
      SetCurrentWidget(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStackedWidget::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
  }
  impl ::stacked_widget::StackedWidget {
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

impl ::cpp_utils::DynamicCast<::stacked_widget::StackedWidget> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::stacked_widget::StackedWidget> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QFrame(self as *mut ::frame::Frame)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::stacked_widget::StackedWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::stacked_widget::StackedWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::stacked_widget::StackedWidget> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::stacked_widget::StackedWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::stacked_widget::StackedWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QObject_ptr(self as *mut ::stacked_widget::StackedWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QObject_ptr(self as *const ::stacked_widget::StackedWidget as *mut ::stacked_widget::StackedWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::stacked_widget::StackedWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::stacked_widget::StackedWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QPaintDevice_ptr(self as *const ::stacked_widget::StackedWidget as *mut ::stacked_widget::StackedWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::stacked_widget::StackedWidget {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QFrame_ptr(self as *mut ::stacked_widget::StackedWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QFrame_ptr(self as *const ::stacked_widget::StackedWidget as *mut ::stacked_widget::StackedWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::stacked_widget::StackedWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QWidget_ptr(self as *mut ::stacked_widget::StackedWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QWidget_ptr(self as *const ::stacked_widget::StackedWidget as *mut ::stacked_widget::StackedWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stacked_widget::StackedWidget> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stacked_widget::StackedWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stacked_widget::StackedWidget {
    let ffi_result = ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stacked_widget::StackedWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stacked_widget::StackedWidget {
    let ffi_result = ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stacked_widget::StackedWidget {
    let ffi_result = ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stacked_widget::StackedWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stacked_widget::StackedWidget {
    let ffi_result = ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stacked_widget::StackedWidget {
    let ffi_result = ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stacked_widget::StackedWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stacked_widget::StackedWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stacked_widget::StackedWidget {
    let ffi_result = ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::stacked_widget::StackedWidget {
  type Target = ::frame::Frame;
  fn deref(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QFrame_ptr(self as *const ::stacked_widget::StackedWidget as *mut ::stacked_widget::StackedWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::stacked_widget::StackedWidget {
  fn deref_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStackedWidget_G_static_cast_QFrame_ptr(self as *mut ::stacked_widget::StackedWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
