/// C++ type: <span style='color: green;'>```QAccessibleTableModelChangeEvent```</span>
#[repr(C)]
pub struct AccessibleTableModelChangeEvent(u8);

impl AccessibleTableModelChangeEvent {
  /// C++ method: <span style='color: green;'>```int QAccessibleTableModelChangeEvent::firstColumn() const```</span>
  ///
  ///
  pub fn first_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_firstColumn(self as *const ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QAccessibleTableModelChangeEvent::firstRow() const```</span>
  ///
  ///
  pub fn first_row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_firstRow(self as *const ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QAccessibleTableModelChangeEvent::lastColumn() const```</span>
  ///
  ///
  pub fn last_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_lastColumn(self as *const ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QAccessibleTableModelChangeEvent::lastRow() const```</span>
  ///
  ///
  pub fn last_row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_lastRow(self as *const ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTableModelChangeEvent::ModelChangeType QAccessibleTableModelChangeEvent::modelChangeType() const```</span>
  ///
  ///
  pub fn model_change_type(&self) -> ::accessible_table_model_change_event::ModelChangeType {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_modelChangeType(self as *const ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTableModelChangeEvent::QAccessibleTableModelChangeEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, ::accessible_table_model_change_event::ModelChangeType)) -> ::cpp_utils::CppBox<::accessible_table_model_change_event::AccessibleTableModelChangeEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTableModelChangeEvent::QAccessibleTableModelChangeEvent(QAccessibleInterface* iface, QAccessibleTableModelChangeEvent::ModelChangeType changeType)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, ::accessible_table_model_change_event::ModelChangeType)) -> ::cpp_utils::CppBox<::accessible_table_model_change_event::AccessibleTableModelChangeEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTableModelChangeEvent::QAccessibleTableModelChangeEvent(QObject* obj, QAccessibleTableModelChangeEvent::ModelChangeType changeType)```</span>
  ///
  ///
  pub unsafe fn new<Args>
    (args: Args)
     -> ::cpp_utils::CppBox<::accessible_table_model_change_event::AccessibleTableModelChangeEvent>
    where Args: overloading::AccessibleTableModelChangeEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QAccessibleTableModelChangeEvent::setFirstColumn(int col)```</span>
  ///
  ///
  pub fn set_first_column(&mut self, col: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_setFirstColumn(self as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent, col) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessibleTableModelChangeEvent::setFirstRow(int row)```</span>
  ///
  ///
  pub fn set_first_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_setFirstRow(self as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent, row) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessibleTableModelChangeEvent::setLastColumn(int col)```</span>
  ///
  ///
  pub fn set_last_column(&mut self, col: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_setLastColumn(self as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent, col) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessibleTableModelChangeEvent::setLastRow(int row)```</span>
  ///
  ///
  pub fn set_last_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_setLastRow(self as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent, row) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessibleTableModelChangeEvent::setModelChangeType(QAccessibleTableModelChangeEvent::ModelChangeType changeType)```</span>
  ///
  ///
  pub fn set_model_change_type(&mut self, change_type: ::accessible_table_model_change_event::ModelChangeType) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_setModelChangeType(self as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent, change_type) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_table_model_change_event::AccessibleTableModelChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QAccessibleTableModelChangeEvent::ModelChangeType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ModelChangeType {
  /// C++ enum variant: <span style='color: green;'>```ModelReset = 0```</span>
  ModelReset = 0,
  /// C++ enum variant: <span style='color: green;'>```DataChanged = 1```</span>
  DataChanged = 1,
  /// C++ enum variant: <span style='color: green;'>```RowsInserted = 2```</span>
  RowsInserted = 2,
  /// C++ enum variant: <span style='color: green;'>```ColumnsInserted = 3```</span>
  ColumnsInserted = 3,
  /// C++ enum variant: <span style='color: green;'>```RowsRemoved = 4```</span>
  RowsRemoved = 4,
  /// C++ enum variant: <span style='color: green;'>```ColumnsRemoved = 5```</span>
  ColumnsRemoved = 5,
}

impl ::cpp_utils::DynamicCast<::accessible_table_model_change_event::AccessibleTableModelChangeEvent> for ::accessible_event::AccessibleEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_dynamic_cast_QAccessibleTableModelChangeEvent_ptr(self as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_table_model_change_event::AccessibleTableModelChangeEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_dynamic_cast_QAccessibleTableModelChangeEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::accessible_event::AccessibleEvent> for ::accessible_table_model_change_event::AccessibleTableModelChangeEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_table_model_change_event::AccessibleTableModelChangeEvent as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_table_model_change_event::AccessibleTableModelChangeEvent> for ::accessible_event::AccessibleEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleTableModelChangeEvent_ptr(self as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_table_model_change_event::AccessibleTableModelChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleTableModelChangeEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_table_model_change_event::AccessibleTableModelChangeEvent {
  type Target = ::accessible_event::AccessibleEvent;
  fn deref(&self) -> &::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_table_model_change_event::AccessibleTableModelChangeEvent as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_table_model_change_event::AccessibleTableModelChangeEvent {
  fn deref_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleTableModelChangeEvent::new](../struct.AccessibleTableModelChangeEvent.html#method.new) method.
  pub trait AccessibleTableModelChangeEventNewArgs {
    unsafe fn exec(self)
                   -> ::cpp_utils::CppBox<::accessible_table_model_change_event::AccessibleTableModelChangeEvent>;
  }
  impl AccessibleTableModelChangeEventNewArgs
    for (*mut ::accessible_interface::AccessibleInterface, ::accessible_table_model_change_event::ModelChangeType) {
    unsafe fn exec(self)
                   -> ::cpp_utils::CppBox<::accessible_table_model_change_event::AccessibleTableModelChangeEvent> {
      let iface = self.0;
      let change_type = self.1;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_new_iface_changeType(iface, change_type);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AccessibleTableModelChangeEventNewArgs
    for (*mut ::qt_core::object::Object, ::accessible_table_model_change_event::ModelChangeType) {
    unsafe fn exec(self)
                   -> ::cpp_utils::CppBox<::accessible_table_model_change_event::AccessibleTableModelChangeEvent> {
      let obj = self.0;
      let change_type = self.1;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTableModelChangeEvent_new_obj_changeType(obj, change_type);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
