/// C++ type: <span style='color: green;'>```QAbstractButton```</span>
#[repr(C)]
pub struct AbstractButton(u8);

impl AbstractButton {
  /// C++ method: <span style='color: green;'>```QAbstractButton::animateClick```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn animate_click(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QAbstractButton::animateClick()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn animate_click(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QAbstractButton::animateClick(int msec = ?)```</span>
  ///
  ///
  pub fn animate_click<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::AbstractButtonAnimateClickArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QAbstractButton::autoExclusive() const```</span>
  ///
  ///
  pub fn auto_exclusive(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_autoExclusive(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractButton::autoRepeat() const```</span>
  ///
  ///
  pub fn auto_repeat(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_autoRepeat(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractButton::autoRepeatDelay() const```</span>
  ///
  ///
  pub fn auto_repeat_delay(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_autoRepeatDelay(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractButton::autoRepeatInterval() const```</span>
  ///
  ///
  pub fn auto_repeat_interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_autoRepeatInterval(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractButton::click()```</span>
  ///
  ///
  pub fn click(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_click(self as *mut ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```QButtonGroup* QAbstractButton::group() const```</span>
  ///
  ///
  pub fn group(&self) -> *mut ::button_group::ButtonGroup {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_group(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QAbstractButton::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_icon_to_output(self as *const ::abstract_button::AbstractButton,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QAbstractButton::iconSize() const```</span>
  ///
  ///
  pub fn icon_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_iconSize_to_output(self as *const ::abstract_button::AbstractButton,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractButton::isCheckable() const```</span>
  ///
  ///
  pub fn is_checkable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_isCheckable(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractButton::isChecked() const```</span>
  ///
  ///
  pub fn is_checked(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_isChecked(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractButton::isDown() const```</span>
  ///
  ///
  pub fn is_down(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_isDown(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractButton::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_metaObject(self as *const ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractButton::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAbstractButton_qt_metacall(self as *mut ::abstract_button::AbstractButton,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAbstractButton::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QAbstractButton_qt_metacast(self as *mut ::abstract_button::AbstractButton, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setAutoExclusive(bool arg1)```</span>
  ///
  ///
  pub fn set_auto_exclusive(&mut self, arg1: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractButton_setAutoExclusive(self as *mut ::abstract_button::AbstractButton, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setAutoRepeat(bool arg1)```</span>
  ///
  ///
  pub fn set_auto_repeat(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_setAutoRepeat(self as *mut ::abstract_button::AbstractButton, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setAutoRepeatDelay(int arg1)```</span>
  ///
  ///
  pub fn set_auto_repeat_delay(&mut self, arg1: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractButton_setAutoRepeatDelay(self as *mut ::abstract_button::AbstractButton, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setAutoRepeatInterval(int arg1)```</span>
  ///
  ///
  pub fn set_auto_repeat_interval(&mut self, arg1: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractButton_setAutoRepeatInterval(self as *mut ::abstract_button::AbstractButton, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setCheckable(bool arg1)```</span>
  ///
  ///
  pub fn set_checkable(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_setCheckable(self as *mut ::abstract_button::AbstractButton, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractButton::setChecked(bool arg1)```</span>
  ///
  ///
  pub fn set_checked(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_setChecked(self as *mut ::abstract_button::AbstractButton, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setDown(bool arg1)```</span>
  ///
  ///
  pub fn set_down(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_setDown(self as *mut ::abstract_button::AbstractButton, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractButton_setIcon(self as *mut ::abstract_button::AbstractButton,
                                                  icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractButton::setIconSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractButton_setIconSize(self as *mut ::abstract_button::AbstractButton,
                                                      size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setShortcut(const QKeySequence& key)```</span>
  ///
  ///
  pub fn set_shortcut(&mut self, key: &::qt_gui::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractButton_setShortcut(self as *mut ::abstract_button::AbstractButton,
                                                      key as *const ::qt_gui::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractButton::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractButton_setText(self as *mut ::abstract_button::AbstractButton,
                                                  text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence QAbstractButton::shortcut() const```</span>
  ///
  ///
  pub fn shortcut(&self) -> ::qt_gui::key_sequence::KeySequence {
    {
      let mut object: ::qt_gui::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_shortcut_to_output(self as *const ::abstract_button::AbstractButton,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QAbstractButton::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_text_to_output(self as *const ::abstract_button::AbstractButton,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractButton::toggle()```</span>
  ///
  ///
  pub fn toggle(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractButton_toggle(self as *mut ::abstract_button::AbstractButton) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractButton::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractButton_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractButton::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractButton_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_button::AbstractButton {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAbstractButton_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractButton`.
  pub struct Signals<'a>(&'a ::abstract_button::AbstractButton);
  /// Represents a built-in Qt signal `QAbstractButton::released`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Released<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Released<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2released()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Released<'a> {}
  /// Represents a built-in Qt signal `QAbstractButton::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct WindowIconTextChanged<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for WindowIconTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconTextChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractButton::toggled`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().toggled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Toggled<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Toggled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2toggled(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Toggled<'a> {}
  /// Represents a built-in Qt signal `QAbstractButton::clicked`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Clicked<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Clicked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clicked(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Clicked<'a> {}
  /// Represents a built-in Qt signal `QAbstractButton::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for CustomContextMenuRequested<'a> {
    type Arguments = (&'static ::qt_core::point::Point,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2customContextMenuRequested(const QPoint&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CustomContextMenuRequested<'a> {}
  /// Represents a built-in Qt signal `QAbstractButton::windowIconChanged`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct WindowIconChanged<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for WindowIconChanged<'a> {
    type Arguments = (&'static ::qt_gui::icon::Icon,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconChanged(const QIcon&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractButton::pressed`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Pressed<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Pressed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Pressed<'a> {}
  /// Represents a built-in Qt signal `QAbstractButton::windowTitleChanged`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct WindowTitleChanged<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for WindowTitleChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowTitleChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowTitleChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractButton::released`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn released(&self) -> Released {
      Released(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractButton::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractButton::toggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggled(&self) -> Toggled {
      Toggled(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractButton::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractButton::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractButton::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractButton::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractButton::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractButton`.
  pub struct Slots<'a>(&'a ::abstract_button::AbstractButton);
  /// Represents a built-in Qt slot `QAbstractButton::setHidden`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetHidden<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::toggle`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().toggle()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Toggle<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Toggle<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toggle()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::raise`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Raise<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::showMaximized`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct ShowMaximized<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setVisible`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetVisible<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setWindowModified`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetWindowModified<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::update`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Update<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setWindowTitle`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetWindowTitle<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::click`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Click<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Click<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1click()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setDisabled`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetDisabled<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setFocus`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetFocus<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::close`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Close<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setStyleSheet`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetStyleSheet<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::updateMicroFocus`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct UpdateMicroFocus<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::animateClick`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().animate_click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct AnimateClick<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for AnimateClick<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1animateClick(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setChecked`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_checked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetChecked<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetChecked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setChecked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::repaint`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Repaint<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::hide`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Hide<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::showNormal`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct ShowNormal<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setEnabled`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetEnabled<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::showMinimized`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct ShowMinimized<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::show`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Show<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::setIconSize`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().set_icon_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct SetIconSize<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for SetIconSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIconSize(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::lower`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct Lower<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractButton::showFullScreen`.
  ///
  /// An object of this type can be created from `AbstractButton` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractButton` object.
  pub struct ShowFullScreen<'a>(&'a ::abstract_button::AbstractButton);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::toggle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggle(&self) -> Toggle {
      Toggle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::click`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn click(&self) -> Click {
      Click(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::animateClick`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn animate_click(&self) -> AnimateClick {
      AnimateClick(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setChecked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_checked(&self) -> SetChecked {
      SetChecked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::setIconSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_icon_size(&self) -> SetIconSize {
      SetIconSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractButton::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
  }
  impl ::abstract_button::AbstractButton {
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

impl ::cpp_utils::DynamicCast<::abstract_button::AbstractButton> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_button::AbstractButton> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QAbstractButton_G_dynamic_cast_QAbstractButton_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_button::AbstractButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractButton_G_dynamic_cast_QAbstractButton_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_button::AbstractButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QObject_ptr(self as *mut ::abstract_button::AbstractButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QObject_ptr(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::abstract_button::AbstractButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QPaintDevice_ptr(self as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QPaintDevice_ptr(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::abstract_button::AbstractButton {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QWidget_ptr(self as *mut ::abstract_button::AbstractButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QWidget_ptr(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_button::AbstractButton> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_button::AbstractButton> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_button::AbstractButton> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result =
      ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_button::AbstractButton {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QWidget_ptr(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_button::AbstractButton {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_G_static_cast_QWidget_ptr(self as *mut ::abstract_button::AbstractButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractButton::animate_click](../struct.AbstractButton.html#method.animate_click) method.
  pub trait AbstractButtonAnimateClickArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_button::AbstractButton) -> ();
  }
  impl<'largs> AbstractButtonAnimateClickArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_button::AbstractButton) -> () {
      let msec = self;
      unsafe {
        ::ffi::qt_widgets_c_QAbstractButton_animateClick_msec(original_self as *mut ::abstract_button::AbstractButton,
                                                              msec)
      }
    }
  }
  impl<'largs> AbstractButtonAnimateClickArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::abstract_button::AbstractButton) -> () {

      unsafe { ::ffi::qt_widgets_c_QAbstractButton_animateClick_no_args(original_self as *mut ::abstract_button::AbstractButton) }
    }
  }
}
