/// C++ type: <span style='color: green;'>```QSystemTrayIcon::ActivationReason```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ActivationReason {
  /// C++ enum variant: <span style='color: green;'>```Unknown = 0```</span>
  Unknown = 0,
  /// C++ enum variant: <span style='color: green;'>```Context = 1```</span>
  Context = 1,
  /// C++ enum variant: <span style='color: green;'>```DoubleClick = 2```</span>
  DoubleClick = 2,
  /// C++ enum variant: <span style='color: green;'>```Trigger = 3```</span>
  Trigger = 3,
  /// C++ enum variant: <span style='color: green;'>```MiddleClick = 4```</span>
  MiddleClick = 4,
}

/// C++ type: <span style='color: green;'>```QSystemTrayIcon::MessageIcon```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MessageIcon {
  /// C++ enum variant: <span style='color: green;'>```NoIcon = 0```</span>
  NoIcon = 0,
  /// C++ enum variant: <span style='color: green;'>```Information = 1```</span>
  Information = 1,
  /// C++ enum variant: <span style='color: green;'>```Warning = 2```</span>
  Warning = 2,
  /// C++ enum variant: <span style='color: green;'>```Critical = 3```</span>
  Critical = 3,
}

/// C++ type: <span style='color: green;'>```QSystemTrayIcon```</span>
#[repr(C)]
pub struct SystemTrayIcon(u8);

