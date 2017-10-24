/// C++ type: <span style='color: green;'>```QActionGroup```</span>
#[repr(C)]
pub struct ActionGroup(u8);

impl ActionGroup {
  /// C++ method: <span style='color: green;'>```QList<QAction*> QActionGroup::actions() const```</span>
  ///
  ///
  pub fn actions(&self) -> ::list::ListActionMutPtr {
    {
      let mut object: ::list::ListActionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QActionGroup_actions_to_output(self as *const ::action_group::ActionGroup, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QActionGroup::addAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_action(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QActionGroup::addAction(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_action(&mut self, &::qt_core::string::String) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QActionGroup::addAction(const QString& text)```</span>
  ///
  ///
  pub fn add_action<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::ActionGroupAddActionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QActionGroup::addAction(QAction* a)```</span>
  ///
  ///
  pub unsafe fn add_action_unsafe(&mut self, a: *mut ::action::Action) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QActionGroup_addAction_a(self as *mut ::action_group::ActionGroup, a)
  }

  /// C++ method: <span style='color: green;'>```QAction* QActionGroup::checkedAction() const```</span>
  ///
  ///
  pub fn checked_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_checkedAction(self as *const ::action_group::ActionGroup) }
  }

  /// C++ method: <span style='color: green;'>```bool QActionGroup::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_isEnabled(self as *const ::action_group::ActionGroup) }
  }

  /// C++ method: <span style='color: green;'>```bool QActionGroup::isExclusive() const```</span>
  ///
  ///
  pub fn is_exclusive(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_isExclusive(self as *const ::action_group::ActionGroup) }
  }

  /// C++ method: <span style='color: green;'>```bool QActionGroup::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_isVisible(self as *const ::action_group::ActionGroup) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QActionGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_metaObject(self as *const ::action_group::ActionGroup) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QActionGroup::QActionGroup(QObject* parent)```</span>
  ///
  ///
  pub unsafe fn new(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::action_group::ActionGroup> {
    let ffi_result = ::ffi::qt_widgets_c_QActionGroup_new(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QActionGroup::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QActionGroup_qt_metacall(self as *mut ::action_group::ActionGroup,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QActionGroup::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QActionGroup_qt_metacast(self as *mut ::action_group::ActionGroup, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QActionGroup::removeAction(QAction* a)```</span>
  ///
  ///
  pub unsafe fn remove_action(&mut self, a: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QActionGroup_removeAction(self as *mut ::action_group::ActionGroup, a)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QActionGroup::setDisabled(bool b)```</span>
  ///
  ///
  pub fn set_disabled(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_setDisabled(self as *mut ::action_group::ActionGroup, b) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QActionGroup::setEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_setEnabled(self as *mut ::action_group::ActionGroup, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QActionGroup::setExclusive(bool arg1)```</span>
  ///
  ///
  pub fn set_exclusive(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_setExclusive(self as *mut ::action_group::ActionGroup, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QActionGroup::setVisible(bool arg1)```</span>
  ///
  ///
  pub fn set_visible(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QActionGroup_setVisible(self as *mut ::action_group::ActionGroup, arg1) }
  }

  /// C++ method: <span style='color: green;'>```static QString QActionGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QActionGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QActionGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QActionGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::action_group::ActionGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QActionGroup_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ActionGroup`.
  pub struct Signals<'a>(&'a ::action_group::ActionGroup);
  /// Represents a built-in Qt signal `QActionGroup::hovered`.
  ///
  /// An object of this type can be created from `ActionGroup` with `object.signals().hovered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionGroup` object.
  pub struct Hovered<'a>(&'a ::action_group::ActionGroup);
  impl<'a> ::qt_core::connection::Receiver for Hovered<'a> {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hovered(QAction*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Hovered<'a> {}
  /// Represents a built-in Qt signal `QActionGroup::triggered`.
  ///
  /// An object of this type can be created from `ActionGroup` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionGroup` object.
  pub struct Triggered<'a>(&'a ::action_group::ActionGroup);
  impl<'a> ::qt_core::connection::Receiver for Triggered<'a> {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2triggered(QAction*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Triggered<'a> {}
  /// Represents a built-in Qt signal `QActionGroup::objectNameChanged`.
  ///
  /// An object of this type can be created from `ActionGroup` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionGroup` object.
  pub struct ObjectNameChanged<'a>(&'a ::action_group::ActionGroup);
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
    /// Returns an object representing a built-in Qt signal `QActionGroup::hovered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hovered(&self) -> Hovered {
      Hovered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QActionGroup::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QActionGroup::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ActionGroup`.
  pub struct Slots<'a>(&'a ::action_group::ActionGroup);
  /// Represents a built-in Qt slot `QActionGroup::setExclusive`.
  ///
  /// An object of this type can be created from `ActionGroup` with `object.slots().set_exclusive()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionGroup` object.
  pub struct SetExclusive<'a>(&'a ::action_group::ActionGroup);
  impl<'a> ::qt_core::connection::Receiver for SetExclusive<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setExclusive(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QActionGroup::setDisabled`.
  ///
  /// An object of this type can be created from `ActionGroup` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionGroup` object.
  pub struct SetDisabled<'a>(&'a ::action_group::ActionGroup);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QActionGroup::setEnabled`.
  ///
  /// An object of this type can be created from `ActionGroup` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionGroup` object.
  pub struct SetEnabled<'a>(&'a ::action_group::ActionGroup);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QActionGroup::setVisible`.
  ///
  /// An object of this type can be created from `ActionGroup` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionGroup` object.
  pub struct SetVisible<'a>(&'a ::action_group::ActionGroup);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QActionGroup::setExclusive`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_exclusive(&self) -> SetExclusive {
      SetExclusive(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QActionGroup::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QActionGroup::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QActionGroup::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
  }
  impl ::action_group::ActionGroup {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::action_group::ActionGroup {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QActionGroup_G_static_cast_QObject_ptr(self as *mut ::action_group::ActionGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QActionGroup_G_static_cast_QObject_ptr(self as *const ::action_group::ActionGroup as *mut ::action_group::ActionGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::action_group::ActionGroup> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::action_group::ActionGroup {
    let ffi_result =
      ::ffi::qt_widgets_c_QActionGroup_G_static_cast_QActionGroup_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::action_group::ActionGroup {
    let ffi_result = ::ffi::qt_widgets_c_QActionGroup_G_static_cast_QActionGroup_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::action_group::ActionGroup {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QActionGroup_G_static_cast_QObject_ptr(self as *const ::action_group::ActionGroup as *mut ::action_group::ActionGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::action_group::ActionGroup {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QActionGroup_G_static_cast_QObject_ptr(self as *mut ::action_group::ActionGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ActionGroup::add_action](../struct.ActionGroup.html#method.add_action) method.
  pub trait ActionGroupAddActionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::action_group::ActionGroup) -> *mut ::action::Action;
  }
  impl<'largs> ActionGroupAddActionArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::action_group::ActionGroup) -> *mut ::action::Action {
      let icon = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QActionGroup_addAction_icon_text(original_self as *mut ::action_group::ActionGroup,
                                                             icon as *const ::qt_gui::icon::Icon,
                                                             text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ActionGroupAddActionArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::action_group::ActionGroup) -> *mut ::action::Action {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QActionGroup_addAction_text(original_self as *mut ::action_group::ActionGroup,
                                                        text as *const ::qt_core::string::String)
      }
    }
  }
}
