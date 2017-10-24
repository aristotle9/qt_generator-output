/// C++ type: <span style='color: green;'>```QInputMethod::Action```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Action {
  /// C++ enum variant: <span style='color: green;'>```Click = 0```</span>
  Click = 0,
  /// C++ enum variant: <span style='color: green;'>```ContextMenu = 1```</span>
  ContextMenu = 1,
}

/// C++ type: <span style='color: green;'>```QInputMethod```</span>
#[repr(C)]
pub struct InputMethod(u8);

impl InputMethod {
  /// C++ method: <span style='color: green;'>```QRectF QInputMethod::anchorRectangle() const```</span>
  ///
  ///
  pub fn anchor_rectangle(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_anchorRectangle_to_output(self as *const ::input_method::InputMethod, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QInputMethod::commit()```</span>
  ///
  ///
  pub fn commit(&mut self) {
    unsafe { ::ffi::qt_gui_c_QInputMethod_commit(self as *mut ::input_method::InputMethod) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QInputMethod::cursorRectangle() const```</span>
  ///
  ///
  pub fn cursor_rectangle(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_cursorRectangle_to_output(self as *const ::input_method::InputMethod, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QInputMethod::hide()```</span>
  ///
  ///
  pub fn hide(&mut self) {
    unsafe { ::ffi::qt_gui_c_QInputMethod_hide(self as *mut ::input_method::InputMethod) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QInputMethod::inputItemClipRectangle() const```</span>
  ///
  ///
  pub fn input_item_clip_rectangle(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_inputItemClipRectangle_to_output(self as *const ::input_method::InputMethod,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QInputMethod::inputItemRectangle() const```</span>
  ///
  ///
  pub fn input_item_rectangle(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_inputItemRectangle_to_output(self as *const ::input_method::InputMethod,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QInputMethod::inputItemTransform() const```</span>
  ///
  ///
  pub fn input_item_transform(&self) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_inputItemTransform_to_output(self as *const ::input_method::InputMethod,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QInputMethod::invokeAction(QInputMethod::Action a, int cursorPosition)```</span>
  ///
  ///
  pub fn invoke_action(&mut self, a: ::input_method::Action, cursor_position: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QInputMethod_invokeAction(self as *mut ::input_method::InputMethod, a, cursor_position) }
  }

  /// C++ method: <span style='color: green;'>```bool QInputMethod::isAnimating() const```</span>
  ///
  ///
  pub fn is_animating(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QInputMethod_isAnimating(self as *const ::input_method::InputMethod) }
  }

  /// C++ method: <span style='color: green;'>```bool QInputMethod::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QInputMethod_isVisible(self as *const ::input_method::InputMethod) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QInputMethod::keyboardRectangle() const```</span>
  ///
  ///
  pub fn keyboard_rectangle(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_keyboardRectangle_to_output(self as *const ::input_method::InputMethod,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLocale QInputMethod::locale() const```</span>
  ///
  ///
  pub fn locale(&self) -> ::qt_core::locale::Locale {
    {
      let mut object: ::qt_core::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_locale_to_output(self as *const ::input_method::InputMethod, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QInputMethod::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QInputMethod_metaObject(self as *const ::input_method::InputMethod) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QInputMethod::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QInputMethod_qt_metacall(self as *mut ::input_method::InputMethod,
                                             arg1 as *const ::qt_core::meta_object::Call,
                                             arg2,
                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QInputMethod::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QInputMethod_qt_metacast(self as *mut ::input_method::InputMethod, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QVariant QInputMethod::queryFocusObject(Qt::InputMethodQuery query, QVariant argument)```</span>
  ///
  ///
  pub fn query_focus_object(query: &::qt_core::qt::InputMethodQuery,
                            argument: &::qt_core::variant::Variant)
                            -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethod_queryFocusObject_to_output(query as *const ::qt_core::qt::InputMethodQuery,
                                                                argument as *const ::qt_core::variant::Variant,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QInputMethod::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_gui_c_QInputMethod_reset(self as *mut ::input_method::InputMethod) }
  }

  /// C++ method: <span style='color: green;'>```void QInputMethod::setInputItemRectangle(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_input_item_rectangle(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QInputMethod_setInputItemRectangle(self as *mut ::input_method::InputMethod,
                                                         rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputMethod::setInputItemTransform(const QTransform& transform)```</span>
  ///
  ///
  pub fn set_input_item_transform(&mut self, transform: &::transform::Transform) {
    unsafe {
      ::ffi::qt_gui_c_QInputMethod_setInputItemTransform(self as *mut ::input_method::InputMethod,
                                                         transform as *const ::transform::Transform)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputMethod::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_gui_c_QInputMethod_setVisible(self as *mut ::input_method::InputMethod, visible) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QInputMethod::show()```</span>
  ///
  ///
  pub fn show(&mut self) {
    unsafe { ::ffi::qt_gui_c_QInputMethod_show(self as *mut ::input_method::InputMethod) }
  }

  /// C++ method: <span style='color: green;'>```static QString QInputMethod::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QInputMethod_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QInputMethod::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QInputMethod_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `InputMethod`.
  pub struct Signals<'a>(&'a ::input_method::InputMethod);
  /// Represents a built-in Qt signal `QInputMethod::animatingChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().animating_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct AnimatingChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for AnimatingChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2animatingChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AnimatingChanged<'a> {}
  /// Represents a built-in Qt signal `QInputMethod::cursorRectangleChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().cursor_rectangle_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct CursorRectangleChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for CursorRectangleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cursorRectangleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CursorRectangleChanged<'a> {}
  /// Represents a built-in Qt signal `QInputMethod::anchorRectangleChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().anchor_rectangle_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct AnchorRectangleChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for AnchorRectangleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2anchorRectangleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AnchorRectangleChanged<'a> {}
  /// Represents a built-in Qt signal `QInputMethod::inputDirectionChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().input_direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct InputDirectionChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for InputDirectionChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::LayoutDirection,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2inputDirectionChanged(Qt::LayoutDirection)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for InputDirectionChanged<'a> {}
  /// Represents a built-in Qt signal `QInputMethod::objectNameChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct ObjectNameChanged<'a>(&'a ::input_method::InputMethod);
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
  /// Represents a built-in Qt signal `QInputMethod::localeChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().locale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct LocaleChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for LocaleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2localeChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LocaleChanged<'a> {}
  /// Represents a built-in Qt signal `QInputMethod::keyboardRectangleChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().keyboard_rectangle_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct KeyboardRectangleChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for KeyboardRectangleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2keyboardRectangleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for KeyboardRectangleChanged<'a> {}
  /// Represents a built-in Qt signal `QInputMethod::inputItemClipRectangleChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().input_item_clip_rectangle_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct InputItemClipRectangleChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for InputItemClipRectangleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2inputItemClipRectangleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for InputItemClipRectangleChanged<'a> {}
  /// Represents a built-in Qt signal `QInputMethod::visibleChanged`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.signals().visible_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct VisibleChanged<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for VisibleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibleChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QInputMethod::animatingChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn animating_changed(&self) -> AnimatingChanged {
      AnimatingChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::cursorRectangleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cursor_rectangle_changed(&self) -> CursorRectangleChanged {
      CursorRectangleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::anchorRectangleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn anchor_rectangle_changed(&self) -> AnchorRectangleChanged {
      AnchorRectangleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::inputDirectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn input_direction_changed(&self) -> InputDirectionChanged {
      InputDirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::localeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn locale_changed(&self) -> LocaleChanged {
      LocaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::keyboardRectangleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn keyboard_rectangle_changed(&self) -> KeyboardRectangleChanged {
      KeyboardRectangleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::inputItemClipRectangleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn input_item_clip_rectangle_changed(&self) -> InputItemClipRectangleChanged {
      InputItemClipRectangleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputMethod::visibleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visible_changed(&self) -> VisibleChanged {
      VisibleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `InputMethod`.
  pub struct Slots<'a>(&'a ::input_method::InputMethod);
  /// Represents a built-in Qt slot `QInputMethod::commit`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.slots().commit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct Commit<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for Commit<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1commit()\0"
    }
  }
  /// Represents a built-in Qt slot `QInputMethod::show`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct Show<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QInputMethod::reset`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct Reset<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QInputMethod::hide`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct Hide<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QInputMethod::invokeAction`.
  ///
  /// An object of this type can be created from `InputMethod` with `object.slots().invoke_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputMethod` object.
  pub struct InvokeAction<'a>(&'a ::input_method::InputMethod);
  impl<'a> ::qt_core::connection::Receiver for InvokeAction<'a> {
    type Arguments = (::input_method::Action, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1invokeAction(QInputMethod::Action,int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QInputMethod::commit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit(&self) -> Commit {
      Commit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputMethod::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputMethod::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputMethod::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputMethod::invokeAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn invoke_action(&self) -> InvokeAction {
      InvokeAction(self.0)
    }
  }
  impl ::input_method::InputMethod {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::input_method::InputMethod {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputMethod_G_static_cast_QObject_ptr(self as *mut ::input_method::InputMethod) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethod_G_static_cast_QObject_ptr(self as *const ::input_method::InputMethod as *mut ::input_method::InputMethod) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_method::InputMethod> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_method::InputMethod {
    let ffi_result =
      ::ffi::qt_gui_c_QInputMethod_G_static_cast_QInputMethod_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_method::InputMethod {
    let ffi_result = ::ffi::qt_gui_c_QInputMethod_G_static_cast_QInputMethod_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_method::InputMethod {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethod_G_static_cast_QObject_ptr(self as *const ::input_method::InputMethod as *mut ::input_method::InputMethod) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_method::InputMethod {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputMethod_G_static_cast_QObject_ptr(self as *mut ::input_method::InputMethod) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
