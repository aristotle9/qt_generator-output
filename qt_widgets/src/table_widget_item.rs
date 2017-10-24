/// C++ type: <span style='color: green;'>```QTableWidgetItem::ItemType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemType {
  /// C++ enum variant: <span style='color: green;'>```Type = 0```</span>
  Type = 0,
  /// C++ enum variant: <span style='color: green;'>```UserType = 1000```</span>
  UserType = 1000,
}

/// C++ type: <span style='color: green;'>```QTableWidgetItem```</span>
#[repr(C)]
pub struct TableWidgetItem(u8);

impl TableWidgetItem {
  /// C++ method: <span style='color: green;'>```QBrush QTableWidgetItem::background() const```</span>
  ///
  ///
  pub fn background(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_background_to_output(self as *const ::table_widget_item::TableWidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QTableWidgetItem::backgroundColor() const```</span>
  ///
  ///
  pub fn background_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_backgroundColor_to_output(self as *const ::table_widget_item::TableWidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QTableWidgetItem* QTableWidgetItem::clone() const```</span>
  ///
  ///
  pub fn clone(&self) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_clone(self as *const ::table_widget_item::TableWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetItem::column() const```</span>
  ///
  ///
  pub fn column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_column(self as *const ::table_widget_item::TableWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QTableWidgetItem::data(int role) const```</span>
  ///
  ///
  pub fn data(&self, role: ::libc::c_int) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_data_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                            role,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont QTableWidgetItem::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_font_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QTableWidgetItem::foreground() const```</span>
  ///
  ///
  pub fn foreground(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_foreground_to_output(self as *const ::table_widget_item::TableWidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon QTableWidgetItem::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_icon_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTableWidgetItem::isSelected() const```</span>
  ///
  ///
  pub fn is_selected(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_isSelected(self as *const ::table_widget_item::TableWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem::QTableWidgetItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetItem::QTableWidgetItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetItem::QTableWidgetItem(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_gui::icon::Icon, &::qt_core::string::String, ::libc::c_int)) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetItem::QTableWidgetItem(const QIcon& icon, const QString& text, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetItem::QTableWidgetItem(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, ::libc::c_int)) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetItem::QTableWidgetItem(const QString& text, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(&::table_widget_item::TableWidgetItem) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetItem::QTableWidgetItem(const QTableWidgetItem& other)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetItem::QTableWidgetItem(int type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>
    where Args: overloading::TableWidgetItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTableWidgetItem& QTableWidgetItem::operator=(const QTableWidgetItem& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::table_widget_item::TableWidgetItem)
                             -> &'l0 mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_operator_assign(self as *mut ::table_widget_item::TableWidgetItem,
                                                             other as *const ::table_widget_item::TableWidgetItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```virtual bool QTableWidgetItem::operator<(const QTableWidgetItem& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::table_widget_item::TableWidgetItem) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_operator_lt(self as *const ::table_widget_item::TableWidgetItem,
                                                       other as *const ::table_widget_item::TableWidgetItem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTableWidgetItem::read(QDataStream& in)```</span>
  ///
  ///
  pub fn read(&mut self, in_: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_read(self as *mut ::table_widget_item::TableWidgetItem,
                                                in_ as *mut ::qt_core::data_stream::DataStream)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetItem::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_row(self as *const ::table_widget_item::TableWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setBackground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_background(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setBackground(self as *mut ::table_widget_item::TableWidgetItem,
                                                         brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setBackgroundColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_background_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setBackgroundColor(self as *mut ::table_widget_item::TableWidgetItem,
                                                              color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setCheckState(Qt::CheckState state)```</span>
  ///
  ///
  pub fn set_check_state(&mut self, state: &::qt_core::qt::CheckState) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setCheckState(self as *mut ::table_widget_item::TableWidgetItem,
                                                         state as *const ::qt_core::qt::CheckState)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTableWidgetItem::setData(int role, const QVariant& value)```</span>
  ///
  ///
  pub fn set_data(&mut self, role: ::libc::c_int, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setData(self as *mut ::table_widget_item::TableWidgetItem,
                                                   role,
                                                   value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setFont(self as *mut ::table_widget_item::TableWidgetItem,
                                                   font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setForeground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_foreground(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setForeground(self as *mut ::table_widget_item::TableWidgetItem,
                                                         brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setIcon(self as *mut ::table_widget_item::TableWidgetItem,
                                                   icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setSelected(bool select)```</span>
  ///
  ///
  pub fn set_selected(&mut self, select: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setSelected(self as *mut ::table_widget_item::TableWidgetItem, select)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setSizeHint(const QSize& size)```</span>
  ///
  ///
  pub fn set_size_hint(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setSizeHint(self as *mut ::table_widget_item::TableWidgetItem,
                                                       size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setStatusTip(const QString& statusTip)```</span>
  ///
  ///
  pub fn set_status_tip(&mut self, status_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setStatusTip(self as *mut ::table_widget_item::TableWidgetItem,
                                                        status_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setText(self as *mut ::table_widget_item::TableWidgetItem,
                                                   text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setTextAlignment(int alignment)```</span>
  ///
  ///
  pub fn set_text_alignment(&mut self, alignment: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setTextAlignment(self as *mut ::table_widget_item::TableWidgetItem,
                                                            alignment)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setTextColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_text_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setTextColor(self as *mut ::table_widget_item::TableWidgetItem,
                                                        color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setToolTip(const QString& toolTip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, tool_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setToolTip(self as *mut ::table_widget_item::TableWidgetItem,
                                                      tool_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidgetItem::setWhatsThis(const QString& whatsThis)```</span>
  ///
  ///
  pub fn set_whats_this(&mut self, whats_this: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_setWhatsThis(self as *mut ::table_widget_item::TableWidgetItem,
                                                        whats_this as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QTableWidgetItem::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_sizeHint_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTableWidgetItem::statusTip() const```</span>
  ///
  ///
  pub fn status_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_statusTip_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidget* QTableWidgetItem::tableWidget() const```</span>
  ///
  ///
  pub fn table_widget(&self) -> *mut ::table_widget::TableWidget {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_tableWidget(self as *const ::table_widget_item::TableWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QString QTableWidgetItem::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_text_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetItem::textAlignment() const```</span>
  ///
  ///
  pub fn text_alignment(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_textAlignment(self as *const ::table_widget_item::TableWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QColor QTableWidgetItem::textColor() const```</span>
  ///
  ///
  pub fn text_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_textColor_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTableWidgetItem::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_toolTip_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_type(self as *const ::table_widget_item::TableWidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```QString QTableWidgetItem::whatsThis() const```</span>
  ///
  ///
  pub fn whats_this(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_whatsThis_to_output(self as *const ::table_widget_item::TableWidgetItem,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTableWidgetItem::write(QDataStream& out) const```</span>
  ///
  ///
  pub fn write(&self, out: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidgetItem_write(self as *const ::table_widget_item::TableWidgetItem,
                                                 out as *mut ::qt_core::data_stream::DataStream)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::table_widget_item::TableWidgetItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTableWidgetItem_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TableWidgetItem::new](../struct.TableWidgetItem.html#method.new) method.
  pub trait TableWidgetItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem>;
  }
  impl<'a> TableWidgetItemNewArgs for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem> {
      let icon = self.0;
      let text = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QTableWidgetItem_new_icon_text(icon as *const ::qt_gui::icon::Icon,
                                                           text as *const ::qt_core::string::String)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TableWidgetItemNewArgs for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem> {
      let icon = self.0;
      let text = self.1;
      let type_ = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QTableWidgetItem_new_icon_text_type(icon as *const ::qt_gui::icon::Icon,
                                                                  text as *const ::qt_core::string::String,
                                                                  type_)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TableWidgetItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TableWidgetItemNewArgs for &'a ::table_widget_item::TableWidgetItem {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem> {
      let other = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_new_other(other as *const ::table_widget_item::TableWidgetItem) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TableWidgetItemNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem> {
      let text = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TableWidgetItemNewArgs for (&'a ::qt_core::string::String, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem> {
      let text = self.0;
      let type_ = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_new_text_type(text as *const ::qt_core::string::String, type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TableWidgetItemNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget_item::TableWidgetItem> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidgetItem_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
