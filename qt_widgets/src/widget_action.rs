/// C++ type: <span style='color: green;'>```QWidgetAction```</span>
#[repr(C)]
pub struct WidgetAction(u8);

impl WidgetAction {
  /// C++ method: <span style='color: green;'>```QWidget* QWidgetAction::defaultWidget() const```</span>
  ///
  ///
  pub fn default_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidgetAction_defaultWidget(self as *const ::widget_action::WidgetAction) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QWidgetAction::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QWidgetAction_metaObject(self as *const ::widget_action::WidgetAction) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QWidgetAction::QWidgetAction(QObject* parent)```</span>
  ///
  ///
  pub unsafe fn new(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::widget_action::WidgetAction> {
    let ffi_result = ::ffi::qt_widgets_c_QWidgetAction_new(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QWidgetAction::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QWidgetAction_qt_metacall(self as *mut ::widget_action::WidgetAction,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QWidgetAction::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QWidgetAction_qt_metacast(self as *mut ::widget_action::WidgetAction, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QWidgetAction::releaseWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn release_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWidgetAction_releaseWidget(self as *mut ::widget_action::WidgetAction, widget)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidgetAction::requestWidget(QWidget* parent)```</span>
  ///
  ///
  pub unsafe fn request_widget(&mut self, parent: *mut ::widget::Widget) -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QWidgetAction_requestWidget(self as *mut ::widget_action::WidgetAction, parent)
  }

  /// C++ method: <span style='color: green;'>```void QWidgetAction::setDefaultWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn set_default_widget(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWidgetAction_setDefaultWidget(self as *mut ::widget_action::WidgetAction, w)
  }

  /// C++ method: <span style='color: green;'>```static QString QWidgetAction::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWidgetAction_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QWidgetAction::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWidgetAction_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::widget_action::WidgetAction {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QWidgetAction_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `WidgetAction`.
  pub struct Signals<'a>(&'a ::widget_action::WidgetAction);
  /// Represents a built-in Qt signal `QWidgetAction::changed`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct Changed<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for Changed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2changed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Changed<'a> {}
  /// Represents a built-in Qt signal `QWidgetAction::hovered`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.signals().hovered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct Hovered<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for Hovered<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hovered()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Hovered<'a> {}
  /// Represents a built-in Qt signal `QWidgetAction::toggled`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.signals().toggled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct Toggled<'a>(&'a ::widget_action::WidgetAction);
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
  /// Represents a built-in Qt signal `QWidgetAction::triggered`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct Triggered<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for Triggered<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2triggered(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Triggered<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QWidgetAction::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWidgetAction::hovered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hovered(&self) -> Hovered {
      Hovered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWidgetAction::toggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggled(&self) -> Toggled {
      Toggled(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWidgetAction::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `WidgetAction`.
  pub struct Slots<'a>(&'a ::widget_action::WidgetAction);
  /// Represents a built-in Qt slot `QWidgetAction::setChecked`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.slots().set_checked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct SetChecked<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for SetChecked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setChecked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidgetAction::hover`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.slots().hover()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct Hover<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for Hover<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hover()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidgetAction::trigger`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.slots().trigger()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct Trigger<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for Trigger<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1trigger()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidgetAction::toggle`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.slots().toggle()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct Toggle<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for Toggle<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toggle()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidgetAction::setEnabled`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct SetEnabled<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidgetAction::setVisible`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct SetVisible<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidgetAction::setDisabled`.
  ///
  /// An object of this type can be created from `WidgetAction` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WidgetAction` object.
  pub struct SetDisabled<'a>(&'a ::widget_action::WidgetAction);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QWidgetAction::setChecked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_checked(&self) -> SetChecked {
      SetChecked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidgetAction::hover`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hover(&self) -> Hover {
      Hover(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidgetAction::trigger`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn trigger(&self) -> Trigger {
      Trigger(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidgetAction::toggle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggle(&self) -> Toggle {
      Toggle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidgetAction::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidgetAction::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidgetAction::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
  }
  impl ::widget_action::WidgetAction {
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

impl ::cpp_utils::DynamicCast<::widget_action::WidgetAction> for ::action::Action {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::widget_action::WidgetAction> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWidgetAction_G_dynamic_cast_QWidgetAction_ptr(self as *mut ::action::Action) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::widget_action::WidgetAction> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidgetAction_G_dynamic_cast_QWidgetAction_ptr(self as *const ::action::Action as *mut ::action::Action) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::widget_action::WidgetAction {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QObject_ptr(self as *mut ::widget_action::WidgetAction)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QObject_ptr(self as *const ::widget_action::WidgetAction as *mut ::widget_action::WidgetAction) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::action::Action> for ::widget_action::WidgetAction {
  fn static_cast_mut(&mut self) -> &mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QAction_ptr(self as *mut ::widget_action::WidgetAction)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::action::Action {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QAction_ptr(self as *const ::widget_action::WidgetAction as *mut ::widget_action::WidgetAction) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::widget_action::WidgetAction> for ::action::Action {
  unsafe fn static_cast_mut(&mut self) -> &mut ::widget_action::WidgetAction {
    let ffi_result =
      ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QAction(self as *mut ::action::Action);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::widget_action::WidgetAction {
    let ffi_result = ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QAction(self as *const ::action::Action as *mut ::action::Action);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::widget_action::WidgetAction> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::widget_action::WidgetAction {
    let ffi_result = ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::widget_action::WidgetAction {
    let ffi_result = ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::widget_action::WidgetAction {
  type Target = ::action::Action;
  fn deref(&self) -> &::action::Action {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QAction_ptr(self as *const ::widget_action::WidgetAction as *mut ::widget_action::WidgetAction) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::widget_action::WidgetAction {
  fn deref_mut(&mut self) -> &mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWidgetAction_G_static_cast_QAction_ptr(self as *mut ::widget_action::WidgetAction)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
