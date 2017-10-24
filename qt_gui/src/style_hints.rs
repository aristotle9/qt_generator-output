/// C++ type: <span style='color: green;'>```QStyleHints```</span>
#[repr(C)]
pub struct StyleHints(u8);

impl StyleHints {
  /// C++ method: <span style='color: green;'>```int QStyleHints::cursorFlashTime() const```</span>
  ///
  ///
  pub fn cursor_flash_time(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_cursorFlashTime(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```double QStyleHints::fontSmoothingGamma() const```</span>
  ///
  ///
  pub fn font_smoothing_gamma(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QStyleHints_fontSmoothingGamma(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::keyboardAutoRepeatRate() const```</span>
  ///
  ///
  pub fn keyboard_auto_repeat_rate(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_keyboardAutoRepeatRate(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::keyboardInputInterval() const```</span>
  ///
  ///
  pub fn keyboard_input_interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_keyboardInputInterval(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStyleHints::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QStyleHints_metaObject(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::mouseDoubleClickInterval() const```</span>
  ///
  ///
  pub fn mouse_double_click_interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_mouseDoubleClickInterval(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::mousePressAndHoldInterval() const```</span>
  ///
  ///
  pub fn mouse_press_and_hold_interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_mousePressAndHoldInterval(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```QChar QStyleHints::passwordMaskCharacter() const```</span>
  ///
  ///
  pub fn password_mask_character(&self) -> ::qt_core::char::Char {
    {
      let mut object: ::qt_core::char::Char =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStyleHints_passwordMaskCharacter_to_output(self as *const ::style_hints::StyleHints,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::passwordMaskDelay() const```</span>
  ///
  ///
  pub fn password_mask_delay(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_passwordMaskDelay(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QStyleHints::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QStyleHints_qt_metacall(self as *mut ::style_hints::StyleHints,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QStyleHints::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QStyleHints_qt_metacast(self as *mut ::style_hints::StyleHints, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setCursorFlashTime(int cursorFlashTime)```</span>
  ///
  ///
  pub fn set_cursor_flash_time(&mut self, cursor_flash_time: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStyleHints_setCursorFlashTime(self as *mut ::style_hints::StyleHints, cursor_flash_time) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleHints::setFocusOnTouchRelease() const```</span>
  ///
  ///
  pub fn set_focus_on_touch_release(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStyleHints_setFocusOnTouchRelease(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setKeyboardInputInterval(int keyboardInputInterval)```</span>
  ///
  ///
  pub fn set_keyboard_input_interval(&mut self, keyboard_input_interval: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QStyleHints_setKeyboardInputInterval(self as *mut ::style_hints::StyleHints,
                                                           keyboard_input_interval)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setMouseDoubleClickInterval(int mouseDoubleClickInterval)```</span>
  ///
  ///
  pub fn set_mouse_double_click_interval(&mut self, mouse_double_click_interval: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QStyleHints_setMouseDoubleClickInterval(self as *mut ::style_hints::StyleHints,
                                                              mouse_double_click_interval)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setMousePressAndHoldInterval(int mousePressAndHoldInterval)```</span>
  ///
  ///
  pub fn set_mouse_press_and_hold_interval(&mut self, mouse_press_and_hold_interval: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QStyleHints_setMousePressAndHoldInterval(self as *mut ::style_hints::StyleHints,
                                                               mouse_press_and_hold_interval)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setStartDragDistance(int startDragDistance)```</span>
  ///
  ///
  pub fn set_start_drag_distance(&mut self, start_drag_distance: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QStyleHints_setStartDragDistance(self as *mut ::style_hints::StyleHints, start_drag_distance)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setStartDragTime(int startDragTime)```</span>
  ///
  ///
  pub fn set_start_drag_time(&mut self, start_drag_time: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStyleHints_setStartDragTime(self as *mut ::style_hints::StyleHints, start_drag_time) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setTabFocusBehavior(Qt::TabFocusBehavior tabFocusBehavior)```</span>
  ///
  ///
  pub fn set_tab_focus_behavior(&mut self, tab_focus_behavior: &::qt_core::qt::TabFocusBehavior) {
    unsafe {
      ::ffi::qt_gui_c_QStyleHints_setTabFocusBehavior(self as *mut ::style_hints::StyleHints,
                                                      tab_focus_behavior as *const ::qt_core::qt::TabFocusBehavior)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setUseHoverEffects(bool useHoverEffects)```</span>
  ///
  ///
  pub fn set_use_hover_effects(&mut self, use_hover_effects: bool) {
    unsafe { ::ffi::qt_gui_c_QStyleHints_setUseHoverEffects(self as *mut ::style_hints::StyleHints, use_hover_effects) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHints::setWheelScrollLines(int scrollLines)```</span>
  ///
  ///
  pub fn set_wheel_scroll_lines(&mut self, scroll_lines: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStyleHints_setWheelScrollLines(self as *mut ::style_hints::StyleHints, scroll_lines) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleHints::showIsFullScreen() const```</span>
  ///
  ///
  pub fn show_is_full_screen(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStyleHints_showIsFullScreen(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleHints::showIsMaximized() const```</span>
  ///
  ///
  pub fn show_is_maximized(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStyleHints_showIsMaximized(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleHints::singleClickActivation() const```</span>
  ///
  ///
  pub fn single_click_activation(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStyleHints_singleClickActivation(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::startDragDistance() const```</span>
  ///
  ///
  pub fn start_drag_distance(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_startDragDistance(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::startDragTime() const```</span>
  ///
  ///
  pub fn start_drag_time(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_startDragTime(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::startDragVelocity() const```</span>
  ///
  ///
  pub fn start_drag_velocity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_startDragVelocity(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```static QString QStyleHints::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QStyleHints_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStyleHints::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QStyleHints_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleHints::useHoverEffects() const```</span>
  ///
  ///
  pub fn use_hover_effects(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStyleHints_useHoverEffects(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleHints::useRtlExtensions() const```</span>
  ///
  ///
  pub fn use_rtl_extensions(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStyleHints_useRtlExtensions(self as *const ::style_hints::StyleHints) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHints::wheelScrollLines() const```</span>
  ///
  ///
  pub fn wheel_scroll_lines(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStyleHints_wheelScrollLines(self as *const ::style_hints::StyleHints) }
  }
}

impl ::cpp_utils::CppDeletable for ::style_hints::StyleHints {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QStyleHints_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StyleHints`.
  pub struct Signals<'a>(&'a ::style_hints::StyleHints);
  /// Represents a built-in Qt signal `QStyleHints::mousePressAndHoldIntervalChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().mouse_press_and_hold_interval_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct MousePressAndHoldIntervalChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for MousePressAndHoldIntervalChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mousePressAndHoldIntervalChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MousePressAndHoldIntervalChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::startDragDistanceChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().start_drag_distance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct StartDragDistanceChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for StartDragDistanceChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2startDragDistanceChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StartDragDistanceChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::cursorFlashTimeChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().cursor_flash_time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct CursorFlashTimeChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for CursorFlashTimeChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cursorFlashTimeChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CursorFlashTimeChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::mouseDoubleClickIntervalChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().mouse_double_click_interval_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct MouseDoubleClickIntervalChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for MouseDoubleClickIntervalChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mouseDoubleClickIntervalChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MouseDoubleClickIntervalChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::keyboardInputIntervalChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().keyboard_input_interval_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct KeyboardInputIntervalChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for KeyboardInputIntervalChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2keyboardInputIntervalChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for KeyboardInputIntervalChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::objectNameChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct ObjectNameChanged<'a>(&'a ::style_hints::StyleHints);
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
  /// Represents a built-in Qt signal `QStyleHints::tabFocusBehaviorChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().tab_focus_behavior_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct TabFocusBehaviorChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for TabFocusBehaviorChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::TabFocusBehavior,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tabFocusBehaviorChanged(Qt::TabFocusBehavior)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TabFocusBehaviorChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::wheelScrollLinesChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().wheel_scroll_lines_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct WheelScrollLinesChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for WheelScrollLinesChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2wheelScrollLinesChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WheelScrollLinesChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::useHoverEffectsChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().use_hover_effects_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct UseHoverEffectsChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for UseHoverEffectsChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2useHoverEffectsChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UseHoverEffectsChanged<'a> {}
  /// Represents a built-in Qt signal `QStyleHints::startDragTimeChanged`.
  ///
  /// An object of this type can be created from `StyleHints` with `object.signals().start_drag_time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StyleHints` object.
  pub struct StartDragTimeChanged<'a>(&'a ::style_hints::StyleHints);
  impl<'a> ::qt_core::connection::Receiver for StartDragTimeChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2startDragTimeChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StartDragTimeChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QStyleHints::mousePressAndHoldIntervalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mouse_press_and_hold_interval_changed(&self) -> MousePressAndHoldIntervalChanged {
      MousePressAndHoldIntervalChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::startDragDistanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start_drag_distance_changed(&self) -> StartDragDistanceChanged {
      StartDragDistanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::cursorFlashTimeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cursor_flash_time_changed(&self) -> CursorFlashTimeChanged {
      CursorFlashTimeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::mouseDoubleClickIntervalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mouse_double_click_interval_changed(&self) -> MouseDoubleClickIntervalChanged {
      MouseDoubleClickIntervalChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::keyboardInputIntervalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn keyboard_input_interval_changed(&self) -> KeyboardInputIntervalChanged {
      KeyboardInputIntervalChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::tabFocusBehaviorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_focus_behavior_changed(&self) -> TabFocusBehaviorChanged {
      TabFocusBehaviorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::wheelScrollLinesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn wheel_scroll_lines_changed(&self) -> WheelScrollLinesChanged {
      WheelScrollLinesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::useHoverEffectsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn use_hover_effects_changed(&self) -> UseHoverEffectsChanged {
      UseHoverEffectsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStyleHints::startDragTimeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start_drag_time_changed(&self) -> StartDragTimeChanged {
      StartDragTimeChanged(self.0)
    }
  }
  impl ::style_hints::StyleHints {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::style_hints::StyleHints {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QStyleHints_G_static_cast_QObject_ptr(self as *mut ::style_hints::StyleHints) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStyleHints_G_static_cast_QObject_ptr(self as *const ::style_hints::StyleHints as *mut ::style_hints::StyleHints) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_hints::StyleHints> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_hints::StyleHints {
    let ffi_result =
      ::ffi::qt_gui_c_QStyleHints_G_static_cast_QStyleHints_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_hints::StyleHints {
    let ffi_result = ::ffi::qt_gui_c_QStyleHints_G_static_cast_QStyleHints_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_hints::StyleHints {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStyleHints_G_static_cast_QObject_ptr(self as *const ::style_hints::StyleHints as *mut ::style_hints::StyleHints) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_hints::StyleHints {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QStyleHints_G_static_cast_QObject_ptr(self as *mut ::style_hints::StyleHints) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
