/// C++ type: <span style='color: green;'>```QSplitter```</span>
#[repr(C)]
pub struct Splitter(u8);

impl Splitter {
  /// C++ method: <span style='color: green;'>```void QSplitter::addWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn add_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QSplitter_addWidget(self as *mut ::splitter::Splitter, widget)
  }

  /// C++ method: <span style='color: green;'>```bool QSplitter::childrenCollapsible() const```</span>
  ///
  ///
  pub fn children_collapsible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSplitter_childrenCollapsible(self as *const ::splitter::Splitter) }
  }

  /// C++ method: <span style='color: green;'>```int QSplitter::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSplitter_count(self as *const ::splitter::Splitter) }
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::getRange(int index, int* arg2, int* arg3) const```</span>
  ///
  ///
  pub unsafe fn get_range(&self, index: ::libc::c_int, arg2: *mut ::libc::c_int, arg3: *mut ::libc::c_int) {
    ::ffi::qt_widgets_c_QSplitter_getRange(self as *const ::splitter::Splitter, index, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```QSplitterHandle* QSplitter::handle(int index) const```</span>
  ///
  ///
  pub fn handle(&self, index: ::libc::c_int) -> *mut ::splitter_handle::SplitterHandle {
    unsafe { ::ffi::qt_widgets_c_QSplitter_handle(self as *const ::splitter::Splitter, index) }
  }

  /// C++ method: <span style='color: green;'>```int QSplitter::handleWidth() const```</span>
  ///
  ///
  pub fn handle_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSplitter_handleWidth(self as *const ::splitter::Splitter) }
  }

  /// C++ method: <span style='color: green;'>```int QSplitter::indexOf(QWidget* w) const```</span>
  ///
  ///
  pub unsafe fn index_of(&self, w: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSplitter_indexOf(self as *const ::splitter::Splitter, w)
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::insertWidget(int index, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn insert_widget(&mut self, index: ::libc::c_int, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QSplitter_insertWidget(self as *mut ::splitter::Splitter, index, widget)
  }

  /// C++ method: <span style='color: green;'>```bool QSplitter::isCollapsible(int index) const```</span>
  ///
  ///
  pub fn is_collapsible(&self, index: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSplitter_isCollapsible(self as *const ::splitter::Splitter, index) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSplitter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QSplitter_metaObject(self as *const ::splitter::Splitter) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSplitter::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSplitter_minimumSizeHint_to_output(self as *const ::splitter::Splitter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSplitter::QSplitter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::splitter::Splitter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSplitter::QSplitter()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::Orientation) -> ::cpp_utils::CppBox<::splitter::Splitter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSplitter::QSplitter(Qt::Orientation arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::splitter::Splitter>
    where Args: overloading::SplitterNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSplitter::QSplitter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::splitter::Splitter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSplitter::QSplitter(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::qt::Orientation, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::splitter::Splitter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSplitter::QSplitter(Qt::Orientation arg1, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::splitter::Splitter>
    where Args: overloading::SplitterNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QSplitter::opaqueResize() const```</span>
  ///
  ///
  pub fn opaque_resize(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSplitter_opaqueResize(self as *const ::splitter::Splitter) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QSplitter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSplitter_qt_metacall(self as *mut ::splitter::Splitter,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSplitter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QSplitter_qt_metacast(self as *mut ::splitter::Splitter, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::refresh()```</span>
  ///
  ///
  pub fn refresh(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QSplitter_refresh(self as *mut ::splitter::Splitter) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QSplitter::replaceWidget(int index, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn replace_widget(&mut self,
                               index: ::libc::c_int,
                               widget: *mut ::widget::Widget)
                               -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QSplitter_replaceWidget(self as *mut ::splitter::Splitter, index, widget)
  }

  /// C++ method: <span style='color: green;'>```bool QSplitter::restoreState(const QByteArray& state)```</span>
  ///
  ///
  pub fn restore_state(&mut self, state: &::qt_core::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QSplitter_restoreState(self as *mut ::splitter::Splitter,
                                                 state as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QSplitter::saveState() const```</span>
  ///
  ///
  pub fn save_state(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSplitter_saveState_to_output(self as *const ::splitter::Splitter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::setChildrenCollapsible(bool arg1)```</span>
  ///
  ///
  pub fn set_children_collapsible(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QSplitter_setChildrenCollapsible(self as *mut ::splitter::Splitter, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::setCollapsible(int index, bool arg2)```</span>
  ///
  ///
  pub fn set_collapsible(&mut self, index: ::libc::c_int, arg2: bool) {
    unsafe { ::ffi::qt_widgets_c_QSplitter_setCollapsible(self as *mut ::splitter::Splitter, index, arg2) }
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::setHandleWidth(int arg1)```</span>
  ///
  ///
  pub fn set_handle_width(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSplitter_setHandleWidth(self as *mut ::splitter::Splitter, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QSplitter::setOpaqueResize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_opaque_resize(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSplitter::setOpaqueResize()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_opaque_resize(&mut self, bool) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSplitter::setOpaqueResize(bool opaque = ?)```</span>
  ///
  ///
  pub fn set_opaque_resize<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SplitterSetOpaqueResizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSplitter::setOrientation(Qt::Orientation arg1)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, arg1: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QSplitter_setOrientation(self as *mut ::splitter::Splitter,
                                                   arg1 as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::setSizes(const QList<int>& list)```</span>
  ///
  ///
  pub fn set_sizes(&mut self, list: &::qt_core::list::ListCInt) {
    unsafe {
      ::ffi::qt_widgets_c_QSplitter_setSizes(self as *mut ::splitter::Splitter,
                                             list as *const ::qt_core::list::ListCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSplitter::setStretchFactor(int index, int stretch)```</span>
  ///
  ///
  pub fn set_stretch_factor(&mut self, index: ::libc::c_int, stretch: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSplitter_setStretchFactor(self as *mut ::splitter::Splitter, index, stretch) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSplitter::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSplitter_sizeHint_to_output(self as *const ::splitter::Splitter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QSplitter::sizes() const```</span>
  ///
  ///
  pub fn sizes(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSplitter_sizes_to_output(self as *const ::splitter::Splitter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSplitter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSplitter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSplitter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSplitter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QSplitter::widget(int index) const```</span>
  ///
  ///
  pub fn widget(&self, index: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QSplitter_widget(self as *const ::splitter::Splitter, index) }
  }
}

impl ::cpp_utils::CppDeletable for ::splitter::Splitter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSplitter_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Splitter`.
  pub struct Signals<'a>(&'a ::splitter::Splitter);
  /// Represents a built-in Qt signal `QSplitter::splitterMoved`.
  ///
  /// An object of this type can be created from `Splitter` with `object.signals().splitter_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Splitter` object.
  pub struct SplitterMoved<'a>(&'a ::splitter::Splitter);
  impl<'a> ::qt_core::connection::Receiver for SplitterMoved<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2splitterMoved(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SplitterMoved<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSplitter::splitterMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn splitter_moved(&self) -> SplitterMoved {
      SplitterMoved(self.0)
    }
  }
  impl ::splitter::Splitter {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ method: <span style='color: green;'>```QTextStream& operator<<(QTextStream& arg1, const QSplitter& arg2)```</span>
///
///
pub fn op_shl<'l0, 'l1>(arg1: &'l0 mut ::qt_core::text_stream::TextStream,
                        arg2: &'l1 ::splitter::Splitter)
                        -> &'l0 mut ::qt_core::text_stream::TextStream {
  let ffi_result = unsafe {
    ::ffi::qt_widgets_c_QSplitter_G_operator_shl(arg1 as *mut ::qt_core::text_stream::TextStream,
                                                 arg2 as *const ::splitter::Splitter)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& operator>>(QTextStream& arg1, QSplitter& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::text_stream::TextStream,
                        arg2: &'l1 mut ::splitter::Splitter)
                        -> &'l0 mut ::qt_core::text_stream::TextStream {
  let ffi_result = unsafe {
    ::ffi::qt_widgets_c_QSplitter_G_operator_shr(arg1 as *mut ::qt_core::text_stream::TextStream,
                                                 arg2 as *mut ::splitter::Splitter)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

impl ::cpp_utils::DynamicCast<::splitter::Splitter> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::splitter::Splitter> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::splitter::Splitter> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::splitter::Splitter> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::splitter::Splitter> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::splitter::Splitter> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::splitter::Splitter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QObject_ptr(self as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QObject_ptr(self as *const ::splitter::Splitter as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::splitter::Splitter {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QPaintDevice_ptr(self as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QPaintDevice_ptr(self as *const ::splitter::Splitter as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::splitter::Splitter {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QFrame_ptr(self as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QFrame_ptr(self as *const ::splitter::Splitter as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::splitter::Splitter {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QWidget_ptr(self as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QWidget_ptr(self as *const ::splitter::Splitter as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splitter::Splitter> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splitter::Splitter {
    let ffi_result = ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splitter::Splitter {
    let ffi_result = ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splitter::Splitter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splitter::Splitter {
    let ffi_result =
      ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splitter::Splitter {
    let ffi_result = ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splitter::Splitter> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splitter::Splitter {
    let ffi_result = ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splitter::Splitter {
    let ffi_result = ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splitter::Splitter> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splitter::Splitter {
    let ffi_result = ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splitter::Splitter {
    let ffi_result = ::ffi::qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::splitter::Splitter {
  type Target = ::frame::Frame;
  fn deref(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QFrame_ptr(self as *const ::splitter::Splitter as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::splitter::Splitter {
  fn deref_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitter_G_static_cast_QFrame_ptr(self as *mut ::splitter::Splitter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Splitter::new](../struct.Splitter.html#method.new) method.
  pub trait SplitterNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::splitter::Splitter>;
  }
  impl<'a> SplitterNewArgs for &'a ::qt_core::qt::Orientation {
    fn exec(self) -> ::cpp_utils::CppBox<::splitter::Splitter> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_new_arg1(arg1 as *const ::qt_core::qt::Orientation) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl SplitterNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::splitter::Splitter> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitter_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Splitter::new_unsafe](../struct.Splitter.html#method.new_unsafe) method.
  pub trait SplitterNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::splitter::Splitter>;
  }
  impl<'a> SplitterNewUnsafeArgs for (&'a ::qt_core::qt::Orientation, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::splitter::Splitter> {
      let arg1 = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QSplitter_new_arg1_parent(arg1 as *const ::qt_core::qt::Orientation, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SplitterNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::splitter::Splitter> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QSplitter_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Splitter::set_opaque_resize](../struct.Splitter.html#method.set_opaque_resize) method.
  pub trait SplitterSetOpaqueResizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::splitter::Splitter) -> ();
  }
  impl<'largs> SplitterSetOpaqueResizeArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::splitter::Splitter) -> () {

      unsafe { ::ffi::qt_widgets_c_QSplitter_setOpaqueResize_no_args(original_self as *mut ::splitter::Splitter) }
    }
  }
  impl<'largs> SplitterSetOpaqueResizeArgs<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::splitter::Splitter) -> () {
      let opaque = self;
      unsafe {
        ::ffi::qt_widgets_c_QSplitter_setOpaqueResize_opaque(original_self as *mut ::splitter::Splitter, opaque)
      }
    }
  }
}
