/// C++ type: <span style='color: green;'>```QScrollArea```</span>
#[repr(C)]
pub struct ScrollArea(u8);

impl ScrollArea {
  /// C++ method: <span style='color: green;'>```QScrollArea::ensureVisible```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScrollArea::ensureVisible(int x, int y)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScrollArea::ensureVisible(int x, int y, int xmargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScrollArea::ensureVisible(int x, int y, int xmargin = ?, int ymargin = ?)```</span>
  ///
  ///
  pub fn ensure_visible<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ScrollAreaEnsureVisibleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScrollArea::ensureWidgetVisible```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ensure_widget_visible(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScrollArea::ensureWidgetVisible(QWidget* childWidget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ensure_widget_visible(&mut self, (*mut ::widget::Widget, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScrollArea::ensureWidgetVisible(QWidget* childWidget, int xmargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ensure_widget_visible(&mut self, (*mut ::widget::Widget, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScrollArea::ensureWidgetVisible(QWidget* childWidget, int xmargin = ?, int ymargin = ?)```</span>
  ///
  ///
  pub unsafe fn ensure_widget_visible<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ScrollAreaEnsureWidgetVisibleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QScrollArea::focusNextPrevChild(bool next)```</span>
  ///
  ///
  pub fn focus_next_prev_child(&mut self, next: bool) -> bool {
    unsafe { ::ffi::qt_widgets_c_QScrollArea_focusNextPrevChild(self as *mut ::scroll_area::ScrollArea, next) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QScrollArea::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QScrollArea_metaObject(self as *const ::scroll_area::ScrollArea) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QScrollArea::QScrollArea()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::scroll_area::ScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QScrollArea::QScrollArea(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::scroll_area::ScrollArea> {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QScrollArea::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QScrollArea_qt_metacall(self as *mut ::scroll_area::ScrollArea,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QScrollArea::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QScrollArea_qt_metacast(self as *mut ::scroll_area::ScrollArea, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QScrollArea::setWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QScrollArea_setWidget(self as *mut ::scroll_area::ScrollArea, widget)
  }

  /// C++ method: <span style='color: green;'>```void QScrollArea::setWidgetResizable(bool resizable)```</span>
  ///
  ///
  pub fn set_widget_resizable(&mut self, resizable: bool) {
    unsafe { ::ffi::qt_widgets_c_QScrollArea_setWidgetResizable(self as *mut ::scroll_area::ScrollArea, resizable) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QScrollArea::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QScrollArea_sizeHint_to_output(self as *const ::scroll_area::ScrollArea, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QScrollArea::takeWidget()```</span>
  ///
  ///
  pub fn take_widget(&mut self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QScrollArea_takeWidget(self as *mut ::scroll_area::ScrollArea) }
  }

  /// C++ method: <span style='color: green;'>```static QString QScrollArea::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QScrollArea_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QScrollArea::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QScrollArea_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QScrollArea::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QScrollArea_widget(self as *const ::scroll_area::ScrollArea) }
  }

  /// C++ method: <span style='color: green;'>```bool QScrollArea::widgetResizable() const```</span>
  ///
  ///
  pub fn widget_resizable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QScrollArea_widgetResizable(self as *const ::scroll_area::ScrollArea) }
  }
}

impl ::cpp_utils::CppDeletable for ::scroll_area::ScrollArea {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QScrollArea_delete
  }
}

impl ::cpp_utils::DynamicCast<::scroll_area::ScrollArea> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::scroll_area::ScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::scroll_area::ScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::scroll_area::ScrollArea> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::scroll_area::ScrollArea> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::scroll_area::ScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::scroll_area::ScrollArea> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::scroll_area::ScrollArea> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::scroll_area::ScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::scroll_area::ScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QObject_ptr(self as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QObject_ptr(self as *const ::scroll_area::ScrollArea as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::scroll_area::ScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QPaintDevice_ptr(self as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QPaintDevice_ptr(self as *const ::scroll_area::ScrollArea as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::scroll_area::ScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::scroll_area::ScrollArea)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QAbstractScrollArea_ptr(self as *const ::scroll_area::ScrollArea as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::scroll_area::ScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QFrame_ptr(self as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QFrame_ptr(self as *const ::scroll_area::ScrollArea as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::scroll_area::ScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QWidget_ptr(self as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QWidget_ptr(self as *const ::scroll_area::ScrollArea as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_area::ScrollArea> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_area::ScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_area::ScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_area::ScrollArea> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_area::ScrollArea {
    let ffi_result =
      ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_area::ScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_area::ScrollArea> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_area::ScrollArea {
    let ffi_result =
      ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_area::ScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_area::ScrollArea> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_area::ScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_area::ScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_area::ScrollArea> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_area::ScrollArea {
    let ffi_result =
      ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_area::ScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::scroll_area::ScrollArea {
  type Target = ::abstract_scroll_area::AbstractScrollArea;
  fn deref(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QAbstractScrollArea_ptr(self as *const ::scroll_area::ScrollArea as *mut ::scroll_area::ScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::scroll_area::ScrollArea {
  fn deref_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QScrollArea_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::scroll_area::ScrollArea)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ScrollArea::ensure_visible](../struct.ScrollArea.html#method.ensure_visible) method.
  pub trait ScrollAreaEnsureVisibleArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> ();
  }
  impl<'largs> ScrollAreaEnsureVisibleArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> () {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QScrollArea_ensureVisible_x_y(original_self as *mut ::scroll_area::ScrollArea, x, y)
      }
    }
  }
  impl<'largs> ScrollAreaEnsureVisibleArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> () {
      let x = self.0;
      let y = self.1;
      let xmargin = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QScrollArea_ensureVisible_x_y_xmargin(original_self as *mut ::scroll_area::ScrollArea,
                                                                  x,
                                                                  y,
                                                                  xmargin)
      }
    }
  }
  impl<'largs> ScrollAreaEnsureVisibleArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> () {
      let x = self.0;
      let y = self.1;
      let xmargin = self.2;
      let ymargin = self.3;
      unsafe { ::ffi::qt_widgets_c_QScrollArea_ensureVisible_x_y_xmargin_ymargin(original_self as *mut ::scroll_area::ScrollArea, x, y, xmargin, ymargin) }
    }
  }
  /// This trait represents a set of arguments accepted by [ScrollArea::ensure_widget_visible](../struct.ScrollArea.html#method.ensure_widget_visible) method.
  pub trait ScrollAreaEnsureWidgetVisibleArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> ();
  }
  impl<'largs> ScrollAreaEnsureWidgetVisibleArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> () {
      let child_widget = self;
      ::ffi::qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget(original_self as *mut ::scroll_area::ScrollArea,
                                                                      child_widget)
    }
  }
  impl<'largs> ScrollAreaEnsureWidgetVisibleArgs<'largs> for (*mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> () {
      let child_widget = self.0;
      let xmargin = self.1;
      ::ffi::qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget_xmargin(original_self as *mut ::scroll_area::ScrollArea, child_widget, xmargin)
    }
  }
  impl<'largs> ScrollAreaEnsureWidgetVisibleArgs<'largs> for (*mut ::widget::Widget, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::scroll_area::ScrollArea) -> () {
      let child_widget = self.0;
      let xmargin = self.1;
      let ymargin = self.2;
      ::ffi::qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget_xmargin_ymargin(original_self as *mut ::scroll_area::ScrollArea, child_widget, xmargin, ymargin)
    }
  }
}
