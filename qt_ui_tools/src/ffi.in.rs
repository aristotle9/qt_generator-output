extern "C" {
  // Header: QUiLoader
  pub fn qt_ui_tools_c_QUiLoader_G_static_cast_QObject_ptr(ptr: *mut ::ui_loader::UiLoader) -> *mut ::qt_core::object::Object;
  pub fn qt_ui_tools_c_QUiLoader_G_static_cast_QUiLoader_ptr(ptr: *mut ::qt_core::object::Object) -> *mut ::ui_loader::UiLoader;
  pub fn qt_ui_tools_c_QUiLoader_addPluginPath(this_ptr: *mut ::ui_loader::UiLoader, path: *const ::qt_core::string::String);
  pub fn qt_ui_tools_c_QUiLoader_availableLayouts_to_output(this_ptr: *const ::ui_loader::UiLoader, output: *mut ::qt_core::string_list::StringList);
  pub fn qt_ui_tools_c_QUiLoader_availableWidgets_to_output(this_ptr: *const ::ui_loader::UiLoader, output: *mut ::qt_core::string_list::StringList);
  pub fn qt_ui_tools_c_QUiLoader_clearPluginPaths(this_ptr: *mut ::ui_loader::UiLoader);
  pub fn qt_ui_tools_c_QUiLoader_createActionGroup_no_args(this_ptr: *mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::action_group::ActionGroup;
  pub fn qt_ui_tools_c_QUiLoader_createActionGroup_parent(this_ptr: *mut ::ui_loader::UiLoader, parent: *mut ::qt_core::object::Object) -> *mut ::qt_widgets::action_group::ActionGroup;
  pub fn qt_ui_tools_c_QUiLoader_createActionGroup_parent_name(this_ptr: *mut ::ui_loader::UiLoader, parent: *mut ::qt_core::object::Object, name: *const ::qt_core::string::String) -> *mut ::qt_widgets::action_group::ActionGroup;
  pub fn qt_ui_tools_c_QUiLoader_createAction_no_args(this_ptr: *mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::action::Action;
  pub fn qt_ui_tools_c_QUiLoader_createAction_parent(this_ptr: *mut ::ui_loader::UiLoader, parent: *mut ::qt_core::object::Object) -> *mut ::qt_widgets::action::Action;
  pub fn qt_ui_tools_c_QUiLoader_createAction_parent_name(this_ptr: *mut ::ui_loader::UiLoader, parent: *mut ::qt_core::object::Object, name: *const ::qt_core::string::String) -> *mut ::qt_widgets::action::Action;
  pub fn qt_ui_tools_c_QUiLoader_createLayout_className(this_ptr: *mut ::ui_loader::UiLoader, className: *const ::qt_core::string::String) -> *mut ::qt_widgets::layout::Layout;
  pub fn qt_ui_tools_c_QUiLoader_createLayout_className_parent(this_ptr: *mut ::ui_loader::UiLoader, className: *const ::qt_core::string::String, parent: *mut ::qt_core::object::Object) -> *mut ::qt_widgets::layout::Layout;
  pub fn qt_ui_tools_c_QUiLoader_createLayout_className_parent_name(this_ptr: *mut ::ui_loader::UiLoader, className: *const ::qt_core::string::String, parent: *mut ::qt_core::object::Object, name: *const ::qt_core::string::String) -> *mut ::qt_widgets::layout::Layout;
  pub fn qt_ui_tools_c_QUiLoader_createWidget_className(this_ptr: *mut ::ui_loader::UiLoader, className: *const ::qt_core::string::String) -> *mut ::qt_widgets::widget::Widget;
  pub fn qt_ui_tools_c_QUiLoader_createWidget_className_parent(this_ptr: *mut ::ui_loader::UiLoader, className: *const ::qt_core::string::String, parent: *mut ::qt_widgets::widget::Widget) -> *mut ::qt_widgets::widget::Widget;
  pub fn qt_ui_tools_c_QUiLoader_createWidget_className_parent_name(this_ptr: *mut ::ui_loader::UiLoader, className: *const ::qt_core::string::String, parent: *mut ::qt_widgets::widget::Widget, name: *const ::qt_core::string::String) -> *mut ::qt_widgets::widget::Widget;
  pub fn qt_ui_tools_c_QUiLoader_delete(this_ptr: *mut ::ui_loader::UiLoader);
  pub fn qt_ui_tools_c_QUiLoader_errorString_to_output(this_ptr: *const ::ui_loader::UiLoader, output: *mut ::qt_core::string::String);
  pub fn qt_ui_tools_c_QUiLoader_isLanguageChangeEnabled(this_ptr: *const ::ui_loader::UiLoader) -> bool;
  pub fn qt_ui_tools_c_QUiLoader_isTranslationEnabled(this_ptr: *const ::ui_loader::UiLoader) -> bool;
  pub fn qt_ui_tools_c_QUiLoader_load_device(this_ptr: *mut ::ui_loader::UiLoader, device: *mut ::qt_core::io_device::IODevice) -> *mut ::qt_widgets::widget::Widget;
  pub fn qt_ui_tools_c_QUiLoader_load_device_parentWidget(this_ptr: *mut ::ui_loader::UiLoader, device: *mut ::qt_core::io_device::IODevice, parentWidget: *mut ::qt_widgets::widget::Widget) -> *mut ::qt_widgets::widget::Widget;
  pub fn qt_ui_tools_c_QUiLoader_metaObject(this_ptr: *const ::ui_loader::UiLoader) -> *const ::qt_core::meta_object::MetaObject;
  pub fn qt_ui_tools_c_QUiLoader_new_no_args() -> *mut ::ui_loader::UiLoader;
  pub fn qt_ui_tools_c_QUiLoader_new_parent(parent: *mut ::qt_core::object::Object) -> *mut ::ui_loader::UiLoader;
  pub fn qt_ui_tools_c_QUiLoader_pluginPaths_to_output(this_ptr: *const ::ui_loader::UiLoader, output: *mut ::qt_core::string_list::StringList);
  pub fn qt_ui_tools_c_QUiLoader_qt_metacall(this_ptr: *mut ::ui_loader::UiLoader, arg1: *const ::qt_core::meta_object::Call, arg2: ::libc::c_int, arg3: *mut *mut ::libc::c_void) -> ::libc::c_int;
  pub fn qt_ui_tools_c_QUiLoader_qt_metacast(this_ptr: *mut ::ui_loader::UiLoader, arg1: *const ::libc::c_char) -> *mut ::libc::c_void;
  pub fn qt_ui_tools_c_QUiLoader_setLanguageChangeEnabled(this_ptr: *mut ::ui_loader::UiLoader, enabled: bool);
  pub fn qt_ui_tools_c_QUiLoader_setTranslationEnabled(this_ptr: *mut ::ui_loader::UiLoader, enabled: bool);
  pub fn qt_ui_tools_c_QUiLoader_setWorkingDirectory(this_ptr: *mut ::ui_loader::UiLoader, dir: *const ::qt_core::dir::Dir);
  pub fn qt_ui_tools_c_QUiLoader_trUtf8_to_output(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int, output: *mut ::qt_core::string::String);
  pub fn qt_ui_tools_c_QUiLoader_tr_to_output(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int, output: *mut ::qt_core::string::String);
  pub fn qt_ui_tools_c_QUiLoader_workingDirectory_to_output(this_ptr: *const ::ui_loader::UiLoader, output: *mut ::qt_core::dir::Dir);

}
