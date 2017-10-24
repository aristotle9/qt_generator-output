/// C++ type: <span style='color: green;'>```QComboBox```</span>
#[repr(C)]
pub struct ComboBox(u8);

impl ComboBox {
  /// C++ method: <span style='color: green;'>```QComboBox::addItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_item(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::addItem(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_item(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String, &::qt_core::variant::Variant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::addItem(const QIcon& icon, const QString& text, const QVariant& userData = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_item(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::addItem(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_item(&mut self, (&::qt_core::string::String, &::qt_core::variant::Variant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::addItem(const QString& text, const QVariant& userData = ?)```</span>
  ///
  ///
  pub fn add_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ComboBoxAddItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QComboBox::addItems(const QStringList& texts)```</span>
  ///
  ///
  pub fn add_items(&mut self, texts: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_addItems(self as *mut ::combo_box::ComboBox,
                                             texts as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QComboBox::autoCompletion() const```</span>
  ///
  ///
  pub fn auto_completion(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QComboBox_autoCompletion(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QComboBox::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_clear(self as *mut ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QComboBox::clearEditText()```</span>
  ///
  ///
  pub fn clear_edit_text(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_clearEditText(self as *mut ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QCompleter* QComboBox::completer() const```</span>
  ///
  ///
  pub fn completer(&self) -> *mut ::completer::Completer {
    unsafe { ::ffi::qt_widgets_c_QComboBox_completer(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```int QComboBox::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QComboBox_count(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QComboBox::currentData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn current_data(&self, ()) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QComboBox::currentData() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn current_data(&self, ::libc::c_int) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QComboBox::currentData(int role = ?) const```</span>
  ///
  ///
  pub fn current_data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::ComboBoxCurrentDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QComboBox::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QComboBox_currentIndex(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QString QComboBox::currentText() const```</span>
  ///
  ///
  pub fn current_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_currentText_to_output(self as *const ::combo_box::ComboBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QComboBox::duplicatesEnabled() const```</span>
  ///
  ///
  pub fn duplicates_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QComboBox_duplicatesEnabled(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QComboBox::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_widgets_c_QComboBox_event(self as *mut ::combo_box::ComboBox, event)
  }

  /// C++ method: <span style='color: green;'>```bool QComboBox::hasFrame() const```</span>
  ///
  ///
  pub fn has_frame(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QComboBox_hasFrame(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QComboBox::hidePopup()```</span>
  ///
  ///
  pub fn hide_popup(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_hidePopup(self as *mut ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QSize QComboBox::iconSize() const```</span>
  ///
  ///
  pub fn icon_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_iconSize_to_output(self as *const ::combo_box::ComboBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QComboBox::inputMethodQuery```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn input_method_query(&self, (&::qt_core::qt::InputMethodQuery, &::qt_core::variant::Variant)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QComboBox::inputMethodQuery(Qt::InputMethodQuery query, const QVariant& argument) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn input_method_query(&self, &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QComboBox::inputMethodQuery(Qt::InputMethodQuery arg1) const```</span>
  ///
  ///
  pub fn input_method_query<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::ComboBoxInputMethodQueryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QComboBox::insertItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_item(&mut self, (::libc::c_int, &::qt_gui::icon::Icon, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::insertItem(int index, const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_item(&mut self, (::libc::c_int, &::qt_gui::icon::Icon, &::qt_core::string::String, &::qt_core::variant::Variant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::insertItem(int index, const QIcon& icon, const QString& text, const QVariant& userData = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert_item(&mut self, (::libc::c_int, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::insertItem(int index, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn insert_item(&mut self, (::libc::c_int, &::qt_core::string::String, &::qt_core::variant::Variant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::insertItem(int index, const QString& text, const QVariant& userData = ?)```</span>
  ///
  ///
  pub fn insert_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ComboBoxInsertItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QComboBox::insertItems(int index, const QStringList& texts)```</span>
  ///
  ///
  pub fn insert_items(&mut self, index: ::libc::c_int, texts: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_insertItems(self as *mut ::combo_box::ComboBox,
                                                index,
                                                texts as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```QComboBox::InsertPolicy QComboBox::insertPolicy() const```</span>
  ///
  ///
  pub fn insert_policy(&self) -> ::combo_box::InsertPolicy {
    unsafe { ::ffi::qt_widgets_c_QComboBox_insertPolicy(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::insertSeparator(int index)```</span>
  ///
  ///
  pub fn insert_separator(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_insertSeparator(self as *mut ::combo_box::ComboBox, index) }
  }

  /// C++ method: <span style='color: green;'>```bool QComboBox::isEditable() const```</span>
  ///
  ///
  pub fn is_editable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QComboBox_isEditable(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QComboBox::itemData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_data(&self, ::libc::c_int) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QComboBox::itemData(int index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_data(&self, (::libc::c_int, ::libc::c_int)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QComboBox::itemData(int index, int role = ?) const```</span>
  ///
  ///
  pub fn item_data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::ComboBoxItemDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemDelegate* QComboBox::itemDelegate() const```</span>
  ///
  ///
  pub fn item_delegate(&self) -> *mut ::abstract_item_delegate::AbstractItemDelegate {
    unsafe { ::ffi::qt_widgets_c_QComboBox_itemDelegate(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QComboBox::itemIcon(int index) const```</span>
  ///
  ///
  pub fn item_icon(&self, index: ::libc::c_int) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_itemIcon_to_output(self as *const ::combo_box::ComboBox, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QComboBox::itemText(int index) const```</span>
  ///
  ///
  pub fn item_text(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_itemText_to_output(self as *const ::combo_box::ComboBox, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineEdit* QComboBox::lineEdit() const```</span>
  ///
  ///
  pub fn line_edit(&self) -> *mut ::line_edit::LineEdit {
    unsafe { ::ffi::qt_widgets_c_QComboBox_lineEdit(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```int QComboBox::maxCount() const```</span>
  ///
  ///
  pub fn max_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QComboBox_maxCount(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```int QComboBox::maxVisibleItems() const```</span>
  ///
  ///
  pub fn max_visible_items(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QComboBox_maxVisibleItems(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QComboBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QComboBox_metaObject(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```int QComboBox::minimumContentsLength() const```</span>
  ///
  ///
  pub fn minimum_contents_length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QComboBox_minimumContentsLength(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QComboBox::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_minimumSizeHint_to_output(self as *const ::combo_box::ComboBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel* QComboBox::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *mut ::qt_core::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_widgets_c_QComboBox_model(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```int QComboBox::modelColumn() const```</span>
  ///
  ///
  pub fn model_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QComboBox_modelColumn(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QComboBox::QComboBox()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::combo_box::ComboBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QComboBox_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QComboBox::QComboBox(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::combo_box::ComboBox> {
    let ffi_result = ::ffi::qt_widgets_c_QComboBox_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QComboBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QComboBox_qt_metacall(self as *mut ::combo_box::ComboBox,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QComboBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QComboBox_qt_metacast(self as *mut ::combo_box::ComboBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::removeItem(int index)```</span>
  ///
  ///
  pub fn remove_item(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_removeItem(self as *mut ::combo_box::ComboBox, index) }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QComboBox::rootModelIndex() const```</span>
  ///
  ///
  pub fn root_model_index(&self) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_rootModelIndex_to_output(self as *const ::combo_box::ComboBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setAutoCompletion(bool enable)```</span>
  ///
  ///
  pub fn set_auto_completion(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setAutoCompletion(self as *mut ::combo_box::ComboBox, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setAutoCompletionCaseSensitivity(Qt::CaseSensitivity sensitivity)```</span>
  ///
  ///
  pub fn set_auto_completion_case_sensitivity(&mut self, sensitivity: &::qt_core::qt::CaseSensitivity) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setAutoCompletionCaseSensitivity(self as *mut ::combo_box::ComboBox, sensitivity as *const ::qt_core::qt::CaseSensitivity) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setCompleter(QCompleter* c)```</span>
  ///
  ///
  pub unsafe fn set_completer(&mut self, c: *mut ::completer::Completer) {
    ::ffi::qt_widgets_c_QComboBox_setCompleter(self as *mut ::combo_box::ComboBox, c)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QComboBox::setCurrentIndex(int index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setCurrentIndex(self as *mut ::combo_box::ComboBox, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QComboBox::setCurrentText(const QString& text)```</span>
  ///
  ///
  pub fn set_current_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_setCurrentText(self as *mut ::combo_box::ComboBox,
                                                   text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setDuplicatesEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_duplicates_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setDuplicatesEnabled(self as *mut ::combo_box::ComboBox, enable) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QComboBox::setEditText(const QString& text)```</span>
  ///
  ///
  pub fn set_edit_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_setEditText(self as *mut ::combo_box::ComboBox,
                                                text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setEditable(bool editable)```</span>
  ///
  ///
  pub fn set_editable(&mut self, editable: bool) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setEditable(self as *mut ::combo_box::ComboBox, editable) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setFrame(bool arg1)```</span>
  ///
  ///
  pub fn set_frame(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setFrame(self as *mut ::combo_box::ComboBox, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setIconSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_setIconSize(self as *mut ::combo_box::ComboBox,
                                                size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setInsertPolicy(QComboBox::InsertPolicy policy)```</span>
  ///
  ///
  pub fn set_insert_policy(&mut self, policy: ::combo_box::InsertPolicy) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setInsertPolicy(self as *mut ::combo_box::ComboBox, policy) }
  }

  /// C++ method: <span style='color: green;'>```QComboBox::setItemData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_item_data(&mut self, (::libc::c_int, &::qt_core::variant::Variant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::setItemData(int index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_item_data(&mut self, (::libc::c_int, &::qt_core::variant::Variant, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QComboBox::setItemData(int index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_item_data<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ComboBoxSetItemDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QComboBox::setItemDelegate(QAbstractItemDelegate* delegate)```</span>
  ///
  ///
  pub unsafe fn set_item_delegate(&mut self, delegate: *mut ::abstract_item_delegate::AbstractItemDelegate) {
    ::ffi::qt_widgets_c_QComboBox_setItemDelegate(self as *mut ::combo_box::ComboBox, delegate)
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setItemIcon(int index, const QIcon& icon)```</span>
  ///
  ///
  pub fn set_item_icon(&mut self, index: ::libc::c_int, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_setItemIcon(self as *mut ::combo_box::ComboBox,
                                                index,
                                                icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setItemText(int index, const QString& text)```</span>
  ///
  ///
  pub fn set_item_text(&mut self, index: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_setItemText(self as *mut ::combo_box::ComboBox,
                                                index,
                                                text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setLineEdit(QLineEdit* edit)```</span>
  ///
  ///
  pub unsafe fn set_line_edit(&mut self, edit: *mut ::line_edit::LineEdit) {
    ::ffi::qt_widgets_c_QComboBox_setLineEdit(self as *mut ::combo_box::ComboBox, edit)
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setMaxCount(int max)```</span>
  ///
  ///
  pub fn set_max_count(&mut self, max: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setMaxCount(self as *mut ::combo_box::ComboBox, max) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setMaxVisibleItems(int maxItems)```</span>
  ///
  ///
  pub fn set_max_visible_items(&mut self, max_items: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setMaxVisibleItems(self as *mut ::combo_box::ComboBox, max_items) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setMinimumContentsLength(int characters)```</span>
  ///
  ///
  pub fn set_minimum_contents_length(&mut self, characters: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setMinimumContentsLength(self as *mut ::combo_box::ComboBox, characters) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QComboBox_setModel(self as *mut ::combo_box::ComboBox, model)
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setModelColumn(int visibleColumn)```</span>
  ///
  ///
  pub fn set_model_column(&mut self, visible_column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setModelColumn(self as *mut ::combo_box::ComboBox, visible_column) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setRootModelIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_root_model_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QComboBox_setRootModelIndex(self as *mut ::combo_box::ComboBox,
                                                      index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setSizeAdjustPolicy(QComboBox::SizeAdjustPolicy policy)```</span>
  ///
  ///
  pub fn set_size_adjust_policy(&mut self, policy: ::combo_box::SizeAdjustPolicy) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_setSizeAdjustPolicy(self as *mut ::combo_box::ComboBox, policy) }
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setValidator(const QValidator* v)```</span>
  ///
  ///
  pub unsafe fn set_validator(&mut self, v: *const ::qt_gui::validator::Validator) {
    ::ffi::qt_widgets_c_QComboBox_setValidator(self as *mut ::combo_box::ComboBox, v)
  }

  /// C++ method: <span style='color: green;'>```void QComboBox::setView(QAbstractItemView* itemView)```</span>
  ///
  ///
  pub unsafe fn set_view(&mut self, item_view: *mut ::abstract_item_view::AbstractItemView) {
    ::ffi::qt_widgets_c_QComboBox_setView(self as *mut ::combo_box::ComboBox, item_view)
  }

  /// C++ method: <span style='color: green;'>```virtual void QComboBox::showPopup()```</span>
  ///
  ///
  pub fn show_popup(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QComboBox_showPopup(self as *mut ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QComboBox::SizeAdjustPolicy QComboBox::sizeAdjustPolicy() const```</span>
  ///
  ///
  pub fn size_adjust_policy(&self) -> ::combo_box::SizeAdjustPolicy {
    unsafe { ::ffi::qt_widgets_c_QComboBox_sizeAdjustPolicy(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QComboBox::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_sizeHint_to_output(self as *const ::combo_box::ComboBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QComboBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QComboBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QComboBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QComboBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QValidator* QComboBox::validator() const```</span>
  ///
  ///
  pub fn validator(&self) -> *const ::qt_gui::validator::Validator {
    unsafe { ::ffi::qt_widgets_c_QComboBox_validator(self as *const ::combo_box::ComboBox) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemView* QComboBox::view() const```</span>
  ///
  ///
  pub fn view(&self) -> *mut ::abstract_item_view::AbstractItemView {
    unsafe { ::ffi::qt_widgets_c_QComboBox_view(self as *const ::combo_box::ComboBox) }
  }
}

impl ::cpp_utils::CppDeletable for ::combo_box::ComboBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QComboBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ComboBox`.
  pub struct Signals<'a>(&'a ::combo_box::ComboBox);
  /// Represents a built-in Qt signal `QComboBox::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for CustomContextMenuRequested<'a> {
    type Arguments = (&'static ::qt_core::point::Point,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2customContextMenuRequested(const QPoint&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CustomContextMenuRequested<'a> {}
  /// Represents a built-in Qt signal `QComboBox::editTextChanged`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().edit_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct EditTextChanged<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for EditTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2editTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EditTextChanged<'a> {}
  /// Represents a built-in Qt signal `QComboBox::highlighted`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().highlighted_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct HighlightedCInt<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for HighlightedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedCInt<'a> {}
  /// Represents a built-in Qt signal `QComboBox::highlighted`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().highlighted_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct HighlightedQtCoreStringRef<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for HighlightedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QComboBox::windowTitleChanged`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct WindowTitleChanged<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for WindowTitleChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowTitleChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowTitleChanged<'a> {}
  /// Represents a built-in Qt signal `QComboBox::windowIconChanged`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct WindowIconChanged<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for WindowIconChanged<'a> {
    type Arguments = (&'static ::qt_gui::icon::Icon,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconChanged(const QIcon&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconChanged<'a> {}
  /// Represents a built-in Qt signal `QComboBox::activated`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().activated_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct ActivatedCInt<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for ActivatedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActivatedCInt<'a> {}
  /// Represents a built-in Qt signal `QComboBox::activated`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().activated_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct ActivatedQtCoreStringRef<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for ActivatedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActivatedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QComboBox::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct WindowIconTextChanged<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for WindowIconTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconTextChanged<'a> {}
  /// Represents a built-in Qt signal `QComboBox::currentIndexChanged`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().current_index_changed_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct CurrentIndexChangedCInt<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for CurrentIndexChangedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentIndexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentIndexChangedCInt<'a> {}
  /// Represents a built-in Qt signal `QComboBox::currentIndexChanged`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().current_index_changed_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct CurrentIndexChangedQtCoreStringRef<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for CurrentIndexChangedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentIndexChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentIndexChangedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QComboBox::currentTextChanged`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.signals().current_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct CurrentTextChanged<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for CurrentTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentTextChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QComboBox::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::editTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit_text_changed(&self) -> EditTextChanged {
      EditTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_c_int(&self) -> HighlightedCInt {
      HighlightedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_qt_core_string_ref(&self) -> HighlightedQtCoreStringRef {
      HighlightedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated_c_int(&self) -> ActivatedCInt {
      ActivatedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated_qt_core_string_ref(&self) -> ActivatedQtCoreStringRef {
      ActivatedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::currentIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_index_changed_c_int(&self) -> CurrentIndexChangedCInt {
      CurrentIndexChangedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::currentIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_index_changed_qt_core_string_ref(&self) -> CurrentIndexChangedQtCoreStringRef {
      CurrentIndexChangedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QComboBox::currentTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_text_changed(&self) -> CurrentTextChanged {
      CurrentTextChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ComboBox`.
  pub struct Slots<'a>(&'a ::combo_box::ComboBox);
  /// Represents a built-in Qt slot `QComboBox::setFocus`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetFocus<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::show`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Show<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::lower`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Lower<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::showNormal`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct ShowNormal<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::repaint`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Repaint<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setWindowModified`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetWindowModified<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::updateMicroFocus`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct UpdateMicroFocus<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::clear`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Clear<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setCurrentText`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_current_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetCurrentText<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::close`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Close<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setWindowTitle`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetWindowTitle<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::raise`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Raise<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::update`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Update<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setVisible`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetVisible<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::showFullScreen`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct ShowFullScreen<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::hide`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct Hide<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setHidden`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetHidden<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setDisabled`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetDisabled<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setCurrentIndex`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetCurrentIndex<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::clearEditText`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().clear_edit_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct ClearEditText<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for ClearEditText<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearEditText()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::showMinimized`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct ShowMinimized<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setEditText`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_edit_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetEditText<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetEditText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEditText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setStyleSheet`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetStyleSheet<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::showMaximized`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct ShowMaximized<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QComboBox::setEnabled`.
  ///
  /// An object of this type can be created from `ComboBox` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComboBox` object.
  pub struct SetEnabled<'a>(&'a ::combo_box::ComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QComboBox::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setCurrentText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_text(&self) -> SetCurrentText {
      SetCurrentText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::clearEditText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_edit_text(&self) -> ClearEditText {
      ClearEditText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setEditText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_edit_text(&self) -> SetEditText {
      SetEditText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QComboBox::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::combo_box::ComboBox {
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

/// C++ type: <span style='color: green;'>```QComboBox::InsertPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InsertPolicy {
  /// C++ enum variant: <span style='color: green;'>```NoInsert = 0```</span>
  NoInsert = 0,
  /// C++ enum variant: <span style='color: green;'>```InsertAtTop = 1```</span>
  InsertAtTop = 1,
  /// C++ enum variant: <span style='color: green;'>```InsertAtCurrent = 2```</span>
  InsertAtCurrent = 2,
  /// C++ enum variant: <span style='color: green;'>```InsertAtBottom = 3```</span>
  InsertAtBottom = 3,
  /// C++ enum variant: <span style='color: green;'>```InsertAfterCurrent = 4```</span>
  InsertAfterCurrent = 4,
  /// C++ enum variant: <span style='color: green;'>```InsertBeforeCurrent = 5```</span>
  InsertBeforeCurrent = 5,
  /// C++ enum variant: <span style='color: green;'>```InsertAlphabetically = 6```</span>
  InsertAlphabetically = 6,
}

/// C++ type: <span style='color: green;'>```QComboBox::SizeAdjustPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SizeAdjustPolicy {
  /// C++ enum variant: <span style='color: green;'>```AdjustToContents = 0```</span>
  Contents = 0,
  /// C++ enum variant: <span style='color: green;'>```AdjustToContentsOnFirstShow = 1```</span>
  ContentsOnFirstShow = 1,
  /// C++ enum variant: <span style='color: green;'>```AdjustToMinimumContentsLength = 2```</span>
  MinimumContentsLength = 2,
  /// C++ enum variant: <span style='color: green;'>```AdjustToMinimumContentsLengthWithIcon = 3```</span>
  MinimumContentsLengthWithIcon = 3,
}

impl ::cpp_utils::DynamicCast<::combo_box::ComboBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::combo_box::ComboBox> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QComboBox_G_dynamic_cast_QComboBox_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::combo_box::ComboBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QComboBox_G_dynamic_cast_QComboBox_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::combo_box::ComboBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QObject_ptr(self as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QObject_ptr(self as *const ::combo_box::ComboBox as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::combo_box::ComboBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QPaintDevice_ptr(self as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QPaintDevice_ptr(self as *const ::combo_box::ComboBox as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::combo_box::ComboBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QWidget_ptr(self as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QWidget_ptr(self as *const ::combo_box::ComboBox as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::combo_box::ComboBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::combo_box::ComboBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::combo_box::ComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::combo_box::ComboBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::combo_box::ComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::combo_box::ComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::combo_box::ComboBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::combo_box::ComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::combo_box::ComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QComboBox_G_static_cast_QComboBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::combo_box::ComboBox {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QWidget_ptr(self as *const ::combo_box::ComboBox as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::combo_box::ComboBox {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QComboBox_G_static_cast_QWidget_ptr(self as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ComboBox::add_item](../struct.ComboBox.html#method.add_item) method.
  pub trait ComboBoxAddItemArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> ();
  }
  impl<'largs> ComboBoxAddItemArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let icon = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_addItem_icon_text(original_self as *mut ::combo_box::ComboBox,
                                                        icon as *const ::qt_gui::icon::Icon,
                                                        text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ComboBoxAddItemArgs<'largs>
    for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let icon = self.0;
      let text = self.1;
      let user_data = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_addItem_icon_text_userData(original_self as *mut ::combo_box::ComboBox,
                                                                 icon as *const ::qt_gui::icon::Icon,
                                                                 text as *const ::qt_core::string::String,
                                                                 user_data as *const ::qt_core::variant::Variant)
      }
    }
  }
  impl<'largs> ComboBoxAddItemArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_addItem_text(original_self as *mut ::combo_box::ComboBox,
                                                   text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ComboBoxAddItemArgs<'largs> for (&'largs ::qt_core::string::String, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let text = self.0;
      let user_data = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_addItem_text_userData(original_self as *mut ::combo_box::ComboBox,
                                                            text as *const ::qt_core::string::String,
                                                            user_data as *const ::qt_core::variant::Variant)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ComboBox::current_data](../struct.ComboBox.html#method.current_data) method.
  pub trait ComboBoxCurrentDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant;
  }
  impl<'largs> ComboBoxCurrentDataArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant {

      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QComboBox_currentData_to_output_no_args(original_self as *const ::combo_box::ComboBox,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ComboBoxCurrentDataArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant {
      let role = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QComboBox_currentData_to_output_role(original_self as *const ::combo_box::ComboBox,
                                                                   role,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ComboBox::input_method_query](../struct.ComboBox.html#method.input_method_query) method.
  pub trait ComboBoxInputMethodQueryArgs<'largs> {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant;
  }
  impl<'largs> ComboBoxInputMethodQueryArgs<'largs> for &'largs ::qt_core::qt::InputMethodQuery {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant {
      let arg1 = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QComboBox_inputMethodQuery_to_output_arg1(original_self as *const ::combo_box::ComboBox, arg1 as *const ::qt_core::qt::InputMethodQuery, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ComboBoxInputMethodQueryArgs<'largs>
    for (&'largs ::qt_core::qt::InputMethodQuery, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant {
      let query = self.0;
      let argument = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QComboBox_inputMethodQuery_to_output_query_argument(original_self as *const ::combo_box::ComboBox, query as *const ::qt_core::qt::InputMethodQuery, argument as *const ::qt_core::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ComboBox::insert_item](../struct.ComboBox.html#method.insert_item) method.
  pub trait ComboBoxInsertItemArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> ();
  }
  impl<'largs> ComboBoxInsertItemArgs<'largs>
    for (::libc::c_int, &'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let index = self.0;
      let icon = self.1;
      let text = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_insertItem_index_icon_text(original_self as *mut ::combo_box::ComboBox,
                                                                 index,
                                                                 icon as *const ::qt_gui::icon::Icon,
                                                                 text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ComboBoxInsertItemArgs<'largs>
    for (::libc::c_int,
                                                       &'largs ::qt_gui::icon::Icon,
                                                       &'largs ::qt_core::string::String,
                                                       &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let index = self.0;
      let icon = self.1;
      let text = self.2;
      let user_data = self.3;
      unsafe { ::ffi::qt_widgets_c_QComboBox_insertItem_index_icon_text_userData(original_self as *mut ::combo_box::ComboBox, index, icon as *const ::qt_gui::icon::Icon, text as *const ::qt_core::string::String, user_data as *const ::qt_core::variant::Variant) }
    }
  }
  impl<'largs> ComboBoxInsertItemArgs<'largs> for (::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let index = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_insertItem_index_text(original_self as *mut ::combo_box::ComboBox,
                                                            index,
                                                            text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ComboBoxInsertItemArgs<'largs>
    for (::libc::c_int, &'largs ::qt_core::string::String, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let index = self.0;
      let text = self.1;
      let user_data = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_insertItem_index_text_userData(original_self as *mut ::combo_box::ComboBox,
                                                                     index,
                                                                     text as *const ::qt_core::string::String,
                                                                     user_data as *const ::qt_core::variant::Variant)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ComboBox::item_data](../struct.ComboBox.html#method.item_data) method.
  pub trait ComboBoxItemDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant;
  }
  impl<'largs> ComboBoxItemDataArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant {
      let index = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QComboBox_itemData_to_output_index(original_self as *const ::combo_box::ComboBox,
                                                                 index,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ComboBoxItemDataArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::combo_box::ComboBox) -> ::qt_core::variant::Variant {
      let index = self.0;
      let role = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QComboBox_itemData_to_output_index_role(original_self as *const ::combo_box::ComboBox,
                                                                      index,
                                                                      role,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ComboBox::set_item_data](../struct.ComboBox.html#method.set_item_data) method.
  pub trait ComboBoxSetItemDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> ();
  }
  impl<'largs> ComboBoxSetItemDataArgs<'largs> for (::libc::c_int, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let index = self.0;
      let value = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_setItemData_index_value(original_self as *mut ::combo_box::ComboBox,
                                                              index,
                                                              value as *const ::qt_core::variant::Variant)
      }
    }
  }
  impl<'largs> ComboBoxSetItemDataArgs<'largs> for (::libc::c_int, &'largs ::qt_core::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::combo_box::ComboBox) -> () {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QComboBox_setItemData_index_value_role(original_self as *mut ::combo_box::ComboBox,
                                                                   index,
                                                                   value as *const ::qt_core::variant::Variant,
                                                                   role)
      }
    }
  }
}
