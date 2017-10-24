/// C++ type: <span style='color: green;'>```QTreeWidgetItem::ChildIndicatorPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ChildIndicatorPolicy {
  /// C++ enum variant: <span style='color: green;'>```ShowIndicator = 0```</span>
  ShowIndicator = 0,
  /// C++ enum variant: <span style='color: green;'>```DontShowIndicator = 1```</span>
  DontShowIndicator = 1,
  /// C++ enum variant: <span style='color: green;'>```DontShowIndicatorWhenChildless = 2```</span>
  DontShowIndicatorWhenChildless = 2,
}

/// C++ type: <span style='color: green;'>```QTreeWidgetItem::ItemType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemType {
  /// C++ enum variant: <span style='color: green;'>```Type = 0```</span>
  Type = 0,
  /// C++ enum variant: <span style='color: green;'>```UserType = 1000```</span>
  UserType = 1000,
}

/// C++ type: <span style='color: green;'>```QTreeWidgetItem```</span>
#[repr(C)]
pub struct TreeWidgetItem(u8);

impl TreeWidgetItem {
  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::addChild(QTreeWidgetItem* child)```</span>
  ///
  ///
  pub unsafe fn add_child(&mut self, child: *mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidgetItem_addChild(self as *mut ::tree_widget_item::TreeWidgetItem, child)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::addChildren(const QList<QTreeWidgetItem*>& children)```</span>
  ///
  ///
  pub fn add_children(&mut self, children: &::list::ListTreeWidgetItemMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_addChildren(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                      children as *const ::list::ListTreeWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QTreeWidgetItem::background(int column) const```</span>
  ///
  ///
  pub fn background(&self, column: ::libc::c_int) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_background_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                                 column,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QTreeWidgetItem::backgroundColor(int column) const```</span>
  ///
  ///
  pub fn background_color(&self, column: ::libc::c_int) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_backgroundColor_to_output(self as *const ::tree_widget_item::TreeWidgetItem, column, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidgetItem::child(int index) const```</span>
  ///
  ///
  pub fn child(&self, index: ::libc::c_int) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_child(self as *const ::tree_widget_item::TreeWidgetItem, index) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidgetItem::childCount() const```</span>
  ///
  ///
  pub fn child_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_childCount(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QTreeWidgetItem* QTreeWidgetItem::clone() const```</span>
  ///
  ///
  pub fn clone(&self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_clone(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidgetItem::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_columnCount(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QTreeWidgetItem::data(int column, int role) const```</span>
  ///
  ///
  pub fn data(&self, column: ::libc::c_int, role: ::libc::c_int) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_data_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                           column,
                                                           role,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont QTreeWidgetItem::font(int column) const```</span>
  ///
  ///
  pub fn font(&self, column: ::libc::c_int) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_font_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                           column,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QTreeWidgetItem::foreground(int column) const```</span>
  ///
  ///
  pub fn foreground(&self, column: ::libc::c_int) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_foreground_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                                 column,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon QTreeWidgetItem::icon(int column) const```</span>
  ///
  ///
  pub fn icon(&self, column: ::libc::c_int) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_icon_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                           column,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidgetItem::indexOfChild(QTreeWidgetItem* child) const```</span>
  ///
  ///
  pub unsafe fn index_of_child(&self, child: *mut ::tree_widget_item::TreeWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTreeWidgetItem_indexOfChild(self as *const ::tree_widget_item::TreeWidgetItem, child)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem* child)```</span>
  ///
  ///
  pub unsafe fn insert_child(&mut self, index: ::libc::c_int, child: *mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidgetItem_insertChild(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                    index,
                                                    child)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::insertChildren(int index, const QList<QTreeWidgetItem*>& children)```</span>
  ///
  ///
  pub fn insert_children(&mut self, index: ::libc::c_int, children: &::list::ListTreeWidgetItemMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_insertChildren(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                         index,
                                                         children as *const ::list::ListTreeWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidgetItem::isDisabled() const```</span>
  ///
  ///
  pub fn is_disabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_isDisabled(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidgetItem::isExpanded() const```</span>
  ///
  ///
  pub fn is_expanded(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_isExpanded(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidgetItem::isFirstColumnSpanned() const```</span>
  ///
  ///
  pub fn is_first_column_spanned(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_isFirstColumnSpanned(self as *const ::tree_widget_item::TreeWidgetItem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidgetItem::isHidden() const```</span>
  ///
  ///
  pub fn is_hidden(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_isHidden(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidgetItem::isSelected() const```</span>
  ///
  ///
  pub fn is_selected(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_isSelected(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem::QTreeWidgetItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string_list::StringList) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(const QStringList& strings)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string_list::StringList, ::libc::c_int)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(const QStringList& strings, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::tree_widget_item::TreeWidgetItem) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(const QTreeWidgetItem& other)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(int type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>
    where Args: overloading::TreeWidgetItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTreeWidgetItem::QTreeWidgetItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::tree_widget::TreeWidget) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget* view)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget::TreeWidget, *mut ::tree_widget_item::TreeWidgetItem)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget* view, QTreeWidgetItem* after)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget::TreeWidget, *mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget* view, QTreeWidgetItem* after, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget::TreeWidget, &::qt_core::string_list::StringList)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget* view, const QStringList& strings)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget::TreeWidget, &::qt_core::string_list::StringList, ::libc::c_int)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget* view, const QStringList& strings, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget::TreeWidget, ::libc::c_int)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget* view, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::tree_widget_item::TreeWidgetItem) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem* parent)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem* parent, QTreeWidgetItem* after)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem* parent, QTreeWidgetItem* after, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget_item::TreeWidgetItem, &::qt_core::string_list::StringList)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem* parent, const QStringList& strings)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget_item::TreeWidgetItem, &::qt_core::string_list::StringList, ::libc::c_int)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem* parent, const QStringList& strings, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem* parent, int type = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>
    where Args: overloading::TreeWidgetItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTreeWidgetItem& QTreeWidgetItem::operator=(const QTreeWidgetItem& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::tree_widget_item::TreeWidgetItem)
                             -> &'l0 mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_operator_assign(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                            other as *const ::tree_widget_item::TreeWidgetItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```virtual bool QTreeWidgetItem::operator<(const QTreeWidgetItem& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::tree_widget_item::TreeWidgetItem) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_operator_lt(self as *const ::tree_widget_item::TreeWidgetItem,
                                                      other as *const ::tree_widget_item::TreeWidgetItem)
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidgetItem::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_parent(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeWidgetItem::read(QDataStream& in)```</span>
  ///
  ///
  pub fn read(&mut self, in_: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_read(self as *mut ::tree_widget_item::TreeWidgetItem,
                                               in_ as *mut ::qt_core::data_stream::DataStream)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::removeChild(QTreeWidgetItem* child)```</span>
  ///
  ///
  pub unsafe fn remove_child(&mut self, child: *mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidgetItem_removeChild(self as *mut ::tree_widget_item::TreeWidgetItem, child)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setBackground(int column, const QBrush& brush)```</span>
  ///
  ///
  pub fn set_background(&mut self, column: ::libc::c_int, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setBackground(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                        column,
                                                        brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setBackgroundColor(int column, const QColor& color)```</span>
  ///
  ///
  pub fn set_background_color(&mut self, column: ::libc::c_int, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setBackgroundColor(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                             column,
                                                             color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setCheckState(int column, Qt::CheckState state)```</span>
  ///
  ///
  pub fn set_check_state(&mut self, column: ::libc::c_int, state: &::qt_core::qt::CheckState) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setCheckState(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                        column,
                                                        state as *const ::qt_core::qt::CheckState)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setChildIndicatorPolicy(QTreeWidgetItem::ChildIndicatorPolicy policy)```</span>
  ///
  ///
  pub fn set_child_indicator_policy(&mut self, policy: &::tree_widget_item::ChildIndicatorPolicy) {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_setChildIndicatorPolicy(self as *mut ::tree_widget_item::TreeWidgetItem, policy as *const ::tree_widget_item::ChildIndicatorPolicy) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeWidgetItem::setData(int column, int role, const QVariant& value)```</span>
  ///
  ///
  pub fn set_data(&mut self, column: ::libc::c_int, role: ::libc::c_int, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setData(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                  column,
                                                  role,
                                                  value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setDisabled(bool disabled)```</span>
  ///
  ///
  pub fn set_disabled(&mut self, disabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setDisabled(self as *mut ::tree_widget_item::TreeWidgetItem, disabled)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setExpanded(bool expand)```</span>
  ///
  ///
  pub fn set_expanded(&mut self, expand: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_setExpanded(self as *mut ::tree_widget_item::TreeWidgetItem, expand) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setFirstColumnSpanned(bool span)```</span>
  ///
  ///
  pub fn set_first_column_spanned(&mut self, span: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setFirstColumnSpanned(self as *mut ::tree_widget_item::TreeWidgetItem, span)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setFont(int column, const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, column: ::libc::c_int, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setFont(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                  column,
                                                  font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setForeground(int column, const QBrush& brush)```</span>
  ///
  ///
  pub fn set_foreground(&mut self, column: ::libc::c_int, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setForeground(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                        column,
                                                        brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setHidden(bool hide)```</span>
  ///
  ///
  pub fn set_hidden(&mut self, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_setHidden(self as *mut ::tree_widget_item::TreeWidgetItem, hide) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setIcon(int column, const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, column: ::libc::c_int, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setIcon(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                  column,
                                                  icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setSelected(bool select)```</span>
  ///
  ///
  pub fn set_selected(&mut self, select: bool) {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_setSelected(self as *mut ::tree_widget_item::TreeWidgetItem, select) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setSizeHint(int column, const QSize& size)```</span>
  ///
  ///
  pub fn set_size_hint(&mut self, column: ::libc::c_int, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setSizeHint(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                      column,
                                                      size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setStatusTip(int column, const QString& statusTip)```</span>
  ///
  ///
  pub fn set_status_tip(&mut self, column: ::libc::c_int, status_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setStatusTip(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                       column,
                                                       status_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setText(int column, const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, column: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setText(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                  column,
                                                  text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setTextAlignment(int column, int alignment)```</span>
  ///
  ///
  pub fn set_text_alignment(&mut self, column: ::libc::c_int, alignment: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setTextAlignment(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                           column,
                                                           alignment)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setTextColor(int column, const QColor& color)```</span>
  ///
  ///
  pub fn set_text_color(&mut self, column: ::libc::c_int, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setTextColor(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                       column,
                                                       color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setToolTip(int column, const QString& toolTip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, column: ::libc::c_int, tool_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setToolTip(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                     column,
                                                     tool_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::setWhatsThis(int column, const QString& whatsThis)```</span>
  ///
  ///
  pub fn set_whats_this(&mut self, column: ::libc::c_int, whats_this: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_setWhatsThis(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                       column,
                                                       whats_this as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QTreeWidgetItem::sizeHint(int column) const```</span>
  ///
  ///
  pub fn size_hint(&self, column: ::libc::c_int) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_sizeHint_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                               column,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidgetItem::sortChildren(int column, Qt::SortOrder order)```</span>
  ///
  ///
  pub fn sort_children(&mut self, column: ::libc::c_int, order: &::qt_core::qt::SortOrder) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_sortChildren(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                       column,
                                                       order as *const ::qt_core::qt::SortOrder)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTreeWidgetItem::statusTip(int column) const```</span>
  ///
  ///
  pub fn status_tip(&self, column: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_statusTip_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                                column,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidgetItem::takeChild(int index)```</span>
  ///
  ///
  pub fn take_child(&mut self, index: ::libc::c_int) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_takeChild(self as *mut ::tree_widget_item::TreeWidgetItem, index) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*> QTreeWidgetItem::takeChildren()```</span>
  ///
  ///
  pub fn take_children(&mut self) -> ::list::ListTreeWidgetItemMutPtr {
    {
      let mut object: ::list::ListTreeWidgetItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_takeChildren_to_output(self as *mut ::tree_widget_item::TreeWidgetItem,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTreeWidgetItem::text(int column) const```</span>
  ///
  ///
  pub fn text(&self, column: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_text_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                           column,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidgetItem::textAlignment(int column) const```</span>
  ///
  ///
  pub fn text_alignment(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_textAlignment(self as *const ::tree_widget_item::TreeWidgetItem, column)
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QTreeWidgetItem::textColor(int column) const```</span>
  ///
  ///
  pub fn text_color(&self, column: ::libc::c_int) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_textColor_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                                column,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTreeWidgetItem::toolTip(int column) const```</span>
  ///
  ///
  pub fn tool_tip(&self, column: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_toolTip_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                              column,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidget* QTreeWidgetItem::treeWidget() const```</span>
  ///
  ///
  pub fn tree_widget(&self) -> *mut ::tree_widget::TreeWidget {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_treeWidget(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidgetItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_type(self as *const ::tree_widget_item::TreeWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QString QTreeWidgetItem::whatsThis(int column) const```</span>
  ///
  ///
  pub fn whats_this(&self, column: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItem_whatsThis_to_output(self as *const ::tree_widget_item::TreeWidgetItem,
                                                                column,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeWidgetItem::write(QDataStream& out) const```</span>
  ///
  ///
  pub fn write(&self, out: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidgetItem_write(self as *const ::tree_widget_item::TreeWidgetItem,
                                                out as *mut ::qt_core::data_stream::DataStream)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::tree_widget_item::TreeWidgetItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTreeWidgetItem_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TreeWidgetItem::new](../struct.TreeWidgetItem.html#method.new) method.
  pub trait TreeWidgetItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>;
  }
  impl TreeWidgetItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TreeWidgetItemNewArgs for &'a ::tree_widget_item::TreeWidgetItem {
    fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let other = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_new_other(other as *const ::tree_widget_item::TreeWidgetItem) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TreeWidgetItemNewArgs for &'a ::qt_core::string_list::StringList {
    fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let strings = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QTreeWidgetItem_new_strings(strings as *const ::qt_core::string_list::StringList)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TreeWidgetItemNewArgs for (&'a ::qt_core::string_list::StringList, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let strings = self.0;
      let type_ = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QTreeWidgetItem_new_strings_type(strings as *const ::qt_core::string_list::StringList,
                                                               type_)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TreeWidgetItemNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidgetItem_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TreeWidgetItem::new_unsafe](../struct.TreeWidgetItem.html#method.new_unsafe) method.
  pub trait TreeWidgetItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem>;
  }
  impl TreeWidgetItemNewUnsafeArgs for *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TreeWidgetItemNewUnsafeArgs
    for (*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let parent = self.0;
      let after = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_parent_after(parent, after);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TreeWidgetItemNewUnsafeArgs
    for (*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let parent = self.0;
      let after = self.1;
      let type_ = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_parent_after_type(parent, after, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TreeWidgetItemNewUnsafeArgs
    for (*mut ::tree_widget_item::TreeWidgetItem, &'a ::qt_core::string_list::StringList) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let parent = self.0;
      let strings = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QTreeWidgetItem_new_parent_strings(parent,
                                                               strings as *const ::qt_core::string_list::StringList);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TreeWidgetItemNewUnsafeArgs
    for (*mut ::tree_widget_item::TreeWidgetItem, &'a ::qt_core::string_list::StringList, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let parent = self.0;
      let strings = self.1;
      let type_ = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_parent_strings_type(parent, strings as *const ::qt_core::string_list::StringList, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TreeWidgetItemNewUnsafeArgs for (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let parent = self.0;
      let type_ = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_parent_type(parent, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TreeWidgetItemNewUnsafeArgs for *mut ::tree_widget::TreeWidget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let view = self;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_view(view);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TreeWidgetItemNewUnsafeArgs for (*mut ::tree_widget::TreeWidget, *mut ::tree_widget_item::TreeWidgetItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let view = self.0;
      let after = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_view_after(view, after);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TreeWidgetItemNewUnsafeArgs
    for (*mut ::tree_widget::TreeWidget, *mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let view = self.0;
      let after = self.1;
      let type_ = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_view_after_type(view, after, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TreeWidgetItemNewUnsafeArgs for (*mut ::tree_widget::TreeWidget, &'a ::qt_core::string_list::StringList) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let view = self.0;
      let strings = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QTreeWidgetItem_new_view_strings(view,
                                                             strings as *const ::qt_core::string_list::StringList);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TreeWidgetItemNewUnsafeArgs
    for (*mut ::tree_widget::TreeWidget, &'a ::qt_core::string_list::StringList, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let view = self.0;
      let strings = self.1;
      let type_ = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_view_strings_type(view, strings as *const ::qt_core::string_list::StringList, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TreeWidgetItemNewUnsafeArgs for (*mut ::tree_widget::TreeWidget, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tree_widget_item::TreeWidgetItem> {
      let view = self.0;
      let type_ = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QTreeWidgetItem_new_view_type(view, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
