/// C++ type: <span style='color: green;'>```QGraphicsItemAnimation```</span>
#[repr(C)]
pub struct GraphicsItemAnimation(u8);

impl GraphicsItemAnimation {
  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItemAnimation_clear(self as *mut ::graphics_item_animation::GraphicsItemAnimation)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItemAnimation::horizontalScaleAt(double step) const```</span>
  ///
  ///
  pub fn horizontal_scale_at(&self, step: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_horizontalScaleAt(self as *const ::graphics_item_animation::GraphicsItemAnimation, step) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItemAnimation::horizontalShearAt(double step) const```</span>
  ///
  ///
  pub fn horizontal_shear_at(&self, step: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_horizontalShearAt(self as *const ::graphics_item_animation::GraphicsItemAnimation, step) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItemAnimation::item() const```</span>
  ///
  ///
  pub fn item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItemAnimation_item(self as *const ::graphics_item_animation::GraphicsItemAnimation)
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QGraphicsItemAnimation::matrixAt(double step) const```</span>
  ///
  ///
  pub fn matrix_at(&self, step: ::libc::c_double) -> ::qt_gui::matrix::Matrix {
    {
      let mut object: ::qt_gui::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemAnimation_matrixAt_to_output(self as *const ::graphics_item_animation::GraphicsItemAnimation, step, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsItemAnimation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_metaObject(self as *const ::graphics_item_animation::GraphicsItemAnimation) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsItemAnimation::QGraphicsItemAnimation()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_item_animation::GraphicsItemAnimation> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsItemAnimation::QGraphicsItemAnimation(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_item_animation::GraphicsItemAnimation> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsItemAnimation_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItemAnimation::posAt(double step) const```</span>
  ///
  ///
  pub fn pos_at(&self, step: ::libc::c_double) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemAnimation_posAt_to_output(self as *const ::graphics_item_animation::GraphicsItemAnimation, step, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>> QGraphicsItemAnimation::posList() const```</span>
  ///
  ///
  pub fn pos_list(&self) -> ::list::ListPairPairCDoubleQtCorePointF {
    {
      let mut object: ::list::ListPairPairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemAnimation_posList_to_output(self as *const ::graphics_item_animation::GraphicsItemAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsItemAnimation::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsItemAnimation_qt_metacall(self as *mut ::graphics_item_animation::GraphicsItemAnimation, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsItemAnimation::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsItemAnimation_qt_metacast(self as *mut ::graphics_item_animation::GraphicsItemAnimation, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsItemAnimation::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItemAnimation_reset(self as *mut ::graphics_item_animation::GraphicsItemAnimation)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItemAnimation::rotationAt(double step) const```</span>
  ///
  ///
  pub fn rotation_at(&self, step: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_rotationAt(self as *const ::graphics_item_animation::GraphicsItemAnimation, step) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>> QGraphicsItemAnimation::rotationList() const```</span>
  ///
  ///
  pub fn rotation_list(&self) -> ::list::ListPairPairCDoubleCDouble {
    {
      let mut object: ::list::ListPairPairCDoubleCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemAnimation_rotationList_to_output(self as *const ::graphics_item_animation::GraphicsItemAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>> QGraphicsItemAnimation::scaleList() const```</span>
  ///
  ///
  pub fn scale_list(&self) -> ::list::ListPairPairCDoubleQtCorePointF {
    {
      let mut object: ::list::ListPairPairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemAnimation_scaleList_to_output(self as *const ::graphics_item_animation::GraphicsItemAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::setItem(QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn set_item(&mut self, item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItemAnimation_setItem(self as *mut ::graphics_item_animation::GraphicsItemAnimation,
                                                       item)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::setPosAt(double step, const QPointF& pos)```</span>
  ///
  ///
  pub fn set_pos_at(&mut self, step: ::libc::c_double, pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItemAnimation_setPosAt(self as *mut ::graphics_item_animation::GraphicsItemAnimation, step, pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::setRotationAt(double step, double angle)```</span>
  ///
  ///
  pub fn set_rotation_at(&mut self, step: ::libc::c_double, angle: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_setRotationAt(self as *mut ::graphics_item_animation::GraphicsItemAnimation, step, angle) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::setScaleAt(double step, double sx, double sy)```</span>
  ///
  ///
  pub fn set_scale_at(&mut self, step: ::libc::c_double, sx: ::libc::c_double, sy: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_setScaleAt(self as *mut ::graphics_item_animation::GraphicsItemAnimation, step, sx, sy) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::setShearAt(double step, double sh, double sv)```</span>
  ///
  ///
  pub fn set_shear_at(&mut self, step: ::libc::c_double, sh: ::libc::c_double, sv: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_setShearAt(self as *mut ::graphics_item_animation::GraphicsItemAnimation, step, sh, sv) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsItemAnimation::setStep(double x)```</span>
  ///
  ///
  pub fn set_step(&mut self, x: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItemAnimation_setStep(self as *mut ::graphics_item_animation::GraphicsItemAnimation, x)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::setTimeLine(QTimeLine* timeLine)```</span>
  ///
  ///
  pub unsafe fn set_time_line(&mut self, time_line: *mut ::qt_core::time_line::TimeLine) {
    ::ffi::qt_widgets_c_QGraphicsItemAnimation_setTimeLine(self as *mut ::graphics_item_animation::GraphicsItemAnimation, time_line)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItemAnimation::setTranslationAt(double step, double dx, double dy)```</span>
  ///
  ///
  pub fn set_translation_at(&mut self, step: ::libc::c_double, dx: ::libc::c_double, dy: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_setTranslationAt(self as *mut ::graphics_item_animation::GraphicsItemAnimation, step, dx, dy) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>> QGraphicsItemAnimation::shearList() const```</span>
  ///
  ///
  pub fn shear_list(&self) -> ::list::ListPairPairCDoubleQtCorePointF {
    {
      let mut object: ::list::ListPairPairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemAnimation_shearList_to_output(self as *const ::graphics_item_animation::GraphicsItemAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeLine* QGraphicsItemAnimation::timeLine() const```</span>
  ///
  ///
  pub fn time_line(&self) -> *mut ::qt_core::time_line::TimeLine {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_timeLine(self as *const ::graphics_item_animation::GraphicsItemAnimation) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsItemAnimation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsItemAnimation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsItemAnimation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsItemAnimation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>> QGraphicsItemAnimation::translationList() const```</span>
  ///
  ///
  pub fn translation_list(&self) -> ::list::ListPairPairCDoubleQtCorePointF {
    {
      let mut object: ::list::ListPairPairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemAnimation_translationList_to_output(self as *const ::graphics_item_animation::GraphicsItemAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItemAnimation::verticalScaleAt(double step) const```</span>
  ///
  ///
  pub fn vertical_scale_at(&self, step: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_verticalScaleAt(self as *const ::graphics_item_animation::GraphicsItemAnimation, step) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItemAnimation::verticalShearAt(double step) const```</span>
  ///
  ///
  pub fn vertical_shear_at(&self, step: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_verticalShearAt(self as *const ::graphics_item_animation::GraphicsItemAnimation, step) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItemAnimation::xTranslationAt(double step) const```</span>
  ///
  ///
  pub fn x_translation_at(&self, step: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_xTranslationAt(self as *const ::graphics_item_animation::GraphicsItemAnimation, step) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItemAnimation::yTranslationAt(double step) const```</span>
  ///
  ///
  pub fn y_translation_at(&self, step: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_yTranslationAt(self as *const ::graphics_item_animation::GraphicsItemAnimation, step) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_item_animation::GraphicsItemAnimation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsItemAnimation_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsItemAnimation`.
  pub struct Signals<'a>(&'a ::graphics_item_animation::GraphicsItemAnimation);
  /// Represents a built-in Qt signal `QGraphicsItemAnimation::objectNameChanged`.
  ///
  /// An object of this type can be created from `GraphicsItemAnimation` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsItemAnimation` object.
  pub struct ObjectNameChanged<'a>(&'a ::graphics_item_animation::GraphicsItemAnimation);
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
    /// Returns an object representing a built-in Qt signal `QGraphicsItemAnimation::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsItemAnimation`.
  pub struct Slots<'a>(&'a ::graphics_item_animation::GraphicsItemAnimation);
  /// Represents a built-in Qt slot `QGraphicsItemAnimation::reset`.
  ///
  /// An object of this type can be created from `GraphicsItemAnimation` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsItemAnimation` object.
  pub struct Reset<'a>(&'a ::graphics_item_animation::GraphicsItemAnimation);
  impl<'a> ::qt_core::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsItemAnimation::setStep`.
  ///
  /// An object of this type can be created from `GraphicsItemAnimation` with `object.slots().set_step()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsItemAnimation` object.
  pub struct SetStep<'a>(&'a ::graphics_item_animation::GraphicsItemAnimation);
  impl<'a> ::qt_core::connection::Receiver for SetStep<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStep(double)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsItemAnimation::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsItemAnimation::setStep`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_step(&self) -> SetStep {
      SetStep(self.0)
    }
  }
  impl ::graphics_item_animation::GraphicsItemAnimation {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_item_animation::GraphicsItemAnimation {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QObject_ptr(self as *mut ::graphics_item_animation::GraphicsItemAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QObject_ptr(self as *const ::graphics_item_animation::GraphicsItemAnimation as *mut ::graphics_item_animation::GraphicsItemAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_item_animation::GraphicsItemAnimation> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_item_animation::GraphicsItemAnimation {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QGraphicsItemAnimation_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_item_animation::GraphicsItemAnimation {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QGraphicsItemAnimation_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_item_animation::GraphicsItemAnimation {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QObject_ptr(self as *const ::graphics_item_animation::GraphicsItemAnimation as *mut ::graphics_item_animation::GraphicsItemAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_item_animation::GraphicsItemAnimation {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QObject_ptr(self as *mut ::graphics_item_animation::GraphicsItemAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
