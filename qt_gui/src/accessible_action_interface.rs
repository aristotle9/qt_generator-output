/// C++ type: <span style='color: green;'>```QAccessibleActionInterface```</span>
#[repr(C)]
pub struct AccessibleActionInterface(u8);

impl AccessibleActionInterface {
  /// C++ method: <span style='color: green;'>```pure virtual QStringList QAccessibleActionInterface::actionNames() const```</span>
  ///
  ///
  pub fn action_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_actionNames_to_output(self as *const ::accessible_action_interface::AccessibleActionInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static const QString& QAccessibleActionInterface::decreaseAction()```</span>
  ///
  ///
  pub fn decrease_action() -> &'static ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleActionInterface_decreaseAction() };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleActionInterface::doAction(const QString& actionName)```</span>
  ///
  ///
  pub fn do_action(&mut self, action_name: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QAccessibleActionInterface_doAction(self as *mut ::accessible_action_interface::AccessibleActionInterface, action_name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static const QString& QAccessibleActionInterface::increaseAction()```</span>
  ///
  ///
  pub fn increase_action() -> &'static ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleActionInterface_increaseAction() };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```pure virtual QStringList QAccessibleActionInterface::keyBindingsForAction(const QString& actionName) const```</span>
  ///
  ///
  pub fn key_bindings_for_action(&self, action_name: &::qt_core::string::String) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_keyBindingsForAction_to_output(self as *const ::accessible_action_interface::AccessibleActionInterface, action_name as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QAccessibleActionInterface::localizedActionDescription(const QString& name) const```</span>
  ///
  ///
  pub fn localized_action_description(&self, name: &::qt_core::string::String) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_localizedActionDescription_to_output(self as *const ::accessible_action_interface::AccessibleActionInterface, name as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QAccessibleActionInterface::localizedActionName(const QString& name) const```</span>
  ///
  ///
  pub fn localized_action_name(&self, name: &::qt_core::string::String) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_localizedActionName_to_output(self as *const ::accessible_action_interface::AccessibleActionInterface, name as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::nextPageAction()```</span>
  ///
  ///
  pub fn next_page_action() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_nextPageAction_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static const QString& QAccessibleActionInterface::pressAction()```</span>
  ///
  ///
  pub fn press_action() -> &'static ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleActionInterface_pressAction() };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::previousPageAction()```</span>
  ///
  ///
  pub fn previous_page_action() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_previousPageAction_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::scrollDownAction()```</span>
  ///
  ///
  pub fn scroll_down_action() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_scrollDownAction_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::scrollLeftAction()```</span>
  ///
  ///
  pub fn scroll_left_action() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_scrollLeftAction_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::scrollRightAction()```</span>
  ///
  ///
  pub fn scroll_right_action() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_scrollRightAction_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::scrollUpAction()```</span>
  ///
  ///
  pub fn scroll_up_action() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleActionInterface_scrollUpAction_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static const QString& QAccessibleActionInterface::setFocusAction()```</span>
  ///
  ///
  pub fn set_focus_action() -> &'static ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleActionInterface_setFocusAction() };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static const QString& QAccessibleActionInterface::showMenuAction()```</span>
  ///
  ///
  pub fn show_menu_action() -> &'static ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleActionInterface_showMenuAction() };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static const QString& QAccessibleActionInterface::toggleAction()```</span>
  ///
  ///
  pub fn toggle_action() -> &'static ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleActionInterface_toggleAction() };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::tr(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr(source_text: *const ::libc::c_char,
                   disambiguation: *const ::libc::c_char,
                   n: ::libc::c_int)
                   -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAccessibleActionInterface_tr_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAccessibleActionInterface::trUtf8(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(source_text: *const ::libc::c_char,
                        disambiguation: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAccessibleActionInterface_trUtf8_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_action_interface::AccessibleActionInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleActionInterface_delete
  }
}
