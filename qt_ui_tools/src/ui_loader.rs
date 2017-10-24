/// C++ type: <span style='color: green;'>```QUiLoader```</span>
#[repr(C)]
pub struct UiLoader(u8);

impl UiLoader {
  /// C++ method: <span style='color: green;'>```void QUiLoader::addPluginPath(const QString& path)```</span>
  ///
  ///
  pub fn add_plugin_path(&mut self, path: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_ui_tools_c_QUiLoader_addPluginPath(self as *mut ::ui_loader::UiLoader,
                                                   path as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QUiLoader::availableLayouts() const```</span>
  ///
  ///
  pub fn available_layouts(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_ui_tools_c_QUiLoader_availableLayouts_to_output(self as *const ::ui_loader::UiLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QUiLoader::availableWidgets() const```</span>
  ///
  ///
  pub fn available_widgets(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_ui_tools_c_QUiLoader_availableWidgets_to_output(self as *const ::ui_loader::UiLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QUiLoader::clearPluginPaths()```</span>
  ///
  ///
  pub fn clear_plugin_paths(&mut self) {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_clearPluginPaths(self as *mut ::ui_loader::UiLoader) }
  }

  /// C++ method: <span style='color: green;'>```virtual QAction* QUiLoader::createAction()```</span>
  ///
  ///
  pub fn create_action(&mut self) -> *mut ::qt_widgets::action::Action {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_createAction_no_args(self as *mut ::ui_loader::UiLoader) }
  }

  /// C++ method: <span style='color: green;'>```virtual QActionGroup* QUiLoader::createActionGroup()```</span>
  ///
  ///
  pub fn create_action_group(&mut self) -> *mut ::qt_widgets::action_group::ActionGroup {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_createActionGroup_no_args(self as *mut ::ui_loader::UiLoader) }
  }

  /// C++ method: <span style='color: green;'>```QUiLoader::createActionGroup```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_action_group_unsafe(&mut self, *mut ::qt_core::object::Object) -> *mut ::qt_widgets::action_group::ActionGroup```<br>
  /// C++ method: <span style='color: green;'>```virtual QActionGroup* QUiLoader::createActionGroup(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_action_group_unsafe(&mut self, (*mut ::qt_core::object::Object, &::qt_core::string::String)) -> *mut ::qt_widgets::action_group::ActionGroup```<br>
  /// C++ method: <span style='color: green;'>```virtual QActionGroup* QUiLoader::createActionGroup(QObject* parent = ?, const QString& name = ?)```</span>
  ///
  ///
  pub unsafe fn create_action_group_unsafe<'largs, Args>(&'largs mut self,
                                                         args: Args)
                                                         -> *mut ::qt_widgets::action_group::ActionGroup
    where Args: overloading::UiLoaderCreateActionGroupUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUiLoader::createAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_action_unsafe(&mut self, *mut ::qt_core::object::Object) -> *mut ::qt_widgets::action::Action```<br>
  /// C++ method: <span style='color: green;'>```virtual QAction* QUiLoader::createAction(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_action_unsafe(&mut self, (*mut ::qt_core::object::Object, &::qt_core::string::String)) -> *mut ::qt_widgets::action::Action```<br>
  /// C++ method: <span style='color: green;'>```virtual QAction* QUiLoader::createAction(QObject* parent = ?, const QString& name = ?)```</span>
  ///
  ///
  pub unsafe fn create_action_unsafe<'largs, Args>(&'largs mut self, args: Args) -> *mut ::qt_widgets::action::Action
    where Args: overloading::UiLoaderCreateActionUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QLayout* QUiLoader::createLayout(const QString& className)```</span>
  ///
  ///
  pub fn create_layout(&mut self, class_name: &::qt_core::string::String) -> *mut ::qt_widgets::layout::Layout {
    unsafe {
      ::ffi::qt_ui_tools_c_QUiLoader_createLayout_className(self as *mut ::ui_loader::UiLoader,
                                                            class_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QUiLoader::createLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_layout_unsafe(&mut self, (&::qt_core::string::String, *mut ::qt_core::object::Object)) -> *mut ::qt_widgets::layout::Layout```<br>
  /// C++ method: <span style='color: green;'>```virtual QLayout* QUiLoader::createLayout(const QString& className, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_layout_unsafe(&mut self, (&::qt_core::string::String, *mut ::qt_core::object::Object, &::qt_core::string::String)) -> *mut ::qt_widgets::layout::Layout```<br>
  /// C++ method: <span style='color: green;'>```virtual QLayout* QUiLoader::createLayout(const QString& className, QObject* parent = ?, const QString& name = ?)```</span>
  ///
  ///
  pub unsafe fn create_layout_unsafe<'largs, Args>(&'largs mut self, args: Args) -> *mut ::qt_widgets::layout::Layout
    where Args: overloading::UiLoaderCreateLayoutUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QWidget* QUiLoader::createWidget(const QString& className)```</span>
  ///
  ///
  pub fn create_widget(&mut self, class_name: &::qt_core::string::String) -> *mut ::qt_widgets::widget::Widget {
    unsafe {
      ::ffi::qt_ui_tools_c_QUiLoader_createWidget_className(self as *mut ::ui_loader::UiLoader,
                                                            class_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QUiLoader::createWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_widget_unsafe(&mut self, (&::qt_core::string::String, *mut ::qt_widgets::widget::Widget)) -> *mut ::qt_widgets::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```virtual QWidget* QUiLoader::createWidget(const QString& className, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_widget_unsafe(&mut self, (&::qt_core::string::String, *mut ::qt_widgets::widget::Widget, &::qt_core::string::String)) -> *mut ::qt_widgets::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```virtual QWidget* QUiLoader::createWidget(const QString& className, QWidget* parent = ?, const QString& name = ?)```</span>
  ///
  ///
  pub unsafe fn create_widget_unsafe<'largs, Args>(&'largs mut self, args: Args) -> *mut ::qt_widgets::widget::Widget
    where Args: overloading::UiLoaderCreateWidgetUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QUiLoader::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_ui_tools_c_QUiLoader_errorString_to_output(self as *const ::ui_loader::UiLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUiLoader::isLanguageChangeEnabled() const```</span>
  ///
  ///
  pub fn is_language_change_enabled(&self) -> bool {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_isLanguageChangeEnabled(self as *const ::ui_loader::UiLoader) }
  }

  /// C++ method: <span style='color: green;'>```bool QUiLoader::isTranslationEnabled() const```</span>
  ///
  ///
  pub fn is_translation_enabled(&self) -> bool {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_isTranslationEnabled(self as *const ::ui_loader::UiLoader) }
  }

  /// C++ method: <span style='color: green;'>```QUiLoader::load```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn load(&mut self, *mut ::qt_core::io_device::IODevice) -> *mut ::qt_widgets::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QUiLoader::load(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn load(&mut self, (*mut ::qt_core::io_device::IODevice, *mut ::qt_widgets::widget::Widget)) -> *mut ::qt_widgets::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QUiLoader::load(QIODevice* device, QWidget* parentWidget = ?)```</span>
  ///
  ///
  pub unsafe fn load<'largs, Args>(&'largs mut self, args: Args) -> *mut ::qt_widgets::widget::Widget
    where Args: overloading::UiLoaderLoadArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QUiLoader::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_metaObject(self as *const ::ui_loader::UiLoader) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QUiLoader::QUiLoader()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::ui_loader::UiLoader> {
    let ffi_result = unsafe { ::ffi::qt_ui_tools_c_QUiLoader_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QUiLoader::QUiLoader(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::ui_loader::UiLoader> {
    let ffi_result = ::ffi::qt_ui_tools_c_QUiLoader_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QStringList QUiLoader::pluginPaths() const```</span>
  ///
  ///
  pub fn plugin_paths(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_ui_tools_c_QUiLoader_pluginPaths_to_output(self as *const ::ui_loader::UiLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QUiLoader::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_ui_tools_c_QUiLoader_qt_metacall(self as *mut ::ui_loader::UiLoader,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QUiLoader::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_ui_tools_c_QUiLoader_qt_metacast(self as *mut ::ui_loader::UiLoader, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QUiLoader::setLanguageChangeEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_language_change_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_setLanguageChangeEnabled(self as *mut ::ui_loader::UiLoader, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QUiLoader::setTranslationEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_translation_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_ui_tools_c_QUiLoader_setTranslationEnabled(self as *mut ::ui_loader::UiLoader, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QUiLoader::setWorkingDirectory(const QDir& dir)```</span>
  ///
  ///
  pub fn set_working_directory(&mut self, dir: &::qt_core::dir::Dir) {
    unsafe {
      ::ffi::qt_ui_tools_c_QUiLoader_setWorkingDirectory(self as *mut ::ui_loader::UiLoader,
                                                         dir as *const ::qt_core::dir::Dir)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUiLoader::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_ui_tools_c_QUiLoader_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUiLoader::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_ui_tools_c_QUiLoader_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDir QUiLoader::workingDirectory() const```</span>
  ///
  ///
  pub fn working_directory(&self) -> ::qt_core::dir::Dir {
    {
      let mut object: ::qt_core::dir::Dir =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_ui_tools_c_QUiLoader_workingDirectory_to_output(self as *const ::ui_loader::UiLoader, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::ui_loader::UiLoader {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_ui_tools_c_QUiLoader_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `UiLoader`.
  pub struct Signals<'a>(&'a ::ui_loader::UiLoader);
  /// Represents a built-in Qt signal `QUiLoader::objectNameChanged`.
  ///
  /// An object of this type can be created from `UiLoader` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UiLoader` object.
  pub struct ObjectNameChanged<'a>(&'a ::ui_loader::UiLoader);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QUiLoader::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::ui_loader::UiLoader {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::ui_loader::UiLoader {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_ui_tools_c_QUiLoader_G_static_cast_QObject_ptr(self as *mut ::ui_loader::UiLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_ui_tools_c_QUiLoader_G_static_cast_QObject_ptr(self as *const ::ui_loader::UiLoader as *mut ::ui_loader::UiLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::ui_loader::UiLoader> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::ui_loader::UiLoader {
    let ffi_result =
      ::ffi::qt_ui_tools_c_QUiLoader_G_static_cast_QUiLoader_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::ui_loader::UiLoader {
    let ffi_result = ::ffi::qt_ui_tools_c_QUiLoader_G_static_cast_QUiLoader_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::ui_loader::UiLoader {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_ui_tools_c_QUiLoader_G_static_cast_QObject_ptr(self as *const ::ui_loader::UiLoader as *mut ::ui_loader::UiLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::ui_loader::UiLoader {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_ui_tools_c_QUiLoader_G_static_cast_QObject_ptr(self as *mut ::ui_loader::UiLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [UiLoader::create_action_group_unsafe](../struct.UiLoader.html#method.create_action_group_unsafe) method.
  pub trait UiLoaderCreateActionGroupUnsafeArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::ui_loader::UiLoader)
                   -> *mut ::qt_widgets::action_group::ActionGroup;
  }
  impl<'largs> UiLoaderCreateActionGroupUnsafeArgs<'largs> for *mut ::qt_core::object::Object {
    unsafe fn exec(self,
                   original_self: &'largs mut ::ui_loader::UiLoader)
                   -> *mut ::qt_widgets::action_group::ActionGroup {
      let parent = self;
      ::ffi::qt_ui_tools_c_QUiLoader_createActionGroup_parent(original_self as *mut ::ui_loader::UiLoader, parent)
    }
  }
  impl<'largs> UiLoaderCreateActionGroupUnsafeArgs<'largs>
    for (*mut ::qt_core::object::Object, &'largs ::qt_core::string::String) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::ui_loader::UiLoader)
                   -> *mut ::qt_widgets::action_group::ActionGroup {
      let parent = self.0;
      let name = self.1;
      ::ffi::qt_ui_tools_c_QUiLoader_createActionGroup_parent_name(original_self as *mut ::ui_loader::UiLoader,
                                                                   parent,
                                                                   name as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [UiLoader::create_action_unsafe](../struct.UiLoader.html#method.create_action_unsafe) method.
  pub trait UiLoaderCreateActionUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::action::Action;
  }
  impl<'largs> UiLoaderCreateActionUnsafeArgs<'largs> for *mut ::qt_core::object::Object {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::action::Action {
      let parent = self;
      ::ffi::qt_ui_tools_c_QUiLoader_createAction_parent(original_self as *mut ::ui_loader::UiLoader, parent)
    }
  }
  impl<'largs> UiLoaderCreateActionUnsafeArgs<'largs>
    for (*mut ::qt_core::object::Object, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::action::Action {
      let parent = self.0;
      let name = self.1;
      ::ffi::qt_ui_tools_c_QUiLoader_createAction_parent_name(original_self as *mut ::ui_loader::UiLoader,
                                                              parent,
                                                              name as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [UiLoader::create_layout_unsafe](../struct.UiLoader.html#method.create_layout_unsafe) method.
  pub trait UiLoaderCreateLayoutUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::layout::Layout;
  }
  impl<'largs> UiLoaderCreateLayoutUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String, *mut ::qt_core::object::Object) {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::layout::Layout {
      let class_name = self.0;
      let parent = self.1;
      ::ffi::qt_ui_tools_c_QUiLoader_createLayout_className_parent(original_self as *mut ::ui_loader::UiLoader,
                                                                   class_name as *const ::qt_core::string::String,
                                                                   parent)
    }
  }
  impl<'largs> UiLoaderCreateLayoutUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String, *mut ::qt_core::object::Object, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::layout::Layout {
      let class_name = self.0;
      let parent = self.1;
      let name = self.2;
      ::ffi::qt_ui_tools_c_QUiLoader_createLayout_className_parent_name(original_self as *mut ::ui_loader::UiLoader, class_name as *const ::qt_core::string::String, parent, name as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [UiLoader::create_widget_unsafe](../struct.UiLoader.html#method.create_widget_unsafe) method.
  pub trait UiLoaderCreateWidgetUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::widget::Widget;
  }
  impl<'largs> UiLoaderCreateWidgetUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String, *mut ::qt_widgets::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::widget::Widget {
      let class_name = self.0;
      let parent = self.1;
      ::ffi::qt_ui_tools_c_QUiLoader_createWidget_className_parent(original_self as *mut ::ui_loader::UiLoader,
                                                                   class_name as *const ::qt_core::string::String,
                                                                   parent)
    }
  }
  impl<'largs> UiLoaderCreateWidgetUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String, *mut ::qt_widgets::widget::Widget, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::widget::Widget {
      let class_name = self.0;
      let parent = self.1;
      let name = self.2;
      ::ffi::qt_ui_tools_c_QUiLoader_createWidget_className_parent_name(original_self as *mut ::ui_loader::UiLoader, class_name as *const ::qt_core::string::String, parent, name as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [UiLoader::load](../struct.UiLoader.html#method.load) method.
  pub trait UiLoaderLoadArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::widget::Widget;
  }
  impl<'largs> UiLoaderLoadArgs<'largs> for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::widget::Widget {
      let device = self;
      ::ffi::qt_ui_tools_c_QUiLoader_load_device(original_self as *mut ::ui_loader::UiLoader, device)
    }
  }
  impl<'largs> UiLoaderLoadArgs<'largs> for (*mut ::qt_core::io_device::IODevice, *mut ::qt_widgets::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::ui_loader::UiLoader) -> *mut ::qt_widgets::widget::Widget {
      let device = self.0;
      let parent_widget = self.1;
      ::ffi::qt_ui_tools_c_QUiLoader_load_device_parentWidget(original_self as *mut ::ui_loader::UiLoader,
                                                              device,
                                                              parent_widget)
    }
  }
}
