/// C++ type: <span style='color: green;'>```QShortcut```</span>
#[repr(C)]
pub struct Shortcut(u8);

impl Shortcut {
  /// C++ method: <span style='color: green;'>```bool QShortcut::autoRepeat() const```</span>
  ///
  ///
  pub fn auto_repeat(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QShortcut_autoRepeat(self as *const ::shortcut::Shortcut) }
  }

  /// C++ method: <span style='color: green;'>```int QShortcut::id() const```</span>
  ///
  ///
  pub fn id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QShortcut_id(self as *const ::shortcut::Shortcut) }
  }

  /// C++ method: <span style='color: green;'>```bool QShortcut::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QShortcut_isEnabled(self as *const ::shortcut::Shortcut) }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence QShortcut::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::qt_gui::key_sequence::KeySequence {
    {
      let mut object: ::qt_gui::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QShortcut_key_to_output(self as *const ::shortcut::Shortcut, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QShortcut::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QShortcut_metaObject(self as *const ::shortcut::Shortcut) }
  }

  /// C++ method: <span style='color: green;'>```QShortcut::QShortcut```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::shortcut::Shortcut>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QShortcut::QShortcut(QWidget* parent)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::shortcut::Shortcut>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QShortcut::QShortcut(const QKeySequence& key, QWidget* parent)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget, *const ::libc::c_char)) -> ::cpp_utils::CppBox<::shortcut::Shortcut>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QShortcut::QShortcut(const QKeySequence& key, QWidget* parent, const char* member = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget, *const ::libc::c_char, *const ::libc::c_char)) -> ::cpp_utils::CppBox<::shortcut::Shortcut>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QShortcut::QShortcut(const QKeySequence& key, QWidget* parent, const char* member = ?, const char* ambiguousMember = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget, *const ::libc::c_char, *const ::libc::c_char, &::qt_core::qt::ShortcutContext)) -> ::cpp_utils::CppBox<::shortcut::Shortcut>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QShortcut::QShortcut(const QKeySequence& key, QWidget* parent, const char* member = ?, const char* ambiguousMember = ?, Qt::ShortcutContext context = ?)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::shortcut::Shortcut>
    where Args: overloading::ShortcutNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QWidget* QShortcut::parentWidget() const```</span>
  ///
  ///
  pub fn parent_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QShortcut_parentWidget(self as *const ::shortcut::Shortcut) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QShortcut::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QShortcut_qt_metacall(self as *mut ::shortcut::Shortcut,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QShortcut::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QShortcut_qt_metacast(self as *mut ::shortcut::Shortcut, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QShortcut::setAutoRepeat(bool on)```</span>
  ///
  ///
  pub fn set_auto_repeat(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QShortcut_setAutoRepeat(self as *mut ::shortcut::Shortcut, on) }
  }

  /// C++ method: <span style='color: green;'>```void QShortcut::setContext(Qt::ShortcutContext context)```</span>
  ///
  ///
  pub fn set_context(&mut self, context: &::qt_core::qt::ShortcutContext) {
    unsafe {
      ::ffi::qt_widgets_c_QShortcut_setContext(self as *mut ::shortcut::Shortcut,
                                               context as *const ::qt_core::qt::ShortcutContext)
    }
  }

  /// C++ method: <span style='color: green;'>```void QShortcut::setEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QShortcut_setEnabled(self as *mut ::shortcut::Shortcut, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QShortcut::setKey(const QKeySequence& key)```</span>
  ///
  ///
  pub fn set_key(&mut self, key: &::qt_gui::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_widgets_c_QShortcut_setKey(self as *mut ::shortcut::Shortcut,
                                           key as *const ::qt_gui::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QShortcut::setWhatsThis(const QString& text)```</span>
  ///
  ///
  pub fn set_whats_this(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QShortcut_setWhatsThis(self as *mut ::shortcut::Shortcut,
                                                 text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QShortcut::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QShortcut_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QShortcut::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QShortcut_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QShortcut::whatsThis() const```</span>
  ///
  ///
  pub fn whats_this(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QShortcut_whatsThis_to_output(self as *const ::shortcut::Shortcut, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::shortcut::Shortcut {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QShortcut_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Shortcut`.
  pub struct Signals<'a>(&'a ::shortcut::Shortcut);
  /// Represents a built-in Qt signal `QShortcut::activated`.
  ///
  /// An object of this type can be created from `Shortcut` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Shortcut` object.
  pub struct Activated<'a>(&'a ::shortcut::Shortcut);
  impl<'a> ::qt_core::connection::Receiver for Activated<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Activated<'a> {}
  /// Represents a built-in Qt signal `QShortcut::activatedAmbiguously`.
  ///
  /// An object of this type can be created from `Shortcut` with `object.signals().activated_ambiguously()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Shortcut` object.
  pub struct ActivatedAmbiguously<'a>(&'a ::shortcut::Shortcut);
  impl<'a> ::qt_core::connection::Receiver for ActivatedAmbiguously<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activatedAmbiguously()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActivatedAmbiguously<'a> {}
  /// Represents a built-in Qt signal `QShortcut::objectNameChanged`.
  ///
  /// An object of this type can be created from `Shortcut` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Shortcut` object.
  pub struct ObjectNameChanged<'a>(&'a ::shortcut::Shortcut);
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
    /// Returns an object representing a built-in Qt signal `QShortcut::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QShortcut::activatedAmbiguously`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated_ambiguously(&self) -> ActivatedAmbiguously {
      ActivatedAmbiguously(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QShortcut::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::shortcut::Shortcut {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::shortcut::Shortcut {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QShortcut_G_static_cast_QObject_ptr(self as *mut ::shortcut::Shortcut) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QShortcut_G_static_cast_QObject_ptr(self as *const ::shortcut::Shortcut as *mut ::shortcut::Shortcut) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shortcut::Shortcut> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shortcut::Shortcut {
    let ffi_result =
      ::ffi::qt_widgets_c_QShortcut_G_static_cast_QShortcut_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shortcut::Shortcut {
    let ffi_result = ::ffi::qt_widgets_c_QShortcut_G_static_cast_QShortcut_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::shortcut::Shortcut {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QShortcut_G_static_cast_QObject_ptr(self as *const ::shortcut::Shortcut as *mut ::shortcut::Shortcut) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::shortcut::Shortcut {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QShortcut_G_static_cast_QObject_ptr(self as *mut ::shortcut::Shortcut) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Shortcut::new](../struct.Shortcut.html#method.new) method.
  pub trait ShortcutNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shortcut::Shortcut>;
  }
  impl<'a> ShortcutNewArgs for (&'a ::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shortcut::Shortcut> {
      let key = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QShortcut_new_key_parent(key as *const ::qt_gui::key_sequence::KeySequence, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ShortcutNewArgs for (&'a ::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shortcut::Shortcut> {
      let key = self.0;
      let parent = self.1;
      let member = self.2;
      let ffi_result =
        ::ffi::qt_widgets_c_QShortcut_new_key_parent_member(key as *const ::qt_gui::key_sequence::KeySequence,
                                                            parent,
                                                            member);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ShortcutNewArgs
    for (&'a ::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget, *const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shortcut::Shortcut> {
      let key = self.0;
      let parent = self.1;
      let member = self.2;
      let ambiguous_member = self.3;
      let ffi_result = ::ffi::qt_widgets_c_QShortcut_new_key_parent_member_ambiguousMember(key as *const ::qt_gui::key_sequence::KeySequence, parent, member, ambiguous_member);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ShortcutNewArgs
    for (&'a ::qt_gui::key_sequence::KeySequence,
                                    *mut ::widget::Widget,
                                    *const ::libc::c_char,
                                    *const ::libc::c_char,
                                    &'a ::qt_core::qt::ShortcutContext) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shortcut::Shortcut> {
      let key = self.0;
      let parent = self.1;
      let member = self.2;
      let ambiguous_member = self.3;
      let context = self.4;
      let ffi_result = ::ffi::qt_widgets_c_QShortcut_new_key_parent_member_ambiguousMember_context(key as *const ::qt_gui::key_sequence::KeySequence, parent, member, ambiguous_member, context as *const ::qt_core::qt::ShortcutContext);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ShortcutNewArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shortcut::Shortcut> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QShortcut_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
