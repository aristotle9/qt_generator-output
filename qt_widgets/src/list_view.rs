/// C++ type: <span style='color: green;'>```QListView::Flow```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Flow {
  /// C++ enum variant: <span style='color: green;'>```LeftToRight = 0```</span>
  LeftToRight = 0,
  /// C++ enum variant: <span style='color: green;'>```TopToBottom = 1```</span>
  TopToBottom = 1,
}

/// C++ type: <span style='color: green;'>```QListView::LayoutMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LayoutMode {
  /// C++ enum variant: <span style='color: green;'>```SinglePass = 0```</span>
  SinglePass = 0,
  /// C++ enum variant: <span style='color: green;'>```Batched = 1```</span>
  Batched = 1,
}

/// C++ type: <span style='color: green;'>```QListView```</span>
#[repr(C)]
pub struct ListView(u8);

impl ListView {
  /// C++ method: <span style='color: green;'>```int QListView::batchSize() const```</span>
  ///
  ///
  pub fn batch_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QListView_batchSize(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::clearPropertyFlags()```</span>
  ///
  ///
  pub fn clear_property_flags(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QListView_clearPropertyFlags(self as *mut ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QListView::doItemsLayout()```</span>
  ///
  ///
  pub fn do_items_layout(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QListView_doItemsLayout(self as *mut ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```QListView::Flow QListView::flow() const```</span>
  ///
  ///
  pub fn flow(&self) -> ::list_view::Flow {
    unsafe { ::ffi::qt_widgets_c_QListView_flow(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```QSize QListView::gridSize() const```</span>
  ///
  ///
  pub fn grid_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListView_gridSize_to_output(self as *const ::list_view::ListView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QListView::indexAt(const QPoint& p) const```</span>
  ///
  ///
  pub fn index_at(&self, p: &::qt_core::point::Point) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListView_indexAt_to_output(self as *const ::list_view::ListView,
                                                        p as *const ::qt_core::point::Point,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QListView::isRowHidden(int row) const```</span>
  ///
  ///
  pub fn is_row_hidden(&self, row: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListView_isRowHidden(self as *const ::list_view::ListView, row) }
  }

  /// C++ method: <span style='color: green;'>```bool QListView::isSelectionRectVisible() const```</span>
  ///
  ///
  pub fn is_selection_rect_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListView_isSelectionRectVisible(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```bool QListView::isWrapping() const```</span>
  ///
  ///
  pub fn is_wrapping(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListView_isWrapping(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```QListView::LayoutMode QListView::layoutMode() const```</span>
  ///
  ///
  pub fn layout_mode(&self) -> ::list_view::LayoutMode {
    unsafe { ::ffi::qt_widgets_c_QListView_layoutMode(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QListView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QListView_metaObject(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```int QListView::modelColumn() const```</span>
  ///
  ///
  pub fn model_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QListView_modelColumn(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```QListView::Movement QListView::movement() const```</span>
  ///
  ///
  pub fn movement(&self) -> ::list_view::Movement {
    unsafe { ::ffi::qt_widgets_c_QListView_movement(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QListView::QListView()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::list_view::ListView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QListView::QListView(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::list_view::ListView> {
    let ffi_result = ::ffi::qt_widgets_c_QListView_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QListView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QListView_qt_metacall(self as *mut ::list_view::ListView,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QListView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QListView_qt_metacast(self as *mut ::list_view::ListView, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual void QListView::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QListView_reset(self as *mut ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```QListView::ResizeMode QListView::resizeMode() const```</span>
  ///
  ///
  pub fn resize_mode(&self) -> ::list_view::ResizeMode {
    unsafe { ::ffi::qt_widgets_c_QListView_resizeMode(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```QListView::scrollTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, &::qt_core::model_index::ModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QListView::scrollTo(const QModelIndex& index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, (&::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QListView::scrollTo(const QModelIndex& index, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub fn scroll_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListViewScrollToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QListView::setBatchSize(int batchSize)```</span>
  ///
  ///
  pub fn set_batch_size(&mut self, batch_size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QListView_setBatchSize(self as *mut ::list_view::ListView, batch_size) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setFlow(QListView::Flow flow)```</span>
  ///
  ///
  pub fn set_flow(&mut self, flow: ::list_view::Flow) {
    unsafe { ::ffi::qt_widgets_c_QListView_setFlow(self as *mut ::list_view::ListView, flow) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setGridSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_grid_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QListView_setGridSize(self as *mut ::list_view::ListView,
                                                size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setLayoutMode(QListView::LayoutMode mode)```</span>
  ///
  ///
  pub fn set_layout_mode(&mut self, mode: ::list_view::LayoutMode) {
    unsafe { ::ffi::qt_widgets_c_QListView_setLayoutMode(self as *mut ::list_view::ListView, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setModelColumn(int column)```</span>
  ///
  ///
  pub fn set_model_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QListView_setModelColumn(self as *mut ::list_view::ListView, column) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setMovement(QListView::Movement movement)```</span>
  ///
  ///
  pub fn set_movement(&mut self, movement: ::list_view::Movement) {
    unsafe { ::ffi::qt_widgets_c_QListView_setMovement(self as *mut ::list_view::ListView, movement) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setResizeMode(QListView::ResizeMode mode)```</span>
  ///
  ///
  pub fn set_resize_mode(&mut self, mode: ::list_view::ResizeMode) {
    unsafe { ::ffi::qt_widgets_c_QListView_setResizeMode(self as *mut ::list_view::ListView, mode) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QListView::setRootIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_root_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QListView_setRootIndex(self as *mut ::list_view::ListView,
                                                 index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setRowHidden(int row, bool hide)```</span>
  ///
  ///
  pub fn set_row_hidden(&mut self, row: ::libc::c_int, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QListView_setRowHidden(self as *mut ::list_view::ListView, row, hide) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setSelectionRectVisible(bool show)```</span>
  ///
  ///
  pub fn set_selection_rect_visible(&mut self, show: bool) {
    unsafe { ::ffi::qt_widgets_c_QListView_setSelectionRectVisible(self as *mut ::list_view::ListView, show) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setSpacing(int space)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, space: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QListView_setSpacing(self as *mut ::list_view::ListView, space) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setUniformItemSizes(bool enable)```</span>
  ///
  ///
  pub fn set_uniform_item_sizes(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QListView_setUniformItemSizes(self as *mut ::list_view::ListView, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setViewMode(QListView::ViewMode mode)```</span>
  ///
  ///
  pub fn set_view_mode(&mut self, mode: ::list_view::ViewMode) {
    unsafe { ::ffi::qt_widgets_c_QListView_setViewMode(self as *mut ::list_view::ListView, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setWordWrap(bool on)```</span>
  ///
  ///
  pub fn set_word_wrap(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QListView_setWordWrap(self as *mut ::list_view::ListView, on) }
  }

  /// C++ method: <span style='color: green;'>```void QListView::setWrapping(bool enable)```</span>
  ///
  ///
  pub fn set_wrapping(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QListView_setWrapping(self as *mut ::list_view::ListView, enable) }
  }

  /// C++ method: <span style='color: green;'>```int QListView::spacing() const```</span>
  ///
  ///
  pub fn spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QListView_spacing(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```static QString QListView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QListView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QListView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QListView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QListView::uniformItemSizes() const```</span>
  ///
  ///
  pub fn uniform_item_sizes(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListView_uniformItemSizes(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```QListView::ViewMode QListView::viewMode() const```</span>
  ///
  ///
  pub fn view_mode(&self) -> ::list_view::ViewMode {
    unsafe { ::ffi::qt_widgets_c_QListView_viewMode(self as *const ::list_view::ListView) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QListView::visualRect(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn visual_rect(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListView_visualRect_to_output(self as *const ::list_view::ListView,
                                                           index as *const ::qt_core::model_index::ModelIndex,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QListView::wordWrap() const```</span>
  ///
  ///
  pub fn word_wrap(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListView_wordWrap(self as *const ::list_view::ListView) }
  }
}

impl ::cpp_utils::CppDeletable for ::list_view::ListView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QListView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ListView`.
  pub struct Signals<'a>(&'a ::list_view::ListView);
  /// Represents a built-in Qt signal `QListView::entered`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct Entered<'a>(&'a ::list_view::ListView);
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
  /// Represents a built-in Qt signal `QListView::pressed`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct Pressed<'a>(&'a ::list_view::ListView);
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
  /// Represents a built-in Qt signal `QListView::viewportEntered`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().viewport_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct ViewportEntered<'a>(&'a ::list_view::ListView);
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
  /// Represents a built-in Qt signal `QListView::doubleClicked`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct DoubleClicked<'a>(&'a ::list_view::ListView);
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
  /// Represents a built-in Qt signal `QListView::indexesMoved`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().indexes_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct IndexesMoved<'a>(&'a ::list_view::ListView);
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
  /// Represents a built-in Qt signal `QListView::clicked`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct Clicked<'a>(&'a ::list_view::ListView);
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
  /// Represents a built-in Qt signal `QListView::activated`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct Activated<'a>(&'a ::list_view::ListView);
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
  /// Represents a built-in Qt signal `QListView::iconSizeChanged`.
  ///
  /// An object of this type can be created from `ListView` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct IconSizeChanged<'a>(&'a ::list_view::ListView);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QListView::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListView::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListView::viewportEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn viewport_entered(&self) -> ViewportEntered {
      ViewportEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListView::doubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_clicked(&self) -> DoubleClicked {
      DoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListView::indexesMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn indexes_moved(&self) -> IndexesMoved {
      IndexesMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListView::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListView::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListView::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ListView`.
  pub struct Slots<'a>(&'a ::list_view::ListView);
  /// Represents a built-in Qt slot `QListView::updateEditorGeometries`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().update_editor_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct UpdateEditorGeometries<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::setCurrentIndex`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct SetCurrentIndex<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::scrollToBottom`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().scroll_to_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct ScrollToBottom<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToBottom<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToBottom()\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::updateEditorData`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().update_editor_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct UpdateEditorData<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorData()\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::verticalScrollbarAction`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().vertical_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct VerticalScrollbarAction<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::clearSelection`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct ClearSelection<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::selectAll`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct SelectAll<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::verticalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().vertical_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct VerticalScrollbarValueChanged<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::edit`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().edit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct Edit<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for Edit<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1edit(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::scrollToTop`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().scroll_to_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct ScrollToTop<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToTop<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToTop()\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::horizontalScrollbarAction`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().horizontal_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct HorizontalScrollbarAction<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::closeEditor`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct CloseEditor<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for CloseEditor<'a> {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeEditor(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::horizontalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().horizontal_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct HorizontalScrollbarValueChanged<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::update`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct Update<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::commitData`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct CommitData<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for CommitData<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1commitData(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QListView::editorDestroyed`.
  ///
  /// An object of this type can be created from `ListView` with `object.slots().editor_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListView` object.
  pub struct EditorDestroyed<'a>(&'a ::list_view::ListView);
  impl<'a> ::qt_core::connection::Receiver for EditorDestroyed<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1editorDestroyed(QObject*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QListView::updateEditorGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_geometries(&self) -> UpdateEditorGeometries {
      UpdateEditorGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::scrollToBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_bottom(&self) -> ScrollToBottom {
      ScrollToBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::updateEditorData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_data(&self) -> UpdateEditorData {
      UpdateEditorData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::verticalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_action(&self) -> VerticalScrollbarAction {
      VerticalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::verticalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_value_changed(&self) -> VerticalScrollbarValueChanged {
      VerticalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::edit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit(&self) -> Edit {
      Edit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::scrollToTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_top(&self) -> ScrollToTop {
      ScrollToTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::horizontalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_action(&self) -> HorizontalScrollbarAction {
      HorizontalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::horizontalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_value_changed(&self) -> HorizontalScrollbarValueChanged {
      HorizontalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListView::editorDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editor_destroyed(&self) -> EditorDestroyed {
      EditorDestroyed(self.0)
    }
  }
  impl ::list_view::ListView {
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

/// C++ type: <span style='color: green;'>```QListView::Movement```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Movement {
  /// C++ enum variant: <span style='color: green;'>```Static = 0```</span>
  Static = 0,
  /// C++ enum variant: <span style='color: green;'>```Free = 1```</span>
  Free = 1,
  /// C++ enum variant: <span style='color: green;'>```Snap = 2```</span>
  Snap = 2,
}

/// C++ type: <span style='color: green;'>```QListView::ResizeMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ResizeMode {
  /// C++ enum variant: <span style='color: green;'>```Fixed = 0```</span>
  Fixed = 0,
  /// C++ enum variant: <span style='color: green;'>```Adjust = 1```</span>
  Adjust = 1,
}

/// C++ type: <span style='color: green;'>```QListView::ViewMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ViewMode {
  /// C++ enum variant: <span style='color: green;'>```ListMode = 0```</span>
  List = 0,
  /// C++ enum variant: <span style='color: green;'>```IconMode = 1```</span>
  Icon = 1,
}

impl ::cpp_utils::DynamicCast<::list_view::ListView> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_view::ListView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_view::ListView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::list_view::ListView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_view::ListView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_view::ListView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::list_view::ListView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_view::ListView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_view::ListView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::list_view::ListView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_view::ListView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_view::ListView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::list_view::ListView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QObject_ptr(self as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QObject_ptr(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::list_view::ListView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QPaintDevice_ptr(self as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QPaintDevice_ptr(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::list_view::ListView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QAbstractItemView_ptr(self as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QAbstractItemView_ptr(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::list_view::ListView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QListView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::list_view::ListView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::list_view::ListView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QFrame_ptr(self as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QFrame_ptr(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::list_view::ListView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QWidget_ptr(self as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QWidget_ptr(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_view::ListView> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_view::ListView> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_view::ListView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_view::ListView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result =
      ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_view::ListView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_view::ListView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = ::ffi::qt_widgets_c_QListView_G_static_cast_QListView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::list_view::ListView {
  type Target = ::abstract_item_view::AbstractItemView;
  fn deref(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QAbstractItemView_ptr(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::list_view::ListView {
  fn deref_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListView_G_static_cast_QAbstractItemView_ptr(self as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ListView::scroll_to](../struct.ListView.html#method.scroll_to) method.
  pub trait ListViewScrollToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list_view::ListView) -> ();
  }
  impl<'largs> ListViewScrollToArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::list_view::ListView) -> () {
      let index = self;
      unsafe {
        ::ffi::qt_widgets_c_QListView_scrollTo_index(original_self as *mut ::list_view::ListView,
                                                     index as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  impl<'largs> ListViewScrollToArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint) {
    fn exec(self, original_self: &'largs mut ::list_view::ListView) -> () {
      let index = self.0;
      let hint = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QListView_scrollTo_index_hint(original_self as *mut ::list_view::ListView,
                                                          index as *const ::qt_core::model_index::ModelIndex,
                                                          hint)
      }
    }
  }
}
