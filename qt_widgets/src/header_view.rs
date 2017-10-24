/// C++ type: <span style='color: green;'>```QHeaderView```</span>
#[repr(C)]
pub struct HeaderView(u8);

impl HeaderView {
  /// C++ method: <span style='color: green;'>```bool QHeaderView::cascadingSectionResizes() const```</span>
  ///
  ///
  pub fn cascading_section_resizes(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_cascadingSectionResizes(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_count(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::defaultSectionSize() const```</span>
  ///
  ///
  pub fn default_section_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_defaultSectionSize(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QHeaderView::doItemsLayout()```</span>
  ///
  ///
  pub fn do_items_layout(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_doItemsLayout(self as *mut ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QHeaderView::headerDataChanged(Qt::Orientation orientation, int logicalFirst, int logicalLast)```</span>
  ///
  ///
  pub fn header_data_changed(&mut self,
                             orientation: &::qt_core::qt::Orientation,
                             logical_first: ::libc::c_int,
                             logical_last: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_headerDataChanged(self as *mut ::header_view::HeaderView,
                                                        orientation as *const ::qt_core::qt::Orientation,
                                                        logical_first,
                                                        logical_last)
    }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::hiddenSectionCount() const```</span>
  ///
  ///
  pub fn hidden_section_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_hiddenSectionCount(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::hideSection(int logicalIndex)```</span>
  ///
  ///
  pub fn hide_section(&mut self, logical_index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_hideSection(self as *mut ::header_view::HeaderView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::highlightSections() const```</span>
  ///
  ///
  pub fn highlight_sections(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_highlightSections(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::isSectionHidden(int logicalIndex) const```</span>
  ///
  ///
  pub fn is_section_hidden(&self, logical_index: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_isSectionHidden(self as *const ::header_view::HeaderView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::isSortIndicatorShown() const```</span>
  ///
  ///
  pub fn is_sort_indicator_shown(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_isSortIndicatorShown(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_length(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::logicalIndex(int visualIndex) const```</span>
  ///
  ///
  pub fn logical_index(&self, visual_index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_logicalIndex(self as *const ::header_view::HeaderView, visual_index) }
  }

  /// C++ method: <span style='color: green;'>```QHeaderView::logicalIndexAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn logical_index_at(&self, &::qt_core::point::Point) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHeaderView::logicalIndexAt(const QPoint& pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn logical_index_at(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHeaderView::logicalIndexAt(int position) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn logical_index_at(&self, (::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHeaderView::logicalIndexAt(int x, int y) const```</span>
  ///
  ///
  pub fn logical_index_at<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::HeaderViewLogicalIndexAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QHeaderView::maximumSectionSize() const```</span>
  ///
  ///
  pub fn maximum_section_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_maximumSectionSize(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QHeaderView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_metaObject(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::minimumSectionSize() const```</span>
  ///
  ///
  pub fn minimum_section_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_minimumSectionSize(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::moveSection(int from, int to)```</span>
  ///
  ///
  pub fn move_section(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_moveSection(self as *mut ::header_view::HeaderView, from, to) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QHeaderView::QHeaderView(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn new(orientation: &::qt_core::qt::Orientation) -> ::cpp_utils::CppBox<::header_view::HeaderView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHeaderView_new_orientation(orientation as *const ::qt_core::qt::Orientation) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QHeaderView::QHeaderView(Qt::Orientation orientation, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(orientation: &::qt_core::qt::Orientation,
                           parent: *mut ::widget::Widget)
                           -> ::cpp_utils::CppBox<::header_view::HeaderView> {
    let ffi_result =
      ::ffi::qt_widgets_c_QHeaderView_new_orientation_parent(orientation as *const ::qt_core::qt::Orientation, parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::offset() const```</span>
  ///
  ///
  pub fn offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_offset(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QHeaderView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QHeaderView_qt_metacall(self as *mut ::header_view::HeaderView,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QHeaderView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QHeaderView_qt_metacast(self as *mut ::header_view::HeaderView, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual void QHeaderView::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_reset(self as *mut ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::resetDefaultSectionSize()```</span>
  ///
  ///
  pub fn reset_default_section_size(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_resetDefaultSectionSize(self as *mut ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::resizeContentsPrecision() const```</span>
  ///
  ///
  pub fn resize_contents_precision(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_resizeContentsPrecision(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::resizeSection(int logicalIndex, int size)```</span>
  ///
  ///
  pub fn resize_section(&mut self, logical_index: ::libc::c_int, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_resizeSection(self as *mut ::header_view::HeaderView, logical_index, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::resizeSections(QHeaderView::ResizeMode mode)```</span>
  ///
  ///
  pub fn resize_sections(&mut self, mode: &::header_view::ResizeMode) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_resizeSections(self as *mut ::header_view::HeaderView,
                                                     mode as *const ::header_view::ResizeMode)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::restoreState(const QByteArray& state)```</span>
  ///
  ///
  pub fn restore_state(&mut self, state: &::qt_core::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_restoreState(self as *mut ::header_view::HeaderView,
                                                   state as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QHeaderView::saveState() const```</span>
  ///
  ///
  pub fn save_state(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_saveState_to_output(self as *const ::header_view::HeaderView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::sectionPosition(int logicalIndex) const```</span>
  ///
  ///
  pub fn section_position(&self, logical_index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sectionPosition(self as *const ::header_view::HeaderView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```QHeaderView::ResizeMode QHeaderView::sectionResizeMode(int logicalIndex) const```</span>
  ///
  ///
  pub fn section_resize_mode(&self, logical_index: ::libc::c_int) -> ::header_view::ResizeMode {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_sectionResizeMode(self as *const ::header_view::HeaderView, logical_index)
    }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::sectionSize(int logicalIndex) const```</span>
  ///
  ///
  pub fn section_size(&self, logical_index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sectionSize(self as *const ::header_view::HeaderView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::sectionSizeHint(int logicalIndex) const```</span>
  ///
  ///
  pub fn section_size_hint(&self, logical_index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sectionSizeHint(self as *const ::header_view::HeaderView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::sectionViewportPosition(int logicalIndex) const```</span>
  ///
  ///
  pub fn section_viewport_position(&self, logical_index: ::libc::c_int) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_sectionViewportPosition(self as *const ::header_view::HeaderView, logical_index)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::sectionsClickable() const```</span>
  ///
  ///
  pub fn sections_clickable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sectionsClickable(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::sectionsHidden() const```</span>
  ///
  ///
  pub fn sections_hidden(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sectionsHidden(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::sectionsMovable() const```</span>
  ///
  ///
  pub fn sections_movable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sectionsMovable(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::sectionsMoved() const```</span>
  ///
  ///
  pub fn sections_moved(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sectionsMoved(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setCascadingSectionResizes(bool enable)```</span>
  ///
  ///
  pub fn set_cascading_section_resizes(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_setCascadingSectionResizes(self as *mut ::header_view::HeaderView, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setDefaultSectionSize(int size)```</span>
  ///
  ///
  pub fn set_default_section_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setDefaultSectionSize(self as *mut ::header_view::HeaderView, size) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setHighlightSections(bool highlight)```</span>
  ///
  ///
  pub fn set_highlight_sections(&mut self, highlight: bool) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setHighlightSections(self as *mut ::header_view::HeaderView, highlight) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setMaximumSectionSize(int size)```</span>
  ///
  ///
  pub fn set_maximum_section_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setMaximumSectionSize(self as *mut ::header_view::HeaderView, size) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setMinimumSectionSize(int size)```</span>
  ///
  ///
  pub fn set_minimum_section_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setMinimumSectionSize(self as *mut ::header_view::HeaderView, size) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QHeaderView::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QHeaderView_setModel(self as *mut ::header_view::HeaderView, model)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QHeaderView::setOffset(int offset)```</span>
  ///
  ///
  pub fn set_offset(&mut self, offset: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setOffset(self as *mut ::header_view::HeaderView, offset) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QHeaderView::setOffsetToLastSection()```</span>
  ///
  ///
  pub fn set_offset_to_last_section(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setOffsetToLastSection(self as *mut ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QHeaderView::setOffsetToSectionPosition(int visualIndex)```</span>
  ///
  ///
  pub fn set_offset_to_section_position(&mut self, visual_index: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_setOffsetToSectionPosition(self as *mut ::header_view::HeaderView, visual_index)
    }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setResizeContentsPrecision(int precision)```</span>
  ///
  ///
  pub fn set_resize_contents_precision(&mut self, precision: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_setResizeContentsPrecision(self as *mut ::header_view::HeaderView, precision)
    }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setSectionHidden(int logicalIndex, bool hide)```</span>
  ///
  ///
  pub fn set_section_hidden(&mut self, logical_index: ::libc::c_int, hide: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_setSectionHidden(self as *mut ::header_view::HeaderView, logical_index, hide)
    }
  }

  /// C++ method: <span style='color: green;'>```QHeaderView::setSectionResizeMode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_section_resize_mode(&mut self, ::header_view::ResizeMode) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QHeaderView::setSectionResizeMode(QHeaderView::ResizeMode mode)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_section_resize_mode(&mut self, (::libc::c_int, ::header_view::ResizeMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QHeaderView::setSectionResizeMode(int logicalIndex, QHeaderView::ResizeMode mode)```</span>
  ///
  ///
  pub fn set_section_resize_mode<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::HeaderViewSetSectionResizeModeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QHeaderView::setSectionsClickable(bool clickable)```</span>
  ///
  ///
  pub fn set_sections_clickable(&mut self, clickable: bool) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setSectionsClickable(self as *mut ::header_view::HeaderView, clickable) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setSectionsMovable(bool movable)```</span>
  ///
  ///
  pub fn set_sections_movable(&mut self, movable: bool) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setSectionsMovable(self as *mut ::header_view::HeaderView, movable) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setSortIndicator(int logicalIndex, Qt::SortOrder order)```</span>
  ///
  ///
  pub fn set_sort_indicator(&mut self, logical_index: ::libc::c_int, order: &::qt_core::qt::SortOrder) {
    unsafe {
      ::ffi::qt_widgets_c_QHeaderView_setSortIndicator(self as *mut ::header_view::HeaderView,
                                                       logical_index,
                                                       order as *const ::qt_core::qt::SortOrder)
    }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setSortIndicatorShown(bool show)```</span>
  ///
  ///
  pub fn set_sort_indicator_shown(&mut self, show: bool) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setSortIndicatorShown(self as *mut ::header_view::HeaderView, show) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::setStretchLastSection(bool stretch)```</span>
  ///
  ///
  pub fn set_stretch_last_section(&mut self, stretch: bool) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setStretchLastSection(self as *mut ::header_view::HeaderView, stretch) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QHeaderView::setVisible(bool v)```</span>
  ///
  ///
  pub fn set_visible(&mut self, v: bool) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_setVisible(self as *mut ::header_view::HeaderView, v) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::showSection(int logicalIndex)```</span>
  ///
  ///
  pub fn show_section(&mut self, logical_index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_showSection(self as *mut ::header_view::HeaderView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QHeaderView::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_sizeHint_to_output(self as *const ::header_view::HeaderView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::sortIndicatorSection() const```</span>
  ///
  ///
  pub fn sort_indicator_section(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_sortIndicatorSection(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```bool QHeaderView::stretchLastSection() const```</span>
  ///
  ///
  pub fn stretch_last_section(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_stretchLastSection(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::stretchSectionCount() const```</span>
  ///
  ///
  pub fn stretch_section_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_stretchSectionCount(self as *const ::header_view::HeaderView) }
  }

  /// C++ method: <span style='color: green;'>```void QHeaderView::swapSections(int first, int second)```</span>
  ///
  ///
  pub fn swap_sections(&mut self, first: ::libc::c_int, second: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_swapSections(self as *mut ::header_view::HeaderView, first, second) }
  }

  /// C++ method: <span style='color: green;'>```static QString QHeaderView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QHeaderView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QHeaderView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QHeaderView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::visualIndex(int logicalIndex) const```</span>
  ///
  ///
  pub fn visual_index(&self, logical_index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_visualIndex(self as *const ::header_view::HeaderView, logical_index) }
  }

  /// C++ method: <span style='color: green;'>```int QHeaderView::visualIndexAt(int position) const```</span>
  ///
  ///
  pub fn visual_index_at(&self, position: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHeaderView_visualIndexAt(self as *const ::header_view::HeaderView, position) }
  }
}

impl ::cpp_utils::CppDeletable for ::header_view::HeaderView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QHeaderView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `HeaderView`.
  pub struct Signals<'a>(&'a ::header_view::HeaderView);
  /// Represents a built-in Qt signal `QHeaderView::sectionDoubleClicked`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionDoubleClicked<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionDoubleClicked<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionDoubleClicked(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionDoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::pressed`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct Pressed<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt signal `QHeaderView::viewportEntered`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().viewport_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct ViewportEntered<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt signal `QHeaderView::sectionCountChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionCountChanged<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionCountChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionCountChanged(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionCountChanged<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::sectionPressed`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionPressed<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionPressed<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionPressed(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionPressed<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::geometriesChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().geometries_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct GeometriesChanged<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for GeometriesChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2geometriesChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GeometriesChanged<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::entered`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct Entered<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt signal `QHeaderView::activated`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct Activated<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt signal `QHeaderView::clicked`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct Clicked<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt signal `QHeaderView::doubleClicked`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct DoubleClicked<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt signal `QHeaderView::sectionMoved`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionMoved<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionMoved<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionMoved(int,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionMoved<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::sectionHandleDoubleClicked`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_handle_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionHandleDoubleClicked<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionHandleDoubleClicked<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionHandleDoubleClicked(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionHandleDoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::sectionEntered`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionEntered<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionEntered<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionEntered(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionEntered<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::sortIndicatorChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().sort_indicator_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SortIndicatorChanged<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SortIndicatorChanged<'a> {
    type Arguments = (::libc::c_int, &'static ::qt_core::qt::SortOrder);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sortIndicatorChanged(int,Qt::SortOrder)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SortIndicatorChanged<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::sectionResized`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionResized<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionResized<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionResized(int,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionResized<'a> {}
  /// Represents a built-in Qt signal `QHeaderView::iconSizeChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct IconSizeChanged<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt signal `QHeaderView::sectionClicked`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.signals().section_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionClicked<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionClicked<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sectionClicked(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SectionClicked<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_double_clicked(&self) -> SectionDoubleClicked {
      SectionDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::viewportEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn viewport_entered(&self) -> ViewportEntered {
      ViewportEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_count_changed(&self) -> SectionCountChanged {
      SectionCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_pressed(&self) -> SectionPressed {
      SectionPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::geometriesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometries_changed(&self) -> GeometriesChanged {
      GeometriesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::doubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_clicked(&self) -> DoubleClicked {
      DoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_moved(&self) -> SectionMoved {
      SectionMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionHandleDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_handle_double_clicked(&self) -> SectionHandleDoubleClicked {
      SectionHandleDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_entered(&self) -> SectionEntered {
      SectionEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sortIndicatorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sort_indicator_changed(&self) -> SortIndicatorChanged {
      SortIndicatorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_resized(&self) -> SectionResized {
      SectionResized(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHeaderView::sectionClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn section_clicked(&self) -> SectionClicked {
      SectionClicked(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `HeaderView`.
  pub struct Slots<'a>(&'a ::header_view::HeaderView);
  /// Represents a built-in Qt slot `QHeaderView::verticalScrollbarAction`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().vertical_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct VerticalScrollbarAction<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::updateEditorGeometries`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().update_editor_geometries()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct UpdateEditorGeometries<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorGeometries<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorGeometries()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::setRootIndex`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().set_root_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SetRootIndex<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SetRootIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRootIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::edit`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().edit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct Edit<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for Edit<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1edit(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::scrollToBottom`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().scroll_to_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct ScrollToBottom<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToBottom<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToBottom()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for RowsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::selectAll`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SelectAll<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::sectionsInserted`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().sections_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionsInserted<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionsInserted<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1sectionsInserted(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::setOffsetToSectionPosition`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().set_offset_to_section_position()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SetOffsetToSectionPosition<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SetOffsetToSectionPosition<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOffsetToSectionPosition(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::scrollToTop`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().scroll_to_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct ScrollToTop<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for ScrollToTop<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToTop()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::updateSection`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().update_section()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct UpdateSection<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for UpdateSection<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateSection(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::resizeSections`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().resize_sections()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct ResizeSections<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for ResizeSections<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeSections()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::horizontalScrollbarAction`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().horizontal_scrollbar_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct HorizontalScrollbarAction<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarAction<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarAction(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::closeEditor`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().close_editor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct CloseEditor<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for CloseEditor<'a> {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeEditor(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::verticalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().vertical_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct VerticalScrollbarValueChanged<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for VerticalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1verticalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::updateEditorData`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().update_editor_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct UpdateEditorData<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for UpdateEditorData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateEditorData()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::clearSelection`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct ClearSelection<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::selectionChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SelectionChanged<'a>(&'a ::header_view::HeaderView);
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
  /// Represents a built-in Qt slot `QHeaderView::horizontalScrollbarValueChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().horizontal_scrollbar_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct HorizontalScrollbarValueChanged<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrollbarValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1horizontalScrollbarValueChanged(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::commitData`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().commit_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct CommitData<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for CommitData<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1commitData(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::editorDestroyed`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().editor_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct EditorDestroyed<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for EditorDestroyed<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1editorDestroyed(QObject*)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::setOffsetToLastSection`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().set_offset_to_last_section()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SetOffsetToLastSection<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SetOffsetToLastSection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOffsetToLastSection()\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::setCurrentIndex`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SetCurrentIndex<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::sectionsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().sections_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SectionsAboutToBeRemoved<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SectionsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1sectionsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::headerDataChanged`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().header_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct HeaderDataChanged<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for HeaderDataChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1headerDataChanged(Qt::Orientation,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::setOffset`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().set_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct SetOffset<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for SetOffset<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOffset(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QHeaderView::update`.
  ///
  /// An object of this type can be created from `HeaderView` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HeaderView` object.
  pub struct Update<'a>(&'a ::header_view::HeaderView);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update(const QModelIndex&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QHeaderView::verticalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_action(&self) -> VerticalScrollbarAction {
      VerticalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::updateEditorGeometries`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_geometries(&self) -> UpdateEditorGeometries {
      UpdateEditorGeometries(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::setRootIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_root_index(&self) -> SetRootIndex {
      SetRootIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::edit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit(&self) -> Edit {
      Edit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::scrollToBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_bottom(&self) -> ScrollToBottom {
      ScrollToBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::sectionsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sections_inserted(&self) -> SectionsInserted {
      SectionsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::setOffsetToSectionPosition`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_offset_to_section_position(&self) -> SetOffsetToSectionPosition {
      SetOffsetToSectionPosition(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::scrollToTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_top(&self) -> ScrollToTop {
      ScrollToTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::updateSection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_section(&self) -> UpdateSection {
      UpdateSection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::resizeSections`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_sections(&self) -> ResizeSections {
      ResizeSections(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::horizontalScrollbarAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_action(&self) -> HorizontalScrollbarAction {
      HorizontalScrollbarAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::closeEditor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_editor(&self) -> CloseEditor {
      CloseEditor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::verticalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertical_scrollbar_value_changed(&self) -> VerticalScrollbarValueChanged {
      VerticalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::updateEditorData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_editor_data(&self) -> UpdateEditorData {
      UpdateEditorData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::horizontalScrollbarValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn horizontal_scrollbar_value_changed(&self) -> HorizontalScrollbarValueChanged {
      HorizontalScrollbarValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::commitData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data(&self) -> CommitData {
      CommitData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::editorDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editor_destroyed(&self) -> EditorDestroyed {
      EditorDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::setOffsetToLastSection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_offset_to_last_section(&self) -> SetOffsetToLastSection {
      SetOffsetToLastSection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::sectionsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sections_about_to_be_removed(&self) -> SectionsAboutToBeRemoved {
      SectionsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::headerDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn header_data_changed(&self) -> HeaderDataChanged {
      HeaderDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::setOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_offset(&self) -> SetOffset {
      SetOffset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QHeaderView::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::header_view::HeaderView {
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

/// C++ type: <span style='color: green;'>```QHeaderView::ResizeMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ResizeMode {
  /// C++ enum variant: <span style='color: green;'>```Interactive = 0```</span>
  Interactive = 0,
  /// C++ enum variant: <span style='color: green;'>```Stretch = 1```</span>
  Stretch = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Fixed = 2```</span>
  /// - <span style='color: green;'>```Custom = 2```</span>
  ///
  Fixed = 2,
  /// C++ enum variant: <span style='color: green;'>```ResizeToContents = 3```</span>
  ResizeToContents = 3,
}

impl ::cpp_utils::DynamicCast<::header_view::HeaderView> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::header_view::HeaderView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::header_view::HeaderView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::header_view::HeaderView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::header_view::HeaderView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::header_view::HeaderView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::header_view::HeaderView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::header_view::HeaderView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::header_view::HeaderView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::header_view::HeaderView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::header_view::HeaderView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::header_view::HeaderView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::header_view::HeaderView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QObject_ptr(self as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QObject_ptr(self as *const ::header_view::HeaderView as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::header_view::HeaderView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QPaintDevice_ptr(self as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QPaintDevice_ptr(self as *const ::header_view::HeaderView as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::header_view::HeaderView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QAbstractItemView_ptr(self as *mut ::header_view::HeaderView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QAbstractItemView_ptr(self as *const ::header_view::HeaderView as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::header_view::HeaderView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::header_view::HeaderView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::header_view::HeaderView as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::header_view::HeaderView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QFrame_ptr(self as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QFrame_ptr(self as *const ::header_view::HeaderView as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::header_view::HeaderView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QWidget_ptr(self as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QWidget_ptr(self as *const ::header_view::HeaderView as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::header_view::HeaderView> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::header_view::HeaderView> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::header_view::HeaderView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::header_view::HeaderView {
    let ffi_result =
      ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::header_view::HeaderView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::header_view::HeaderView {
    let ffi_result =
      ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::header_view::HeaderView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::header_view::HeaderView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::header_view::HeaderView {
    let ffi_result =
      ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::header_view::HeaderView {
    let ffi_result = ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::header_view::HeaderView {
  type Target = ::abstract_item_view::AbstractItemView;
  fn deref(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QAbstractItemView_ptr(self as *const ::header_view::HeaderView as *mut ::header_view::HeaderView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::header_view::HeaderView {
  fn deref_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_G_static_cast_QAbstractItemView_ptr(self as *mut ::header_view::HeaderView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [HeaderView::logical_index_at](../struct.HeaderView.html#method.logical_index_at) method.
  pub trait HeaderViewLogicalIndexAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::header_view::HeaderView) -> ::libc::c_int;
  }
  impl<'largs> HeaderViewLogicalIndexAtArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::header_view::HeaderView) -> ::libc::c_int {
      let pos = self;
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_logicalIndexAt_pos(original_self as *const ::header_view::HeaderView,
                                                           pos as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> HeaderViewLogicalIndexAtArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::header_view::HeaderView) -> ::libc::c_int {
      let position = self;
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_logicalIndexAt_position(original_self as *const ::header_view::HeaderView,
                                                                position)
      }
    }
  }
  impl<'largs> HeaderViewLogicalIndexAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::header_view::HeaderView) -> ::libc::c_int {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_logicalIndexAt_x_y(original_self as *const ::header_view::HeaderView, x, y)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HeaderView::set_section_resize_mode](../struct.HeaderView.html#method.set_section_resize_mode) method.
  pub trait HeaderViewSetSectionResizeModeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::header_view::HeaderView) -> ();
  }
  impl<'largs> HeaderViewSetSectionResizeModeArgs<'largs> for (::libc::c_int, ::header_view::ResizeMode) {
    fn exec(self, original_self: &'largs mut ::header_view::HeaderView) -> () {
      let logical_index = self.0;
      let mode = self.1;
      unsafe { ::ffi::qt_widgets_c_QHeaderView_setSectionResizeMode_logicalIndex_mode(original_self as *mut ::header_view::HeaderView, logical_index, mode) }
    }
  }
  impl<'largs> HeaderViewSetSectionResizeModeArgs<'largs> for ::header_view::ResizeMode {
    fn exec(self, original_self: &'largs mut ::header_view::HeaderView) -> () {
      let mode = self;
      unsafe {
        ::ffi::qt_widgets_c_QHeaderView_setSectionResizeMode_mode(original_self as *mut ::header_view::HeaderView, mode)
      }
    }
  }
}
