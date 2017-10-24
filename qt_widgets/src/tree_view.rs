/// C++ type: <span style='color: green;'>```QTreeView```</span>
#[repr(C)]
pub struct TreeView(u8);

impl TreeView {
  /// C++ method: <span style='color: green;'>```bool QTreeView::allColumnsShowFocus() const```</span>
  ///
  ///
  pub fn all_columns_show_focus(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_allColumnsShowFocus(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeView::autoExpandDelay() const```</span>
  ///
  ///
  pub fn auto_expand_delay(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeView_autoExpandDelay(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::collapse(const QModelIndex& index)```</span>
  ///
  ///
  pub fn collapse(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_collapse(self as *mut ::tree_view::TreeView,
                                             index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::collapseAll()```</span>
  ///
  ///
  pub fn collapse_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_collapseAll(self as *mut ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeView::columnAt(int x) const```</span>
  ///
  ///
  pub fn column_at(&self, x: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeView_columnAt(self as *const ::tree_view::TreeView, x) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeView::columnViewportPosition(int column) const```</span>
  ///
  ///
  pub fn column_viewport_position(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeView_columnViewportPosition(self as *const ::tree_view::TreeView, column) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeView::columnWidth(int column) const```</span>
  ///
  ///
  pub fn column_width(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeView_columnWidth(self as *const ::tree_view::TreeView, column) }
  }

  /// C++ method: <span style='color: green;'>```QTreeView::dataChanged```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data_changed(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::model_index::ModelIndex)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QTreeView::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data_changed(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::model_index::ModelIndex, &::qt_core::vector::VectorCInt)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QTreeView::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight, const QVector<int>& roles = ?)```</span>
  ///
  ///
  pub fn data_changed<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeViewDataChangedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QTreeView::doItemsLayout()```</span>
  ///
  ///
  pub fn do_items_layout(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_doItemsLayout(self as *mut ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::expand(const QModelIndex& index)```</span>
  ///
  ///
  pub fn expand(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_expand(self as *mut ::tree_view::TreeView,
                                           index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::expandAll()```</span>
  ///
  ///
  pub fn expand_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_expandAll(self as *mut ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::expandToDepth(int depth)```</span>
  ///
  ///
  pub fn expand_to_depth(&mut self, depth: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_expandToDepth(self as *mut ::tree_view::TreeView, depth) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::expandsOnDoubleClick() const```</span>
  ///
  ///
  pub fn expands_on_double_click(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_expandsOnDoubleClick(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```QHeaderView* QTreeView::header() const```</span>
  ///
  ///
  pub fn header(&self) -> *mut ::header_view::HeaderView {
    unsafe { ::ffi::qt_widgets_c_QTreeView_header(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::hideColumn(int column)```</span>
  ///
  ///
  pub fn hide_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_hideColumn(self as *mut ::tree_view::TreeView, column) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeView::indentation() const```</span>
  ///
  ///
  pub fn indentation(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeView_indentation(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QTreeView::indexAbove(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn index_above(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_indexAbove_to_output(self as *const ::tree_view::TreeView,
                                                           index as *const ::qt_core::model_index::ModelIndex,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QTreeView::indexAt(const QPoint& p) const```</span>
  ///
  ///
  pub fn index_at(&self, p: &::qt_core::point::Point) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_indexAt_to_output(self as *const ::tree_view::TreeView,
                                                        p as *const ::qt_core::point::Point,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QTreeView::indexBelow(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn index_below(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_indexBelow_to_output(self as *const ::tree_view::TreeView,
                                                           index as *const ::qt_core::model_index::ModelIndex,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::isAnimated() const```</span>
  ///
  ///
  pub fn is_animated(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_isAnimated(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::isColumnHidden(int column) const```</span>
  ///
  ///
  pub fn is_column_hidden(&self, column: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_isColumnHidden(self as *const ::tree_view::TreeView, column) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::isExpanded(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn is_expanded(&self, index: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_isExpanded(self as *const ::tree_view::TreeView,
                                               index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn is_first_column_spanned(&self, row: ::libc::c_int, parent: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_isFirstColumnSpanned(self as *const ::tree_view::TreeView,
                                                         row,
                                                         parent as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::isHeaderHidden() const```</span>
  ///
  ///
  pub fn is_header_hidden(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_isHeaderHidden(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::isRowHidden(int row, const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn is_row_hidden(&self, row: ::libc::c_int, parent: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_isRowHidden(self as *const ::tree_view::TreeView,
                                                row,
                                                parent as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::isSortingEnabled() const```</span>
  ///
  ///
  pub fn is_sorting_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_isSortingEnabled(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::itemsExpandable() const```</span>
  ///
  ///
  pub fn items_expandable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_itemsExpandable(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeView::keyboardSearch(const QString& search)```</span>
  ///
  ///
  pub fn keyboard_search(&mut self, search: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_keyboardSearch(self as *mut ::tree_view::TreeView,
                                                   search as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTreeView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTreeView_metaObject(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTreeView::QTreeView()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::tree_view::TreeView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTreeView::QTreeView(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::tree_view::TreeView> {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTreeView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTreeView_qt_metacall(self as *mut ::tree_view::TreeView,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTreeView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTreeView_qt_metacast(self as *mut ::tree_view::TreeView, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeView::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_reset(self as *mut ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::resetIndentation()```</span>
  ///
  ///
  pub fn reset_indentation(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_resetIndentation(self as *mut ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::resizeColumnToContents(int column)```</span>
  ///
  ///
  pub fn resize_column_to_contents(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_resizeColumnToContents(self as *mut ::tree_view::TreeView, column) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::rootIsDecorated() const```</span>
  ///
  ///
  pub fn root_is_decorated(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_rootIsDecorated(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```QTreeView::scrollTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, &::qt_core::model_index::ModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QTreeView::scrollTo(const QModelIndex& index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, (&::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QTreeView::scrollTo(const QModelIndex& index, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub fn scroll_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeViewScrollToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QTreeView::selectAll()```</span>
  ///
  ///
  pub fn select_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_selectAll(self as *mut ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setAllColumnsShowFocus(bool enable)```</span>
  ///
  ///
  pub fn set_all_columns_show_focus(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setAllColumnsShowFocus(self as *mut ::tree_view::TreeView, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setAnimated(bool enable)```</span>
  ///
  ///
  pub fn set_animated(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setAnimated(self as *mut ::tree_view::TreeView, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setAutoExpandDelay(int delay)```</span>
  ///
  ///
  pub fn set_auto_expand_delay(&mut self, delay: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setAutoExpandDelay(self as *mut ::tree_view::TreeView, delay) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setColumnHidden(int column, bool hide)```</span>
  ///
  ///
  pub fn set_column_hidden(&mut self, column: ::libc::c_int, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setColumnHidden(self as *mut ::tree_view::TreeView, column, hide) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setColumnWidth(int column, int width)```</span>
  ///
  ///
  pub fn set_column_width(&mut self, column: ::libc::c_int, width: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setColumnWidth(self as *mut ::tree_view::TreeView, column, width) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setExpanded(const QModelIndex& index, bool expand)```</span>
  ///
  ///
  pub fn set_expanded(&mut self, index: &::qt_core::model_index::ModelIndex, expand: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_setExpanded(self as *mut ::tree_view::TreeView,
                                                index as *const ::qt_core::model_index::ModelIndex,
                                                expand)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setExpandsOnDoubleClick(bool enable)```</span>
  ///
  ///
  pub fn set_expands_on_double_click(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setExpandsOnDoubleClick(self as *mut ::tree_view::TreeView, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setFirstColumnSpanned(int row, const QModelIndex& parent, bool span)```</span>
  ///
  ///
  pub fn set_first_column_spanned(&mut self,
                                  row: ::libc::c_int,
                                  parent: &::qt_core::model_index::ModelIndex,
                                  span: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_setFirstColumnSpanned(self as *mut ::tree_view::TreeView,
                                                          row,
                                                          parent as *const ::qt_core::model_index::ModelIndex,
                                                          span)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setHeader(QHeaderView* header)```</span>
  ///
  ///
  pub unsafe fn set_header(&mut self, header: *mut ::header_view::HeaderView) {
    ::ffi::qt_widgets_c_QTreeView_setHeader(self as *mut ::tree_view::TreeView, header)
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setHeaderHidden(bool hide)```</span>
  ///
  ///
  pub fn set_header_hidden(&mut self, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setHeaderHidden(self as *mut ::tree_view::TreeView, hide) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setIndentation(int i)```</span>
  ///
  ///
  pub fn set_indentation(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setIndentation(self as *mut ::tree_view::TreeView, i) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setItemsExpandable(bool enable)```</span>
  ///
  ///
  pub fn set_items_expandable(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setItemsExpandable(self as *mut ::tree_view::TreeView, enable) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeView::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QTreeView_setModel(self as *mut ::tree_view::TreeView, model)
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeView::setRootIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_root_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_setRootIndex(self as *mut ::tree_view::TreeView,
                                                 index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setRootIsDecorated(bool show)```</span>
  ///
  ///
  pub fn set_root_is_decorated(&mut self, show: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setRootIsDecorated(self as *mut ::tree_view::TreeView, show) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setRowHidden(int row, const QModelIndex& parent, bool hide)```</span>
  ///
  ///
  pub fn set_row_hidden(&mut self, row: ::libc::c_int, parent: &::qt_core::model_index::ModelIndex, hide: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeView_setRowHidden(self as *mut ::tree_view::TreeView,
                                                 row,
                                                 parent as *const ::qt_core::model_index::ModelIndex,
                                                 hide)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeView::setSelectionModel(QItemSelectionModel* selectionModel)```</span>
  ///
  ///
  pub unsafe fn set_selection_model(&mut self,
                                    selection_model: *mut ::qt_core::item_selection_model::ItemSelectionModel) {
    ::ffi::qt_widgets_c_QTreeView_setSelectionModel(self as *mut ::tree_view::TreeView, selection_model)
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setSortingEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_sorting_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setSortingEnabled(self as *mut ::tree_view::TreeView, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setTreePosition(int logicalIndex)```</span>
  ///
  ///
  pub fn set_tree_position(&mut self, logical_index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setTreePosition(self as *mut ::tree_view::TreeView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setUniformRowHeights(bool uniform)```</span>
  ///
  ///
  pub fn set_uniform_row_heights(&mut self, uniform: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setUniformRowHeights(self as *mut ::tree_view::TreeView, uniform) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeView::setWordWrap(bool on)```</span>
  ///
  ///
  pub fn set_word_wrap(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_setWordWrap(self as *mut ::tree_view::TreeView, on) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::showColumn(int column)```</span>
  ///
  ///
  pub fn show_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeView_showColumn(self as *mut ::tree_view::TreeView, column) }
  }

  /// C++ method: <span style='color: green;'>```QTreeView::sortByColumn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort_by_column(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTreeView::sortByColumn(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort_by_column(&mut self, (::libc::c_int, &::qt_core::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeView::sortByColumn(int column, Qt::SortOrder order)```</span>
  ///
  ///
  pub fn sort_by_column<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeViewSortByColumnArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QTreeView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTreeView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTreeView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTreeView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTreeView::treePosition() const```</span>
  ///
  ///
  pub fn tree_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeView_treePosition(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::uniformRowHeights() const```</span>
  ///
  ///
  pub fn uniform_row_heights(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_uniformRowHeights(self as *const ::tree_view::TreeView) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QTreeView::visualRect(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn visual_rect(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_visualRect_to_output(self as *const ::tree_view::TreeView,
                                                           index as *const ::qt_core::model_index::ModelIndex,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeView::wordWrap() const```</span>
  ///
  ///
  pub fn word_wrap(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeView_wordWrap(self as *const ::tree_view::TreeView) }
  }
}

impl ::cpp_utils::CppDeletable for ::tree_view::TreeView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTreeView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TreeView`.
  pub struct Signals<'a>(&'a ::tree_view::TreeView);
  /// Represents a built-in Qt signal `QTreeView::iconSizeChanged`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct IconSizeChanged<'a>(&'a ::tree_view::TreeView);
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
  /// Represents a built-in Qt signal `QTreeView::collapsed`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().collapsed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Collapsed<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for Collapsed<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2collapsed(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Collapsed<'a> {}
  /// Represents a built-in Qt signal `QTreeView::expanded`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().expanded()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Expanded<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for Expanded<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2expanded(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Expanded<'a> {}
  /// Represents a built-in Qt signal `QTreeView::clicked`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Clicked<'a>(&'a ::tree_view::TreeView);
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
  /// Represents a built-in Qt signal `QTreeView::entered`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Entered<'a>(&'a ::tree_view::TreeView);
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
  /// Represents a built-in Qt signal `QTreeView::pressed`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Pressed<'a>(&'a ::tree_view::TreeView);
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
  /// Represents a built-in Qt signal `QTreeView::activated`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Activated<'a>(&'a ::tree_view::TreeView);
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
  /// Represents a built-in Qt signal `QTreeView::viewportEntered`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().viewport_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ViewportEntered<'a>(&'a ::tree_view::TreeView);
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
  /// Represents a built-in Qt signal `QTreeView::doubleClicked`.
  ///
  /// An object of this type can be created from `TreeView` with `object.signals().double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct DoubleClicked<'a>(&'a ::tree_view::TreeView);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTreeView::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::collapsed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn collapsed(&self) -> Collapsed {
      Collapsed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::expanded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expanded(&self) -> Expanded {
      Expanded(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::viewportEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn viewport_entered(&self) -> ViewportEntered {
      ViewportEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeView::doubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_clicked(&self) -> DoubleClicked {
      DoubleClicked(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TreeView`.
  pub struct Slots<'a>(&'a ::tree_view::TreeView);
  /// Represents a built-in Qt slot `QTreeView::commitData`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct CommitData<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for CommitData<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1commitData(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::resizeColumnToContents`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().resize_column_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ResizeColumnToContents<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ResizeColumnToContents<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeColumnToContents(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::rowsRemoved`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct RowsRemoved<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for RowsRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::updateEditorData`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().update_editor_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct UpdateEditorData<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorData()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::columnResized`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().column_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ColumnResized<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ColumnResized<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnResized(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::verticalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().vertical_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct VerticalScrollbarValueChanged<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::sortByColumn`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().sort_by_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct SortByColumn<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for SortByColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1sortByColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::expand`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().expand()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Expand<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for Expand<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1expand(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::updateEditorGeometries`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().update_editor_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct UpdateEditorGeometries<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::verticalScrollbarAction`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().vertical_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct VerticalScrollbarAction<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::closeEditor`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct CloseEditor<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for CloseEditor<'a> {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeEditor(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::columnMoved`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().column_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ColumnMoved<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ColumnMoved<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnMoved()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::clearSelection`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ClearSelection<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::hideColumn`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().hide_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct HideColumn<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for HideColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hideColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::collapseAll`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().collapse_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct CollapseAll<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for CollapseAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1collapseAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::expandToDepth`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().expand_to_depth()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ExpandToDepth<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ExpandToDepth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1expandToDepth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::edit`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().edit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Edit<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for Edit<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1edit(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::setCurrentIndex`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct SetCurrentIndex<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::columnCountChanged`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().column_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ColumnCountChanged<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ColumnCountChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnCountChanged(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::reexpand`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().reexpand()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Reexpand<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for Reexpand<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reexpand()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::update`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Update<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::scrollToBottom`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().scroll_to_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ScrollToBottom<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToBottom<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToBottom()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::collapse`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().collapse()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct Collapse<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for Collapse<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1collapse(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::editorDestroyed`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().editor_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct EditorDestroyed<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for EditorDestroyed<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1editorDestroyed(QObject*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::showColumn`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().show_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ShowColumn<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ShowColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::expandAll`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().expand_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ExpandAll<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ExpandAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1expandAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::scrollToTop`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().scroll_to_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct ScrollToTop<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToTop<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToTop()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeView::horizontalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `TreeView` with `object.slots().horizontal_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeView` object.
  pub struct HorizontalScrollbarValueChanged<'a>(&'a ::tree_view::TreeView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarValueChanged(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTreeView::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::resizeColumnToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_column_to_contents(&self) -> ResizeColumnToContents {
      ResizeColumnToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::updateEditorData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_data(&self) -> UpdateEditorData {
      UpdateEditorData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::columnResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_resized(&self) -> ColumnResized {
      ColumnResized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::verticalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_value_changed(&self) -> VerticalScrollbarValueChanged {
      VerticalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::sortByColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sort_by_column(&self) -> SortByColumn {
      SortByColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::expand`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expand(&self) -> Expand {
      Expand(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::updateEditorGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_geometries(&self) -> UpdateEditorGeometries {
      UpdateEditorGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::verticalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_action(&self) -> VerticalScrollbarAction {
      VerticalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::columnMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_moved(&self) -> ColumnMoved {
      ColumnMoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::hideColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide_column(&self) -> HideColumn {
      HideColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::collapseAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn collapse_all(&self) -> CollapseAll {
      CollapseAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::expandToDepth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expand_to_depth(&self) -> ExpandToDepth {
      ExpandToDepth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::edit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit(&self) -> Edit {
      Edit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::columnCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_count_changed(&self) -> ColumnCountChanged {
      ColumnCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::reexpand`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reexpand(&self) -> Reexpand {
      Reexpand(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::scrollToBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_bottom(&self) -> ScrollToBottom {
      ScrollToBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::collapse`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn collapse(&self) -> Collapse {
      Collapse(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::editorDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editor_destroyed(&self) -> EditorDestroyed {
      EditorDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::showColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_column(&self) -> ShowColumn {
      ShowColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::expandAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expand_all(&self) -> ExpandAll {
      ExpandAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::scrollToTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_top(&self) -> ScrollToTop {
      ScrollToTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeView::horizontalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_value_changed(&self) -> HorizontalScrollbarValueChanged {
      HorizontalScrollbarValueChanged(self.0)
    }
  }
  impl ::tree_view::TreeView {
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

impl ::cpp_utils::DynamicCast<::tree_view::TreeView> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_view::TreeView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_view::TreeView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tree_view::TreeView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_view::TreeView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_view::TreeView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tree_view::TreeView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_view::TreeView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_view::TreeView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tree_view::TreeView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_view::TreeView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_view::TreeView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tree_view::TreeView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QObject_ptr(self as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QObject_ptr(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::tree_view::TreeView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QPaintDevice_ptr(self as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QPaintDevice_ptr(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::tree_view::TreeView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QAbstractItemView_ptr(self as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QAbstractItemView_ptr(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::tree_view::TreeView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::tree_view::TreeView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::tree_view::TreeView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QFrame_ptr(self as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QFrame_ptr(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::tree_view::TreeView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QWidget_ptr(self as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QWidget_ptr(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_view::TreeView> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_view::TreeView> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_view::TreeView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_view::TreeView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result =
      ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_view::TreeView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_view::TreeView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_view::TreeView {
    let ffi_result = ::ffi::qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tree_view::TreeView {
  type Target = ::abstract_item_view::AbstractItemView;
  fn deref(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QAbstractItemView_ptr(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tree_view::TreeView {
  fn deref_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeView_G_static_cast_QAbstractItemView_ptr(self as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TreeView::data_changed](../struct.TreeView.html#method.data_changed) method.
  pub trait TreeViewDataChangedArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> ();
  }
  impl<'largs> TreeViewDataChangedArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> () {
      let top_left = self.0;
      let bottom_right = self.1;
      unsafe { ::ffi::qt_widgets_c_QTreeView_dataChanged_topLeft_bottomRight(original_self as *mut ::tree_view::TreeView, top_left as *const ::qt_core::model_index::ModelIndex, bottom_right as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  impl<'largs> TreeViewDataChangedArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex,
                                                        &'largs ::qt_core::model_index::ModelIndex,
                                                        &'largs ::qt_core::vector::VectorCInt) {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> () {
      let top_left = self.0;
      let bottom_right = self.1;
      let roles = self.2;
      unsafe { ::ffi::qt_widgets_c_QTreeView_dataChanged_topLeft_bottomRight_roles(original_self as *mut ::tree_view::TreeView, top_left as *const ::qt_core::model_index::ModelIndex, bottom_right as *const ::qt_core::model_index::ModelIndex, roles as *const ::qt_core::vector::VectorCInt) }
    }
  }
  /// This trait represents a set of arguments accepted by [TreeView::scroll_to](../struct.TreeView.html#method.scroll_to) method.
  pub trait TreeViewScrollToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> ();
  }
  impl<'largs> TreeViewScrollToArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> () {
      let index = self;
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_scrollTo_index(original_self as *mut ::tree_view::TreeView,
                                                     index as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  impl<'largs> TreeViewScrollToArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint) {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> () {
      let index = self.0;
      let hint = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_scrollTo_index_hint(original_self as *mut ::tree_view::TreeView,
                                                          index as *const ::qt_core::model_index::ModelIndex,
                                                          hint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TreeView::sort_by_column](../struct.TreeView.html#method.sort_by_column) method.
  pub trait TreeViewSortByColumnArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> ();
  }
  impl<'largs> TreeViewSortByColumnArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> () {
      let column = self;
      unsafe { ::ffi::qt_widgets_c_QTreeView_sortByColumn_column(original_self as *mut ::tree_view::TreeView, column) }
    }
  }
  impl<'largs> TreeViewSortByColumnArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::tree_view::TreeView) -> () {
      let column = self.0;
      let order = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QTreeView_sortByColumn_column_order(original_self as *mut ::tree_view::TreeView,
                                                                column,
                                                                order as *const ::qt_core::qt::SortOrder)
      }
    }
  }
}
