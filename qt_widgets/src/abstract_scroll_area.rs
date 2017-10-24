/// C++ type: <span style='color: green;'>```QAbstractScrollArea```</span>
#[repr(C)]
pub struct AbstractScrollArea(u8);

impl AbstractScrollArea {
  /// C++ method: <span style='color: green;'>```QWidget* QAbstractScrollArea::cornerWidget() const```</span>
  ///
  ///
  pub fn corner_widget(&self) -> *mut ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractScrollArea_cornerWidget(self as *const ::abstract_scroll_area::AbstractScrollArea)
    }
  }

  /// C++ method: <span style='color: green;'>```QScrollBar* QAbstractScrollArea::horizontalScrollBar() const```</span>
  ///
  ///
  pub fn horizontal_scroll_bar(&self) -> *mut ::scroll_bar::ScrollBar {
    unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_horizontalScrollBar(self as *const ::abstract_scroll_area::AbstractScrollArea) }
  }

  /// C++ method: <span style='color: green;'>```QSize QAbstractScrollArea::maximumViewportSize() const```</span>
  ///
  ///
  pub fn maximum_viewport_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractScrollArea_maximumViewportSize_to_output(self as *const ::abstract_scroll_area::AbstractScrollArea, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractScrollArea::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractScrollArea_metaObject(self as *const ::abstract_scroll_area::AbstractScrollArea)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QAbstractScrollArea::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractScrollArea_minimumSizeHint_to_output(self as *const ::abstract_scroll_area::AbstractScrollArea, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractScrollArea::QAbstractScrollArea()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::abstract_scroll_area::AbstractScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractScrollArea::QAbstractScrollArea(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget)
                           -> ::cpp_utils::CppBox<::abstract_scroll_area::AbstractScrollArea> {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractScrollArea::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAbstractScrollArea_qt_metacall(self as *mut ::abstract_scroll_area::AbstractScrollArea,
                                                        arg1 as *const ::qt_core::meta_object::Call,
                                                        arg2,
                                                        arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAbstractScrollArea::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QAbstractScrollArea_qt_metacast(self as *mut ::abstract_scroll_area::AbstractScrollArea,
                                                        arg1)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractScrollArea::setCornerWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_corner_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QAbstractScrollArea_setCornerWidget(self as *mut ::abstract_scroll_area::AbstractScrollArea,
                                                            widget)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractScrollArea::setHorizontalScrollBar(QScrollBar* scrollbar)```</span>
  ///
  ///
  pub unsafe fn set_horizontal_scroll_bar(&mut self, scrollbar: *mut ::scroll_bar::ScrollBar) {
    ::ffi::qt_widgets_c_QAbstractScrollArea_setHorizontalScrollBar(self as *mut ::abstract_scroll_area::AbstractScrollArea, scrollbar)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractScrollArea::setHorizontalScrollBarPolicy(Qt::ScrollBarPolicy arg1)```</span>
  ///
  ///
  pub fn set_horizontal_scroll_bar_policy(&mut self, arg1: &::qt_core::qt::ScrollBarPolicy) {
    unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_setHorizontalScrollBarPolicy(self as *mut ::abstract_scroll_area::AbstractScrollArea, arg1 as *const ::qt_core::qt::ScrollBarPolicy) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractScrollArea::setSizeAdjustPolicy(QAbstractScrollArea::SizeAdjustPolicy policy)```</span>
  ///
  ///
  pub fn set_size_adjust_policy(&mut self, policy: ::abstract_scroll_area::SizeAdjustPolicy) {
    unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_setSizeAdjustPolicy(self as *mut ::abstract_scroll_area::AbstractScrollArea, policy) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractScrollArea::setVerticalScrollBar(QScrollBar* scrollbar)```</span>
  ///
  ///
  pub unsafe fn set_vertical_scroll_bar(&mut self, scrollbar: *mut ::scroll_bar::ScrollBar) {
    ::ffi::qt_widgets_c_QAbstractScrollArea_setVerticalScrollBar(self as *mut ::abstract_scroll_area::AbstractScrollArea, scrollbar)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractScrollArea::setVerticalScrollBarPolicy(Qt::ScrollBarPolicy arg1)```</span>
  ///
  ///
  pub fn set_vertical_scroll_bar_policy(&mut self, arg1: &::qt_core::qt::ScrollBarPolicy) {
    unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_setVerticalScrollBarPolicy(self as *mut ::abstract_scroll_area::AbstractScrollArea, arg1 as *const ::qt_core::qt::ScrollBarPolicy) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractScrollArea::setViewport(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_viewport(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QAbstractScrollArea_setViewport(self as *mut ::abstract_scroll_area::AbstractScrollArea,
                                                        widget)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractScrollArea::setupViewport(QWidget* viewport)```</span>
  ///
  ///
  pub unsafe fn setup_viewport(&mut self, viewport: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QAbstractScrollArea_setupViewport(self as *mut ::abstract_scroll_area::AbstractScrollArea,
                                                          viewport)
  }

  /// C++ method: <span style='color: green;'>```QAbstractScrollArea::SizeAdjustPolicy QAbstractScrollArea::sizeAdjustPolicy() const```</span>
  ///
  ///
  pub fn size_adjust_policy(&self) -> ::abstract_scroll_area::SizeAdjustPolicy {
    unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_sizeAdjustPolicy(self as *const ::abstract_scroll_area::AbstractScrollArea) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QAbstractScrollArea::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractScrollArea_sizeHint_to_output(self as *const ::abstract_scroll_area::AbstractScrollArea, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractScrollArea::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractScrollArea_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractScrollArea::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractScrollArea_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QScrollBar* QAbstractScrollArea::verticalScrollBar() const```</span>
  ///
  ///
  pub fn vertical_scroll_bar(&self) -> *mut ::scroll_bar::ScrollBar {
    unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_verticalScrollBar(self as *const ::abstract_scroll_area::AbstractScrollArea) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QAbstractScrollArea::viewport() const```</span>
  ///
  ///
  pub fn viewport(&self) -> *mut ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractScrollArea_viewport(self as *const ::abstract_scroll_area::AbstractScrollArea)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_scroll_area::AbstractScrollArea {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAbstractScrollArea_delete
  }
}

/// C++ type: <span style='color: green;'>```QAbstractScrollArea::SizeAdjustPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SizeAdjustPolicy {
  /// C++ enum variant: <span style='color: green;'>```AdjustIgnored = 0```</span>
  Ignored = 0,
  /// C++ enum variant: <span style='color: green;'>```AdjustToContentsOnFirstShow = 1```</span>
  ToContentsOnFirstShow = 1,
  /// C++ enum variant: <span style='color: green;'>```AdjustToContents = 2```</span>
  ToContents = 2,
}

impl ::cpp_utils::DynamicCast<::abstract_scroll_area::AbstractScrollArea> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_scroll_area::AbstractScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_dynamic_cast_QAbstractScrollArea_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_scroll_area::AbstractScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_dynamic_cast_QAbstractScrollArea_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::abstract_scroll_area::AbstractScrollArea> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_scroll_area::AbstractScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_dynamic_cast_QAbstractScrollArea_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_scroll_area::AbstractScrollArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_dynamic_cast_QAbstractScrollArea_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_scroll_area::AbstractScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QObject_ptr(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QObject_ptr(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::abstract_scroll_area::AbstractScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QPaintDevice_ptr(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QPaintDevice_ptr(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::abstract_scroll_area::AbstractScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QFrame_ptr(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QFrame_ptr(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::abstract_scroll_area::AbstractScrollArea {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QWidget_ptr(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QWidget_ptr(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_scroll_area::AbstractScrollArea> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_scroll_area::AbstractScrollArea> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_scroll_area::AbstractScrollArea> for ::qt_gui::paint_device::PaintDevice {
unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::abstract_scroll_area::AbstractScrollArea> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_scroll_area::AbstractScrollArea {
  type Target = ::frame::Frame;
  fn deref(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QFrame_ptr(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_scroll_area::AbstractScrollArea {
  fn deref_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractScrollArea_G_static_cast_QFrame_ptr(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
