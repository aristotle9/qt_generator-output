/// C++ type: <span style='color: green;'>```QAbstractItemView```</span>
#[repr(C)]
pub struct AbstractItemView(u8);

impl AbstractItemView {
  /// C++ method: <span style='color: green;'>```bool QAbstractItemView::alternatingRowColors() const```</span>
  ///
  ///
  pub fn alternating_row_colors(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_alternatingRowColors(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractItemView::autoScrollMargin() const```</span>
  ///
  ///
  pub fn auto_scroll_margin(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_autoScrollMargin(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractItemView::clearSelection()```</span>
  ///
  ///
  pub fn clear_selection(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_clearSelection(self as *mut ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::closePersistentEditor(const QModelIndex& index)```</span>
  ///
  ///
  pub fn close_persistent_editor(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_closePersistentEditor(self as *mut ::abstract_item_view::AbstractItemView,
                                                                  index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QAbstractItemView::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_currentIndex_to_output(self as *const ::abstract_item_view::AbstractItemView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QAbstractItemView::doItemsLayout()```</span>
  ///
  ///
  pub fn do_items_layout(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_doItemsLayout(self as *mut ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemView::DragDropMode QAbstractItemView::dragDropMode() const```</span>
  ///
  ///
  pub fn drag_drop_mode(&self) -> ::abstract_item_view::DragDropMode {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_dragDropMode(self as *const ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractItemView::dragDropOverwriteMode() const```</span>
  ///
  ///
  pub fn drag_drop_overwrite_mode(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_dragDropOverwriteMode(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractItemView::dragEnabled() const```</span>
  ///
  ///
  pub fn drag_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_dragEnabled(self as *const ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractItemView::edit(const QModelIndex& index)```</span>
  ///
  ///
  pub fn edit(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_edit(self as *mut ::abstract_item_view::AbstractItemView,
                                                 index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QAbstractItemView::EditTrigger> QAbstractItemView::editTriggers() const```</span>
  ///
  ///
  pub fn edit_triggers(&self) -> ::qt_core::flags::Flags<::abstract_item_view::EditTrigger> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_editTriggers(self as *const ::abstract_item_view::AbstractItemView)
      };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractItemView::hasAutoScroll() const```</span>
  ///
  ///
  pub fn has_auto_scroll(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_hasAutoScroll(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemView::ScrollMode QAbstractItemView::horizontalScrollMode() const```</span>
  ///
  ///
  pub fn horizontal_scroll_mode(&self) -> ::abstract_item_view::ScrollMode {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_horizontalScrollMode(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QAbstractItemView::iconSize() const```</span>
  ///
  ///
  pub fn icon_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_iconSize_to_output(self as *const ::abstract_item_view::AbstractItemView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QModelIndex QAbstractItemView::indexAt(const QPoint& point) const```</span>
  ///
  ///
  pub fn index_at(&self, point: &::qt_core::point::Point) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_indexAt_to_output(self as *const ::abstract_item_view::AbstractItemView, point as *const ::qt_core::point::Point, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QAbstractItemView::indexWidget(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn index_widget(&self, index: &::qt_core::model_index::ModelIndex) -> *mut ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_indexWidget(self as *const ::abstract_item_view::AbstractItemView,
                                                        index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractItemView::inputMethodQuery(Qt::InputMethodQuery query) const```</span>
  ///
  ///
  pub fn input_method_query(&self, query: &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_inputMethodQuery_to_output(self as *const ::abstract_item_view::AbstractItemView, query as *const ::qt_core::qt::InputMethodQuery, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemView::itemDelegate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_delegate(&self, ()) -> *mut ::abstract_item_delegate::AbstractItemDelegate```<br>
  /// C++ method: <span style='color: green;'>```QAbstractItemDelegate* QAbstractItemView::itemDelegate() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_delegate(&self, &::qt_core::model_index::ModelIndex) -> *mut ::abstract_item_delegate::AbstractItemDelegate```<br>
  /// C++ method: <span style='color: green;'>```QAbstractItemDelegate* QAbstractItemView::itemDelegate(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn item_delegate<'largs, Args>(&'largs self, args: Args) -> *mut ::abstract_item_delegate::AbstractItemDelegate
    where Args: overloading::AbstractItemViewItemDelegateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemDelegate* QAbstractItemView::itemDelegateForColumn(int column) const```</span>
  ///
  ///
  pub fn item_delegate_for_column(&self, column: ::libc::c_int) -> *mut ::abstract_item_delegate::AbstractItemDelegate {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_itemDelegateForColumn(self as *const ::abstract_item_view::AbstractItemView, column)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemDelegate* QAbstractItemView::itemDelegateForRow(int row) const```</span>
  ///
  ///
  pub fn item_delegate_for_row(&self, row: ::libc::c_int) -> *mut ::abstract_item_delegate::AbstractItemDelegate {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_itemDelegateForRow(self as *const ::abstract_item_view::AbstractItemView,
                                                               row)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemView::keyboardSearch(const QString& search)```</span>
  ///
  ///
  pub fn keyboard_search(&mut self, search: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_keyboardSearch(self as *mut ::abstract_item_view::AbstractItemView,
                                                           search as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractItemView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_metaObject(self as *const ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel* QAbstractItemView::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *mut ::qt_core::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_model(self as *const ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::openPersistentEditor(const QModelIndex& index)```</span>
  ///
  ///
  pub fn open_persistent_editor(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_openPersistentEditor(self as *mut ::abstract_item_view::AbstractItemView,
                                                                 index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractItemView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAbstractItemView_qt_metacall(self as *mut ::abstract_item_view::AbstractItemView,
                                                      arg1 as *const ::qt_core::meta_object::Call,
                                                      arg2,
                                                      arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAbstractItemView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QAbstractItemView_qt_metacast(self as *mut ::abstract_item_view::AbstractItemView, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QAbstractItemView::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_reset(self as *mut ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::resetHorizontalScrollMode()```</span>
  ///
  ///
  pub fn reset_horizontal_scroll_mode(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_resetHorizontalScrollMode(self as *mut ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::resetVerticalScrollMode()```</span>
  ///
  ///
  pub fn reset_vertical_scroll_mode(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_resetVerticalScrollMode(self as *mut ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QAbstractItemView::rootIndex() const```</span>
  ///
  ///
  pub fn root_index(&self) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_rootIndex_to_output(self as *const ::abstract_item_view::AbstractItemView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemView::scrollTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, &::qt_core::model_index::ModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractItemView::scrollTo(const QModelIndex& index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, (&::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractItemView::scrollTo(const QModelIndex& index, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub fn scroll_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::AbstractItemViewScrollToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QAbstractItemView::scrollToBottom()```</span>
  ///
  ///
  pub fn scroll_to_bottom(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_scrollToBottom(self as *mut ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractItemView::scrollToTop()```</span>
  ///
  ///
  pub fn scroll_to_top(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_scrollToTop(self as *mut ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QAbstractItemView::selectAll()```</span>
  ///
  ///
  pub fn select_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_selectAll(self as *mut ::abstract_item_view::AbstractItemView) }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionModel* QAbstractItemView::selectionModel() const```</span>
  ///
  ///
  pub fn selection_model(&self) -> *mut ::qt_core::item_selection_model::ItemSelectionModel {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_selectionModel(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setAlternatingRowColors(bool enable)```</span>
  ///
  ///
  pub fn set_alternating_row_colors(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setAlternatingRowColors(self as *mut ::abstract_item_view::AbstractItemView, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setAutoScroll(bool enable)```</span>
  ///
  ///
  pub fn set_auto_scroll(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setAutoScroll(self as *mut ::abstract_item_view::AbstractItemView, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setAutoScrollMargin(int margin)```</span>
  ///
  ///
  pub fn set_auto_scroll_margin(&mut self, margin: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setAutoScrollMargin(self as *mut ::abstract_item_view::AbstractItemView,
                                                                margin)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractItemView::setCurrentIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setCurrentIndex(self as *mut ::abstract_item_view::AbstractItemView,
                                                            index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setDefaultDropAction(Qt::DropAction dropAction)```</span>
  ///
  ///
  pub fn set_default_drop_action(&mut self, drop_action: &::qt_core::qt::DropAction) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setDefaultDropAction(self as *mut ::abstract_item_view::AbstractItemView,
                                                                 drop_action as *const ::qt_core::qt::DropAction)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setDragDropMode(QAbstractItemView::DragDropMode behavior)```</span>
  ///
  ///
  pub fn set_drag_drop_mode(&mut self, behavior: ::abstract_item_view::DragDropMode) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setDragDropMode(self as *mut ::abstract_item_view::AbstractItemView,
                                                            behavior)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setDragDropOverwriteMode(bool overwrite)```</span>
  ///
  ///
  pub fn set_drag_drop_overwrite_mode(&mut self, overwrite: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_setDragDropOverwriteMode(self as *mut ::abstract_item_view::AbstractItemView, overwrite) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setDragEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_drag_enabled(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setDragEnabled(self as *mut ::abstract_item_view::AbstractItemView, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setDropIndicatorShown(bool enable)```</span>
  ///
  ///
  pub fn set_drop_indicator_shown(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setDropIndicatorShown(self as *mut ::abstract_item_view::AbstractItemView,
                                                                  enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setEditTriggers(QFlags<QAbstractItemView::EditTrigger> triggers)```</span>
  ///
  ///
  pub fn set_edit_triggers(&mut self, triggers: ::qt_core::flags::Flags<::abstract_item_view::EditTrigger>) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setEditTriggers(self as *mut ::abstract_item_view::AbstractItemView,
                                                            triggers.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setHorizontalScrollMode(QAbstractItemView::ScrollMode mode)```</span>
  ///
  ///
  pub fn set_horizontal_scroll_mode(&mut self, mode: ::abstract_item_view::ScrollMode) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setHorizontalScrollMode(self as *mut ::abstract_item_view::AbstractItemView, mode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setIconSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setIconSize(self as *mut ::abstract_item_view::AbstractItemView,
                                                        size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setIndexWidget(const QModelIndex& index, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_index_widget(&mut self,
                                 index: &::qt_core::model_index::ModelIndex,
                                 widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QAbstractItemView_setIndexWidget(self as *mut ::abstract_item_view::AbstractItemView,
                                                         index as *const ::qt_core::model_index::ModelIndex,
                                                         widget)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setItemDelegate(QAbstractItemDelegate* delegate)```</span>
  ///
  ///
  pub unsafe fn set_item_delegate(&mut self, delegate: *mut ::abstract_item_delegate::AbstractItemDelegate) {
    ::ffi::qt_widgets_c_QAbstractItemView_setItemDelegate(self as *mut ::abstract_item_view::AbstractItemView,
                                                          delegate)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setItemDelegateForColumn(int column, QAbstractItemDelegate* delegate)```</span>
  ///
  ///
  pub unsafe fn set_item_delegate_for_column(&mut self,
                                             column: ::libc::c_int,
                                             delegate: *mut ::abstract_item_delegate::AbstractItemDelegate) {
    ::ffi::qt_widgets_c_QAbstractItemView_setItemDelegateForColumn(self as *mut ::abstract_item_view::AbstractItemView, column, delegate)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setItemDelegateForRow(int row, QAbstractItemDelegate* delegate)```</span>
  ///
  ///
  pub unsafe fn set_item_delegate_for_row(&mut self,
                                          row: ::libc::c_int,
                                          delegate: *mut ::abstract_item_delegate::AbstractItemDelegate) {
    ::ffi::qt_widgets_c_QAbstractItemView_setItemDelegateForRow(self as *mut ::abstract_item_view::AbstractItemView,
                                                                row,
                                                                delegate)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemView::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QAbstractItemView_setModel(self as *mut ::abstract_item_view::AbstractItemView, model)
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QAbstractItemView::setRootIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_root_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setRootIndex(self as *mut ::abstract_item_view::AbstractItemView,
                                                         index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setSelectionBehavior(QAbstractItemView::SelectionBehavior behavior)```</span>
  ///
  ///
  pub fn set_selection_behavior(&mut self, behavior: &::abstract_item_view::SelectionBehavior) {
    unsafe { ::ffi::qt_widgets_c_QAbstractItemView_setSelectionBehavior(self as *mut ::abstract_item_view::AbstractItemView, behavior as *const ::abstract_item_view::SelectionBehavior) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setSelectionMode(QAbstractItemView::SelectionMode mode)```</span>
  ///
  ///
  pub fn set_selection_mode(&mut self, mode: &::abstract_item_view::SelectionMode) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setSelectionMode(self as *mut ::abstract_item_view::AbstractItemView,
                                                             mode as *const ::abstract_item_view::SelectionMode)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemView::setSelectionModel(QItemSelectionModel* selectionModel)```</span>
  ///
  ///
  pub unsafe fn set_selection_model(&mut self,
                                    selection_model: *mut ::qt_core::item_selection_model::ItemSelectionModel) {
    ::ffi::qt_widgets_c_QAbstractItemView_setSelectionModel(self as *mut ::abstract_item_view::AbstractItemView,
                                                            selection_model)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setTabKeyNavigation(bool enable)```</span>
  ///
  ///
  pub fn set_tab_key_navigation(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setTabKeyNavigation(self as *mut ::abstract_item_view::AbstractItemView,
                                                                enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setTextElideMode(Qt::TextElideMode mode)```</span>
  ///
  ///
  pub fn set_text_elide_mode(&mut self, mode: &::qt_core::qt::TextElideMode) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setTextElideMode(self as *mut ::abstract_item_view::AbstractItemView,
                                                             mode as *const ::qt_core::qt::TextElideMode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractItemView::setVerticalScrollMode(QAbstractItemView::ScrollMode mode)```</span>
  ///
  ///
  pub fn set_vertical_scroll_mode(&mut self, mode: ::abstract_item_view::ScrollMode) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_setVerticalScrollMode(self as *mut ::abstract_item_view::AbstractItemView,
                                                                  mode)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractItemView::showDropIndicator() const```</span>
  ///
  ///
  pub fn show_drop_indicator(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_showDropIndicator(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractItemView::sizeHintForColumn(int column) const```</span>
  ///
  ///
  pub fn size_hint_for_column(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_sizeHintForColumn(self as *const ::abstract_item_view::AbstractItemView,
                                                              column)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QAbstractItemView::sizeHintForIndex(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn size_hint_for_index(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_sizeHintForIndex_to_output(self as *const ::abstract_item_view::AbstractItemView, index as *const ::qt_core::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractItemView::sizeHintForRow(int row) const```</span>
  ///
  ///
  pub fn size_hint_for_row(&self, row: ::libc::c_int) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_sizeHintForRow(self as *const ::abstract_item_view::AbstractItemView, row)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractItemView::tabKeyNavigation() const```</span>
  ///
  ///
  pub fn tab_key_navigation(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_tabKeyNavigation(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractItemView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractItemView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractItemView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractItemView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractItemView::update(const QModelIndex& index)```</span>
  ///
  ///
  pub fn update(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_update(self as *mut ::abstract_item_view::AbstractItemView,
                                                   index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemView::ScrollMode QAbstractItemView::verticalScrollMode() const```</span>
  ///
  ///
  pub fn vertical_scroll_mode(&self) -> ::abstract_item_view::ScrollMode {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractItemView_verticalScrollMode(self as *const ::abstract_item_view::AbstractItemView)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QRect QAbstractItemView::visualRect(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn visual_rect(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_visualRect_to_output(self as *const ::abstract_item_view::AbstractItemView, index as *const ::qt_core::model_index::ModelIndex, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_item_view::AbstractItemView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAbstractItemView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractItemView`.
  pub struct Signals<'a>(&'a ::abstract_item_view::AbstractItemView);
  /// Represents a built-in Qt signal `QAbstractItemView::viewportEntered`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.signals().viewport_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct ViewportEntered<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt signal `QAbstractItemView::iconSizeChanged`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct IconSizeChanged<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt signal `QAbstractItemView::clicked`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct Clicked<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt signal `QAbstractItemView::pressed`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct Pressed<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt signal `QAbstractItemView::activated`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct Activated<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt signal `QAbstractItemView::doubleClicked`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.signals().double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct DoubleClicked<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt signal `QAbstractItemView::entered`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct Entered<'a>(&'a ::abstract_item_view::AbstractItemView);
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
    /// Returns an object representing a built-in Qt signal `QAbstractItemView::viewportEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn viewport_entered(&self) -> ViewportEntered {
      ViewportEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemView::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemView::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemView::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemView::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemView::doubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_clicked(&self) -> DoubleClicked {
      DoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemView::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractItemView`.
  pub struct Slots<'a>(&'a ::abstract_item_view::AbstractItemView);
  /// Represents a built-in Qt slot `QAbstractItemView::editorDestroyed`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().editor_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct EditorDestroyed<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for EditorDestroyed<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1editorDestroyed(QObject*)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::updateEditorGeometries`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().update_editor_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct UpdateEditorGeometries<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::rowsInserted`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct RowsInserted<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for RowsInserted<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsInserted(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::commitData`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct CommitData<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for CommitData<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1commitData(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::setRootIndex`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().set_root_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct SetRootIndex<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for SetRootIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRootIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::verticalScrollbarAction`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().vertical_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct VerticalScrollbarAction<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::horizontalScrollbarAction`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().horizontal_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct HorizontalScrollbarAction<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::reset`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct Reset<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::edit`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().edit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct Edit<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for Edit<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1edit(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::scrollToBottom`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().scroll_to_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct ScrollToBottom<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToBottom<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToBottom()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::selectionChanged`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct SelectionChanged<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt slot `QAbstractItemView::dataChanged`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct DataChanged<'a>(&'a ::abstract_item_view::AbstractItemView);
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
  /// Represents a built-in Qt slot `QAbstractItemView::updateGeometries`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().update_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct UpdateGeometries<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for UpdateGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::clearSelection`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct ClearSelection<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::updateEditorData`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().update_editor_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct UpdateEditorData<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorData()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for RowsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::selectAll`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct SelectAll<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::update`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct Update<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::currentChanged`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct CurrentChanged<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for CurrentChanged<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, &'static ::qt_core::model_index::ModelIndex);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1currentChanged(const QModelIndex&,const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::verticalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().vertical_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct VerticalScrollbarValueChanged<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::closeEditor`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct CloseEditor<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for CloseEditor<'a> {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeEditor(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::scrollToTop`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().scroll_to_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct ScrollToTop<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToTop<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToTop()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::setCurrentIndex`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct SetCurrentIndex<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::horizontalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().horizontal_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct HorizontalScrollbarValueChanged<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemView::doItemsLayout`.
  ///
  /// An object of this type can be created from `AbstractItemView` with `object.slots().do_items_layout()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemView` object.
  pub struct DoItemsLayout<'a>(&'a ::abstract_item_view::AbstractItemView);
  impl<'a> ::qt_core::connection::Receiver for DoItemsLayout<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1doItemsLayout()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::editorDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editor_destroyed(&self) -> EditorDestroyed {
      EditorDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::updateEditorGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_geometries(&self) -> UpdateEditorGeometries {
      UpdateEditorGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::setRootIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_root_index(&self) -> SetRootIndex {
      SetRootIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::verticalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_action(&self) -> VerticalScrollbarAction {
      VerticalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::horizontalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_action(&self) -> HorizontalScrollbarAction {
      HorizontalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::edit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit(&self) -> Edit {
      Edit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::scrollToBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_bottom(&self) -> ScrollToBottom {
      ScrollToBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::updateGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_geometries(&self) -> UpdateGeometries {
      UpdateGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::updateEditorData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_data(&self) -> UpdateEditorData {
      UpdateEditorData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::verticalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_value_changed(&self) -> VerticalScrollbarValueChanged {
      VerticalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::scrollToTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_top(&self) -> ScrollToTop {
      ScrollToTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::horizontalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_value_changed(&self) -> HorizontalScrollbarValueChanged {
      HorizontalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemView::doItemsLayout`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn do_items_layout(&self) -> DoItemsLayout {
      DoItemsLayout(self.0)
    }
  }
  impl ::abstract_item_view::AbstractItemView {
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

/// C++ type: <span style='color: green;'>```QAbstractItemView::CursorAction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CursorAction {
  /// C++ enum variant: <span style='color: green;'>```MoveUp = 0```</span>
  Up = 0,
  /// C++ enum variant: <span style='color: green;'>```MoveDown = 1```</span>
  Down = 1,
  /// C++ enum variant: <span style='color: green;'>```MoveLeft = 2```</span>
  Left = 2,
  /// C++ enum variant: <span style='color: green;'>```MoveRight = 3```</span>
  Right = 3,
  /// C++ enum variant: <span style='color: green;'>```MoveHome = 4```</span>
  Home = 4,
  /// C++ enum variant: <span style='color: green;'>```MoveEnd = 5```</span>
  End = 5,
  /// C++ enum variant: <span style='color: green;'>```MovePageUp = 6```</span>
  PageUp = 6,
  /// C++ enum variant: <span style='color: green;'>```MovePageDown = 7```</span>
  PageDown = 7,
  /// C++ enum variant: <span style='color: green;'>```MoveNext = 8```</span>
  Next = 8,
  /// C++ enum variant: <span style='color: green;'>```MovePrevious = 9```</span>
  Previous = 9,
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::DragDropMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DragDropMode {
  /// C++ enum variant: <span style='color: green;'>```NoDragDrop = 0```</span>
  NoDragDrop = 0,
  /// C++ enum variant: <span style='color: green;'>```DragOnly = 1```</span>
  DragOnly = 1,
  /// C++ enum variant: <span style='color: green;'>```DropOnly = 2```</span>
  DropOnly = 2,
  /// C++ enum variant: <span style='color: green;'>```DragDrop = 3```</span>
  DragDrop = 3,
  /// C++ enum variant: <span style='color: green;'>```InternalMove = 4```</span>
  InternalMove = 4,
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::DropIndicatorPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DropIndicatorPosition {
  /// C++ enum variant: <span style='color: green;'>```OnItem = 0```</span>
  OnItem = 0,
  /// C++ enum variant: <span style='color: green;'>```AboveItem = 1```</span>
  AboveItem = 1,
  /// C++ enum variant: <span style='color: green;'>```BelowItem = 2```</span>
  BelowItem = 2,
  /// C++ enum variant: <span style='color: green;'>```OnViewport = 3```</span>
  OnViewport = 3,
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::EditTrigger```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum EditTrigger {
  /// C++ enum variant: <span style='color: green;'>```NoEditTriggers = 0```</span>
  NoEditTriggers = 0,
  /// C++ enum variant: <span style='color: green;'>```CurrentChanged = 1```</span>
  CurrentChanged = 1,
  /// C++ enum variant: <span style='color: green;'>```DoubleClicked = 2```</span>
  DoubleClicked = 2,
  /// C++ enum variant: <span style='color: green;'>```SelectedClicked = 4```</span>
  SelectedClicked = 4,
  /// C++ enum variant: <span style='color: green;'>```EditKeyPressed = 8```</span>
  EditKeyPressed = 8,
  /// C++ enum variant: <span style='color: green;'>```AnyKeyPressed = 16```</span>
  AnyKeyPressed = 16,
  /// C++ enum variant: <span style='color: green;'>```AllEditTriggers = 31```</span>
  AllEditTriggers = 31,
}

impl ::qt_core::flags::FlaggableEnum for EditTrigger {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "EditTrigger"
  }
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::ScrollHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScrollHint {
  /// C++ enum variant: <span style='color: green;'>```EnsureVisible = 0```</span>
  EnsureVisible = 0,
  /// C++ enum variant: <span style='color: green;'>```PositionAtTop = 1```</span>
  PositionAtTop = 1,
  /// C++ enum variant: <span style='color: green;'>```PositionAtBottom = 2```</span>
  PositionAtBottom = 2,
  /// C++ enum variant: <span style='color: green;'>```PositionAtCenter = 3```</span>
  PositionAtCenter = 3,
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::ScrollMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScrollMode {
  /// C++ enum variant: <span style='color: green;'>```ScrollPerItem = 0```</span>
  Item = 0,
  /// C++ enum variant: <span style='color: green;'>```ScrollPerPixel = 1```</span>
  Pixel = 1,
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::SelectionBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectionBehavior {
  /// C++ enum variant: <span style='color: green;'>```SelectItems = 0```</span>
  Items = 0,
  /// C++ enum variant: <span style='color: green;'>```SelectRows = 1```</span>
  Rows = 1,
  /// C++ enum variant: <span style='color: green;'>```SelectColumns = 2```</span>
  Columns = 2,
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::SelectionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectionMode {
  /// C++ enum variant: <span style='color: green;'>```NoSelection = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```SingleSelection = 1```</span>
  Single = 1,
  /// C++ enum variant: <span style='color: green;'>```MultiSelection = 2```</span>
  Multi = 2,
  /// C++ enum variant: <span style='color: green;'>```ExtendedSelection = 3```</span>
  Extended = 3,
  /// C++ enum variant: <span style='color: green;'>```ContiguousSelection = 4```</span>
  Contiguous = 4,
}

/// C++ type: <span style='color: green;'>```QAbstractItemView::State```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum State {
  /// C++ enum variant: <span style='color: green;'>```NoState = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```DraggingState = 1```</span>
  Dragging = 1,
  /// C++ enum variant: <span style='color: green;'>```DragSelectingState = 2```</span>
  DragSelecting = 2,
  /// C++ enum variant: <span style='color: green;'>```EditingState = 3```</span>
  Editing = 3,
  /// C++ enum variant: <span style='color: green;'>```ExpandingState = 4```</span>
  Expanding = 4,
  /// C++ enum variant: <span style='color: green;'>```CollapsingState = 5```</span>
  Collapsing = 5,
  /// C++ enum variant: <span style='color: green;'>```AnimatingState = 6```</span>
  Animating = 6,
}

impl ::cpp_utils::DynamicCast<::abstract_item_view::AbstractItemView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_item_view::AbstractItemView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_item_view::AbstractItemView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::abstract_item_view::AbstractItemView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_item_view::AbstractItemView> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QFrame(self as *mut ::frame::Frame)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_item_view::AbstractItemView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::abstract_item_view::AbstractItemView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_item_view::AbstractItemView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_item_view::AbstractItemView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_item_view::AbstractItemView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QObject_ptr(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QObject_ptr(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::abstract_item_view::AbstractItemView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QPaintDevice_ptr(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QPaintDevice_ptr(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::abstract_item_view::AbstractItemView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::abstract_item_view::AbstractItemView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QFrame_ptr(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QFrame_ptr(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::abstract_item_view::AbstractItemView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QWidget_ptr(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QWidget_ptr(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_item_view::AbstractItemView> for ::abstract_scroll_area::AbstractScrollArea {
unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::abstract_item_view::AbstractItemView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_item_view::AbstractItemView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_item_view::AbstractItemView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_item_view::AbstractItemView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_item_view::AbstractItemView {
  type Target = ::abstract_scroll_area::AbstractScrollArea;
  fn deref(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_item_view::AbstractItemView {
  fn deref_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractItemView::item_delegate](../struct.AbstractItemView.html#method.item_delegate) method.
  pub trait AbstractItemViewItemDelegateArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::abstract_item_view::AbstractItemView)
            -> *mut ::abstract_item_delegate::AbstractItemDelegate;
  }
  impl<'largs> AbstractItemViewItemDelegateArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self,
            original_self: &'largs ::abstract_item_view::AbstractItemView)
            -> *mut ::abstract_item_delegate::AbstractItemDelegate {
      let index = self;
      unsafe { ::ffi::qt_widgets_c_QAbstractItemView_itemDelegate_index(original_self as *const ::abstract_item_view::AbstractItemView, index as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  impl<'largs> AbstractItemViewItemDelegateArgs<'largs> for () {
    fn exec(self,
            original_self: &'largs ::abstract_item_view::AbstractItemView)
            -> *mut ::abstract_item_delegate::AbstractItemDelegate {

      unsafe { ::ffi::qt_widgets_c_QAbstractItemView_itemDelegate_no_args(original_self as *const ::abstract_item_view::AbstractItemView) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemView::scroll_to](../struct.AbstractItemView.html#method.scroll_to) method.
  pub trait AbstractItemViewScrollToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_view::AbstractItemView) -> ();
  }
  impl<'largs> AbstractItemViewScrollToArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::abstract_item_view::AbstractItemView) -> () {
      let index = self;
      unsafe { ::ffi::qt_widgets_c_QAbstractItemView_scrollTo_index(original_self as *mut ::abstract_item_view::AbstractItemView, index as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  impl<'largs> AbstractItemViewScrollToArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, ::abstract_item_view::ScrollHint) {
    fn exec(self, original_self: &'largs mut ::abstract_item_view::AbstractItemView) -> () {
      let index = self.0;
      let hint = self.1;
      unsafe { ::ffi::qt_widgets_c_QAbstractItemView_scrollTo_index_hint(original_self as *mut ::abstract_item_view::AbstractItemView, index as *const ::qt_core::model_index::ModelIndex, hint) }
    }
  }
}
