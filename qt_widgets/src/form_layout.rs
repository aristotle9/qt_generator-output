/// C++ type: <span style='color: green;'>```QFormLayout::FieldGrowthPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FieldGrowthPolicy {
  /// C++ enum variant: <span style='color: green;'>```FieldsStayAtSizeHint = 0```</span>
  FieldsStayAtSizeHint = 0,
  /// C++ enum variant: <span style='color: green;'>```ExpandingFieldsGrow = 1```</span>
  ExpandingFieldsGrow = 1,
  /// C++ enum variant: <span style='color: green;'>```AllNonFixedFieldsGrow = 2```</span>
  AllNonFixedFieldsGrow = 2,
}

/// C++ type: <span style='color: green;'>```QFormLayout```</span>
#[repr(C)]
pub struct FormLayout(u8);

impl FormLayout {
  /// C++ method: <span style='color: green;'>```virtual void QFormLayout::addItem(QLayoutItem* item)```</span>
  ///
  ///
  pub unsafe fn add_item(&mut self, item: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QFormLayout_addItem(self as *mut ::form_layout::FormLayout, item)
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::addRow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_row(&mut self, *mut ::layout::Layout) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::addRow(QLayout* layout)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_row(&mut self, (*mut ::widget::Widget, *mut ::layout::Layout)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::addRow(QWidget* label, QLayout* field)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_row(&mut self, (*mut ::widget::Widget, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::addRow(QWidget* label, QWidget* field)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_row(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::addRow(QWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn add_row(&mut self, (&::qt_core::string::String, *mut ::layout::Layout)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::addRow(const QString& labelText, QLayout* field)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn add_row(&mut self, (&::qt_core::string::String, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::addRow(const QString& labelText, QWidget* field)```</span>
  ///
  ///
  pub unsafe fn add_row<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FormLayoutAddRowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QFormLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_count(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::FieldGrowthPolicy QFormLayout::fieldGrowthPolicy() const```</span>
  ///
  ///
  pub fn field_growth_policy(&self) -> ::form_layout::FieldGrowthPolicy {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_fieldGrowthPolicy(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::getItemPosition(int index, int* rowPtr, QFormLayout::ItemRole* rolePtr) const```</span>
  ///
  ///
  pub unsafe fn get_item_position(&self,
                                  index: ::libc::c_int,
                                  row_ptr: *mut ::libc::c_int,
                                  role_ptr: *mut ::form_layout::ItemRole) {
    ::ffi::qt_widgets_c_QFormLayout_getItemPosition(self as *const ::form_layout::FormLayout,
                                                    index,
                                                    row_ptr,
                                                    role_ptr)
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::getLayoutPosition(QLayout* layout, int* rowPtr, QFormLayout::ItemRole* rolePtr) const```</span>
  ///
  ///
  pub unsafe fn get_layout_position(&self,
                                    layout: *mut ::layout::Layout,
                                    row_ptr: *mut ::libc::c_int,
                                    role_ptr: *mut ::form_layout::ItemRole) {
    ::ffi::qt_widgets_c_QFormLayout_getLayoutPosition(self as *const ::form_layout::FormLayout,
                                                      layout,
                                                      row_ptr,
                                                      role_ptr)
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::getWidgetPosition(QWidget* widget, int* rowPtr, QFormLayout::ItemRole* rolePtr) const```</span>
  ///
  ///
  pub unsafe fn get_widget_position(&self,
                                    widget: *mut ::widget::Widget,
                                    row_ptr: *mut ::libc::c_int,
                                    role_ptr: *mut ::form_layout::ItemRole) {
    ::ffi::qt_widgets_c_QFormLayout_getWidgetPosition(self as *const ::form_layout::FormLayout,
                                                      widget,
                                                      row_ptr,
                                                      role_ptr)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QFormLayout::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_hasHeightForWidth(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QFormLayout::heightForWidth(int width) const```</span>
  ///
  ///
  pub fn height_for_width(&self, width: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_heightForWidth(self as *const ::form_layout::FormLayout, width) }
  }

  /// C++ method: <span style='color: green;'>```int QFormLayout::horizontalSpacing() const```</span>
  ///
  ///
  pub fn horizontal_spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_horizontalSpacing(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::insertRow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, *mut ::layout::Layout)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::insertRow(int row, QLayout* layout)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, *mut ::widget::Widget, *mut ::layout::Layout)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::insertRow(int row, QWidget* label, QLayout* field)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, *mut ::widget::Widget, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::insertRow(int row, QWidget* label, QWidget* field)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::insertRow(int row, QWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, &::qt_core::string::String, *mut ::layout::Layout)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::insertRow(int row, const QString& labelText, QLayout* field)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, &::qt_core::string::String, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::insertRow(int row, const QString& labelText, QWidget* field)```</span>
  ///
  ///
  pub unsafe fn insert_row<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FormLayoutInsertRowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QFormLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_invalidate(self as *mut ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::itemAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_at(&self, (::libc::c_int, ::form_layout::ItemRole)) -> *mut ::layout_item::LayoutItem```<br>
  /// C++ method: <span style='color: green;'>```QLayoutItem* QFormLayout::itemAt(int row, QFormLayout::ItemRole role) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_at(&self, ::libc::c_int) -> *mut ::layout_item::LayoutItem```<br>
  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QFormLayout::itemAt(int index) const```</span>
  ///
  ///
  pub fn item_at<'largs, Args>(&'largs self, args: Args) -> *mut ::layout_item::LayoutItem
    where Args: overloading::FormLayoutItemAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFormLayout::labelForField```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn label_for_field(&self, *mut ::layout::Layout) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QFormLayout::labelForField(QLayout* field) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn label_for_field(&self, *mut ::widget::Widget) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QFormLayout::labelForField(QWidget* field) const```</span>
  ///
  ///
  pub unsafe fn label_for_field<'largs, Args>(&'largs self, args: Args) -> *mut ::widget::Widget
    where Args: overloading::FormLayoutLabelForFieldArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFormLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_metaObject(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QFormLayout::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFormLayout_minimumSize_to_output(self as *const ::form_layout::FormLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFormLayout::QFormLayout()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::form_layout::FormLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFormLayout::QFormLayout(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::form_layout::FormLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QFormLayout_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QFormLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QFormLayout_qt_metacall(self as *mut ::form_layout::FormLayout,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QFormLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QFormLayout_qt_metacast(self as *mut ::form_layout::FormLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::removeRow(int row)```</span>
  ///
  ///
  pub fn remove_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_removeRow_row(self as *mut ::form_layout::FormLayout, row) }
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::removeRow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_row_unsafe(&mut self, *mut ::layout::Layout) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::removeRow(QLayout* layout)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_row_unsafe(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFormLayout::removeRow(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn remove_row_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FormLayoutRemoveRowUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QFormLayout::rowCount() const```</span>
  ///
  ///
  pub fn row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_rowCount(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::RowWrapPolicy QFormLayout::rowWrapPolicy() const```</span>
  ///
  ///
  pub fn row_wrap_policy(&self) -> ::form_layout::RowWrapPolicy {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_rowWrapPolicy(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setFieldGrowthPolicy(QFormLayout::FieldGrowthPolicy policy)```</span>
  ///
  ///
  pub fn set_field_growth_policy(&mut self, policy: ::form_layout::FieldGrowthPolicy) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_setFieldGrowthPolicy(self as *mut ::form_layout::FormLayout, policy) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QFormLayout::setGeometry(const QRect& rect)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, rect: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QFormLayout_setGeometry(self as *mut ::form_layout::FormLayout,
                                                  rect as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setHorizontalSpacing(int spacing)```</span>
  ///
  ///
  pub fn set_horizontal_spacing(&mut self, spacing: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_setHorizontalSpacing(self as *mut ::form_layout::FormLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setItem(int row, QFormLayout::ItemRole role, QLayoutItem* item)```</span>
  ///
  ///
  pub unsafe fn set_item(&mut self,
                         row: ::libc::c_int,
                         role: ::form_layout::ItemRole,
                         item: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QFormLayout_setItem(self as *mut ::form_layout::FormLayout, row, role, item)
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setLayout(int row, QFormLayout::ItemRole role, QLayout* layout)```</span>
  ///
  ///
  pub unsafe fn set_layout(&mut self,
                           row: ::libc::c_int,
                           role: ::form_layout::ItemRole,
                           layout: *mut ::layout::Layout) {
    ::ffi::qt_widgets_c_QFormLayout_setLayout(self as *mut ::form_layout::FormLayout, row, role, layout)
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setRowWrapPolicy(QFormLayout::RowWrapPolicy policy)```</span>
  ///
  ///
  pub fn set_row_wrap_policy(&mut self, policy: ::form_layout::RowWrapPolicy) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_setRowWrapPolicy(self as *mut ::form_layout::FormLayout, policy) }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setSpacing(int arg1)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_setSpacing(self as *mut ::form_layout::FormLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setVerticalSpacing(int spacing)```</span>
  ///
  ///
  pub fn set_vertical_spacing(&mut self, spacing: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_setVerticalSpacing(self as *mut ::form_layout::FormLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::setWidget(int row, QFormLayout::ItemRole role, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self,
                           row: ::libc::c_int,
                           role: ::form_layout::ItemRole,
                           widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QFormLayout_setWidget(self as *mut ::form_layout::FormLayout, row, role, widget)
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QFormLayout::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFormLayout_sizeHint_to_output(self as *const ::form_layout::FormLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QFormLayout::spacing() const```</span>
  ///
  ///
  pub fn spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_spacing(self as *const ::form_layout::FormLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QFormLayout::takeAt(int index)```</span>
  ///
  ///
  pub fn take_at(&mut self, index: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_takeAt(self as *mut ::form_layout::FormLayout, index) }
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::TakeRowResult QFormLayout::takeRow(int row)```</span>
  ///
  ///
  pub fn take_row(&mut self, row: ::libc::c_int) -> ::form_layout::TakeRowResult {
    {
      let mut object: ::form_layout::TakeRowResult =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFormLayout_takeRow_to_output_row(self as *mut ::form_layout::FormLayout, row, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFormLayout::takeRow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn take_row_unsafe(&mut self, *mut ::layout::Layout) -> ::form_layout::TakeRowResult```<br>
  /// C++ method: <span style='color: green;'>```QFormLayout::TakeRowResult QFormLayout::takeRow(QLayout* layout)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn take_row_unsafe(&mut self, *mut ::widget::Widget) -> ::form_layout::TakeRowResult```<br>
  /// C++ method: <span style='color: green;'>```QFormLayout::TakeRowResult QFormLayout::takeRow(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn take_row_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ::form_layout::TakeRowResult
    where Args: overloading::FormLayoutTakeRowUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QFormLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFormLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFormLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFormLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QFormLayout::verticalSpacing() const```</span>
  ///
  ///
  pub fn vertical_spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_verticalSpacing(self as *const ::form_layout::FormLayout) }
  }
}

impl ::cpp_utils::CppDeletable for ::form_layout::FormLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QFormLayout_delete
  }
}

/// C++ type: <span style='color: green;'>```QFormLayout::ItemRole```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemRole {
  /// C++ enum variant: <span style='color: green;'>```LabelRole = 0```</span>
  Label = 0,
  /// C++ enum variant: <span style='color: green;'>```FieldRole = 1```</span>
  Field = 1,
  /// C++ enum variant: <span style='color: green;'>```SpanningRole = 2```</span>
  Spanning = 2,
}

/// C++ type: <span style='color: green;'>```QFormLayout::RowWrapPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RowWrapPolicy {
  /// C++ enum variant: <span style='color: green;'>```DontWrapRows = 0```</span>
  DontWrap = 0,
  /// C++ enum variant: <span style='color: green;'>```WrapLongRows = 1```</span>
  WrapLong = 1,
  /// C++ enum variant: <span style='color: green;'>```WrapAllRows = 2```</span>
  WrapAll = 2,
}

/// C++ type: <span style='color: green;'>```QFormLayout::TakeRowResult```</span>
#[repr(C)]
pub struct TakeRowResult([u8; ::type_sizes::QT_WIDGETS_FORM_LAYOUT_TAKE_ROW_RESULT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TakeRowResult {
  unsafe fn new_uninitialized() -> TakeRowResult {
    TakeRowResult(::std::mem::uninitialized())
  }
}

impl TakeRowResult {
  /// C++ method: <span style='color: green;'>```QLayoutItem* QFormLayout::TakeRowResult::fieldItem() const```</span>
  ///
  ///
  pub fn field_item(&self) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_TakeRowResult_fieldItem(self as *const ::form_layout::TakeRowResult) }
  }

  /// C++ method: <span style='color: green;'>```QLayoutItem* QFormLayout::TakeRowResult::labelItem() const```</span>
  ///
  ///
  pub fn label_item(&self) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_TakeRowResult_labelItem(self as *const ::form_layout::TakeRowResult) }
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::TakeRowResult::set_fieldItem(QLayoutItem* value)```</span>
  ///
  ///
  pub unsafe fn set_field_item(&mut self, value: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QFormLayout_TakeRowResult_set_fieldItem(self as *mut ::form_layout::TakeRowResult, value)
  }

  /// C++ method: <span style='color: green;'>```void QFormLayout::TakeRowResult::set_labelItem(QLayoutItem* value)```</span>
  ///
  ///
  pub unsafe fn set_label_item(&mut self, value: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QFormLayout_TakeRowResult_set_labelItem(self as *mut ::form_layout::TakeRowResult, value)
  }
}

impl Drop for ::form_layout::TakeRowResult {
  /// C++ method: <span style='color: green;'>```[destructor] void QFormLayout::TakeRowResult::~QFormLayout::TakeRowResult()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QFormLayout_TakeRowResult_destructor(self as *mut ::form_layout::TakeRowResult) }
  }
}

impl ::cpp_utils::DynamicCast<::form_layout::FormLayout> for ::layout::Layout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::form_layout::FormLayout> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayout(self as *mut ::layout::Layout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::form_layout::FormLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::form_layout::FormLayout> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::form_layout::FormLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::form_layout::FormLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::form_layout::FormLayout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QObject_ptr(self as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QObject_ptr(self as *const ::form_layout::FormLayout as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout::Layout> for ::form_layout::FormLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QLayout_ptr(self as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QLayout_ptr(self as *const ::form_layout::FormLayout as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::form_layout::FormLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QLayoutItem_ptr(self as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QLayoutItem_ptr(self as *const ::form_layout::FormLayout as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::form_layout::FormLayout> for ::layout::Layout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::form_layout::FormLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayout(self as *mut ::layout::Layout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::form_layout::FormLayout {
    let ffi_result = ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::form_layout::FormLayout> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::form_layout::FormLayout {
    let ffi_result = ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::form_layout::FormLayout {
    let ffi_result = ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::form_layout::FormLayout> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::form_layout::FormLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::form_layout::FormLayout {
    let ffi_result = ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::form_layout::FormLayout {
  type Target = ::layout::Layout;
  fn deref(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QLayout_ptr(self as *const ::form_layout::FormLayout as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::form_layout::FormLayout {
  fn deref_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFormLayout_G_static_cast_QLayout_ptr(self as *mut ::form_layout::FormLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FormLayout::add_row](../struct.FormLayout.html#method.add_row) method.
  pub trait FormLayoutAddRowArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> ();
  }
  impl<'largs> FormLayoutAddRowArgs<'largs> for *mut ::layout::Layout {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let layout = self;
      ::ffi::qt_widgets_c_QFormLayout_addRow_QLayout(original_self as *mut ::form_layout::FormLayout, layout)
    }
  }
  impl<'largs> FormLayoutAddRowArgs<'largs> for (&'largs ::qt_core::string::String, *mut ::layout::Layout) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let label_text = self.0;
      let field = self.1;
      ::ffi::qt_widgets_c_QFormLayout_addRow_QString_QLayout(original_self as *mut ::form_layout::FormLayout,
                                                             label_text as *const ::qt_core::string::String,
                                                             field)
    }
  }
  impl<'largs> FormLayoutAddRowArgs<'largs> for (&'largs ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let label_text = self.0;
      let field = self.1;
      ::ffi::qt_widgets_c_QFormLayout_addRow_QString_QWidget(original_self as *mut ::form_layout::FormLayout,
                                                             label_text as *const ::qt_core::string::String,
                                                             field)
    }
  }
  impl<'largs> FormLayoutAddRowArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QFormLayout_addRow_QWidget(original_self as *mut ::form_layout::FormLayout, widget)
    }
  }
  impl<'largs> FormLayoutAddRowArgs<'largs> for (*mut ::widget::Widget, *mut ::layout::Layout) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let label = self.0;
      let field = self.1;
      ::ffi::qt_widgets_c_QFormLayout_addRow_QWidget_QLayout(original_self as *mut ::form_layout::FormLayout,
                                                             label,
                                                             field)
    }
  }
  impl<'largs> FormLayoutAddRowArgs<'largs> for (*mut ::widget::Widget, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let label = self.0;
      let field = self.1;
      ::ffi::qt_widgets_c_QFormLayout_addRow_QWidget_QWidget(original_self as *mut ::form_layout::FormLayout,
                                                             label,
                                                             field)
    }
  }
  /// This trait represents a set of arguments accepted by [FormLayout::insert_row](../struct.FormLayout.html#method.insert_row) method.
  pub trait FormLayoutInsertRowArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> ();
  }
  impl<'largs> FormLayoutInsertRowArgs<'largs> for (::libc::c_int, *mut ::layout::Layout) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let row = self.0;
      let layout = self.1;
      ::ffi::qt_widgets_c_QFormLayout_insertRow_int_QLayout(original_self as *mut ::form_layout::FormLayout,
                                                            row,
                                                            layout)
    }
  }
  impl<'largs> FormLayoutInsertRowArgs<'largs>
    for (::libc::c_int, &'largs ::qt_core::string::String, *mut ::layout::Layout) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let row = self.0;
      let label_text = self.1;
      let field = self.2;
      ::ffi::qt_widgets_c_QFormLayout_insertRow_int_QString_QLayout(original_self as *mut ::form_layout::FormLayout,
                                                                    row,
                                                                    label_text as *const ::qt_core::string::String,
                                                                    field)
    }
  }
  impl<'largs> FormLayoutInsertRowArgs<'largs>
    for (::libc::c_int, &'largs ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let row = self.0;
      let label_text = self.1;
      let field = self.2;
      ::ffi::qt_widgets_c_QFormLayout_insertRow_int_QString_QWidget(original_self as *mut ::form_layout::FormLayout,
                                                                    row,
                                                                    label_text as *const ::qt_core::string::String,
                                                                    field)
    }
  }
  impl<'largs> FormLayoutInsertRowArgs<'largs> for (::libc::c_int, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let row = self.0;
      let widget = self.1;
      ::ffi::qt_widgets_c_QFormLayout_insertRow_int_QWidget(original_self as *mut ::form_layout::FormLayout,
                                                            row,
                                                            widget)
    }
  }
  impl<'largs> FormLayoutInsertRowArgs<'largs> for (::libc::c_int, *mut ::widget::Widget, *mut ::layout::Layout) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let row = self.0;
      let label = self.1;
      let field = self.2;
      ::ffi::qt_widgets_c_QFormLayout_insertRow_int_QWidget_QLayout(original_self as *mut ::form_layout::FormLayout,
                                                                    row,
                                                                    label,
                                                                    field)
    }
  }
  impl<'largs> FormLayoutInsertRowArgs<'largs> for (::libc::c_int, *mut ::widget::Widget, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let row = self.0;
      let label = self.1;
      let field = self.2;
      ::ffi::qt_widgets_c_QFormLayout_insertRow_int_QWidget_QWidget(original_self as *mut ::form_layout::FormLayout,
                                                                    row,
                                                                    label,
                                                                    field)
    }
  }
  /// This trait represents a set of arguments accepted by [FormLayout::item_at](../struct.FormLayout.html#method.item_at) method.
  pub trait FormLayoutItemAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::form_layout::FormLayout) -> *mut ::layout_item::LayoutItem;
  }
  impl<'largs> FormLayoutItemAtArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::form_layout::FormLayout) -> *mut ::layout_item::LayoutItem {
      let index = self;
      unsafe { ::ffi::qt_widgets_c_QFormLayout_itemAt_index(original_self as *const ::form_layout::FormLayout, index) }
    }
  }
  impl<'largs> FormLayoutItemAtArgs<'largs> for (::libc::c_int, ::form_layout::ItemRole) {
    fn exec(self, original_self: &'largs ::form_layout::FormLayout) -> *mut ::layout_item::LayoutItem {
      let row = self.0;
      let role = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QFormLayout_itemAt_row_role(original_self as *const ::form_layout::FormLayout, row, role)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FormLayout::label_for_field](../struct.FormLayout.html#method.label_for_field) method.
  pub trait FormLayoutLabelForFieldArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::form_layout::FormLayout) -> *mut ::widget::Widget;
  }
  impl<'largs> FormLayoutLabelForFieldArgs<'largs> for *mut ::layout::Layout {
    unsafe fn exec(self, original_self: &'largs ::form_layout::FormLayout) -> *mut ::widget::Widget {
      let field = self;
      ::ffi::qt_widgets_c_QFormLayout_labelForField_QLayout(original_self as *const ::form_layout::FormLayout, field)
    }
  }
  impl<'largs> FormLayoutLabelForFieldArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs ::form_layout::FormLayout) -> *mut ::widget::Widget {
      let field = self;
      ::ffi::qt_widgets_c_QFormLayout_labelForField_QWidget(original_self as *const ::form_layout::FormLayout, field)
    }
  }
  /// This trait represents a set of arguments accepted by [FormLayout::remove_row_unsafe](../struct.FormLayout.html#method.remove_row_unsafe) method.
  pub trait FormLayoutRemoveRowUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> ();
  }
  impl<'largs> FormLayoutRemoveRowUnsafeArgs<'largs> for *mut ::layout::Layout {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let layout = self;
      ::ffi::qt_widgets_c_QFormLayout_removeRow_layout(original_self as *mut ::form_layout::FormLayout, layout)
    }
  }
  impl<'largs> FormLayoutRemoveRowUnsafeArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QFormLayout_removeRow_widget(original_self as *mut ::form_layout::FormLayout, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [FormLayout::take_row_unsafe](../struct.FormLayout.html#method.take_row_unsafe) method.
  pub trait FormLayoutTakeRowUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> ::form_layout::TakeRowResult;
  }
  impl<'largs> FormLayoutTakeRowUnsafeArgs<'largs> for *mut ::layout::Layout {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> ::form_layout::TakeRowResult {
      let layout = self;
      {
        let mut object: ::form_layout::TakeRowResult =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFormLayout_takeRow_to_output_layout(original_self as *mut ::form_layout::FormLayout,
                                                                 layout,
                                                                 &mut object);
        object
      }
    }
  }
  impl<'largs> FormLayoutTakeRowUnsafeArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::form_layout::FormLayout) -> ::form_layout::TakeRowResult {
      let widget = self;
      {
        let mut object: ::form_layout::TakeRowResult =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFormLayout_takeRow_to_output_widget(original_self as *mut ::form_layout::FormLayout,
                                                                 widget,
                                                                 &mut object);
        object
      }
    }
  }
}
