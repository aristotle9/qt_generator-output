/// C++ type: <span style='color: green;'>```QGradient::CoordinateMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CoordinateMode {
  /// C++ enum variant: <span style='color: green;'>```LogicalMode = 0```</span>
  Logical = 0,
  /// C++ enum variant: <span style='color: green;'>```StretchToDeviceMode = 1```</span>
  StretchToDevice = 1,
  /// C++ enum variant: <span style='color: green;'>```ObjectBoundingMode = 2```</span>
  ObjectBounding = 2,
}

/// C++ type: <span style='color: green;'>```QGradient```</span>
#[repr(C)]
pub struct Gradient(u8);

impl Gradient {
  /// C++ method: <span style='color: green;'>```QGradient::CoordinateMode QGradient::coordinateMode() const```</span>
  ///
  ///
  pub fn coordinate_mode(&self) -> ::gradient::CoordinateMode {
    unsafe { ::ffi::qt_gui_c_QGradient_coordinateMode(self as *const ::gradient::Gradient) }
  }

  /// C++ method: <span style='color: green;'>```QGradient::InterpolationMode QGradient::interpolationMode() const```</span>
  ///
  ///
  pub fn interpolation_mode(&self) -> ::gradient::InterpolationMode {
    unsafe { ::ffi::qt_gui_c_QGradient_interpolationMode(self as *const ::gradient::Gradient) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGradient::QGradient()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::gradient::Gradient> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGradient_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```bool QGradient::operator==(const QGradient& gradient) const```</span>
  ///
  ///
  pub fn op_eq(&self, gradient: &::gradient::Gradient) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QGradient_operator_eq(self as *const ::gradient::Gradient,
                                            gradient as *const ::gradient::Gradient)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGradient::operator!=(const QGradient& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::gradient::Gradient) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QGradient_operator_neq(self as *const ::gradient::Gradient,
                                             other as *const ::gradient::Gradient)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGradient::setColorAt(double pos, const QColor& color)```</span>
  ///
  ///
  pub fn set_color_at(&mut self, pos: ::libc::c_double, color: &::color::Color) {
    unsafe {
      ::ffi::qt_gui_c_QGradient_setColorAt(self as *mut ::gradient::Gradient,
                                           pos,
                                           color as *const ::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGradient::setCoordinateMode(QGradient::CoordinateMode mode)```</span>
  ///
  ///
  pub fn set_coordinate_mode(&mut self, mode: ::gradient::CoordinateMode) {
    unsafe { ::ffi::qt_gui_c_QGradient_setCoordinateMode(self as *mut ::gradient::Gradient, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QGradient::setInterpolationMode(QGradient::InterpolationMode mode)```</span>
  ///
  ///
  pub fn set_interpolation_mode(&mut self, mode: ::gradient::InterpolationMode) {
    unsafe { ::ffi::qt_gui_c_QGradient_setInterpolationMode(self as *mut ::gradient::Gradient, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QGradient::setSpread(QGradient::Spread spread)```</span>
  ///
  ///
  pub fn set_spread(&mut self, spread: ::gradient::Spread) {
    unsafe { ::ffi::qt_gui_c_QGradient_setSpread(self as *mut ::gradient::Gradient, spread) }
  }

  /// C++ method: <span style='color: green;'>```void QGradient::setStops(const QVector<QPair<double, QColor>>& stops)```</span>
  ///
  ///
  pub fn set_stops(&mut self, stops: &::vector::VectorPairPairCDoubleColor) {
    unsafe {
      ::ffi::qt_gui_c_QGradient_setStops(self as *mut ::gradient::Gradient,
                                         stops as *const ::vector::VectorPairPairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QGradient::Spread QGradient::spread() const```</span>
  ///
  ///
  pub fn spread(&self) -> ::gradient::Spread {
    unsafe { ::ffi::qt_gui_c_QGradient_spread(self as *const ::gradient::Gradient) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>> QGradient::stops() const```</span>
  ///
  ///
  pub fn stops(&self) -> ::vector::VectorPairPairCDoubleColor {
    {
      let mut object: ::vector::VectorPairPairCDoubleColor =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGradient_stops_to_output(self as *const ::gradient::Gradient, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGradient::Type QGradient::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::gradient::Type {
    unsafe { ::ffi::qt_gui_c_QGradient_type(self as *const ::gradient::Gradient) }
  }
}

impl ::cpp_utils::CppDeletable for ::gradient::Gradient {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QGradient_delete
  }
}

/// C++ type: <span style='color: green;'>```QGradient::InterpolationMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InterpolationMode {
  /// C++ enum variant: <span style='color: green;'>```ColorInterpolation = 0```</span>
  Color = 0,
  /// C++ enum variant: <span style='color: green;'>```ComponentInterpolation = 1```</span>
  Component = 1,
}

/// C++ type: <span style='color: green;'>```QGradient::Spread```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Spread {
  /// C++ enum variant: <span style='color: green;'>```PadSpread = 0```</span>
  Pad = 0,
  /// C++ enum variant: <span style='color: green;'>```ReflectSpread = 1```</span>
  Reflect = 1,
  /// C++ enum variant: <span style='color: green;'>```RepeatSpread = 2```</span>
  Repeat = 2,
}

/// C++ type: <span style='color: green;'>```QGradient::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```LinearGradient = 0```</span>
  Linear = 0,
  /// C++ enum variant: <span style='color: green;'>```RadialGradient = 1```</span>
  Radial = 1,
  /// C++ enum variant: <span style='color: green;'>```ConicalGradient = 2```</span>
  Conical = 2,
  /// C++ enum variant: <span style='color: green;'>```NoGradient = 3```</span>
  No = 3,
}
