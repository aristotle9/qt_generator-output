/// C++ type: <span style='color: green;'>```QInputMethodEvent::Attribute```</span>
#[repr(C)]
pub struct Attribute([u8; ::type_sizes::QT_GUI_INPUT_METHOD_EVENT_ATTRIBUTE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Attribute {
  unsafe fn new_uninitialized() -> Attribute {
    Attribute(::std::mem::uninitialized())
  }
}

impl Attribute {
  /// C++ method: <span style='color: green;'>```int QInputMethodEvent::Attribute::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QInputMethodEvent_Attribute_length(self as *const ::input_method_event::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute::Attribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((::input_method_event::AttributeType, ::libc::c_int, ::libc::c_int)) -> ::input_method_event::Attribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QInputMethodEvent::Attribute::Attribute(QInputMethodEvent::AttributeType typ, int s, int l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::input_method_event::AttributeType, ::libc::c_int, ::libc::c_int, &::qt_core::variant::Variant)) -> ::input_method_event::Attribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QInputMethodEvent::Attribute::Attribute(QInputMethodEvent::AttributeType typ, int s, int l, QVariant val)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::input_method_event::Attribute
    where Args: overloading::AttributeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QInputMethodEvent::Attribute::set_length(int value)```</span>
  ///
  ///
  pub fn set_length(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QInputMethodEvent_Attribute_set_length(self as *mut ::input_method_event::Attribute, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputMethodEvent::Attribute::set_start(int value)```</span>
  ///
  ///
  pub fn set_start(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QInputMethodEvent_Attribute_set_start(self as *mut ::input_method_event::Attribute, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputMethodEvent::Attribute::set_type(QInputMethodEvent::AttributeType value)```</span>
  ///
  ///
  pub fn set_type(&mut self, value: ::input_method_event::AttributeType) {
    unsafe { ::ffi::qt_gui_c_QInputMethodEvent_Attribute_set_type(self as *mut ::input_method_event::Attribute, value) }
  }

  /// C++ method: <span style='color: green;'>```void QInputMethodEvent::Attribute::set_value(QVariant value)```</span>
  ///
  ///
  pub fn set_value(&mut self, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_gui_c_QInputMethodEvent_Attribute_set_value(self as *mut ::input_method_event::Attribute,
                                                            value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```int QInputMethodEvent::Attribute::start() const```</span>
  ///
  ///
  pub fn start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QInputMethodEvent_Attribute_start(self as *const ::input_method_event::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::AttributeType QInputMethodEvent::Attribute::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::input_method_event::AttributeType {
    unsafe { ::ffi::qt_gui_c_QInputMethodEvent_Attribute_type(self as *const ::input_method_event::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QInputMethodEvent::Attribute::value() const```</span>
  ///
  ///
  pub fn value<'l0>(&'l0 self) -> &'l0 ::qt_core::variant::Variant {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_Attribute_value(self as *const ::input_method_event::Attribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QInputMethodEvent::Attribute::value_mut()```</span>
  ///
  ///
  pub fn value_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::variant::Variant {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_Attribute_value_mut(self as *mut ::input_method_event::Attribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::input_method_event::Attribute {
  /// C++ method: <span style='color: green;'>```[destructor] void QInputMethodEvent::Attribute::~QInputMethodEvent::Attribute()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QInputMethodEvent_Attribute_destructor(self as *mut ::input_method_event::Attribute) }
  }
}

/// C++ type: <span style='color: green;'>```QInputMethodEvent::AttributeType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AttributeType {
  /// C++ enum variant: <span style='color: green;'>```TextFormat = 0```</span>
  TextFormat = 0,
  /// C++ enum variant: <span style='color: green;'>```Cursor = 1```</span>
  Cursor = 1,
  /// C++ enum variant: <span style='color: green;'>```Language = 2```</span>
  Language = 2,
  /// C++ enum variant: <span style='color: green;'>```Ruby = 3```</span>
  Ruby = 3,
  /// C++ enum variant: <span style='color: green;'>```Selection = 4```</span>
  Selection = 4,
}

/// C++ type: <span style='color: green;'>```QInputMethodEvent```</span>
#[repr(C)]
pub struct InputMethodEvent(u8);

impl InputMethodEvent {
  /// C++ method: <span style='color: green;'>```const QList<QInputMethodEvent::Attribute>& QInputMethodEvent::attributes() const```</span>
  ///
  ///
  pub fn attributes<'l0>(&'l0 self) -> &'l0 ::list::ListInputMethodEventAttribute {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_attributes(self as *const ::input_method_event::InputMethodEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QInputMethodEvent::commitString() const```</span>
  ///
  ///
  pub fn commit_string<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_commitString(self as *const ::input_method_event::InputMethodEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::QInputMethodEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QInputMethodEvent::QInputMethodEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::input_method_event::InputMethodEvent) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::list::ListInputMethodEventAttribute)) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QInputMethodEvent::QInputMethodEvent(const QString& preeditText, const QList<QInputMethodEvent::Attribute>& attributes)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent>
    where Args: overloading::InputMethodEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QString& QInputMethodEvent::preeditString() const```</span>
  ///
  ///
  pub fn preedit_string<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_preeditString(self as *const ::input_method_event::InputMethodEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QInputMethodEvent::replacementLength() const```</span>
  ///
  ///
  pub fn replacement_length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QInputMethodEvent_replacementLength(self as *const ::input_method_event::InputMethodEvent)
    }
  }

  /// C++ method: <span style='color: green;'>```int QInputMethodEvent::replacementStart() const```</span>
  ///
  ///
  pub fn replacement_start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QInputMethodEvent_replacementStart(self as *const ::input_method_event::InputMethodEvent) }
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::setCommitString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_commit_string(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QInputMethodEvent::setCommitString(const QString& commitString)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_commit_string(&mut self, (&::qt_core::string::String, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QInputMethodEvent::setCommitString(const QString& commitString, int replaceFrom = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_commit_string(&mut self, (&::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QInputMethodEvent::setCommitString(const QString& commitString, int replaceFrom = ?, int replaceLength = ?)```</span>
  ///
  ///
  pub fn set_commit_string<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::InputMethodEventSetCommitStringArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::input_method_event::InputMethodEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QInputMethodEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::input_method_event::InputMethodEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QInputMethodEvent_G_static_cast_QEvent_ptr(self as *mut ::input_method_event::InputMethodEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodEvent_G_static_cast_QEvent_ptr(self as *const ::input_method_event::InputMethodEvent as *mut ::input_method_event::InputMethodEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_method_event::InputMethodEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_method_event::InputMethodEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QInputMethodEvent_G_static_cast_QInputMethodEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_method_event::InputMethodEvent {
    let ffi_result = ::ffi::qt_gui_c_QInputMethodEvent_G_static_cast_QInputMethodEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_method_event::InputMethodEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodEvent_G_static_cast_QEvent_ptr(self as *const ::input_method_event::InputMethodEvent as *mut ::input_method_event::InputMethodEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_method_event::InputMethodEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QInputMethodEvent_G_static_cast_QEvent_ptr(self as *mut ::input_method_event::InputMethodEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Attribute::new](../struct.Attribute.html#method.new) method.
  pub trait AttributeNewArgs {
    fn exec(self) -> ::input_method_event::Attribute;
  }
  impl AttributeNewArgs for (::input_method_event::AttributeType, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::input_method_event::Attribute {
      let typ = self.0;
      let s = self.1;
      let l = self.2;
      {
        let mut object: ::input_method_event::Attribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QInputMethodEvent_Attribute_constructor_typ_s_l(typ, s, l, &mut object);
        }
        object
      }
    }
  }
  impl<'a> AttributeNewArgs
    for (::input_method_event::AttributeType, ::libc::c_int, ::libc::c_int, &'a ::qt_core::variant::Variant) {
    fn exec(self) -> ::input_method_event::Attribute {
      let typ = self.0;
      let s = self.1;
      let l = self.2;
      let val = self.3;
      {
        let mut object: ::input_method_event::Attribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QInputMethodEvent_Attribute_constructor_typ_s_l_val(typ, s, l, val as *const ::qt_core::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [InputMethodEvent::new](../struct.InputMethodEvent.html#method.new) method.
  pub trait InputMethodEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent>;
  }
  impl InputMethodEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> InputMethodEventNewArgs for &'a ::input_method_event::InputMethodEvent {
    fn exec(self) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent> {
      let other = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QInputMethodEvent_new_other(other as *const ::input_method_event::InputMethodEvent) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> InputMethodEventNewArgs for (&'a ::qt_core::string::String, &'a ::list::ListInputMethodEventAttribute) {
    fn exec(self) -> ::cpp_utils::CppBox<::input_method_event::InputMethodEvent> {
      let preedit_text = self.0;
      let attributes = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodEvent_new_preeditText_attributes(preedit_text as *const ::qt_core::string::String, attributes as *const ::list::ListInputMethodEventAttribute) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [InputMethodEvent::set_commit_string](../struct.InputMethodEvent.html#method.set_commit_string) method.
  pub trait InputMethodEventSetCommitStringArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::input_method_event::InputMethodEvent) -> ();
  }
  impl<'largs> InputMethodEventSetCommitStringArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::input_method_event::InputMethodEvent) -> () {
      let commit_string = self;
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_setCommitString_commitString(original_self as *mut ::input_method_event::InputMethodEvent, commit_string as *const ::qt_core::string::String) }
    }
  }
  impl<'largs> InputMethodEventSetCommitStringArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::input_method_event::InputMethodEvent) -> () {
      let commit_string = self.0;
      let replace_from = self.1;
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_setCommitString_commitString_replaceFrom(original_self as *mut ::input_method_event::InputMethodEvent, commit_string as *const ::qt_core::string::String, replace_from) }
    }
  }
  impl<'largs> InputMethodEventSetCommitStringArgs<'largs>
    for (&'largs ::qt_core::string::String, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::input_method_event::InputMethodEvent) -> () {
      let commit_string = self.0;
      let replace_from = self.1;
      let replace_length = self.2;
      unsafe { ::ffi::qt_gui_c_QInputMethodEvent_setCommitString_commitString_replaceFrom_replaceLength(original_self as *mut ::input_method_event::InputMethodEvent, commit_string as *const ::qt_core::string::String, replace_from, replace_length) }
    }
  }
}
