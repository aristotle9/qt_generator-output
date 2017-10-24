/// C++ type: <span style='color: green;'>```QItemDelegate```</span>
#[repr(C)]
pub struct ItemDelegate(u8);

impl ItemDelegate {
  /// C++ method: <span style='color: green;'>```virtual QWidget* QItemDelegate::createEditor(QWidget* parent, const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn create_editor(&self,
                              parent: *mut ::widget::Widget,
                              option: &::style_option_view_item::StyleOptionViewItem,
                              index: &::qt_core::model_index::ModelIndex)
                              -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QItemDelegate_createEditor(self as *const ::item_delegate::ItemDelegate,
                                                   parent,
                                                   option as *const ::style_option_view_item::StyleOptionViewItem,
                                                   index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```bool QItemDelegate::hasClipping() const```</span>
  ///
  ///
  pub fn has_clipping(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QItemDelegate_hasClipping(self as *const ::item_delegate::ItemDelegate) }
  }

  /// C++ method: <span style='color: green;'>```QItemEditorFactory* QItemDelegate::itemEditorFactory() const```</span>
  ///
  ///
  pub fn item_editor_factory(&self) -> *mut ::item_editor_factory::ItemEditorFactory {
    unsafe { ::ffi::qt_widgets_c_QItemDelegate_itemEditorFactory(self as *const ::item_delegate::ItemDelegate) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QItemDelegate::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QItemDelegate_metaObject(self as *const ::item_delegate::ItemDelegate) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QItemDelegate::QItemDelegate()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::item_delegate::ItemDelegate> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QItemDelegate::QItemDelegate(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::item_delegate::ItemDelegate> {
    let ffi_result = ::ffi::qt_widgets_c_QItemDelegate_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual void QItemDelegate::paint(QPainter* painter, const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn paint(&self,
                      painter: *mut ::qt_gui::painter::Painter,
                      option: &::style_option_view_item::StyleOptionViewItem,
                      index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QItemDelegate_paint(self as *const ::item_delegate::ItemDelegate,
                                            painter,
                                            option as *const ::style_option_view_item::StyleOptionViewItem,
                                            index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual int QItemDelegate::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QItemDelegate_qt_metacall(self as *mut ::item_delegate::ItemDelegate,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QItemDelegate::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QItemDelegate_qt_metacast(self as *mut ::item_delegate::ItemDelegate, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QItemDelegate::setClipping(bool clip)```</span>
  ///
  ///
  pub fn set_clipping(&mut self, clip: bool) {
    unsafe { ::ffi::qt_widgets_c_QItemDelegate_setClipping(self as *mut ::item_delegate::ItemDelegate, clip) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QItemDelegate::setEditorData(QWidget* editor, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn set_editor_data(&self, editor: *mut ::widget::Widget, index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QItemDelegate_setEditorData(self as *const ::item_delegate::ItemDelegate,
                                                    editor,
                                                    index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```void QItemDelegate::setItemEditorFactory(QItemEditorFactory* factory)```</span>
  ///
  ///
  pub unsafe fn set_item_editor_factory(&mut self, factory: *mut ::item_editor_factory::ItemEditorFactory) {
    ::ffi::qt_widgets_c_QItemDelegate_setItemEditorFactory(self as *mut ::item_delegate::ItemDelegate, factory)
  }

  /// C++ method: <span style='color: green;'>```virtual void QItemDelegate::setModelData(QWidget* editor, QAbstractItemModel* model, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn set_model_data(&self,
                               editor: *mut ::widget::Widget,
                               model: *mut ::qt_core::abstract_item_model::AbstractItemModel,
                               index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QItemDelegate_setModelData(self as *const ::item_delegate::ItemDelegate,
                                                   editor,
                                                   model,
                                                   index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QItemDelegate::sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
  ///
  ///
  pub fn size_hint(&self,
                   option: &::style_option_view_item::StyleOptionViewItem,
                   index: &::qt_core::model_index::ModelIndex)
                   -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QItemDelegate_sizeHint_to_output(self as *const ::item_delegate::ItemDelegate, option as *const ::style_option_view_item::StyleOptionViewItem, index as *const ::qt_core::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QItemDelegate::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QItemDelegate_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QItemDelegate::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QItemDelegate_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QItemDelegate::updateEditorGeometry(QWidget* editor, const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn update_editor_geometry(&self,
                                       editor: *mut ::widget::Widget,
                                       option: &::style_option_view_item::StyleOptionViewItem,
                                       index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QItemDelegate_updateEditorGeometry(self as *const ::item_delegate::ItemDelegate, editor, option as *const ::style_option_view_item::StyleOptionViewItem, index as *const ::qt_core::model_index::ModelIndex)
  }
}

impl ::cpp_utils::CppDeletable for ::item_delegate::ItemDelegate {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QItemDelegate_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ItemDelegate`.
  pub struct Signals<'a>(&'a ::item_delegate::ItemDelegate);
  /// Represents a built-in Qt signal `QItemDelegate::commitData`.
  ///
  /// An object of this type can be created from `ItemDelegate` with `object.signals().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemDelegate` object.
  pub struct CommitData<'a>(&'a ::item_delegate::ItemDelegate);
  impl<'a> ::qt_core::connection::Receiver for CommitData<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2commitData(QWidget*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CommitData<'a> {}
  /// Represents a built-in Qt signal `QItemDelegate::closeEditor`.
  ///
  /// An object of this type can be created from `ItemDelegate` with `object.signals().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemDelegate` object.
  pub struct CloseEditor<'a>(&'a ::item_delegate::ItemDelegate);
  impl<'a> ::qt_core::connection::Receiver for CloseEditor<'a> {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2closeEditor(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CloseEditor<'a> {}
  /// Represents a built-in Qt signal `QItemDelegate::sizeHintChanged`.
  ///
  /// An object of this type can be created from `ItemDelegate` with `object.signals().size_hint_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemDelegate` object.
  pub struct SizeHintChanged<'a>(&'a ::item_delegate::ItemDelegate);
  impl<'a> ::qt_core::connection::Receiver for SizeHintChanged<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sizeHintChanged(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SizeHintChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QItemDelegate::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QItemDelegate::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QItemDelegate::sizeHintChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn size_hint_changed(&self) -> SizeHintChanged {
      SizeHintChanged(self.0)
    }
  }
  impl ::item_delegate::ItemDelegate {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::item_delegate::ItemDelegate> for ::abstract_item_delegate::AbstractItemDelegate {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::item_delegate::ItemDelegate> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_G_dynamic_cast_QItemDelegate_ptr(self as *mut ::abstract_item_delegate::AbstractItemDelegate) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::item_delegate::ItemDelegate> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_G_dynamic_cast_QItemDelegate_ptr(self as *const ::abstract_item_delegate::AbstractItemDelegate as *mut ::abstract_item_delegate::AbstractItemDelegate) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::item_delegate::ItemDelegate {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QObject_ptr(self as *mut ::item_delegate::ItemDelegate)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QObject_ptr(self as *const ::item_delegate::ItemDelegate as *mut ::item_delegate::ItemDelegate) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_delegate::AbstractItemDelegate> for ::item_delegate::ItemDelegate {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_delegate::AbstractItemDelegate {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(self as *mut ::item_delegate::ItemDelegate) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_delegate::AbstractItemDelegate {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(self as *const ::item_delegate::ItemDelegate as *mut ::item_delegate::ItemDelegate) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::item_delegate::ItemDelegate> for ::abstract_item_delegate::AbstractItemDelegate {
  unsafe fn static_cast_mut(&mut self) -> &mut ::item_delegate::ItemDelegate {
    let ffi_result = ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QAbstractItemDelegate(self as *mut ::abstract_item_delegate::AbstractItemDelegate);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::item_delegate::ItemDelegate {
    let ffi_result = ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QAbstractItemDelegate(self as *const ::abstract_item_delegate::AbstractItemDelegate as *mut ::abstract_item_delegate::AbstractItemDelegate);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::item_delegate::ItemDelegate> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::item_delegate::ItemDelegate {
    let ffi_result = ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::item_delegate::ItemDelegate {
    let ffi_result = ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::item_delegate::ItemDelegate {
  type Target = ::abstract_item_delegate::AbstractItemDelegate;
  fn deref(&self) -> &::abstract_item_delegate::AbstractItemDelegate {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(self as *const ::item_delegate::ItemDelegate as *mut ::item_delegate::ItemDelegate) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::item_delegate::ItemDelegate {
  fn deref_mut(&mut self) -> &mut ::abstract_item_delegate::AbstractItemDelegate {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(self as *mut ::item_delegate::ItemDelegate) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
