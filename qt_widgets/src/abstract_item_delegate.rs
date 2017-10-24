/// C++ type: <span style='color: green;'>```QAbstractItemDelegate```</span>
#[repr(C)]
pub struct AbstractItemDelegate(u8);

impl AbstractItemDelegate {
  /// C++ method: <span style='color: green;'>```virtual QWidget* QAbstractItemDelegate::createEditor(QWidget* parent, const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn create_editor(&self,
                              parent: *mut ::widget::Widget,
                              option: &::style_option_view_item::StyleOptionViewItem,
                              index: &::qt_core::model_index::ModelIndex)
                              -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_createEditor(self as *const ::abstract_item_delegate::AbstractItemDelegate, parent, option as *const ::style_option_view_item::StyleOptionViewItem, index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemDelegate::destroyEditor(QWidget* editor, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn destroy_editor(&self, editor: *mut ::widget::Widget, index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_destroyEditor(self as *const ::abstract_item_delegate::AbstractItemDelegate, editor, index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemDelegate::editorEvent(QEvent* event, QAbstractItemModel* model, const QStyleOptionViewItem& option, const QModelIndex& index)```</span>
  ///
  ///
  pub unsafe fn editor_event(&mut self,
                             event: *mut ::qt_core::event::Event,
                             model: *mut ::qt_core::abstract_item_model::AbstractItemModel,
                             option: &::style_option_view_item::StyleOptionViewItem,
                             index: &::qt_core::model_index::ModelIndex)
                             -> bool {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_editorEvent(self as *mut ::abstract_item_delegate::AbstractItemDelegate, event, model, option as *const ::style_option_view_item::StyleOptionViewItem, index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractItemDelegate::elidedText(const QFontMetrics& fontMetrics, int width, Qt::TextElideMode mode, const QString& text)```</span>
  ///
  ///
  pub fn elided_text(font_metrics: &::qt_gui::font_metrics::FontMetrics,
                     width: ::libc::c_int,
                     mode: &::qt_core::qt::TextElideMode,
                     text: &::qt_core::string::String)
                     -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemDelegate_elidedText_to_output(font_metrics as *const ::qt_gui::font_metrics::FontMetrics, width, mode as *const ::qt_core::qt::TextElideMode, text as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemDelegate::helpEvent(QHelpEvent* event, QAbstractItemView* view, const QStyleOptionViewItem& option, const QModelIndex& index)```</span>
  ///
  ///
  pub unsafe fn help_event(&mut self,
                           event: *mut ::qt_gui::help_event::HelpEvent,
                           view: *mut ::abstract_item_view::AbstractItemView,
                           option: &::style_option_view_item::StyleOptionViewItem,
                           index: &::qt_core::model_index::ModelIndex)
                           -> bool {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_helpEvent(self as *mut ::abstract_item_delegate::AbstractItemDelegate, event, view, option as *const ::style_option_view_item::StyleOptionViewItem, index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractItemDelegate::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemDelegate_metaObject(self as *const ::abstract_item_delegate::AbstractItemDelegate) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractItemDelegate::paint(QPainter* painter, const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn paint(&self,
                      painter: *mut ::qt_gui::painter::Painter,
                      option: &::style_option_view_item::StyleOptionViewItem,
                      index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_paint(self as *const ::abstract_item_delegate::AbstractItemDelegate,
                                                    painter,
                                                    option as *const ::style_option_view_item::StyleOptionViewItem,
                                                    index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual QVector<int> QAbstractItemDelegate::paintingRoles() const```</span>
  ///
  ///
  pub fn painting_roles(&self) -> ::qt_core::vector::VectorCInt {
    {
      let mut object: ::qt_core::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemDelegate_paintingRoles_to_output(self as *const ::abstract_item_delegate::AbstractItemDelegate, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractItemDelegate::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_qt_metacall(self as *mut ::abstract_item_delegate::AbstractItemDelegate,
                                                          arg1 as *const ::qt_core::meta_object::Call,
                                                          arg2,
                                                          arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAbstractItemDelegate::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_qt_metacast(self as *mut ::abstract_item_delegate::AbstractItemDelegate,
                                                          arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemDelegate::setEditorData(QWidget* editor, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn set_editor_data(&self, editor: *mut ::widget::Widget, index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_setEditorData(self as *const ::abstract_item_delegate::AbstractItemDelegate, editor, index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemDelegate::setModelData(QWidget* editor, QAbstractItemModel* model, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn set_model_data(&self,
                               editor: *mut ::widget::Widget,
                               model: *mut ::qt_core::abstract_item_model::AbstractItemModel,
                               index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_setModelData(self as *const ::abstract_item_delegate::AbstractItemDelegate, editor, model, index as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSize QAbstractItemDelegate::sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
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
        ::ffi::qt_widgets_c_QAbstractItemDelegate_sizeHint_to_output(self as *const ::abstract_item_delegate::AbstractItemDelegate, option as *const ::style_option_view_item::StyleOptionViewItem, index as *const ::qt_core::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractItemDelegate::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractItemDelegate_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractItemDelegate::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractItemDelegate_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemDelegate::updateEditorGeometry(QWidget* editor, const QStyleOptionViewItem& option, const QModelIndex& index) const```</span>
  ///
  ///
  pub unsafe fn update_editor_geometry(&self,
                                       editor: *mut ::widget::Widget,
                                       option: &::style_option_view_item::StyleOptionViewItem,
                                       index: &::qt_core::model_index::ModelIndex) {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_updateEditorGeometry(self as *const ::abstract_item_delegate::AbstractItemDelegate, editor, option as *const ::style_option_view_item::StyleOptionViewItem, index as *const ::qt_core::model_index::ModelIndex)
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_item_delegate::AbstractItemDelegate {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAbstractItemDelegate_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractItemDelegate`.
  pub struct Signals<'a>(&'a ::abstract_item_delegate::AbstractItemDelegate);
  /// Represents a built-in Qt signal `QAbstractItemDelegate::sizeHintChanged`.
  ///
  /// An object of this type can be created from `AbstractItemDelegate` with `object.signals().size_hint_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemDelegate` object.
  pub struct SizeHintChanged<'a>(&'a ::abstract_item_delegate::AbstractItemDelegate);
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
  /// Represents a built-in Qt signal `QAbstractItemDelegate::commitData`.
  ///
  /// An object of this type can be created from `AbstractItemDelegate` with `object.signals().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemDelegate` object.
  pub struct CommitData<'a>(&'a ::abstract_item_delegate::AbstractItemDelegate);
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
  /// Represents a built-in Qt signal `QAbstractItemDelegate::closeEditor`.
  ///
  /// An object of this type can be created from `AbstractItemDelegate` with `object.signals().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemDelegate` object.
  pub struct CloseEditor<'a>(&'a ::abstract_item_delegate::AbstractItemDelegate);
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
  /// Represents a built-in Qt signal `QAbstractItemDelegate::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractItemDelegate` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemDelegate` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_item_delegate::AbstractItemDelegate);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractItemDelegate::sizeHintChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn size_hint_changed(&self) -> SizeHintChanged {
      SizeHintChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemDelegate::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemDelegate::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemDelegate::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::abstract_item_delegate::AbstractItemDelegate {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QAbstractItemDelegate::EndEditHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum EndEditHint {
  /// C++ enum variant: <span style='color: green;'>```NoHint = 0```</span>
  NoHint = 0,
  /// C++ enum variant: <span style='color: green;'>```EditNextItem = 1```</span>
  EditNextItem = 1,
  /// C++ enum variant: <span style='color: green;'>```EditPreviousItem = 2```</span>
  EditPreviousItem = 2,
  /// C++ enum variant: <span style='color: green;'>```SubmitModelCache = 3```</span>
  SubmitModelCache = 3,
  /// C++ enum variant: <span style='color: green;'>```RevertModelCache = 4```</span>
  RevertModelCache = 4,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_item_delegate::AbstractItemDelegate {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemDelegate_G_static_cast_QObject_ptr(self as *mut ::abstract_item_delegate::AbstractItemDelegate) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemDelegate_G_static_cast_QObject_ptr(self as *const ::abstract_item_delegate::AbstractItemDelegate as *mut ::abstract_item_delegate::AbstractItemDelegate) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_item_delegate::AbstractItemDelegate> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_item_delegate::AbstractItemDelegate {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_item_delegate::AbstractItemDelegate {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_item_delegate::AbstractItemDelegate {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemDelegate_G_static_cast_QObject_ptr(self as *const ::abstract_item_delegate::AbstractItemDelegate as *mut ::abstract_item_delegate::AbstractItemDelegate) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_item_delegate::AbstractItemDelegate {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemDelegate_G_static_cast_QObject_ptr(self as *mut ::abstract_item_delegate::AbstractItemDelegate) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