impl SystemTrayIcon {
  /// C++ method: <span style='color: green;'>```QMenu* QSystemTrayIcon::contextMenu() const```</span>
  ///
  ///
  pub fn context_menu(&self) -> *mut ::menu::Menu {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_contextMenu(self as *const ::system_tray_icon::SystemTrayIcon) }
  }

  /// C++ method: <span style='color: green;'>```QRect QSystemTrayIcon::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSystemTrayIcon_geometry_to_output(self as *const ::system_tray_icon::SystemTrayIcon,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::hide()```</span>
  ///
  ///
  pub fn hide(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_hide(self as *mut ::system_tray_icon::SystemTrayIcon) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QSystemTrayIcon::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSystemTrayIcon_icon_to_output(self as *const ::system_tray_icon::SystemTrayIcon,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QSystemTrayIcon::isSystemTrayAvailable()```</span>
  ///
  ///
  pub fn is_system_tray_available() -> bool {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_isSystemTrayAvailable() }
  }

  /// C++ method: <span style='color: green;'>```bool QSystemTrayIcon::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_isVisible(self as *const ::system_tray_icon::SystemTrayIcon) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSystemTrayIcon::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_metaObject(self as *const ::system_tray_icon::SystemTrayIcon) }
  }

  /// C++ method: <span style='color: green;'>```QSystemTrayIcon::QSystemTrayIcon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSystemTrayIcon::QSystemTrayIcon()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::icon::Icon) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSystemTrayIcon::QSystemTrayIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>
    where Args: overloading::SystemTrayIconNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSystemTrayIcon::QSystemTrayIcon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSystemTrayIcon::QSystemTrayIcon(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::icon::Icon, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSystemTrayIcon::QSystemTrayIcon(const QIcon& icon, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>
    where Args: overloading::SystemTrayIconNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QSystemTrayIcon::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSystemTrayIcon_qt_metacall(self as *mut ::system_tray_icon::SystemTrayIcon,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSystemTrayIcon::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QSystemTrayIcon_qt_metacast(self as *mut ::system_tray_icon::SystemTrayIcon, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSystemTrayIcon::setContextMenu(QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn set_context_menu(&mut self, menu: *mut ::menu::Menu) {
    ::ffi::qt_widgets_c_QSystemTrayIcon_setContextMenu(self as *mut ::system_tray_icon::SystemTrayIcon, menu)
  }

  /// C++ method: <span style='color: green;'>```void QSystemTrayIcon::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QSystemTrayIcon_setIcon(self as *mut ::system_tray_icon::SystemTrayIcon,
                                                  icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSystemTrayIcon::setToolTip(const QString& tip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QSystemTrayIcon_setToolTip(self as *mut ::system_tray_icon::SystemTrayIcon,
                                                     tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_setVisible(self as *mut ::system_tray_icon::SystemTrayIcon, visible) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::show()```</span>
  ///
  ///
  pub fn show(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_show(self as *mut ::system_tray_icon::SystemTrayIcon) }
  }

  /// C++ method: <span style='color: green;'>```QSystemTrayIcon::showMessage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::showMessage(const QString& title, const QString& msg)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, &::qt_core::string::String, &::system_tray_icon::MessageIcon)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::showMessage(const QString& title, const QString& msg, QSystemTrayIcon::MessageIcon icon = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, &::qt_core::string::String, &::system_tray_icon::MessageIcon, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::showMessage(const QString& title, const QString& msg, QSystemTrayIcon::MessageIcon icon = ?, int msecs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, &::qt_core::string::String, &::qt_gui::icon::Icon)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::showMessage(const QString& title, const QString& msg, const QIcon& icon)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, &::qt_core::string::String, &::qt_gui::icon::Icon, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSystemTrayIcon::showMessage(const QString& title, const QString& msg, const QIcon& icon, int msecs = ?)```</span>
  ///
  ///
  pub fn show_message<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SystemTrayIconShowMessageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static bool QSystemTrayIcon::supportsMessages()```</span>
  ///
  ///
  pub fn supports_messages() -> bool {
    unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_supportsMessages() }
  }

  /// C++ method: <span style='color: green;'>```QString QSystemTrayIcon::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSystemTrayIcon_toolTip_to_output(self as *const ::system_tray_icon::SystemTrayIcon,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSystemTrayIcon::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSystemTrayIcon_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSystemTrayIcon::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSystemTrayIcon_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::system_tray_icon::SystemTrayIcon {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSystemTrayIcon_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SystemTrayIcon`.
  pub struct Signals<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  /// Represents a built-in Qt signal `QSystemTrayIcon::objectNameChanged`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct ObjectNameChanged<'a>(&'a ::system_tray_icon::SystemTrayIcon);
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
  /// Represents a built-in Qt signal `QSystemTrayIcon::activated`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct Activated<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  impl<'a> ::qt_core::connection::Receiver for Activated<'a> {
    type Arguments = (&'static ::system_tray_icon::ActivationReason,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(QSystemTrayIcon::ActivationReason)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Activated<'a> {}
  /// Represents a built-in Qt signal `QSystemTrayIcon::messageClicked`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.signals().message_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct MessageClicked<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  impl<'a> ::qt_core::connection::Receiver for MessageClicked<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2messageClicked()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MessageClicked<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSystemTrayIcon::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSystemTrayIcon::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSystemTrayIcon::messageClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn message_clicked(&self) -> MessageClicked {
      MessageClicked(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SystemTrayIcon`.
  pub struct Slots<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  /// Represents a built-in Qt slot `QSystemTrayIcon::showMessage`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.slots().show_message_qt_core_string_ref_qt_core_string_ref_qt_gui_icon_ref_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct ShowMessageQtCoreStringRefQtCoreStringRefQtGuiIconRefCInt<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  impl<'a> ::qt_core::connection::Receiver for ShowMessageQtCoreStringRefQtCoreStringRefQtGuiIconRefCInt<'a> {
    type Arguments = (&'static ::qt_core::string::String,
     &'static ::qt_core::string::String,
     &'static ::qt_gui::icon::Icon,
     ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMessage(const QString&,const QString&,const QIcon&,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QSystemTrayIcon::showMessage`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.slots().show_message_qt_core_string_ref_qt_core_string_ref_message_icon_ref_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct ShowMessageQtCoreStringRefQtCoreStringRefMessageIconRefCInt<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  impl<'a> ::qt_core::connection::Receiver for ShowMessageQtCoreStringRefQtCoreStringRefMessageIconRefCInt<'a> {
    type Arguments = (&'static ::qt_core::string::String,
     &'static ::qt_core::string::String,
     &'static ::system_tray_icon::MessageIcon,
     ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMessage(const QString&,const QString&,QSystemTrayIcon::MessageIcon,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QSystemTrayIcon::setVisible`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct SetVisible<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSystemTrayIcon::hide`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct Hide<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QSystemTrayIcon::show`.
  ///
  /// An object of this type can be created from `SystemTrayIcon` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SystemTrayIcon` object.
  pub struct Show<'a>(&'a ::system_tray_icon::SystemTrayIcon);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSystemTrayIcon::showMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_message_qt_core_string_ref_qt_core_string_ref_qt_gui_icon_ref_c_int
      (&self)
       -> ShowMessageQtCoreStringRefQtCoreStringRefQtGuiIconRefCInt {
      ShowMessageQtCoreStringRefQtCoreStringRefQtGuiIconRefCInt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSystemTrayIcon::showMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_message_qt_core_string_ref_qt_core_string_ref_message_icon_ref_c_int
      (&self)
       -> ShowMessageQtCoreStringRefQtCoreStringRefMessageIconRefCInt {
      ShowMessageQtCoreStringRefQtCoreStringRefMessageIconRefCInt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSystemTrayIcon::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSystemTrayIcon::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSystemTrayIcon::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
  }
  impl ::system_tray_icon::SystemTrayIcon {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::system_tray_icon::SystemTrayIcon {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSystemTrayIcon_G_static_cast_QObject_ptr(self as *mut ::system_tray_icon::SystemTrayIcon)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_G_static_cast_QObject_ptr(self as *const ::system_tray_icon::SystemTrayIcon as *mut ::system_tray_icon::SystemTrayIcon) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::system_tray_icon::SystemTrayIcon> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::system_tray_icon::SystemTrayIcon {
    let ffi_result =
      ::ffi::qt_widgets_c_QSystemTrayIcon_G_static_cast_QSystemTrayIcon_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::system_tray_icon::SystemTrayIcon {
    let ffi_result = ::ffi::qt_widgets_c_QSystemTrayIcon_G_static_cast_QSystemTrayIcon_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::system_tray_icon::SystemTrayIcon {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_G_static_cast_QObject_ptr(self as *const ::system_tray_icon::SystemTrayIcon as *mut ::system_tray_icon::SystemTrayIcon) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::system_tray_icon::SystemTrayIcon {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSystemTrayIcon_G_static_cast_QObject_ptr(self as *mut ::system_tray_icon::SystemTrayIcon)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SystemTrayIcon::new](../struct.SystemTrayIcon.html#method.new) method.
  pub trait SystemTrayIconNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>;
  }
  impl<'a> SystemTrayIconNewArgs for &'a ::qt_gui::icon::Icon {
    fn exec(self) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon> {
      let icon = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_new_icon(icon as *const ::qt_gui::icon::Icon) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl SystemTrayIconNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [SystemTrayIcon::new_unsafe](../struct.SystemTrayIcon.html#method.new_unsafe) method.
  pub trait SystemTrayIconNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon>;
  }
  impl<'a> SystemTrayIconNewUnsafeArgs for (&'a ::qt_gui::icon::Icon, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon> {
      let icon = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QSystemTrayIcon_new_icon_parent(icon as *const ::qt_gui::icon::Icon, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SystemTrayIconNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::system_tray_icon::SystemTrayIcon> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QSystemTrayIcon_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [SystemTrayIcon::show_message](../struct.SystemTrayIcon.html#method.show_message) method.
  pub trait SystemTrayIconShowMessageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::system_tray_icon::SystemTrayIcon) -> ();
  }
  impl<'largs> SystemTrayIconShowMessageArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::system_tray_icon::SystemTrayIcon) -> () {
      let title = self.0;
      let msg = self.1;
      unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString(original_self as *mut ::system_tray_icon::SystemTrayIcon, title as *const ::qt_core::string::String, msg as *const ::qt_core::string::String) }
    }
  }
  impl<'largs> SystemTrayIconShowMessageArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String, &'largs ::qt_gui::icon::Icon) {
    fn exec(self, original_self: &'largs mut ::system_tray_icon::SystemTrayIcon) -> () {
      let title = self.0;
      let msg = self.1;
      let icon = self.2;
      unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QIcon(original_self as *mut ::system_tray_icon::SystemTrayIcon, title as *const ::qt_core::string::String, msg as *const ::qt_core::string::String, icon as *const ::qt_gui::icon::Icon) }
    }
  }
  impl<'largs> SystemTrayIconShowMessageArgs<'largs>
    for (&'largs ::qt_core::string::String,
                                                              &'largs ::qt_core::string::String,
                                                              &'largs ::qt_gui::icon::Icon,
                                                              ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::system_tray_icon::SystemTrayIcon) -> () {
      let title = self.0;
      let msg = self.1;
      let icon = self.2;
      let msecs = self.3;
      unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QIcon_int(original_self as *mut ::system_tray_icon::SystemTrayIcon, title as *const ::qt_core::string::String, msg as *const ::qt_core::string::String, icon as *const ::qt_gui::icon::Icon, msecs) }
    }
  }
  impl<'largs> SystemTrayIconShowMessageArgs<'largs>
    for (&'largs ::qt_core::string::String,
                                                              &'largs ::qt_core::string::String,
                                                              &'largs ::system_tray_icon::MessageIcon) {
    fn exec(self, original_self: &'largs mut ::system_tray_icon::SystemTrayIcon) -> () {
      let title = self.0;
      let msg = self.1;
      let icon = self.2;
      unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QSystemTrayIcon_MessageIcon(original_self as *mut ::system_tray_icon::SystemTrayIcon, title as *const ::qt_core::string::String, msg as *const ::qt_core::string::String, icon as *const ::system_tray_icon::MessageIcon) }
    }
  }
  impl<'largs> SystemTrayIconShowMessageArgs<'largs>
    for (&'largs ::qt_core::string::String,
                                                              &'largs ::qt_core::string::String,
                                                              &'largs ::system_tray_icon::MessageIcon,
                                                              ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::system_tray_icon::SystemTrayIcon) -> () {
      let title = self.0;
      let msg = self.1;
      let icon = self.2;
      let msecs = self.3;
      unsafe { ::ffi::qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QSystemTrayIcon_MessageIcon_int(original_self as *mut ::system_tray_icon::SystemTrayIcon, title as *const ::qt_core::string::String, msg as *const ::qt_core::string::String, icon as *const ::system_tray_icon::MessageIcon, msecs) }
    }
  }
}
