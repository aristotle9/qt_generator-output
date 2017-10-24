/// C++ type: <span style='color: green;'>```QAbstractSlider```</span>
#[repr(C)]
pub struct AbstractSlider(u8);

impl AbstractSlider {
  /// C++ method: <span style='color: green;'>```bool QAbstractSlider::hasTracking() const```</span>
  ///
  ///
  pub fn has_tracking(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_hasTracking(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSlider::invertedAppearance() const```</span>
  ///
  ///
  pub fn inverted_appearance(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_invertedAppearance(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSlider::invertedControls() const```</span>
  ///
  ///
  pub fn inverted_controls(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_invertedControls(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractSlider::isSliderDown() const```</span>
  ///
  ///
  pub fn is_slider_down(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_isSliderDown(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractSlider::maximum() const```</span>
  ///
  ///
  pub fn maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_maximum(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractSlider::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_metaObject(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractSlider::minimum() const```</span>
  ///
  ///
  pub fn minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_minimum(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractSlider::QAbstractSlider()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::abstract_slider::AbstractSlider> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSlider_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractSlider::QAbstractSlider(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::abstract_slider::AbstractSlider> {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSlider_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```int QAbstractSlider::pageStep() const```</span>
  ///
  ///
  pub fn page_step(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_pageStep(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractSlider::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAbstractSlider_qt_metacall(self as *mut ::abstract_slider::AbstractSlider,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAbstractSlider::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QAbstractSlider_qt_metacast(self as *mut ::abstract_slider::AbstractSlider, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setInvertedAppearance(bool arg1)```</span>
  ///
  ///
  pub fn set_inverted_appearance(&mut self, arg1: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSlider_setInvertedAppearance(self as *mut ::abstract_slider::AbstractSlider, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setInvertedControls(bool arg1)```</span>
  ///
  ///
  pub fn set_inverted_controls(&mut self, arg1: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSlider_setInvertedControls(self as *mut ::abstract_slider::AbstractSlider, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setMaximum(int arg1)```</span>
  ///
  ///
  pub fn set_maximum(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setMaximum(self as *mut ::abstract_slider::AbstractSlider, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setMinimum(int arg1)```</span>
  ///
  ///
  pub fn set_minimum(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setMinimum(self as *mut ::abstract_slider::AbstractSlider, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractSlider::setOrientation(Qt::Orientation arg1)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, arg1: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSlider_setOrientation(self as *mut ::abstract_slider::AbstractSlider,
                                                         arg1 as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setPageStep(int arg1)```</span>
  ///
  ///
  pub fn set_page_step(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setPageStep(self as *mut ::abstract_slider::AbstractSlider, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractSlider::setRange(int min, int max)```</span>
  ///
  ///
  pub fn set_range(&mut self, min: ::libc::c_int, max: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setRange(self as *mut ::abstract_slider::AbstractSlider, min, max) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setSingleStep(int arg1)```</span>
  ///
  ///
  pub fn set_single_step(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setSingleStep(self as *mut ::abstract_slider::AbstractSlider, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setSliderDown(bool arg1)```</span>
  ///
  ///
  pub fn set_slider_down(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setSliderDown(self as *mut ::abstract_slider::AbstractSlider, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setSliderPosition(int arg1)```</span>
  ///
  ///
  pub fn set_slider_position(&mut self, arg1: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QAbstractSlider_setSliderPosition(self as *mut ::abstract_slider::AbstractSlider, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::setTracking(bool enable)```</span>
  ///
  ///
  pub fn set_tracking(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setTracking(self as *mut ::abstract_slider::AbstractSlider, enable) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractSlider::setValue(int arg1)```</span>
  ///
  ///
  pub fn set_value(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_setValue(self as *mut ::abstract_slider::AbstractSlider, arg1) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractSlider::singleStep() const```</span>
  ///
  ///
  pub fn single_step(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_singleStep(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractSlider::sliderPosition() const```</span>
  ///
  ///
  pub fn slider_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_sliderPosition(self as *const ::abstract_slider::AbstractSlider) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractSlider::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractSlider_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractSlider::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAbstractSlider_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractSlider::triggerAction(QAbstractSlider::SliderAction action)```</span>
  ///
  ///
  pub fn trigger_action(&mut self, action: ::abstract_slider::SliderAction) {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_triggerAction(self as *mut ::abstract_slider::AbstractSlider, action) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractSlider::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QAbstractSlider_value(self as *const ::abstract_slider::AbstractSlider) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_slider::AbstractSlider {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAbstractSlider_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractSlider`.
  pub struct Signals<'a>(&'a ::abstract_slider::AbstractSlider);
  /// Represents a built-in Qt signal `QAbstractSlider::sliderPressed`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().slider_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SliderPressed<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SliderPressed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sliderPressed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SliderPressed<'a> {}
  /// Represents a built-in Qt signal `QAbstractSlider::windowTitleChanged`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct WindowTitleChanged<'a>(&'a ::abstract_slider::AbstractSlider);
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
  /// Represents a built-in Qt signal `QAbstractSlider::windowIconChanged`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct WindowIconChanged<'a>(&'a ::abstract_slider::AbstractSlider);
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
  /// Represents a built-in Qt signal `QAbstractSlider::rangeChanged`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().range_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct RangeChanged<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for RangeChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rangeChanged(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RangeChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractSlider::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::abstract_slider::AbstractSlider);
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
  /// Represents a built-in Qt signal `QAbstractSlider::valueChanged`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct ValueChanged<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for ValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractSlider::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct WindowIconTextChanged<'a>(&'a ::abstract_slider::AbstractSlider);
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
  /// Represents a built-in Qt signal `QAbstractSlider::actionTriggered`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().action_triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct ActionTriggered<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for ActionTriggered<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2actionTriggered(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActionTriggered<'a> {}
  /// Represents a built-in Qt signal `QAbstractSlider::sliderMoved`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().slider_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SliderMoved<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SliderMoved<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sliderMoved(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SliderMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractSlider::sliderReleased`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.signals().slider_released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SliderReleased<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SliderReleased<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sliderReleased()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SliderReleased<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::sliderPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_pressed(&self) -> SliderPressed {
      SliderPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::rangeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn range_changed(&self) -> RangeChanged {
      RangeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::actionTriggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn action_triggered(&self) -> ActionTriggered {
      ActionTriggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::sliderMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_moved(&self) -> SliderMoved {
      SliderMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractSlider::sliderReleased`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_released(&self) -> SliderReleased {
      SliderReleased(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractSlider`.
  pub struct Slots<'a>(&'a ::abstract_slider::AbstractSlider);
  /// Represents a built-in Qt slot `QAbstractSlider::showNormal`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct ShowNormal<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setWindowTitle`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetWindowTitle<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::update`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct Update<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setValue`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetValue<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setEnabled`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetEnabled<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::showFullScreen`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct ShowFullScreen<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::close`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct Close<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setVisible`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetVisible<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::raise`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct Raise<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::showMaximized`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct ShowMaximized<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setRange`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_range()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetRange<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetRange<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRange(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::repaint`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct Repaint<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::lower`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct Lower<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::updateMicroFocus`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct UpdateMicroFocus<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setWindowModified`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetWindowModified<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setStyleSheet`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetStyleSheet<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setOrientation`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_orientation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetOrientation<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetOrientation<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOrientation(Qt::Orientation)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setFocus`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetFocus<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::show`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct Show<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::showMinimized`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct ShowMinimized<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setDisabled`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetDisabled<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::setHidden`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct SetHidden<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractSlider::hide`.
  ///
  /// An object of this type can be created from `AbstractSlider` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractSlider` object.
  pub struct Hide<'a>(&'a ::abstract_slider::AbstractSlider);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setRange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_range(&self) -> SetRange {
      SetRange(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setOrientation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_orientation(&self) -> SetOrientation {
      SetOrientation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractSlider::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
  }
  impl ::abstract_slider::AbstractSlider {
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

/// C++ type: <span style='color: green;'>```QAbstractSlider::SliderAction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SliderAction {
  /// C++ enum variant: <span style='color: green;'>```SliderNoAction = 0```</span>
  NoAction = 0,
  /// C++ enum variant: <span style='color: green;'>```SliderSingleStepAdd = 1```</span>
  SingleStepAdd = 1,
  /// C++ enum variant: <span style='color: green;'>```SliderSingleStepSub = 2```</span>
  SingleStepSub = 2,
  /// C++ enum variant: <span style='color: green;'>```SliderPageStepAdd = 3```</span>
  PageStepAdd = 3,
  /// C++ enum variant: <span style='color: green;'>```SliderPageStepSub = 4```</span>
  PageStepSub = 4,
  /// C++ enum variant: <span style='color: green;'>```SliderToMinimum = 5```</span>
  ToMinimum = 5,
  /// C++ enum variant: <span style='color: green;'>```SliderToMaximum = 6```</span>
  ToMaximum = 6,
  /// C++ enum variant: <span style='color: green;'>```SliderMove = 7```</span>
  Move = 7,
}

/// C++ type: <span style='color: green;'>```QAbstractSlider::SliderChange```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SliderChange {
  /// C++ enum variant: <span style='color: green;'>```SliderRangeChange = 0```</span>
  Range = 0,
  /// C++ enum variant: <span style='color: green;'>```SliderOrientationChange = 1```</span>
  Orientation = 1,
  /// C++ enum variant: <span style='color: green;'>```SliderStepsChange = 2```</span>
  Steps = 2,
  /// C++ enum variant: <span style='color: green;'>```SliderValueChange = 3```</span>
  Value = 3,
}

impl ::cpp_utils::DynamicCast<::abstract_slider::AbstractSlider> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_slider::AbstractSlider> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QAbstractSlider_G_dynamic_cast_QAbstractSlider_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_slider::AbstractSlider> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSlider_G_dynamic_cast_QAbstractSlider_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_slider::AbstractSlider {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QObject_ptr(self as *mut ::abstract_slider::AbstractSlider)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QObject_ptr(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::abstract_slider::AbstractSlider {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QPaintDevice_ptr(self as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QPaintDevice_ptr(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::abstract_slider::AbstractSlider {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QWidget_ptr(self as *mut ::abstract_slider::AbstractSlider)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QWidget_ptr(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_slider::AbstractSlider> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_slider::AbstractSlider> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_slider::AbstractSlider> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result =
      ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result = ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_slider::AbstractSlider {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QWidget_ptr(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_slider::AbstractSlider {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAbstractSlider_G_static_cast_QWidget_ptr(self as *mut ::abstract_slider::AbstractSlider)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
