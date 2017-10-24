/// C++ type: <span style='color: green;'>```QAccessibleInterface```</span>
#[repr(C)]
pub struct AccessibleInterface(u8);

impl AccessibleInterface {
  /// C++ method: <span style='color: green;'>```QAccessibleActionInterface* QAccessibleInterface::actionInterface()```</span>
  ///
  ///
  pub fn action_interface(&mut self) -> *mut ::accessible_action_interface::AccessibleActionInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_actionInterface(self as *mut ::accessible_interface::AccessibleInterface)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QColor QAccessibleInterface::backgroundColor() const```</span>
  ///
  ///
  pub fn background_color(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleInterface_backgroundColor_to_output(self as *const ::accessible_interface::AccessibleInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QAccessibleInterface* QAccessibleInterface::child(int index) const```</span>
  ///
  ///
  pub fn child(&self, index: ::libc::c_int) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_child(self as *const ::accessible_interface::AccessibleInterface,
                                                 index)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QAccessibleInterface* QAccessibleInterface::childAt(int x, int y) const```</span>
  ///
  ///
  pub fn child_at(&self, x: ::libc::c_int, y: ::libc::c_int) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_childAt(self as *const ::accessible_interface::AccessibleInterface,
                                                   x,
                                                   y)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleInterface::childCount() const```</span>
  ///
  ///
  pub fn child_count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_childCount(self as *const ::accessible_interface::AccessibleInterface)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleEditableTextInterface* QAccessibleInterface::editableTextInterface()```</span>
  ///
  ///
  pub fn editable_text_interface(&mut self)
                                 -> *mut ::accessible_editable_text_interface::AccessibleEditableTextInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleInterface_editableTextInterface(self as *mut ::accessible_interface::AccessibleInterface) }
  }

  /// C++ method: <span style='color: green;'>```virtual QAccessibleInterface* QAccessibleInterface::focusChild() const```</span>
  ///
  ///
  pub fn focus_child(&self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_focusChild(self as *const ::accessible_interface::AccessibleInterface)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QColor QAccessibleInterface::foregroundColor() const```</span>
  ///
  ///
  pub fn foreground_color(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleInterface_foregroundColor_to_output(self as *const ::accessible_interface::AccessibleInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleInterface::indexOfChild(const QAccessibleInterface* arg1) const```</span>
  ///
  ///
  pub unsafe fn index_of_child(&self, arg1: *const ::accessible_interface::AccessibleInterface) -> ::libc::c_int {
    ::ffi::qt_gui_c_QAccessibleInterface_indexOfChild(self as *const ::accessible_interface::AccessibleInterface,
                                                      arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAccessibleInterface::interface_cast(QAccessible::InterfaceType arg1)```</span>
  ///
  ///
  pub fn interface_cast(&mut self, arg1: &::accessible::InterfaceType) -> *mut ::libc::c_void {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_interface_cast(self as *mut ::accessible_interface::AccessibleInterface,
                                                          arg1 as *const ::accessible::InterfaceType)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleInterface::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleInterface_isValid(self as *const ::accessible_interface::AccessibleInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QObject* QAccessibleInterface::object() const```</span>
  ///
  ///
  pub fn object(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QAccessibleInterface_object(self as *const ::accessible_interface::AccessibleInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QAccessibleInterface* QAccessibleInterface::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleInterface_parent(self as *const ::accessible_interface::AccessibleInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QRect QAccessibleInterface::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleInterface_rect_to_output(self as *const ::accessible_interface::AccessibleInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleInterface::setText(QAccessible::Text t, const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, t: &::accessible::Text, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_setText(self as *mut ::accessible_interface::AccessibleInterface,
                                                   t as *const ::accessible::Text,
                                                   text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QAccessible::State QAccessibleInterface::state() const```</span>
  ///
  ///
  pub fn state(&self) -> ::accessible::State {
    {
      let mut object: ::accessible::State =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleInterface_state_to_output(self as *const ::accessible_interface::AccessibleInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTableCellInterface* QAccessibleInterface::tableCellInterface()```</span>
  ///
  ///
  pub fn table_cell_interface(&mut self) -> *mut ::accessible_table_cell_interface::AccessibleTableCellInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_tableCellInterface(self as *mut ::accessible_interface::AccessibleInterface)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTableInterface* QAccessibleInterface::tableInterface()```</span>
  ///
  ///
  pub fn table_interface(&mut self) -> *mut ::accessible_table_interface::AccessibleTableInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_tableInterface(self as *mut ::accessible_interface::AccessibleInterface)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QString QAccessibleInterface::text(QAccessible::Text t) const```</span>
  ///
  ///
  pub fn text(&self, t: &::accessible::Text) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleInterface_text_to_output(self as *const ::accessible_interface::AccessibleInterface, t as *const ::accessible::Text, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTextInterface* QAccessibleInterface::textInterface()```</span>
  ///
  ///
  pub fn text_interface(&mut self) -> *mut ::accessible_text_interface::AccessibleTextInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_textInterface(self as *mut ::accessible_interface::AccessibleInterface)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleValueInterface* QAccessibleInterface::valueInterface()```</span>
  ///
  ///
  pub fn value_interface(&mut self) -> *mut ::accessible_value_interface::AccessibleValueInterface {
    unsafe {
      ::ffi::qt_gui_c_QAccessibleInterface_valueInterface(self as *mut ::accessible_interface::AccessibleInterface)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAccessibleInterface::virtual_hook(int id, void* data)```</span>
  ///
  ///
  pub unsafe fn virtual_hook(&mut self, id: ::libc::c_int, data: *mut ::libc::c_void) {
    ::ffi::qt_gui_c_QAccessibleInterface_virtual_hook(self as *mut ::accessible_interface::AccessibleInterface,
                                                      id,
                                                      data)
  }

  /// C++ method: <span style='color: green;'>```virtual QWindow* QAccessibleInterface::window() const```</span>
  ///
  ///
  pub fn window(&self) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QAccessibleInterface_window(self as *const ::accessible_interface::AccessibleInterface) }
  }
}
