/// C++ type: <span style='color: green;'>```QUndoView```</span>
#[repr(C)]
pub struct UndoView(u8);

impl UndoView {
  /// C++ method: <span style='color: green;'>```QIcon QUndoView::cleanIcon() const```</span>
  ///
  ///
  pub fn clean_icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoView_cleanIcon_to_output(self as *const ::undo_view::UndoView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QUndoView::emptyLabel() const```</span>
  ///
  ///
  pub fn empty_label(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoView_emptyLabel_to_output(self as *const ::undo_view::UndoView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUndoGroup* QUndoView::group() const```</span>
  ///
  ///
  pub fn group(&self) -> *mut ::undo_group::UndoGroup {
    unsafe { ::ffi::qt_widgets_c_QUndoView_group(self as *const ::undo_view::UndoView) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QUndoView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QUndoView_metaObject(self as *const ::undo_view::UndoView) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QUndoView::QUndoView()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QUndoView::QUndoView```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::undo_group::UndoGroup) -> ::cpp_utils::CppBox<::undo_view::UndoView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoView::QUndoView(QUndoGroup* group)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::undo_group::UndoGroup, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::undo_view::UndoView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoView::QUndoView(QUndoGroup* group, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::undo_stack::UndoStack) -> ::cpp_utils::CppBox<::undo_view::UndoView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoView::QUndoView(QUndoStack* stack)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::undo_stack::UndoStack, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::undo_view::UndoView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoView::QUndoView(QUndoStack* stack, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::undo_view::UndoView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoView::QUndoView(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::undo_view::UndoView>
    where Args: overloading::UndoViewNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QUndoView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QUndoView_qt_metacall(self as *mut ::undo_view::UndoView,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QUndoView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QUndoView_qt_metacast(self as *mut ::undo_view::UndoView, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QUndoView::setCleanIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_clean_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QUndoView_setCleanIcon(self as *mut ::undo_view::UndoView,
                                                 icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QUndoView::setEmptyLabel(const QString& label)```</span>
  ///
  ///
  pub fn set_empty_label(&mut self, label: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QUndoView_setEmptyLabel(self as *mut ::undo_view::UndoView,
                                                  label as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoView::setGroup(QUndoGroup* group)```</span>
  ///
  ///
  pub unsafe fn set_group(&mut self, group: *mut ::undo_group::UndoGroup) {
    ::ffi::qt_widgets_c_QUndoView_setGroup(self as *mut ::undo_view::UndoView, group)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoView::setStack(QUndoStack* stack)```</span>
  ///
  ///
  pub unsafe fn set_stack(&mut self, stack: *mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QUndoView_setStack(self as *mut ::undo_view::UndoView, stack)
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* QUndoView::stack() const```</span>
  ///
  ///
  pub fn stack(&self) -> *mut ::undo_stack::UndoStack {
    unsafe { ::ffi::qt_widgets_c_QUndoView_stack(self as *const ::undo_view::UndoView) }
  }

  /// C++ method: <span style='color: green;'>```static QString QUndoView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QUndoView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUndoView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QUndoView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::undo_view::UndoView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QUndoView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `UndoView`.
  pub struct Signals<'a>(&'a ::undo_view::UndoView);
  /// Represents a built-in Qt signal `QUndoView::indexesMoved`.
  ///
  /// An object of this type can be created from `UndoView` with `object.signals().indexes_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoView` object.
  pub struct IndexesMoved<'a>(&'a ::undo_view::UndoView);
  impl<'a> ::qt_core::connection::Receiver for IndexesMoved<'a> {
    type Arguments = (&'static ::qt_core::list::ListModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2indexesMoved(const QList< QModelIndex >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IndexesMoved<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QUndoView::indexesMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn indexes_moved(&self) -> IndexesMoved {
      IndexesMoved(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `UndoView`.
  pub struct Slots<'a>(&'a ::undo_view::UndoView);
  /// Represents a built-in Qt slot `QUndoView::setGroup`.
  ///
  /// An object of this type can be created from `UndoView` with `object.slots().set_group()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoView` object.
  pub struct SetGroup<'a>(&'a ::undo_view::UndoView);
  impl<'a> ::qt_core::connection::Receiver for SetGroup<'a> {
    type Arguments = (*mut ::undo_group::UndoGroup,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGroup(QUndoGroup*)\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoView::setStack`.
  ///
  /// An object of this type can be created from `UndoView` with `object.slots().set_stack()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoView` object.
  pub struct SetStack<'a>(&'a ::undo_view::UndoView);
  impl<'a> ::qt_core::connection::Receiver for SetStack<'a> {
    type Arguments = (*mut ::undo_stack::UndoStack,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStack(QUndoStack*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QUndoView::setGroup`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_group(&self) -> SetGroup {
      SetGroup(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoView::setStack`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_stack(&self) -> SetStack {
      SetStack(self.0)
    }
  }
  impl ::undo_view::UndoView {
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

impl ::cpp_utils::DynamicCast<::undo_view::UndoView> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::undo_view::UndoView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::undo_view::UndoView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::undo_view::UndoView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::undo_view::UndoView> for ::list_view::ListView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::undo_view::UndoView> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QListView(self as *mut ::list_view::ListView)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QListView(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::undo_view::UndoView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::undo_view::UndoView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::undo_view::UndoView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::undo_view::UndoView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QObject_ptr(self as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QObject_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::undo_view::UndoView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QPaintDevice_ptr(self as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QPaintDevice_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::undo_view::UndoView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QAbstractItemView_ptr(self as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QAbstractItemView_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::undo_view::UndoView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QUndoView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::undo_view::UndoView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::undo_view::UndoView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QFrame_ptr(self as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QFrame_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::list_view::ListView> for ::undo_view::UndoView {
  fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QListView_ptr(self as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QListView_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::undo_view::UndoView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QWidget_ptr(self as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QWidget_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_view::UndoView> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_view::UndoView> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_view::UndoView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_view::UndoView> for ::list_view::ListView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_view::UndoView {
    let ffi_result =
      ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QListView(self as *mut ::list_view::ListView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QListView(self as *const ::list_view::ListView as *mut ::list_view::ListView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_view::UndoView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_view::UndoView {
    let ffi_result =
      ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_view::UndoView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_view::UndoView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_view::UndoView {
    let ffi_result = ::ffi::qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::undo_view::UndoView {
  type Target = ::list_view::ListView;
  fn deref(&self) -> &::list_view::ListView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QListView_ptr(self as *const ::undo_view::UndoView as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::undo_view::UndoView {
  fn deref_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoView_G_static_cast_QListView_ptr(self as *mut ::undo_view::UndoView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [UndoView::new_unsafe](../struct.UndoView.html#method.new_unsafe) method.
  pub trait UndoViewNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_view::UndoView>;
  }
  impl UndoViewNewUnsafeArgs for *mut ::undo_group::UndoGroup {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_view::UndoView> {
      let group = self;
      let ffi_result = ::ffi::qt_widgets_c_QUndoView_new_group(group);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl UndoViewNewUnsafeArgs for (*mut ::undo_group::UndoGroup, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_view::UndoView> {
      let group = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QUndoView_new_group_parent(group, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl UndoViewNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_view::UndoView> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QUndoView_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl UndoViewNewUnsafeArgs for *mut ::undo_stack::UndoStack {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_view::UndoView> {
      let stack = self;
      let ffi_result = ::ffi::qt_widgets_c_QUndoView_new_stack(stack);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl UndoViewNewUnsafeArgs for (*mut ::undo_stack::UndoStack, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_view::UndoView> {
      let stack = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QUndoView_new_stack_parent(stack, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
