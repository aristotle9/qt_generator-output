/// C++ type: <span style='color: green;'>```QAccessibleWidget```</span>
#[repr(C)]
pub struct AccessibleWidget(u8);

impl AccessibleWidget {
  /// C++ method: <span style='color: green;'>```virtual QStringList QAccessibleWidget::actionNames() const```</span>
  ///
  ///
  pub fn action_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAccessibleWidget_actionNames_to_output(self as *const ::accessible_widget::AccessibleWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QColor QAccessibleWidget::backgroundColor() const```</span>
  ///
  ///
  pub fn background_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAccessibleWidget_backgroundColor_to_output(self as *const ::accessible_widget::AccessibleWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QAccessibleInterface* QAccessibleWidget::child(int index) const```</span>
  ///
  ///
  pub fn child(&self, index: ::libc::c_int) -> *mut ::qt_gui::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_child(self as *const ::accessible_widget::AccessibleWidget, index) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAccessibleWidget::childCount() const```</span>
  ///
  ///
  pub fn child_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_childCount(self as *const ::accessible_widget::AccessibleWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAccessibleWidget::doAction(const QString& actionName)```</span>
  ///
  ///
  pub fn do_action(&mut self, action_name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAccessibleWidget_doAction(self as *mut ::accessible_widget::AccessibleWidget,
                                                     action_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QAccessibleInterface* QAccessibleWidget::focusChild() const```</span>
  ///
  ///
  pub fn focus_child(&self) -> *mut ::qt_gui::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_focusChild(self as *const ::accessible_widget::AccessibleWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual QColor QAccessibleWidget::foregroundColor() const```</span>
  ///
  ///
  pub fn foreground_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAccessibleWidget_foregroundColor_to_output(self as *const ::accessible_widget::AccessibleWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAccessibleWidget::indexOfChild(const QAccessibleInterface* child) const```</span>
  ///
  ///
  pub unsafe fn index_of_child(&self,
                               child: *const ::qt_gui::accessible_interface::AccessibleInterface)
                               -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAccessibleWidget_indexOfChild(self as *const ::accessible_widget::AccessibleWidget, child)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAccessibleWidget::interface_cast(QAccessible::InterfaceType t)```</span>
  ///
  ///
  pub fn interface_cast(&mut self, t: &::qt_gui::accessible::InterfaceType) -> *mut ::libc::c_void {
    unsafe {
      ::ffi::qt_widgets_c_QAccessibleWidget_interface_cast(self as *mut ::accessible_widget::AccessibleWidget,
                                                           t as *const ::qt_gui::accessible::InterfaceType)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAccessibleWidget::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_isValid(self as *const ::accessible_widget::AccessibleWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QAccessibleWidget::keyBindingsForAction(const QString& actionName) const```</span>
  ///
  ///
  pub fn key_bindings_for_action(&self, action_name: &::qt_core::string::String) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAccessibleWidget_keyBindingsForAction_to_output(self as *const ::accessible_widget::AccessibleWidget, action_name as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QAccessibleInterface* QAccessibleWidget::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> *mut ::qt_gui::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_parent(self as *const ::accessible_widget::AccessibleWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QAccessibleWidget::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAccessibleWidget_rect_to_output(self as *const ::accessible_widget::AccessibleWidget,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QAccessible::State QAccessibleWidget::state() const```</span>
  ///
  ///
  pub fn state(&self) -> ::qt_gui::accessible::State {
    {
      let mut object: ::qt_gui::accessible::State =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAccessibleWidget_state_to_output(self as *const ::accessible_widget::AccessibleWidget,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QAccessibleWidget::text(QAccessible::Text t) const```</span>
  ///
  ///
  pub fn text(&self, t: &::qt_gui::accessible::Text) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAccessibleWidget_text_to_output(self as *const ::accessible_widget::AccessibleWidget,
                                                             t as *const ::qt_gui::accessible::Text,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QWindow* QAccessibleWidget::window() const```</span>
  ///
  ///
  pub fn window(&self) -> *mut ::qt_gui::window::Window {
    unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_window(self as *const ::accessible_widget::AccessibleWidget) }
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::accessible_action_interface::AccessibleActionInterface> for ::accessible_widget::AccessibleWidget {
fn static_cast_mut(&mut self) -> &mut ::qt_gui::accessible_action_interface::AccessibleActionInterface {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleActionInterface_ptr(self as *mut ::accessible_widget::AccessibleWidget) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_gui::accessible_action_interface::AccessibleActionInterface {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleActionInterface_ptr(self as *const ::accessible_widget::AccessibleWidget as *mut ::accessible_widget::AccessibleWidget) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_gui::accessible_interface::AccessibleInterface> for ::accessible_widget::AccessibleWidget {
fn static_cast_mut(&mut self) -> &mut ::qt_gui::accessible_interface::AccessibleInterface {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleInterface_ptr(self as *mut ::accessible_widget::AccessibleWidget) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_gui::accessible_interface::AccessibleInterface {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleInterface_ptr(self as *const ::accessible_widget::AccessibleWidget as *mut ::accessible_widget::AccessibleWidget) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_gui::accessible_object::AccessibleObject> for ::accessible_widget::AccessibleWidget {
fn static_cast_mut(&mut self) -> &mut ::qt_gui::accessible_object::AccessibleObject {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleObject_ptr(self as *mut ::accessible_widget::AccessibleWidget) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_gui::accessible_object::AccessibleObject {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleObject_ptr(self as *const ::accessible_widget::AccessibleWidget as *mut ::accessible_widget::AccessibleWidget) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_widget::AccessibleWidget> for ::qt_gui::accessible_action_interface::AccessibleActionInterface {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_widget::AccessibleWidget {
let ffi_result = ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleActionInterface(self as *mut ::qt_gui::accessible_action_interface::AccessibleActionInterface);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_widget::AccessibleWidget {
let ffi_result = ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleActionInterface(self as *const ::qt_gui::accessible_action_interface::AccessibleActionInterface as *mut ::qt_gui::accessible_action_interface::AccessibleActionInterface);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_widget::AccessibleWidget> for ::qt_gui::accessible_interface::AccessibleInterface {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_widget::AccessibleWidget {
let ffi_result = ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleInterface(self as *mut ::qt_gui::accessible_interface::AccessibleInterface);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_widget::AccessibleWidget {
let ffi_result = ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleInterface(self as *const ::qt_gui::accessible_interface::AccessibleInterface as *mut ::qt_gui::accessible_interface::AccessibleInterface);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_widget::AccessibleWidget> for ::qt_gui::accessible_object::AccessibleObject {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_widget::AccessibleWidget {
let ffi_result = ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleObject(self as *mut ::qt_gui::accessible_object::AccessibleObject);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_widget::AccessibleWidget {
let ffi_result = ::ffi::qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleObject(self as *const ::qt_gui::accessible_object::AccessibleObject as *mut ::qt_gui::accessible_object::AccessibleObject);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}
