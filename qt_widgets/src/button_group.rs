/// C++ type: <span style='color: green;'>```QButtonGroup```</span>
#[repr(C)]
pub struct ButtonGroup(u8);

impl ButtonGroup {
  /// C++ method: <span style='color: green;'>```QButtonGroup::addButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_button(&mut self, *mut ::abstract_button::AbstractButton) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QButtonGroup::addButton(QAbstractButton* arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_button(&mut self, (*mut ::abstract_button::AbstractButton, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QButtonGroup::addButton(QAbstractButton* arg1, int id = ?)```</span>
  ///
  ///
  pub unsafe fn add_button<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ButtonGroupAddButtonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractButton* QButtonGroup::button(int id) const```</span>
  ///
  ///
  pub fn button(&self, id: ::libc::c_int) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QButtonGroup_button(self as *const ::button_group::ButtonGroup, id) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*> QButtonGroup::buttons() const```</span>
  ///
  ///
  pub fn buttons(&self) -> ::list::ListAbstractButtonMutPtr {
    {
      let mut object: ::list::ListAbstractButtonMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QButtonGroup_buttons_to_output(self as *const ::button_group::ButtonGroup, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QButtonGroup::checkedButton() const```</span>
  ///
  ///
  pub fn checked_button(&self) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QButtonGroup_checkedButton(self as *const ::button_group::ButtonGroup) }
  }

  /// C++ method: <span style='color: green;'>```int QButtonGroup::checkedId() const```</span>
  ///
  ///
  pub fn checked_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QButtonGroup_checkedId(self as *const ::button_group::ButtonGroup) }
  }

  /// C++ method: <span style='color: green;'>```bool QButtonGroup::exclusive() const```</span>
  ///
  ///
  pub fn exclusive(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QButtonGroup_exclusive(self as *const ::button_group::ButtonGroup) }
  }

  /// C++ method: <span style='color: green;'>```int QButtonGroup::id(QAbstractButton* button) const```</span>
  ///
  ///
  pub unsafe fn id(&self, button: *mut ::abstract_button::AbstractButton) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QButtonGroup_id(self as *const ::button_group::ButtonGroup, button)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QButtonGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QButtonGroup_metaObject(self as *const ::button_group::ButtonGroup) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QButtonGroup::QButtonGroup()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::button_group::ButtonGroup> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QButtonGroup_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QButtonGroup::QButtonGroup(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::button_group::ButtonGroup> {
    let ffi_result = ::ffi::qt_widgets_c_QButtonGroup_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QButtonGroup::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QButtonGroup_qt_metacall(self as *mut ::button_group::ButtonGroup,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QButtonGroup::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QButtonGroup_qt_metacast(self as *mut ::button_group::ButtonGroup, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QButtonGroup::removeButton(QAbstractButton* arg1)```</span>
  ///
  ///
  pub unsafe fn remove_button(&mut self, arg1: *mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QButtonGroup_removeButton(self as *mut ::button_group::ButtonGroup, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QButtonGroup::setExclusive(bool arg1)```</span>
  ///
  ///
  pub fn set_exclusive(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QButtonGroup_setExclusive(self as *mut ::button_group::ButtonGroup, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QButtonGroup::setId(QAbstractButton* button, int id)```</span>
  ///
  ///
  pub unsafe fn set_id(&mut self, button: *mut ::abstract_button::AbstractButton, id: ::libc::c_int) {
    ::ffi::qt_widgets_c_QButtonGroup_setId(self as *mut ::button_group::ButtonGroup, button, id)
  }

  /// C++ method: <span style='color: green;'>```static QString QButtonGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QButtonGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QButtonGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QButtonGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::button_group::ButtonGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QButtonGroup_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ButtonGroup`.
  pub struct Signals<'a>(&'a ::button_group::ButtonGroup);
  /// Represents a built-in Qt signal `QButtonGroup::buttonToggled`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_toggled_abstract_button_mut_ptr_bool()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonToggledAbstractButtonMutPtrBool<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonToggledAbstractButtonMutPtrBool<'a> {
    type Arguments = (*mut ::abstract_button::AbstractButton, bool);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonToggled(QAbstractButton*,bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonToggledAbstractButtonMutPtrBool<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::buttonToggled`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_toggled_c_int_bool()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonToggledCIntBool<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonToggledCIntBool<'a> {
    type Arguments = (::libc::c_int, bool);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonToggled(int,bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonToggledCIntBool<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::buttonClicked`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_clicked_abstract_button_mut_ptr()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonClickedAbstractButtonMutPtr<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonClickedAbstractButtonMutPtr<'a> {
    type Arguments = (*mut ::abstract_button::AbstractButton,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonClicked(QAbstractButton*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonClickedAbstractButtonMutPtr<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::buttonClicked`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_clicked_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonClickedCInt<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonClickedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonClicked(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonClickedCInt<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::buttonPressed`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_pressed_abstract_button_mut_ptr()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonPressedAbstractButtonMutPtr<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonPressedAbstractButtonMutPtr<'a> {
    type Arguments = (*mut ::abstract_button::AbstractButton,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonPressed(QAbstractButton*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonPressedAbstractButtonMutPtr<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::buttonPressed`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_pressed_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonPressedCInt<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonPressedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonPressed(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonPressedCInt<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::buttonReleased`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_released_abstract_button_mut_ptr()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonReleasedAbstractButtonMutPtr<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonReleasedAbstractButtonMutPtr<'a> {
    type Arguments = (*mut ::abstract_button::AbstractButton,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonReleased(QAbstractButton*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonReleasedAbstractButtonMutPtr<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::buttonReleased`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().button_released_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ButtonReleasedCInt<'a>(&'a ::button_group::ButtonGroup);
  impl<'a> ::qt_core::connection::Receiver for ButtonReleasedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonReleased(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonReleasedCInt<'a> {}
  /// Represents a built-in Qt signal `QButtonGroup::objectNameChanged`.
  ///
  /// An object of this type can be created from `ButtonGroup` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonGroup` object.
  pub struct ObjectNameChanged<'a>(&'a ::button_group::ButtonGroup);
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
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonToggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_toggled_abstract_button_mut_ptr_bool(&self) -> ButtonToggledAbstractButtonMutPtrBool {
      ButtonToggledAbstractButtonMutPtrBool(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonToggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_toggled_c_int_bool(&self) -> ButtonToggledCIntBool {
      ButtonToggledCIntBool(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_clicked_abstract_button_mut_ptr(&self) -> ButtonClickedAbstractButtonMutPtr {
      ButtonClickedAbstractButtonMutPtr(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_clicked_c_int(&self) -> ButtonClickedCInt {
      ButtonClickedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_pressed_abstract_button_mut_ptr(&self) -> ButtonPressedAbstractButtonMutPtr {
      ButtonPressedAbstractButtonMutPtr(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_pressed_c_int(&self) -> ButtonPressedCInt {
      ButtonPressedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonReleased`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_released_abstract_button_mut_ptr(&self) -> ButtonReleasedAbstractButtonMutPtr {
      ButtonReleasedAbstractButtonMutPtr(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::buttonReleased`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_released_c_int(&self) -> ButtonReleasedCInt {
      ButtonReleasedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QButtonGroup::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::button_group::ButtonGroup {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::button_group::ButtonGroup {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QButtonGroup_G_static_cast_QObject_ptr(self as *mut ::button_group::ButtonGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QButtonGroup_G_static_cast_QObject_ptr(self as *const ::button_group::ButtonGroup as *mut ::button_group::ButtonGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::button_group::ButtonGroup> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::button_group::ButtonGroup {
    let ffi_result =
      ::ffi::qt_widgets_c_QButtonGroup_G_static_cast_QButtonGroup_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::button_group::ButtonGroup {
    let ffi_result = ::ffi::qt_widgets_c_QButtonGroup_G_static_cast_QButtonGroup_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::button_group::ButtonGroup {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QButtonGroup_G_static_cast_QObject_ptr(self as *const ::button_group::ButtonGroup as *mut ::button_group::ButtonGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::button_group::ButtonGroup {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QButtonGroup_G_static_cast_QObject_ptr(self as *mut ::button_group::ButtonGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ButtonGroup::add_button](../struct.ButtonGroup.html#method.add_button) method.
  pub trait ButtonGroupAddButtonArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::button_group::ButtonGroup) -> ();
  }
  impl<'largs> ButtonGroupAddButtonArgs<'largs> for *mut ::abstract_button::AbstractButton {
    unsafe fn exec(self, original_self: &'largs mut ::button_group::ButtonGroup) -> () {
      let arg1 = self;
      ::ffi::qt_widgets_c_QButtonGroup_addButton_arg1(original_self as *mut ::button_group::ButtonGroup, arg1)
    }
  }
  impl<'largs> ButtonGroupAddButtonArgs<'largs> for (*mut ::abstract_button::AbstractButton, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::button_group::ButtonGroup) -> () {
      let arg1 = self.0;
      let id = self.1;
      ::ffi::qt_widgets_c_QButtonGroup_addButton_arg1_id(original_self as *mut ::button_group::ButtonGroup, arg1, id)
    }
  }
}
