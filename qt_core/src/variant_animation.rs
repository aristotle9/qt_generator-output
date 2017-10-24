/// C++ type: <span style='color: green;'>```QVariantAnimation```</span>
#[repr(C)]
pub struct VariantAnimation(u8);

impl VariantAnimation {
  /// C++ method: <span style='color: green;'>```QVariant QVariantAnimation::currentValue() const```</span>
  ///
  ///
  pub fn current_value(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_currentValue_to_output(self as *const ::variant_animation::VariantAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QVariantAnimation::duration() const```</span>
  ///
  ///
  pub fn duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVariantAnimation_duration(self as *const ::variant_animation::VariantAnimation) }
  }

  /// C++ method: <span style='color: green;'>```QEasingCurve QVariantAnimation::easingCurve() const```</span>
  ///
  ///
  pub fn easing_curve(&self) -> ::easing_curve::EasingCurve {
    {
      let mut object: ::easing_curve::EasingCurve =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_easingCurve_to_output(self as *const ::variant_animation::VariantAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QVariantAnimation::endValue() const```</span>
  ///
  ///
  pub fn end_value(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_endValue_to_output(self as *const ::variant_animation::VariantAnimation,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QVariantAnimation::keyValueAt(double step) const```</span>
  ///
  ///
  pub fn key_value_at(&self, step: ::libc::c_double) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_keyValueAt_to_output(self as *const ::variant_animation::VariantAnimation,
                                                                step,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>> QVariantAnimation::keyValues() const```</span>
  ///
  ///
  pub fn key_values(&self) -> ::vector::VectorPairPairCDoubleVariant {
    {
      let mut object: ::vector::VectorPairPairCDoubleVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_keyValues_to_output(self as *const ::variant_animation::VariantAnimation,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QVariantAnimation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QVariantAnimation_metaObject(self as *const ::variant_animation::VariantAnimation) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QVariantAnimation::QVariantAnimation()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::variant_animation::VariantAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QVariantAnimation::QVariantAnimation(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object)
                           -> ::cpp_utils::CppBox<::variant_animation::VariantAnimation> {
    let ffi_result = ::ffi::qt_core_c_QVariantAnimation_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QVariantAnimation::setDuration(int msecs)```</span>
  ///
  ///
  pub fn set_duration(&mut self, msecs: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVariantAnimation_setDuration(self as *mut ::variant_animation::VariantAnimation, msecs) }
  }

  /// C++ method: <span style='color: green;'>```void QVariantAnimation::setEasingCurve(const QEasingCurve& easing)```</span>
  ///
  ///
  pub fn set_easing_curve(&mut self, easing: &::easing_curve::EasingCurve) {
    unsafe {
      ::ffi::qt_core_c_QVariantAnimation_setEasingCurve(self as *mut ::variant_animation::VariantAnimation,
                                                        easing as *const ::easing_curve::EasingCurve)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVariantAnimation::setEndValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_end_value(&mut self, value: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QVariantAnimation_setEndValue(self as *mut ::variant_animation::VariantAnimation,
                                                     value as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVariantAnimation::setKeyValueAt(double step, const QVariant& value)```</span>
  ///
  ///
  pub fn set_key_value_at(&mut self, step: ::libc::c_double, value: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QVariantAnimation_setKeyValueAt(self as *mut ::variant_animation::VariantAnimation,
                                                       step,
                                                       value as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVariantAnimation::setKeyValues(const QVector<QPair<double, QVariant>>& values)```</span>
  ///
  ///
  pub fn set_key_values(&mut self, values: &::vector::VectorPairPairCDoubleVariant) {
    unsafe {
      ::ffi::qt_core_c_QVariantAnimation_setKeyValues(self as *mut ::variant_animation::VariantAnimation,
                                                      values as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVariantAnimation::setStartValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_start_value(&mut self, value: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QVariantAnimation_setStartValue(self as *mut ::variant_animation::VariantAnimation,
                                                       value as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QVariantAnimation::startValue() const```</span>
  ///
  ///
  pub fn start_value(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_startValue_to_output(self as *const ::variant_animation::VariantAnimation,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QVariantAnimation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QVariantAnimation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QVariantAnimation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QVariantAnimation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::variant_animation::VariantAnimation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QVariantAnimation_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `VariantAnimation`.
  pub struct Signals<'a>(&'a ::variant_animation::VariantAnimation);
  /// Represents a built-in Qt signal `QVariantAnimation::directionChanged`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.signals().direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct DirectionChanged<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for DirectionChanged<'a> {
    type Arguments = (&'static ::abstract_animation::Direction,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2directionChanged(QAbstractAnimation::Direction)\0"
    }
  }
  impl<'a> ::connection::Signal for DirectionChanged<'a> {}
  /// Represents a built-in Qt signal `QVariantAnimation::currentLoopChanged`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.signals().current_loop_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct CurrentLoopChanged<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for CurrentLoopChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentLoopChanged(int)\0"
    }
  }
  impl<'a> ::connection::Signal for CurrentLoopChanged<'a> {}
  /// Represents a built-in Qt signal `QVariantAnimation::stateChanged`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct StateChanged<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for StateChanged<'a> {
    type Arguments = (&'static ::abstract_animation::State, &'static ::abstract_animation::State);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stateChanged(QAbstractAnimation::State,QAbstractAnimation::State)\0"
    }
  }
  impl<'a> ::connection::Signal for StateChanged<'a> {}
  /// Represents a built-in Qt signal `QVariantAnimation::valueChanged`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct ValueChanged<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for ValueChanged<'a> {
    type Arguments = (&'static ::variant::Variant,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(const QVariant&)\0"
    }
  }
  impl<'a> ::connection::Signal for ValueChanged<'a> {}
  /// Represents a built-in Qt signal `QVariantAnimation::finished`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct Finished<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for Finished<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished()\0"
    }
  }
  impl<'a> ::connection::Signal for Finished<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QVariantAnimation::directionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn direction_changed(&self) -> DirectionChanged {
      DirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QVariantAnimation::currentLoopChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_loop_changed(&self) -> CurrentLoopChanged {
      CurrentLoopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QVariantAnimation::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QVariantAnimation::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QVariantAnimation::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `VariantAnimation`.
  pub struct Slots<'a>(&'a ::variant_animation::VariantAnimation);
  /// Represents a built-in Qt slot `QVariantAnimation::resume`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.slots().resume()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct Resume<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for Resume<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resume()\0"
    }
  }
  /// Represents a built-in Qt slot `QVariantAnimation::stop`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct Stop<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for Stop<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stop()\0"
    }
  }
  /// Represents a built-in Qt slot `QVariantAnimation::start`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct Start<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = (&'static ::abstract_animation::DeletionPolicy,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start(QAbstractAnimation::DeletionPolicy)\0"
    }
  }
  /// Represents a built-in Qt slot `QVariantAnimation::setPaused`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.slots().set_paused()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct SetPaused<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for SetPaused<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPaused(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QVariantAnimation::setCurrentTime`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.slots().set_current_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct SetCurrentTime<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for SetCurrentTime<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentTime(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QVariantAnimation::pause`.
  ///
  /// An object of this type can be created from `VariantAnimation` with `object.slots().pause()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `VariantAnimation` object.
  pub struct Pause<'a>(&'a ::variant_animation::VariantAnimation);
  impl<'a> ::connection::Receiver for Pause<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1pause()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QVariantAnimation::resume`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resume(&self) -> Resume {
      Resume(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QVariantAnimation::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QVariantAnimation::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QVariantAnimation::setPaused`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_paused(&self) -> SetPaused {
      SetPaused(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QVariantAnimation::setCurrentTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_time(&self) -> SetCurrentTime {
      SetCurrentTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QVariantAnimation::pause`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pause(&self) -> Pause {
      Pause(self.0)
    }
  }
  impl ::variant_animation::VariantAnimation {
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

impl ::cpp_utils::DynamicCast<::variant_animation::VariantAnimation> for ::abstract_animation::AbstractAnimation {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::variant_animation::VariantAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::variant_animation::VariantAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::variant_animation::VariantAnimation> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::variant_animation::VariantAnimation> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::variant_animation::VariantAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_animation::AbstractAnimation> for ::variant_animation::VariantAnimation {
  fn static_cast_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QAbstractAnimation_ptr(self as *mut ::variant_animation::VariantAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QAbstractAnimation_ptr(self as *const ::variant_animation::VariantAnimation as *mut ::variant_animation::VariantAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::variant_animation::VariantAnimation {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QObject_ptr(self as *mut ::variant_animation::VariantAnimation)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QObject_ptr(self as *const ::variant_animation::VariantAnimation as *mut ::variant_animation::VariantAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::variant_animation::VariantAnimation> for ::abstract_animation::AbstractAnimation {
unsafe fn static_cast_mut(&mut self) -> &mut ::variant_animation::VariantAnimation {
let ffi_result = ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::variant_animation::VariantAnimation {
let ffi_result = ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::variant_animation::VariantAnimation> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::variant_animation::VariantAnimation {
    let ffi_result =
      ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::variant_animation::VariantAnimation {
    let ffi_result = ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::variant_animation::VariantAnimation {
  type Target = ::abstract_animation::AbstractAnimation;
  fn deref(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QAbstractAnimation_ptr(self as *const ::variant_animation::VariantAnimation as *mut ::variant_animation::VariantAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::variant_animation::VariantAnimation {
  fn deref_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVariantAnimation_G_static_cast_QAbstractAnimation_ptr(self as *mut ::variant_animation::VariantAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
