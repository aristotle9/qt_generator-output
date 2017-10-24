/// C++ type: <span style='color: green;'>```QToolBox```</span>
#[repr(C)]
pub struct ToolBox(u8);

impl ToolBox {
  /// C++ method: <span style='color: green;'>```QToolBox::addItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_item(&mut self, (*mut ::widget::Widget, &::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QToolBox::addItem(QWidget* widget, const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_item(&mut self, (*mut ::widget::Widget, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QToolBox::addItem(QWidget* widget, const QString& text)```</span>
  ///
  ///
  pub unsafe fn add_item<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::ToolBoxAddItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QToolBox::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QToolBox_count(self as *const ::tool_box::ToolBox) }
  }

  /// C++ method: <span style='color: green;'>```int QToolBox::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QToolBox_currentIndex(self as *const ::tool_box::ToolBox) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QToolBox::currentWidget() const```</span>
  ///
  ///
  pub fn current_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QToolBox_currentWidget(self as *const ::tool_box::ToolBox) }
  }

  /// C++ method: <span style='color: green;'>```int QToolBox::indexOf(QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn index_of(&self, widget: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QToolBox_indexOf(self as *const ::tool_box::ToolBox, widget)
  }

  /// C++ method: <span style='color: green;'>```QToolBox::insertItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_item(&mut self, (::libc::c_int, *mut ::widget::Widget, &::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QToolBox::insertItem(int index, QWidget* widget, const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_item(&mut self, (::libc::c_int, *mut ::widget::Widget, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QToolBox::insertItem(int index, QWidget* widget, const QString& text)```</span>
  ///
  ///
  pub unsafe fn insert_item<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::ToolBoxInsertItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QToolBox::isItemEnabled(int index) const```</span>
  ///
  ///
  pub fn is_item_enabled(&self, index: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QToolBox_isItemEnabled(self as *const ::tool_box::ToolBox, index) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QToolBox::itemIcon(int index) const```</span>
  ///
  ///
  pub fn item_icon(&self, index: ::libc::c_int) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolBox_itemIcon_to_output(self as *const ::tool_box::ToolBox, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QToolBox::itemText(int index) const```</span>
  ///
  ///
  pub fn item_text(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolBox_itemText_to_output(self as *const ::tool_box::ToolBox, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QToolBox::itemToolTip(int index) const```</span>
  ///
  ///
  pub fn item_tool_tip(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolBox_itemToolTip_to_output(self as *const ::tool_box::ToolBox, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QToolBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QToolBox_metaObject(self as *const ::tool_box::ToolBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QToolBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QToolBox_qt_metacall(self as *mut ::tool_box::ToolBox,
                                             arg1 as *const ::qt_core::meta_object::Call,
                                             arg2,
                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QToolBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QToolBox_qt_metacast(self as *mut ::tool_box::ToolBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QToolBox::removeItem(int index)```</span>
  ///
  ///
  pub fn remove_item(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QToolBox_removeItem(self as *mut ::tool_box::ToolBox, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QToolBox::setCurrentIndex(int index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QToolBox_setCurrentIndex(self as *mut ::tool_box::ToolBox, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QToolBox::setCurrentWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_current_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QToolBox_setCurrentWidget(self as *mut ::tool_box::ToolBox, widget)
  }

  /// C++ method: <span style='color: green;'>```void QToolBox::setItemEnabled(int index, bool enabled)```</span>
  ///
  ///
  pub fn set_item_enabled(&mut self, index: ::libc::c_int, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QToolBox_setItemEnabled(self as *mut ::tool_box::ToolBox, index, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QToolBox::setItemIcon(int index, const QIcon& icon)```</span>
  ///
  ///
  pub fn set_item_icon(&mut self, index: ::libc::c_int, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QToolBox_setItemIcon(self as *mut ::tool_box::ToolBox,
                                               index,
                                               icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QToolBox::setItemText(int index, const QString& text)```</span>
  ///
  ///
  pub fn set_item_text(&mut self, index: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QToolBox_setItemText(self as *mut ::tool_box::ToolBox,
                                               index,
                                               text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QToolBox::setItemToolTip(int index, const QString& toolTip)```</span>
  ///
  ///
  pub fn set_item_tool_tip(&mut self, index: ::libc::c_int, tool_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QToolBox_setItemToolTip(self as *mut ::tool_box::ToolBox,
                                                  index,
                                                  tool_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QToolBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QToolBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QToolBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QToolBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QToolBox::widget(int index) const```</span>
  ///
  ///
  pub fn widget(&self, index: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QToolBox_widget(self as *const ::tool_box::ToolBox, index) }
  }
}

impl ::cpp_utils::CppDeletable for ::tool_box::ToolBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QToolBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ToolBox`.
  pub struct Signals<'a>(&'a ::tool_box::ToolBox);
  /// Represents a built-in Qt signal `QToolBox::currentChanged`.
  ///
  /// An object of this type can be created from `ToolBox` with `object.signals().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBox` object.
  pub struct CurrentChanged<'a>(&'a ::tool_box::ToolBox);
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
    /// Returns an object representing a built-in Qt signal `QToolBox::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ToolBox`.
  pub struct Slots<'a>(&'a ::tool_box::ToolBox);
  /// Represents a built-in Qt slot `QToolBox::setCurrentWidget`.
  ///
  /// An object of this type can be created from `ToolBox` with `object.slots().set_current_widget()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBox` object.
  pub struct SetCurrentWidget<'a>(&'a ::tool_box::ToolBox);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentWidget<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentWidget(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBox::setCurrentIndex`.
  ///
  /// An object of this type can be created from `ToolBox` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBox` object.
  pub struct SetCurrentIndex<'a>(&'a ::tool_box::ToolBox);
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
    /// Returns an object representing a built-in Qt slot `QToolBox::setCurrentWidget`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_widget(&self) -> SetCurrentWidget {
      SetCurrentWidget(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBox::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
  }
  impl ::tool_box::ToolBox {
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

impl ::cpp_utils::DynamicCast<::tool_box::ToolBox> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tool_box::ToolBox> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tool_box::ToolBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tool_box::ToolBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tool_box::ToolBox> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tool_box::ToolBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tool_box::ToolBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QObject_ptr(self as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QObject_ptr(self as *const ::tool_box::ToolBox as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::tool_box::ToolBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QPaintDevice_ptr(self as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QPaintDevice_ptr(self as *const ::tool_box::ToolBox as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::tool_box::ToolBox {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QFrame_ptr(self as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QFrame_ptr(self as *const ::tool_box::ToolBox as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::tool_box::ToolBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QWidget_ptr(self as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QWidget_ptr(self as *const ::tool_box::ToolBox as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_box::ToolBox> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_box::ToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_box::ToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_box::ToolBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_box::ToolBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_box::ToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_box::ToolBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_box::ToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_box::ToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_box::ToolBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_box::ToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_box::ToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tool_box::ToolBox {
  type Target = ::frame::Frame;
  fn deref(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QFrame_ptr(self as *const ::tool_box::ToolBox as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tool_box::ToolBox {
  fn deref_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBox_G_static_cast_QFrame_ptr(self as *mut ::tool_box::ToolBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ToolBox::add_item](../struct.ToolBox.html#method.add_item) method.
  pub trait ToolBoxAddItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tool_box::ToolBox) -> ::libc::c_int;
  }
  impl<'largs> ToolBoxAddItemArgs<'largs>
    for (*mut ::widget::Widget, &'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tool_box::ToolBox) -> ::libc::c_int {
      let widget = self.0;
      let icon = self.1;
      let text = self.2;
      ::ffi::qt_widgets_c_QToolBox_addItem_widget_icon_text(original_self as *mut ::tool_box::ToolBox,
                                                            widget,
                                                            icon as *const ::qt_gui::icon::Icon,
                                                            text as *const ::qt_core::string::String)
    }
  }
  impl<'largs> ToolBoxAddItemArgs<'largs> for (*mut ::widget::Widget, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tool_box::ToolBox) -> ::libc::c_int {
      let widget = self.0;
      let text = self.1;
      ::ffi::qt_widgets_c_QToolBox_addItem_widget_text(original_self as *mut ::tool_box::ToolBox,
                                                       widget,
                                                       text as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [ToolBox::insert_item](../struct.ToolBox.html#method.insert_item) method.
  pub trait ToolBoxInsertItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tool_box::ToolBox) -> ::libc::c_int;
  }
  impl<'largs> ToolBoxInsertItemArgs<'largs>
    for (::libc::c_int, *mut ::widget::Widget, &'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tool_box::ToolBox) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      let icon = self.2;
      let text = self.3;
      ::ffi::qt_widgets_c_QToolBox_insertItem_index_widget_icon_text(original_self as *mut ::tool_box::ToolBox,
                                                                     index,
                                                                     widget,
                                                                     icon as *const ::qt_gui::icon::Icon,
                                                                     text as *const ::qt_core::string::String)
    }
  }
  impl<'largs> ToolBoxInsertItemArgs<'largs>
    for (::libc::c_int, *mut ::widget::Widget, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tool_box::ToolBox) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      let text = self.2;
      ::ffi::qt_widgets_c_QToolBox_insertItem_index_widget_text(original_self as *mut ::tool_box::ToolBox,
                                                                index,
                                                                widget,
                                                                text as *const ::qt_core::string::String)
    }
  }
}
