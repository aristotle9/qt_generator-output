/// C++ type: <span style='color: green;'>```QListWidgetItem::ItemType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemType {
  /// C++ enum variant: <span style='color: green;'>```Type = 0```</span>
  Type = 0,
  /// C++ enum variant: <span style='color: green;'>```UserType = 1000```</span>
  UserType = 1000,
}

/// C++ type: <span style='color: green;'>```QListWidgetItem```</span>
#[repr(C)]
pub struct ListWidgetItem(u8);

impl ListWidgetItem {
  /// C++ method: <span style='color: green;'>```QBrush QListWidgetItem::background() const```</span>
  ///
  ///
  pub fn background(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_background_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QListWidgetItem::backgroundColor() const```</span>
  ///
  ///
  pub fn background_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_backgroundColor_to_output(self as *const ::list_widget_item::ListWidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QListWidgetItem* QListWidgetItem::clone() const```</span>
  ///
  ///
  pub fn clone(&self) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_clone(self as *const ::list_widget_item::ListWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QListWidgetItem::data(int role) const```</span>
  ///
  ///
  pub fn data(&self, role: ::libc::c_int) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_data_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                           role,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont QListWidgetItem::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_font_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QListWidgetItem::foreground() const```</span>
  ///
  ///
  pub fn foreground(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_foreground_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon QListWidgetItem::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_icon_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QListWidgetItem::isHidden() const```</span>
  ///
  ///
  pub fn is_hidden(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_isHidden(self as *const ::list_widget_item::ListWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QListWidgetItem::isSelected() const```</span>
  ///
  ///
  pub fn is_selected(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_isSelected(self as *const ::list_widget_item::ListWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QListWidget* QListWidgetItem::listWidget() const```</span>
  ///
  ///
  pub fn list_widget(&self) -> *mut ::list_widget::ListWidget {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_listWidget(self as *const ::list_widget_item::ListWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem::QListWidgetItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::list_widget_item::ListWidgetItem) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(const QListWidgetItem& other)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>
    where Args: overloading::ListWidgetItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QListWidgetItem::QListWidgetItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::list_widget::ListWidget) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(QListWidget* view = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::list_widget::ListWidget, ::libc::c_int)) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(QListWidget* view = ?, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::icon::Icon, &::qt_core::string::String, *mut ::list_widget::ListWidget)) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(const QIcon& icon, const QString& text, QListWidget* view = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::icon::Icon, &::qt_core::string::String, *mut ::list_widget::ListWidget, ::libc::c_int)) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(const QIcon& icon, const QString& text, QListWidget* view = ?, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::list_widget::ListWidget)) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(const QString& text, QListWidget* view = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::list_widget::ListWidget, ::libc::c_int)) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QListWidgetItem::QListWidgetItem(const QString& text, QListWidget* view = ?, int type = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>
    where Args: overloading::ListWidgetItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QListWidgetItem& QListWidgetItem::operator=(const QListWidgetItem& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::list_widget_item::ListWidgetItem)
                             -> &'l0 mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_operator_assign(self as *mut ::list_widget_item::ListWidgetItem,
                                                            other as *const ::list_widget_item::ListWidgetItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```virtual bool QListWidgetItem::operator<(const QListWidgetItem& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::list_widget_item::ListWidgetItem) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_operator_lt(self as *const ::list_widget_item::ListWidgetItem,
                                                      other as *const ::list_widget_item::ListWidgetItem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QListWidgetItem::read(QDataStream& in)```</span>
  ///
  ///
  pub fn read(&mut self, in_: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_read(self as *mut ::list_widget_item::ListWidgetItem,
                                               in_ as *mut ::qt_core::data_stream::DataStream)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setBackground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_background(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setBackground(self as *mut ::list_widget_item::ListWidgetItem,
                                                        brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QListWidgetItem::setBackgroundColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_background_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setBackgroundColor(self as *mut ::list_widget_item::ListWidgetItem,
                                                             color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setCheckState(Qt::CheckState state)```</span>
  ///
  ///
  pub fn set_check_state(&mut self, state: &::qt_core::qt::CheckState) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setCheckState(self as *mut ::list_widget_item::ListWidgetItem,
                                                        state as *const ::qt_core::qt::CheckState)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QListWidgetItem::setData(int role, const QVariant& value)```</span>
  ///
  ///
  pub fn set_data(&mut self, role: ::libc::c_int, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setData(self as *mut ::list_widget_item::ListWidgetItem,
                                                  role,
                                                  value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setFont(self as *mut ::list_widget_item::ListWidgetItem,
                                                  font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setForeground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_foreground(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setForeground(self as *mut ::list_widget_item::ListWidgetItem,
                                                        brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setHidden(bool hide)```</span>
  ///
  ///
  pub fn set_hidden(&mut self, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_setHidden(self as *mut ::list_widget_item::ListWidgetItem, hide) }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setIcon(self as *mut ::list_widget_item::ListWidgetItem,
                                                  icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setSelected(bool select)```</span>
  ///
  ///
  pub fn set_selected(&mut self, select: bool) {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_setSelected(self as *mut ::list_widget_item::ListWidgetItem, select) }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setSizeHint(const QSize& size)```</span>
  ///
  ///
  pub fn set_size_hint(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setSizeHint(self as *mut ::list_widget_item::ListWidgetItem,
                                                      size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setStatusTip(const QString& statusTip)```</span>
  ///
  ///
  pub fn set_status_tip(&mut self, status_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setStatusTip(self as *mut ::list_widget_item::ListWidgetItem,
                                                       status_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setText(self as *mut ::list_widget_item::ListWidgetItem,
                                                  text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setTextAlignment(int alignment)```</span>
  ///
  ///
  pub fn set_text_alignment(&mut self, alignment: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setTextAlignment(self as *mut ::list_widget_item::ListWidgetItem, alignment)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setTextColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_text_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setTextColor(self as *mut ::list_widget_item::ListWidgetItem,
                                                       color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setToolTip(const QString& toolTip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, tool_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setToolTip(self as *mut ::list_widget_item::ListWidgetItem,
                                                     tool_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidgetItem::setWhatsThis(const QString& whatsThis)```</span>
  ///
  ///
  pub fn set_whats_this(&mut self, whats_this: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_setWhatsThis(self as *mut ::list_widget_item::ListWidgetItem,
                                                       whats_this as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QListWidgetItem::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_sizeHint_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QListWidgetItem::statusTip() const```</span>
  ///
  ///
  pub fn status_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_statusTip_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QListWidgetItem::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_text_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QListWidgetItem::textAlignment() const```</span>
  ///
  ///
  pub fn text_alignment(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_textAlignment(self as *const ::list_widget_item::ListWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QColor QListWidgetItem::textColor() const```</span>
  ///
  ///
  pub fn text_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_textColor_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QListWidgetItem::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_toolTip_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QListWidgetItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QListWidgetItem_type(self as *const ::list_widget_item::ListWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QString QListWidgetItem::whatsThis() const```</span>
  ///
  ///
  pub fn whats_this(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_whatsThis_to_output(self as *const ::list_widget_item::ListWidgetItem,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QListWidgetItem::write(QDataStream& out) const```</span>
  ///
  ///
  pub fn write(&self, out: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidgetItem_write(self as *const ::list_widget_item::ListWidgetItem,
                                                out as *mut ::qt_core::data_stream::DataStream)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::list_widget_item::ListWidgetItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QListWidgetItem_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ListWidgetItem::new](../struct.ListWidgetItem.html#method.new) method.
  pub trait ListWidgetItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>;
  }
  impl<'a> ListWidgetItemNewArgs for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let icon = self.0;
      let text = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QListWidgetItem_new_icon_text(icon as *const ::qt_gui::icon::Icon,
                                                          text as *const ::qt_core::string::String)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ListWidgetItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidgetItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ListWidgetItemNewArgs for &'a ::list_widget_item::ListWidgetItem {
    fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let other = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QListWidgetItem_new_other(other as *const ::list_widget_item::ListWidgetItem) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ListWidgetItemNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let text = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QListWidgetItem_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidgetItem::new_unsafe](../struct.ListWidgetItem.html#method.new_unsafe) method.
  pub trait ListWidgetItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem>;
  }
  impl<'a> ListWidgetItemNewUnsafeArgs
    for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String, *mut ::list_widget::ListWidget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let icon = self.0;
      let text = self.1;
      let view = self.2;
      let ffi_result =
        ::ffi::qt_widgets_c_QListWidgetItem_new_icon_text_view(icon as *const ::qt_gui::icon::Icon,
                                                               text as *const ::qt_core::string::String,
                                                               view);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ListWidgetItemNewUnsafeArgs
    for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String, *mut ::list_widget::ListWidget, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let icon = self.0;
      let text = self.1;
      let view = self.2;
      let type_ = self.3;
      let ffi_result =
        ::ffi::qt_widgets_c_QListWidgetItem_new_icon_text_view_type(icon as *const ::qt_gui::icon::Icon,
                                                                    text as *const ::qt_core::string::String,
                                                                    view,
                                                                    type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ListWidgetItemNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::list_widget::ListWidget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let text = self.0;
      let view = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QListWidgetItem_new_text_view(text as *const ::qt_core::string::String,
                                                                         view);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ListWidgetItemNewUnsafeArgs
    for (&'a ::qt_core::string::String, *mut ::list_widget::ListWidget, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let text = self.0;
      let view = self.1;
      let type_ = self.2;
      let ffi_result =
        ::ffi::qt_widgets_c_QListWidgetItem_new_text_view_type(text as *const ::qt_core::string::String, view, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ListWidgetItemNewUnsafeArgs for *mut ::list_widget::ListWidget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let view = self;
      let ffi_result = ::ffi::qt_widgets_c_QListWidgetItem_new_view(view);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ListWidgetItemNewUnsafeArgs for (*mut ::list_widget::ListWidget, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::list_widget_item::ListWidgetItem> {
      let view = self.0;
      let type_ = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QListWidgetItem_new_view_type(view, type_);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
