/// C++ type: <span style='color: green;'>```QAbstractSpinBox```</span>
#[repr(C)]
pub struct AbstractSpinBox(u8);

impl AbstractSpinBox {
  /// C++ method: <span style='color: green;'>```QAbstractSpinBox::ButtonSymbols QAbstractSpinBox::buttonSymbols() const```</span>
  ///
  ///
  pub fn button_symbols(&self) -> ::abstract_spin_box::ButtonSymbols {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_buttonSymbols(self as *const ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QAbstractSpinBox::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_clear(self as *mut ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractSpinBox::CorrectionMode QAbstractSpinBox::correctionMode() const```</span>
  ///
  ///
  pub fn correction_mode(&self) -> ::abstract_spin_box::CorrectionMode {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_correctionMode(self as *const ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractSpinBox::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_widgets_c_QAbstractSpinBox_event(self as *mut ::abstract_spin_box::AbstractSpinBox, event)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractSpinBox::fixup(QString& input) const```</span>
  ///
  ///
  pub fn fixup(&self, input: &mut ::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_fixup(self as *const ::abstract_spin_box::AbstractSpinBox,
                                                 input as *mut ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSpinBox::hasAcceptableInput() const```</span>
  ///
  ///
  pub fn has_acceptable_input(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_hasAcceptableInput(self as *const ::abstract_spin_box::AbstractSpinBox)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSpinBox::hasFrame() const```</span>
  ///
  ///
  pub fn has_frame(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_hasFrame(self as *const ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractSpinBox::inputMethodQuery(Qt::InputMethodQuery arg1) const```</span>
  ///
  ///
  pub fn input_method_query(&self, arg1: &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSpinBox_inputMethodQuery_to_output(self as *const ::abstract_spin_box::AbstractSpinBox, arg1 as *const ::qt_core::qt::InputMethodQuery, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::interpretText()```</span>
  ///
  ///
  pub fn interpret_text(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_interpretText(self as *mut ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSpinBox::isAccelerated() const```</span>
  ///
  ///
  pub fn is_accelerated(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_isAccelerated(self as *const ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSpinBox::isGroupSeparatorShown() const```</span>
  ///
  ///
  pub fn is_group_separator_shown(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_isGroupSeparatorShown(self as *const ::abstract_spin_box::AbstractSpinBox)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSpinBox::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_isReadOnly(self as *const ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSpinBox::keyboardTracking() const```</span>
  ///
  ///
  pub fn keyboard_tracking(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_keyboardTracking(self as *const ::abstract_spin_box::AbstractSpinBox)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractSpinBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_metaObject(self as *const ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QAbstractSpinBox::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSpinBox_minimumSizeHint_to_output(self as *const ::abstract_spin_box::AbstractSpinBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractSpinBox::QAbstractSpinBox()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::abstract_spin_box::AbstractSpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractSpinBox::QAbstractSpinBox(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::abstract_spin_box::AbstractSpinBox> {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSpinBox_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractSpinBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAbstractSpinBox_qt_metacall(self as *mut ::abstract_spin_box::AbstractSpinBox,
                                                     arg1 as *const ::qt_core::meta_object::Call,
                                                     arg2,
                                                     arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAbstractSpinBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QAbstractSpinBox_qt_metacast(self as *mut ::abstract_spin_box::AbstractSpinBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractSpinBox::selectAll()```</span>
  ///
  ///
  pub fn select_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_selectAll(self as *mut ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setAccelerated(bool on)```</span>
  ///
  ///
  pub fn set_accelerated(&mut self, on: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_setAccelerated(self as *mut ::abstract_spin_box::AbstractSpinBox, on)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setButtonSymbols(QAbstractSpinBox::ButtonSymbols bs)```</span>
  ///
  ///
  pub fn set_button_symbols(&mut self, bs: ::abstract_spin_box::ButtonSymbols) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_setButtonSymbols(self as *mut ::abstract_spin_box::AbstractSpinBox, bs)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setCorrectionMode(QAbstractSpinBox::CorrectionMode cm)```</span>
  ///
  ///
  pub fn set_correction_mode(&mut self, cm: ::abstract_spin_box::CorrectionMode) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_setCorrectionMode(self as *mut ::abstract_spin_box::AbstractSpinBox, cm)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setFrame(bool arg1)```</span>
  ///
  ///
  pub fn set_frame(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_setFrame(self as *mut ::abstract_spin_box::AbstractSpinBox, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setGroupSeparatorShown(bool shown)```</span>
  ///
  ///
  pub fn set_group_separator_shown(&mut self, shown: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_setGroupSeparatorShown(self as *mut ::abstract_spin_box::AbstractSpinBox,
                                                                  shown)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setKeyboardTracking(bool kt)```</span>
  ///
  ///
  pub fn set_keyboard_tracking(&mut self, kt: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_setKeyboardTracking(self as *mut ::abstract_spin_box::AbstractSpinBox, kt)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setReadOnly(bool r)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, r: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_setReadOnly(self as *mut ::abstract_spin_box::AbstractSpinBox, r) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setSpecialValueText(const QString& txt)```</span>
  ///
  ///
  pub fn set_special_value_text(&mut self, txt: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSpinBox_setSpecialValueText(self as *mut ::abstract_spin_box::AbstractSpinBox,
                                                               txt as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSpinBox::setWrapping(bool w)```</span>
  ///
  ///
  pub fn set_wrapping(&mut self, w: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_setWrapping(self as *mut ::abstract_spin_box::AbstractSpinBox, w) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QAbstractSpinBox::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSpinBox_sizeHint_to_output(self as *const ::abstract_spin_box::AbstractSpinBox,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QAbstractSpinBox::specialValueText() const```</span>
  ///
  ///
  pub fn special_value_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSpinBox_specialValueText_to_output(self as *const ::abstract_spin_box::AbstractSpinBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractSpinBox::stepBy(int steps)```</span>
  ///
  ///
  pub fn step_by(&mut self, steps: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_stepBy(self as *mut ::abstract_spin_box::AbstractSpinBox, steps) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractSpinBox::stepDown()```</span>
  ///
  ///
  pub fn step_down(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_stepDown(self as *mut ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractSpinBox::stepUp()```</span>
  ///
  ///
  pub fn step_up(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_stepUp(self as *mut ::abstract_spin_box::AbstractSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```QString QAbstractSpinBox::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSpinBox_text_to_output(self as *const ::abstract_spin_box::AbstractSpinBox,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractSpinBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractSpinBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractSpinBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractSpinBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSpinBox::wrapping() const```</span>
  ///
  ///
  pub fn wrapping(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_wrapping(self as *const ::abstract_spin_box::AbstractSpinBox) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_spin_box::AbstractSpinBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAbstractSpinBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractSpinBox`.
  pub struct Signals<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  /// Represents a built-in Qt signal `QAbstractSpinBox::editingFinished`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.signals().editing_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct EditingFinished<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for EditingFinished<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2editingFinished()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EditingFinished<'a> {}
  /// Represents a built-in Qt signal `QAbstractSpinBox::windowIconChanged`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct WindowIconChanged<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
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
  /// Represents a built-in Qt signal `QAbstractSpinBox::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct WindowIconTextChanged<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
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
  /// Represents a built-in Qt signal `QAbstractSpinBox::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
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
  /// Represents a built-in Qt signal `QAbstractSpinBox::windowTitleChanged`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct WindowTitleChanged<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
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
    /// Returns an object representing a built-in Qt signal `QAbstractSpinBox::editingFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editing_finished(&self) -> EditingFinished {
      EditingFinished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSpinBox::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSpinBox::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSpinBox::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSpinBox::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractSpinBox`.
  pub struct Slots<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  /// Represents a built-in Qt slot `QAbstractSpinBox::setFocus`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetFocus<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::stepUp`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().step_up()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct StepUp<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for StepUp<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepUp()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::showMaximized`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct ShowMaximized<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::setHidden`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetHidden<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::close`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Close<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::updateMicroFocus`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct UpdateMicroFocus<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::showNormal`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct ShowNormal<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::raise`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Raise<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::setWindowTitle`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetWindowTitle<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::setEnabled`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetEnabled<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::showMinimized`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct ShowMinimized<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::hide`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Hide<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::show`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Show<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::stepDown`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().step_down()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct StepDown<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for StepDown<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepDown()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::showFullScreen`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct ShowFullScreen<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::setWindowModified`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetWindowModified<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::lower`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Lower<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::setStyleSheet`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetStyleSheet<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::repaint`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Repaint<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::setDisabled`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetDisabled<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::update`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Update<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::clear`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct Clear<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::selectAll`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SelectAll<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSpinBox::setVisible`.
  ///
  /// An object of this type can be created from `AbstractSpinBox` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSpinBox` object.
  pub struct SetVisible<'a>(&'a ::abstract_spin_box::AbstractSpinBox);
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
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::stepUp`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_up(&self) -> StepUp {
      StepUp(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::stepDown`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_down(&self) -> StepDown {
      StepDown(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSpinBox::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
  }
  impl ::abstract_spin_box::AbstractSpinBox {
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

/// C++ type: <span style='color: green;'>```QAbstractSpinBox::ButtonSymbols```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ButtonSymbols {
  /// C++ enum variant: <span style='color: green;'>```UpDownArrows = 0```</span>
  UpDownArrows = 0,
  /// C++ enum variant: <span style='color: green;'>```PlusMinus = 1```</span>
  PlusMinus = 1,
  /// C++ enum variant: <span style='color: green;'>```NoButtons = 2```</span>
  NoButtons = 2,
}

/// C++ type: <span style='color: green;'>```QAbstractSpinBox::CorrectionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CorrectionMode {
  /// C++ enum variant: <span style='color: green;'>```CorrectToPreviousValue = 0```</span>
  Previous = 0,
  /// C++ enum variant: <span style='color: green;'>```CorrectToNearestValue = 1```</span>
  Nearest = 1,
}

/// C++ type: <span style='color: green;'>```QAbstractSpinBox::StepEnabledFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StepEnabledFlag {
  /// C++ enum variant: <span style='color: green;'>```StepNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```StepUpEnabled = 1```</span>
  UpEnabled = 1,
  /// C++ enum variant: <span style='color: green;'>```StepDownEnabled = 2```</span>
  DownEnabled = 2,
}

impl ::qt_core::flags::FlaggableEnum for StepEnabledFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "StepEnabledFlag"
  }
}

impl ::cpp_utils::DynamicCast<::abstract_spin_box::AbstractSpinBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_spin_box::AbstractSpinBox> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSpinBox_G_dynamic_cast_QAbstractSpinBox_ptr(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_spin_box::AbstractSpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_dynamic_cast_QAbstractSpinBox_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_spin_box::AbstractSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QObject_ptr(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QObject_ptr(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::abstract_spin_box::AbstractSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QPaintDevice_ptr(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QPaintDevice_ptr(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::abstract_spin_box::AbstractSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QWidget_ptr(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QWidget_ptr(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_spin_box::AbstractSpinBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_spin_box::AbstractSpinBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_spin_box::AbstractSpinBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_spin_box::AbstractSpinBox {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QWidget_ptr(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_spin_box::AbstractSpinBox {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSpinBox_G_static_cast_QWidget_ptr(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
