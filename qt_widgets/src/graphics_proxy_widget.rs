/// C++ type: <span style='color: green;'>```QGraphicsProxyWidget```</span>
#[repr(C)]
pub struct GraphicsProxyWidget(u8);

impl GraphicsProxyWidget {
  /// C++ method: <span style='color: green;'>```QGraphicsProxyWidget* QGraphicsProxyWidget::createProxyForChildWidget(QWidget* child)```</span>
  ///
  ///
  pub unsafe fn create_proxy_for_child_widget(&mut self,
                                              child: *mut ::widget::Widget)
                                              -> *mut ::graphics_proxy_widget::GraphicsProxyWidget {
    ::ffi::qt_widgets_c_QGraphicsProxyWidget_createProxyForChildWidget(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget, child)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsProxyWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsProxyWidget_metaObject(self as *const ::graphics_proxy_widget::GraphicsProxyWidget)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsProxyWidget::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn paint(&mut self,
                      painter: *mut ::qt_gui::painter::Painter,
                      option: *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                      widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGraphicsProxyWidget_paint(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget,
                                                   painter,
                                                   option,
                                                   widget)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsProxyWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsProxyWidget_qt_metacall(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget,
                                                         arg1 as *const ::qt_core::meta_object::Call,
                                                         arg2,
                                                         arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsProxyWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsProxyWidget_qt_metacast(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget,
                                                         arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsProxyWidget::setGeometry(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsProxyWidget_setGeometry(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget,
                                                           rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsProxyWidget::setWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGraphicsProxyWidget_setWidget(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget,
                                                       widget)
  }

  /// C++ method: <span style='color: green;'>```QRectF QGraphicsProxyWidget::subWidgetRect(const QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn sub_widget_rect(&self, widget: *const ::widget::Widget) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsProxyWidget_subWidgetRect_to_output(self as *const ::graphics_proxy_widget::GraphicsProxyWidget, widget, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsProxyWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsProxyWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsProxyWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsProxyWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsProxyWidget::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsProxyWidget_type(self as *const ::graphics_proxy_widget::GraphicsProxyWidget)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QGraphicsProxyWidget::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *mut ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsProxyWidget_widget(self as *const ::graphics_proxy_widget::GraphicsProxyWidget)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_proxy_widget::GraphicsProxyWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsProxyWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsProxyWidget`.
  pub struct Signals<'a>(&'a ::graphics_proxy_widget::GraphicsProxyWidget);
  /// Represents a built-in Qt signal `QGraphicsProxyWidget::layoutChanged`.
  ///
  /// An object of this type can be created from `GraphicsProxyWidget` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsProxyWidget` object.
  pub struct LayoutChanged<'a>(&'a ::graphics_proxy_widget::GraphicsProxyWidget);
  impl<'a> ::qt_core::connection::Receiver for LayoutChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LayoutChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsProxyWidget::geometryChanged`.
  ///
  /// An object of this type can be created from `GraphicsProxyWidget` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsProxyWidget` object.
  pub struct GeometryChanged<'a>(&'a ::graphics_proxy_widget::GraphicsProxyWidget);
  impl<'a> ::qt_core::connection::Receiver for GeometryChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2geometryChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GeometryChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsProxyWidget::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsProxyWidget::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsProxyWidget`.
  pub struct Slots<'a>(&'a ::graphics_proxy_widget::GraphicsProxyWidget);
  /// Represents a built-in Qt slot `QGraphicsProxyWidget::newProxyWidget`.
  ///
  /// An object of this type can be created from `GraphicsProxyWidget` with `object.slots().new_proxy_widget()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsProxyWidget` object.
  pub struct NewProxyWidget<'a>(&'a ::graphics_proxy_widget::GraphicsProxyWidget);
  impl<'a> ::qt_core::connection::Receiver for NewProxyWidget<'a> {
    type Arguments = (*const ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1newProxyWidget(const QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsProxyWidget::close`.
  ///
  /// An object of this type can be created from `GraphicsProxyWidget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsProxyWidget` object.
  pub struct Close<'a>(&'a ::graphics_proxy_widget::GraphicsProxyWidget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsProxyWidget::newProxyWidget`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn new_proxy_widget(&self) -> NewProxyWidget {
      NewProxyWidget(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsProxyWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
  }
  impl ::graphics_proxy_widget::GraphicsProxyWidget {
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

impl ::cpp_utils::DynamicCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_proxy_widget::GraphicsProxyWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_proxy_widget::GraphicsProxyWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_layout_item::GraphicsLayoutItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_proxy_widget::GraphicsProxyWidget> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_proxy_widget::GraphicsProxyWidget> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_object::GraphicsObject {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_proxy_widget::GraphicsProxyWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsObject(self as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_proxy_widget::GraphicsProxyWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsObject(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_widget::GraphicsWidget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_proxy_widget::GraphicsProxyWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsWidget(self as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_proxy_widget::GraphicsProxyWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsWidget(self as *const ::graphics_widget::GraphicsWidget as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_proxy_widget::GraphicsProxyWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QObject_ptr(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QObject_ptr(self as *const ::graphics_proxy_widget::GraphicsProxyWidget as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_proxy_widget::GraphicsProxyWidget {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_proxy_widget::GraphicsProxyWidget as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_layout_item::GraphicsLayoutItem> for ::graphics_proxy_widget::GraphicsProxyWidget {
fn static_cast_mut(&mut self) -> &mut ::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsLayoutItem_ptr(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsLayoutItem_ptr(self as *const ::graphics_proxy_widget::GraphicsProxyWidget as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_object::GraphicsObject> for ::graphics_proxy_widget::GraphicsProxyWidget {
  fn static_cast_mut(&mut self) -> &mut ::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsObject_ptr(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsObject_ptr(self as *const ::graphics_proxy_widget::GraphicsProxyWidget as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_widget::GraphicsWidget> for ::graphics_proxy_widget::GraphicsProxyWidget {
  fn static_cast_mut(&mut self) -> &mut ::graphics_widget::GraphicsWidget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsWidget_ptr(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_widget::GraphicsWidget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsWidget_ptr(self as *const ::graphics_proxy_widget::GraphicsProxyWidget as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_proxy_widget::GraphicsProxyWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_proxy_widget::GraphicsProxyWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_layout_item::GraphicsLayoutItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_proxy_widget::GraphicsProxyWidget {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_proxy_widget::GraphicsProxyWidget {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_object::GraphicsObject {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_proxy_widget::GraphicsProxyWidget {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsObject(self as *mut ::graphics_object::GraphicsObject);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_proxy_widget::GraphicsProxyWidget {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsObject(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::graphics_widget::GraphicsWidget {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_proxy_widget::GraphicsProxyWidget {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsWidget(self as *mut ::graphics_widget::GraphicsWidget);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_proxy_widget::GraphicsProxyWidget {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsWidget(self as *const ::graphics_widget::GraphicsWidget as *mut ::graphics_widget::GraphicsWidget);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_proxy_widget::GraphicsProxyWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_proxy_widget::GraphicsProxyWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_proxy_widget::GraphicsProxyWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_proxy_widget::GraphicsProxyWidget {
  type Target = ::graphics_widget::GraphicsWidget;
  fn deref(&self) -> &::graphics_widget::GraphicsWidget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsWidget_ptr(self as *const ::graphics_proxy_widget::GraphicsProxyWidget as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_proxy_widget::GraphicsProxyWidget {
  fn deref_mut(&mut self) -> &mut ::graphics_widget::GraphicsWidget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsWidget_ptr(self as *mut ::graphics_proxy_widget::GraphicsProxyWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
