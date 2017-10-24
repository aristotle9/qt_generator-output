/// C++ type: <span style='color: green;'>```QColumnView```</span>
#[repr(C)]
pub struct ColumnView(u8);

impl ColumnView {
  /// C++ method: <span style='color: green;'>```QList<int> QColumnView::columnWidths() const```</span>
  ///
  ///
  pub fn column_widths(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_columnWidths_to_output(self as *const ::column_view::ColumnView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QColumnView::indexAt(const QPoint& point) const```</span>
  ///
  ///
  pub fn index_at(&self, point: &::qt_core::point::Point) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_indexAt_to_output(self as *const ::column_view::ColumnView,
                                                          point as *const ::qt_core::point::Point,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QColumnView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QColumnView_metaObject(self as *const ::column_view::ColumnView) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QColumnView::QColumnView()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::column_view::ColumnView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QColumnView::QColumnView(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::column_view::ColumnView> {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QColumnView::previewWidget() const```</span>
  ///
  ///
  pub fn preview_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QColumnView_previewWidget(self as *const ::column_view::ColumnView) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QColumnView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QColumnView_qt_metacall(self as *mut ::column_view::ColumnView,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QColumnView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QColumnView_qt_metacast(self as *mut ::column_view::ColumnView, arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QColumnView::resizeGripsVisible() const```</span>
  ///
  ///
  pub fn resize_grips_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QColumnView_resizeGripsVisible(self as *const ::column_view::ColumnView) }
  }

  /// C++ method: <span style='color: green;'>```QColumnView::scrollTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, &::qt_core::model_index::ModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QColumnView::scrollTo(const QModelIndex& index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, (&::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QColumnView::scrollTo(const QModelIndex& index, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub fn scroll_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColumnViewScrollToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QColumnView::selectAll()```</span>
  ///
  ///
  pub fn select_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QColumnView_selectAll(self as *mut ::column_view::ColumnView) }
  }

  /// C++ method: <span style='color: green;'>```void QColumnView::setColumnWidths(const QList<int>& list)```</span>
  ///
  ///
  pub fn set_column_widths(&mut self, list: &::qt_core::list::ListCInt) {
    unsafe {
      ::ffi::qt_widgets_c_QColumnView_setColumnWidths(self as *mut ::column_view::ColumnView,
                                                      list as *const ::qt_core::list::ListCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QColumnView::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QColumnView_setModel(self as *mut ::column_view::ColumnView, model)
  }

  /// C++ method: <span style='color: green;'>```void QColumnView::setPreviewWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_preview_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QColumnView_setPreviewWidget(self as *mut ::column_view::ColumnView, widget)
  }

  /// C++ method: <span style='color: green;'>```void QColumnView::setResizeGripsVisible(bool visible)```</span>
  ///
  ///
  pub fn set_resize_grips_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QColumnView_setResizeGripsVisible(self as *mut ::column_view::ColumnView, visible) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QColumnView::setRootIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_root_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QColumnView_setRootIndex(self as *mut ::column_view::ColumnView,
                                                   index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QColumnView::setSelectionModel(QItemSelectionModel* selectionModel)```</span>
  ///
  ///
  pub unsafe fn set_selection_model(&mut self,
                                    selection_model: *mut ::qt_core::item_selection_model::ItemSelectionModel) {
    ::ffi::qt_widgets_c_QColumnView_setSelectionModel(self as *mut ::column_view::ColumnView, selection_model)
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QColumnView::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_sizeHint_to_output(self as *const ::column_view::ColumnView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QColumnView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QColumnView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QColumnView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QColumnView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QColumnView::visualRect(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn visual_rect(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_visualRect_to_output(self as *const ::column_view::ColumnView,
                                                             index as *const ::qt_core::model_index::ModelIndex,
                                                             &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::column_view::ColumnView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QColumnView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ColumnView`.
  pub struct Signals<'a>(&'a ::column_view::ColumnView);
  /// Represents a built-in Qt signal `QColumnView::iconSizeChanged`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct IconSizeChanged<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for IconSizeChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2iconSizeChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IconSizeChanged<'a> {}
  /// Represents a built-in Qt signal `QColumnView::doubleClicked`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct DoubleClicked<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for DoubleClicked<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2doubleClicked(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QColumnView::pressed`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct Pressed<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for Pressed<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressed(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Pressed<'a> {}
  /// Represents a built-in Qt signal `QColumnView::viewportEntered`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().viewport_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct ViewportEntered<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for ViewportEntered<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2viewportEntered()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ViewportEntered<'a> {}
  /// Represents a built-in Qt signal `QColumnView::activated`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct Activated<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for Activated<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Activated<'a> {}
  /// Represents a built-in Qt signal `QColumnView::updatePreviewWidget`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().update_preview_widget()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct UpdatePreviewWidget<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for UpdatePreviewWidget<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2updatePreviewWidget(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UpdatePreviewWidget<'a> {}
  /// Represents a built-in Qt signal `QColumnView::entered`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct Entered<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for Entered<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2entered(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Entered<'a> {}
  /// Represents a built-in Qt signal `QColumnView::clicked`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct Clicked<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for Clicked<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clicked(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Clicked<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QColumnView::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColumnView::doubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_clicked(&self) -> DoubleClicked {
      DoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColumnView::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColumnView::viewportEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn viewport_entered(&self) -> ViewportEntered {
      ViewportEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColumnView::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColumnView::updatePreviewWidget`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_preview_widget(&self) -> UpdatePreviewWidget {
      UpdatePreviewWidget(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColumnView::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColumnView::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ColumnView`.
  pub struct Slots<'a>(&'a ::column_view::ColumnView);
  /// Represents a built-in Qt slot `QColumnView::updateGeometries`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().update_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct UpdateGeometries<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for UpdateGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::editorDestroyed`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().editor_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct EditorDestroyed<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for EditorDestroyed<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1editorDestroyed(QObject*)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for RowsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::clearSelection`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct ClearSelection<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::commitData`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct CommitData<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for CommitData<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1commitData(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::reset`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct Reset<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::setCurrentIndex`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct SetCurrentIndex<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::dataChanged`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct DataChanged<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for DataChanged<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,
     &'static ::qt_core::model_index::ModelIndex,
     &'static ::qt_core::vector::VectorCInt);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1dataChanged(const QModelIndex&,const QModelIndex&,const QVector< int >&)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::closeEditor`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct CloseEditor<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for CloseEditor<'a> {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeEditor(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::scrollToBottom`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().scroll_to_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct ScrollToBottom<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToBottom<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToBottom()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::doItemsLayout`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().do_items_layout()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct DoItemsLayout<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for DoItemsLayout<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1doItemsLayout()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::update`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct Update<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::verticalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().vertical_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct VerticalScrollbarValueChanged<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::selectionChanged`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct SelectionChanged<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for SelectionChanged<'a> {
    type Arguments = (&'static ::qt_core::item_selection::ItemSelection,
     &'static ::qt_core::item_selection::ItemSelection);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectionChanged(const QItemSelection&,const QItemSelection&)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::scrollToTop`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().scroll_to_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct ScrollToTop<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToTop<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToTop()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::horizontalScrollbarAction`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().horizontal_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct HorizontalScrollbarAction<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::horizontalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().horizontal_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct HorizontalScrollbarValueChanged<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::updateEditorGeometries`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().update_editor_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct UpdateEditorGeometries<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::updateEditorData`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().update_editor_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct UpdateEditorData<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorData()\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::edit`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().edit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct Edit<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for Edit<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1edit(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QColumnView::verticalScrollbarAction`.
  ///
  /// An object of this type can be created from `ColumnView` with `object.slots().vertical_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColumnView` object.
  pub struct VerticalScrollbarAction<'a>(&'a ::column_view::ColumnView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarAction(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QColumnView::updateGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_geometries(&self) -> UpdateGeometries {
      UpdateGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::editorDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editor_destroyed(&self) -> EditorDestroyed {
      EditorDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::scrollToBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_bottom(&self) -> ScrollToBottom {
      ScrollToBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::doItemsLayout`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn do_items_layout(&self) -> DoItemsLayout {
      DoItemsLayout(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::verticalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_value_changed(&self) -> VerticalScrollbarValueChanged {
      VerticalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::scrollToTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_top(&self) -> ScrollToTop {
      ScrollToTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::horizontalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_action(&self) -> HorizontalScrollbarAction {
      HorizontalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::horizontalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_value_changed(&self) -> HorizontalScrollbarValueChanged {
      HorizontalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::updateEditorGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_geometries(&self) -> UpdateEditorGeometries {
      UpdateEditorGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::updateEditorData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_data(&self) -> UpdateEditorData {
      UpdateEditorData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::edit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit(&self) -> Edit {
      Edit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColumnView::verticalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_action(&self) -> VerticalScrollbarAction {
      VerticalScrollbarAction(self.0)
    }
  }
  impl ::column_view::ColumnView {
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

impl ::cpp_utils::DynamicCast<::column_view::ColumnView> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::column_view::ColumnView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::column_view::ColumnView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::column_view::ColumnView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::column_view::ColumnView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::column_view::ColumnView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::column_view::ColumnView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::column_view::ColumnView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::column_view::ColumnView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::column_view::ColumnView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::column_view::ColumnView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::column_view::ColumnView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::column_view::ColumnView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QObject_ptr(self as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QObject_ptr(self as *const ::column_view::ColumnView as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::column_view::ColumnView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QPaintDevice_ptr(self as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QPaintDevice_ptr(self as *const ::column_view::ColumnView as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::column_view::ColumnView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_G_static_cast_QAbstractItemView_ptr(self as *mut ::column_view::ColumnView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QAbstractItemView_ptr(self as *const ::column_view::ColumnView as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::column_view::ColumnView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::column_view::ColumnView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::column_view::ColumnView as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::column_view::ColumnView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QFrame_ptr(self as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QFrame_ptr(self as *const ::column_view::ColumnView as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::column_view::ColumnView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QWidget_ptr(self as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QWidget_ptr(self as *const ::column_view::ColumnView as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::column_view::ColumnView> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::column_view::ColumnView> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::column_view::ColumnView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::column_view::ColumnView {
    let ffi_result =
      ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::column_view::ColumnView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::column_view::ColumnView {
    let ffi_result =
      ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::column_view::ColumnView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::column_view::ColumnView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::column_view::ColumnView {
    let ffi_result =
      ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::column_view::ColumnView {
    let ffi_result = ::ffi::qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::column_view::ColumnView {
  type Target = ::abstract_item_view::AbstractItemView;
  fn deref(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColumnView_G_static_cast_QAbstractItemView_ptr(self as *const ::column_view::ColumnView as *mut ::column_view::ColumnView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::column_view::ColumnView {
  fn deref_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_G_static_cast_QAbstractItemView_ptr(self as *mut ::column_view::ColumnView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ColumnView::scroll_to](../struct.ColumnView.html#method.scroll_to) method.
  pub trait ColumnViewScrollToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::column_view::ColumnView) -> ();
  }
  impl<'largs> ColumnViewScrollToArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::column_view::ColumnView) -> () {
      let index = self;
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_scrollTo_index(original_self as *mut ::column_view::ColumnView,
                                                       index as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  impl<'largs> ColumnViewScrollToArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint) {
    fn exec(self, original_self: &'largs mut ::column_view::ColumnView) -> () {
      let index = self.0;
      let hint = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QColumnView_scrollTo_index_hint(original_self as *mut ::column_view::ColumnView,
                                                            index as *const ::qt_core::model_index::ModelIndex,
                                                            hint)
      }
    }
  }
}
