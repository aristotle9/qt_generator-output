/// C++ type: <span style='color: green;'>```QTableView```</span>
#[repr(C)]
pub struct TableView(u8);

impl TableView {
  /// C++ method: <span style='color: green;'>```void QTableView::clearSpans()```</span>
  ///
  ///
  pub fn clear_spans(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTableView_clearSpans(self as *mut ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::columnAt(int x) const```</span>
  ///
  ///
  pub fn column_at(&self, x: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_columnAt(self as *const ::table_view::TableView, x) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::columnSpan(int row, int column) const```</span>
  ///
  ///
  pub fn column_span(&self, row: ::libc::c_int, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_columnSpan(self as *const ::table_view::TableView, row, column) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::columnViewportPosition(int column) const```</span>
  ///
  ///
  pub fn column_viewport_position(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_columnViewportPosition(self as *const ::table_view::TableView, column) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::columnWidth(int column) const```</span>
  ///
  ///
  pub fn column_width(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_columnWidth(self as *const ::table_view::TableView, column) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTableView::doItemsLayout()```</span>
  ///
  ///
  pub fn do_items_layout(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTableView_doItemsLayout(self as *mut ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::hideColumn(int column)```</span>
  ///
  ///
  pub fn hide_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_hideColumn(self as *mut ::table_view::TableView, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::hideRow(int row)```</span>
  ///
  ///
  pub fn hide_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_hideRow(self as *mut ::table_view::TableView, row) }
  }

  /// C++ method: <span style='color: green;'>```QHeaderView* QTableView::horizontalHeader() const```</span>
  ///
  ///
  pub fn horizontal_header(&self) -> *mut ::header_view::HeaderView {
    unsafe { ::ffi::qt_widgets_c_QTableView_horizontalHeader(self as *const ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QTableView::indexAt(const QPoint& p) const```</span>
  ///
  ///
  pub fn index_at(&self, p: &::qt_core::point::Point) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableView_indexAt_to_output(self as *const ::table_view::TableView,
                                                         p as *const ::qt_core::point::Point,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTableView::isColumnHidden(int column) const```</span>
  ///
  ///
  pub fn is_column_hidden(&self, column: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableView_isColumnHidden(self as *const ::table_view::TableView, column) }
  }

  /// C++ method: <span style='color: green;'>```bool QTableView::isCornerButtonEnabled() const```</span>
  ///
  ///
  pub fn is_corner_button_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableView_isCornerButtonEnabled(self as *const ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```bool QTableView::isRowHidden(int row) const```</span>
  ///
  ///
  pub fn is_row_hidden(&self, row: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableView_isRowHidden(self as *const ::table_view::TableView, row) }
  }

  /// C++ method: <span style='color: green;'>```bool QTableView::isSortingEnabled() const```</span>
  ///
  ///
  pub fn is_sorting_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableView_isSortingEnabled(self as *const ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTableView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTableView_metaObject(self as *const ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTableView::QTableView()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::table_view::TableView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTableView::QTableView(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::table_view::TableView> {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTableView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTableView_qt_metacall(self as *mut ::table_view::TableView,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTableView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTableView_qt_metacast(self as *mut ::table_view::TableView, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::resizeColumnToContents(int column)```</span>
  ///
  ///
  pub fn resize_column_to_contents(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_resizeColumnToContents(self as *mut ::table_view::TableView, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::resizeColumnsToContents()```</span>
  ///
  ///
  pub fn resize_columns_to_contents(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTableView_resizeColumnsToContents(self as *mut ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::resizeRowToContents(int row)```</span>
  ///
  ///
  pub fn resize_row_to_contents(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_resizeRowToContents(self as *mut ::table_view::TableView, row) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::resizeRowsToContents()```</span>
  ///
  ///
  pub fn resize_rows_to_contents(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTableView_resizeRowsToContents(self as *mut ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::rowAt(int y) const```</span>
  ///
  ///
  pub fn row_at(&self, y: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_rowAt(self as *const ::table_view::TableView, y) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::rowHeight(int row) const```</span>
  ///
  ///
  pub fn row_height(&self, row: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_rowHeight(self as *const ::table_view::TableView, row) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::rowSpan(int row, int column) const```</span>
  ///
  ///
  pub fn row_span(&self, row: ::libc::c_int, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_rowSpan(self as *const ::table_view::TableView, row, column) }
  }

  /// C++ method: <span style='color: green;'>```int QTableView::rowViewportPosition(int row) const```</span>
  ///
  ///
  pub fn row_viewport_position(&self, row: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableView_rowViewportPosition(self as *const ::table_view::TableView, row) }
  }

  /// C++ method: <span style='color: green;'>```QTableView::scrollTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, &::qt_core::model_index::ModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QTableView::scrollTo(const QModelIndex& index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, (&::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QTableView::scrollTo(const QModelIndex& index, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub fn scroll_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TableViewScrollToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QTableView::selectColumn(int column)```</span>
  ///
  ///
  pub fn select_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_selectColumn(self as *mut ::table_view::TableView, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::selectRow(int row)```</span>
  ///
  ///
  pub fn select_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_selectRow(self as *mut ::table_view::TableView, row) }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setColumnHidden(int column, bool hide)```</span>
  ///
  ///
  pub fn set_column_hidden(&mut self, column: ::libc::c_int, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setColumnHidden(self as *mut ::table_view::TableView, column, hide) }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setColumnWidth(int column, int width)```</span>
  ///
  ///
  pub fn set_column_width(&mut self, column: ::libc::c_int, width: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setColumnWidth(self as *mut ::table_view::TableView, column, width) }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setCornerButtonEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_corner_button_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setCornerButtonEnabled(self as *mut ::table_view::TableView, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setGridStyle(Qt::PenStyle style)```</span>
  ///
  ///
  pub fn set_grid_style(&mut self, style: &::qt_core::qt::PenStyle) {
    unsafe {
      ::ffi::qt_widgets_c_QTableView_setGridStyle(self as *mut ::table_view::TableView,
                                                  style as *const ::qt_core::qt::PenStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setHorizontalHeader(QHeaderView* header)```</span>
  ///
  ///
  pub unsafe fn set_horizontal_header(&mut self, header: *mut ::header_view::HeaderView) {
    ::ffi::qt_widgets_c_QTableView_setHorizontalHeader(self as *mut ::table_view::TableView, header)
  }

  /// C++ method: <span style='color: green;'>```virtual void QTableView::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QTableView_setModel(self as *mut ::table_view::TableView, model)
  }

  /// C++ method: <span style='color: green;'>```virtual void QTableView::setRootIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_root_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QTableView_setRootIndex(self as *mut ::table_view::TableView,
                                                  index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setRowHeight(int row, int height)```</span>
  ///
  ///
  pub fn set_row_height(&mut self, row: ::libc::c_int, height: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setRowHeight(self as *mut ::table_view::TableView, row, height) }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setRowHidden(int row, bool hide)```</span>
  ///
  ///
  pub fn set_row_hidden(&mut self, row: ::libc::c_int, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setRowHidden(self as *mut ::table_view::TableView, row, hide) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTableView::setSelectionModel(QItemSelectionModel* selectionModel)```</span>
  ///
  ///
  pub unsafe fn set_selection_model(&mut self,
                                    selection_model: *mut ::qt_core::item_selection_model::ItemSelectionModel) {
    ::ffi::qt_widgets_c_QTableView_setSelectionModel(self as *mut ::table_view::TableView, selection_model)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::setShowGrid(bool show)```</span>
  ///
  ///
  pub fn set_show_grid(&mut self, show: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setShowGrid(self as *mut ::table_view::TableView, show) }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setSortingEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_sorting_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setSortingEnabled(self as *mut ::table_view::TableView, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setSpan(int row, int column, int rowSpan, int columnSpan)```</span>
  ///
  ///
  pub fn set_span(&mut self,
                  row: ::libc::c_int,
                  column: ::libc::c_int,
                  row_span: ::libc::c_int,
                  column_span: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QTableView_setSpan(self as *mut ::table_view::TableView,
                                             row,
                                             column,
                                             row_span,
                                             column_span)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setVerticalHeader(QHeaderView* header)```</span>
  ///
  ///
  pub unsafe fn set_vertical_header(&mut self, header: *mut ::header_view::HeaderView) {
    ::ffi::qt_widgets_c_QTableView_setVerticalHeader(self as *mut ::table_view::TableView, header)
  }

  /// C++ method: <span style='color: green;'>```void QTableView::setWordWrap(bool on)```</span>
  ///
  ///
  pub fn set_word_wrap(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableView_setWordWrap(self as *mut ::table_view::TableView, on) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::showColumn(int column)```</span>
  ///
  ///
  pub fn show_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_showColumn(self as *mut ::table_view::TableView, column) }
  }

  /// C++ method: <span style='color: green;'>```bool QTableView::showGrid() const```</span>
  ///
  ///
  pub fn show_grid(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableView_showGrid(self as *const ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableView::showRow(int row)```</span>
  ///
  ///
  pub fn show_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableView_showRow(self as *mut ::table_view::TableView, row) }
  }

  /// C++ method: <span style='color: green;'>```QTableView::sortByColumn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort_by_column(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTableView::sortByColumn(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort_by_column(&mut self, (::libc::c_int, &::qt_core::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTableView::sortByColumn(int column, Qt::SortOrder order)```</span>
  ///
  ///
  pub fn sort_by_column<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TableViewSortByColumnArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QTableView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTableView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTableView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTableView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QHeaderView* QTableView::verticalHeader() const```</span>
  ///
  ///
  pub fn vertical_header(&self) -> *mut ::header_view::HeaderView {
    unsafe { ::ffi::qt_widgets_c_QTableView_verticalHeader(self as *const ::table_view::TableView) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QTableView::visualRect(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn visual_rect(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableView_visualRect_to_output(self as *const ::table_view::TableView,
                                                            index as *const ::qt_core::model_index::ModelIndex,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTableView::wordWrap() const```</span>
  ///
  ///
  pub fn word_wrap(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableView_wordWrap(self as *const ::table_view::TableView) }
  }
}

impl ::cpp_utils::CppDeletable for ::table_view::TableView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTableView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TableView`.
  pub struct Signals<'a>(&'a ::table_view::TableView);
  /// Represents a built-in Qt signal `QTableView::pressed`.
  ///
  /// An object of this type can be created from `TableView` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct Pressed<'a>(&'a ::table_view::TableView);
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
  /// Represents a built-in Qt signal `QTableView::doubleClicked`.
  ///
  /// An object of this type can be created from `TableView` with `object.signals().double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct DoubleClicked<'a>(&'a ::table_view::TableView);
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
  /// Represents a built-in Qt signal `QTableView::viewportEntered`.
  ///
  /// An object of this type can be created from `TableView` with `object.signals().viewport_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ViewportEntered<'a>(&'a ::table_view::TableView);
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
  /// Represents a built-in Qt signal `QTableView::iconSizeChanged`.
  ///
  /// An object of this type can be created from `TableView` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct IconSizeChanged<'a>(&'a ::table_view::TableView);
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
  /// Represents a built-in Qt signal `QTableView::activated`.
  ///
  /// An object of this type can be created from `TableView` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct Activated<'a>(&'a ::table_view::TableView);
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
  /// Represents a built-in Qt signal `QTableView::clicked`.
  ///
  /// An object of this type can be created from `TableView` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct Clicked<'a>(&'a ::table_view::TableView);
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
  /// Represents a built-in Qt signal `QTableView::entered`.
  ///
  /// An object of this type can be created from `TableView` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct Entered<'a>(&'a ::table_view::TableView);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTableView::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableView::doubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_clicked(&self) -> DoubleClicked {
      DoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableView::viewportEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn viewport_entered(&self) -> ViewportEntered {
      ViewportEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableView::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableView::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableView::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableView::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TableView`.
  pub struct Slots<'a>(&'a ::table_view::TableView);
  /// Represents a built-in Qt slot `QTableView::rowsInserted`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct RowsInserted<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for RowsInserted<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsInserted(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::sortByColumn`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().sort_by_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct SortByColumn<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for SortByColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1sortByColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::resizeColumnsToContents`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().resize_columns_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ResizeColumnsToContents<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ResizeColumnsToContents<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeColumnsToContents()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::setShowGrid`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().set_show_grid()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct SetShowGrid<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for SetShowGrid<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShowGrid(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::editorDestroyed`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().editor_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct EditorDestroyed<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for EditorDestroyed<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1editorDestroyed(QObject*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::update`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct Update<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::setCurrentIndex`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct SetCurrentIndex<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::verticalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().vertical_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct VerticalScrollbarValueChanged<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::columnMoved`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().column_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ColumnMoved<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ColumnMoved<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnMoved(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::closeEditor`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct CloseEditor<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for CloseEditor<'a> {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeEditor(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::showRow`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().show_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ShowRow<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ShowRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::updateEditorData`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().update_editor_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct UpdateEditorData<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorData()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::columnResized`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().column_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ColumnResized<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ColumnResized<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnResized(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::updateEditorGeometries`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().update_editor_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct UpdateEditorGeometries<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::rowResized`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().row_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct RowResized<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for RowResized<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowResized(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::scrollToBottom`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().scroll_to_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ScrollToBottom<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToBottom<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToBottom()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::rowCountChanged`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().row_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct RowCountChanged<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for RowCountChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowCountChanged(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::clearSelection`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ClearSelection<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::rowMoved`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().row_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct RowMoved<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for RowMoved<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowMoved(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for RowsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::columnCountChanged`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().column_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ColumnCountChanged<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ColumnCountChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnCountChanged(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::selectRow`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().select_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct SelectRow<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for SelectRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::dataChanged`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct DataChanged<'a>(&'a ::table_view::TableView);
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
  /// Represents a built-in Qt slot `QTableView::hideColumn`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().hide_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct HideColumn<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for HideColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hideColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::resizeColumnToContents`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().resize_column_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ResizeColumnToContents<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ResizeColumnToContents<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeColumnToContents(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::reset`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct Reset<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::edit`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().edit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct Edit<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for Edit<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1edit(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::resizeRowToContents`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().resize_row_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ResizeRowToContents<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ResizeRowToContents<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeRowToContents(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::horizontalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().horizontal_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct HorizontalScrollbarValueChanged<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::resizeRowsToContents`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().resize_rows_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ResizeRowsToContents<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ResizeRowsToContents<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeRowsToContents()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::selectColumn`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().select_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct SelectColumn<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for SelectColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::selectAll`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct SelectAll<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::hideRow`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().hide_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct HideRow<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for HideRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hideRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::commitData`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct CommitData<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for CommitData<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1commitData(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::scrollToTop`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().scroll_to_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ScrollToTop<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToTop<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToTop()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableView::showColumn`.
  ///
  /// An object of this type can be created from `TableView` with `object.slots().show_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableView` object.
  pub struct ShowColumn<'a>(&'a ::table_view::TableView);
  impl<'a> ::qt_core::connection::Receiver for ShowColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showColumn(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTableView::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::sortByColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sort_by_column(&self) -> SortByColumn {
      SortByColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::resizeColumnsToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_columns_to_contents(&self) -> ResizeColumnsToContents {
      ResizeColumnsToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::setShowGrid`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_show_grid(&self) -> SetShowGrid {
      SetShowGrid(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::editorDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editor_destroyed(&self) -> EditorDestroyed {
      EditorDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::verticalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_value_changed(&self) -> VerticalScrollbarValueChanged {
      VerticalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::columnMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_moved(&self) -> ColumnMoved {
      ColumnMoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::showRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_row(&self) -> ShowRow {
      ShowRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::updateEditorData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_data(&self) -> UpdateEditorData {
      UpdateEditorData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::columnResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_resized(&self) -> ColumnResized {
      ColumnResized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::updateEditorGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_geometries(&self) -> UpdateEditorGeometries {
      UpdateEditorGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::rowResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn row_resized(&self) -> RowResized {
      RowResized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::scrollToBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_bottom(&self) -> ScrollToBottom {
      ScrollToBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::rowCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn row_count_changed(&self) -> RowCountChanged {
      RowCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::rowMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn row_moved(&self) -> RowMoved {
      RowMoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::columnCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_count_changed(&self) -> ColumnCountChanged {
      ColumnCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::selectRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_row(&self) -> SelectRow {
      SelectRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::hideColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide_column(&self) -> HideColumn {
      HideColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::resizeColumnToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_column_to_contents(&self) -> ResizeColumnToContents {
      ResizeColumnToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::edit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit(&self) -> Edit {
      Edit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::resizeRowToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_row_to_contents(&self) -> ResizeRowToContents {
      ResizeRowToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::horizontalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_value_changed(&self) -> HorizontalScrollbarValueChanged {
      HorizontalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::resizeRowsToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_rows_to_contents(&self) -> ResizeRowsToContents {
      ResizeRowsToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::selectColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_column(&self) -> SelectColumn {
      SelectColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::hideRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide_row(&self) -> HideRow {
      HideRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::scrollToTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_top(&self) -> ScrollToTop {
      ScrollToTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableView::showColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_column(&self) -> ShowColumn {
      ShowColumn(self.0)
    }
  }
  impl ::table_view::TableView {
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

impl ::cpp_utils::DynamicCast<::table_view::TableView> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_view::TableView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_view::TableView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::table_view::TableView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_view::TableView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_view::TableView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::table_view::TableView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_view::TableView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_view::TableView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::table_view::TableView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_view::TableView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_view::TableView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::table_view::TableView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QObject_ptr(self as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QObject_ptr(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::table_view::TableView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QPaintDevice_ptr(self as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QPaintDevice_ptr(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::table_view::TableView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableView_G_static_cast_QAbstractItemView_ptr(self as *mut ::table_view::TableView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QAbstractItemView_ptr(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::table_view::TableView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::table_view::TableView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::table_view::TableView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QFrame_ptr(self as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QFrame_ptr(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::table_view::TableView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QWidget_ptr(self as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QWidget_ptr(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_view::TableView> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_view::TableView> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_view::TableView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_view::TableView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result =
      ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_view::TableView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_view::TableView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result =
      ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_view::TableView {
    let ffi_result = ::ffi::qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::table_view::TableView {
  type Target = ::abstract_item_view::AbstractItemView;
  fn deref(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableView_G_static_cast_QAbstractItemView_ptr(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::table_view::TableView {
  fn deref_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableView_G_static_cast_QAbstractItemView_ptr(self as *mut ::table_view::TableView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TableView::scroll_to](../struct.TableView.html#method.scroll_to) method.
  pub trait TableViewScrollToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::table_view::TableView) -> ();
  }
  impl<'largs> TableViewScrollToArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::table_view::TableView) -> () {
      let index = self;
      unsafe {
        ::ffi::qt_widgets_c_QTableView_scrollTo_index(original_self as *mut ::table_view::TableView,
                                                      index as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  impl<'largs> TableViewScrollToArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint) {
    fn exec(self, original_self: &'largs mut ::table_view::TableView) -> () {
      let index = self.0;
      let hint = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QTableView_scrollTo_index_hint(original_self as *mut ::table_view::TableView,
                                                           index as *const ::qt_core::model_index::ModelIndex,
                                                           hint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TableView::sort_by_column](../struct.TableView.html#method.sort_by_column) method.
  pub trait TableViewSortByColumnArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::table_view::TableView) -> ();
  }
  impl<'largs> TableViewSortByColumnArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::table_view::TableView) -> () {
      let column = self;
      unsafe {
        ::ffi::qt_widgets_c_QTableView_sortByColumn_column(original_self as *mut ::table_view::TableView, column)
      }
    }
  }
  impl<'largs> TableViewSortByColumnArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::table_view::TableView) -> () {
      let column = self.0;
      let order = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QTableView_sortByColumn_column_order(original_self as *mut ::table_view::TableView,
                                                                 column,
                                                                 order as *const ::qt_core::qt::SortOrder)
      }
    }
  }
}
