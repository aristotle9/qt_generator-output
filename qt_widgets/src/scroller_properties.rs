/// C++ type: <span style='color: green;'>```QScrollerProperties::FrameRates```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FrameRates {
  /// C++ enum variant: <span style='color: green;'>```Standard = 0```</span>
  Standard = 0,
  /// C++ enum variant: <span style='color: green;'>```Fps60 = 1```</span>
  Fps60 = 1,
  /// C++ enum variant: <span style='color: green;'>```Fps30 = 2```</span>
  Fps30 = 2,
  /// C++ enum variant: <span style='color: green;'>```Fps20 = 3```</span>
  Fps20 = 3,
}

/// C++ type: <span style='color: green;'>```QScrollerProperties::OvershootPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OvershootPolicy {
  /// C++ enum variant: <span style='color: green;'>```OvershootWhenScrollable = 0```</span>
  WhenScrollable = 0,
  /// C++ enum variant: <span style='color: green;'>```OvershootAlwaysOff = 1```</span>
  AlwaysOff = 1,
  /// C++ enum variant: <span style='color: green;'>```OvershootAlwaysOn = 2```</span>
  AlwaysOn = 2,
}

/// C++ type: <span style='color: green;'>```QScrollerProperties::ScrollMetric```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScrollMetric {
  /// C++ enum variant: <span style='color: green;'>```MousePressEventDelay = 0```</span>
  MousePressEventDelay = 0,
  /// C++ enum variant: <span style='color: green;'>```DragStartDistance = 1```</span>
  DragStartDistance = 1,
  /// C++ enum variant: <span style='color: green;'>```DragVelocitySmoothingFactor = 2```</span>
  DragVelocitySmoothingFactor = 2,
  /// C++ enum variant: <span style='color: green;'>```AxisLockThreshold = 3```</span>
  AxisLockThreshold = 3,
  /// C++ enum variant: <span style='color: green;'>```ScrollingCurve = 4```</span>
  ScrollingCurve = 4,
  /// C++ enum variant: <span style='color: green;'>```DecelerationFactor = 5```</span>
  DecelerationFactor = 5,
  /// C++ enum variant: <span style='color: green;'>```MinimumVelocity = 6```</span>
  MinimumVelocity = 6,
  /// C++ enum variant: <span style='color: green;'>```MaximumVelocity = 7```</span>
  MaximumVelocity = 7,
  /// C++ enum variant: <span style='color: green;'>```MaximumClickThroughVelocity = 8```</span>
  MaximumClickThroughVelocity = 8,
  /// C++ enum variant: <span style='color: green;'>```AcceleratingFlickMaximumTime = 9```</span>
  AcceleratingFlickMaximumTime = 9,
  /// C++ enum variant: <span style='color: green;'>```AcceleratingFlickSpeedupFactor = 10```</span>
  AcceleratingFlickSpeedupFactor = 10,
  /// C++ enum variant: <span style='color: green;'>```SnapPositionRatio = 11```</span>
  SnapPositionRatio = 11,
  /// C++ enum variant: <span style='color: green;'>```SnapTime = 12```</span>
  SnapTime = 12,
  /// C++ enum variant: <span style='color: green;'>```OvershootDragResistanceFactor = 13```</span>
  OvershootDragResistanceFactor = 13,
  /// C++ enum variant: <span style='color: green;'>```OvershootDragDistanceFactor = 14```</span>
  OvershootDragDistanceFactor = 14,
  /// C++ enum variant: <span style='color: green;'>```OvershootScrollDistanceFactor = 15```</span>
  OvershootScrollDistanceFactor = 15,
  /// C++ enum variant: <span style='color: green;'>```OvershootScrollTime = 16```</span>
  OvershootScrollTime = 16,
  /// C++ enum variant: <span style='color: green;'>```HorizontalOvershootPolicy = 17```</span>
  HorizontalOvershootPolicy = 17,
  /// C++ enum variant: <span style='color: green;'>```VerticalOvershootPolicy = 18```</span>
  VerticalOvershootPolicy = 18,
  /// C++ enum variant: <span style='color: green;'>```FrameRate = 19```</span>
  FrameRate = 19,
  /// C++ enum variant: <span style='color: green;'>```ScrollMetricCount = 20```</span>
  ScrollMetricCount = 20,
}

/// C++ type: <span style='color: green;'>```QScrollerProperties```</span>
#[repr(C)]
pub struct ScrollerProperties(u8);

impl ScrollerProperties {
  /// C++ method: <span style='color: green;'>```QScrollerProperties::QScrollerProperties```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::scroller_properties::ScrollerProperties>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QScrollerProperties::QScrollerProperties()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::scroller_properties::ScrollerProperties) -> ::cpp_utils::CppBox<::scroller_properties::ScrollerProperties>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QScrollerProperties::QScrollerProperties(const QScrollerProperties& sp)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::scroller_properties::ScrollerProperties>
    where Args: overloading::ScrollerPropertiesNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QScrollerProperties& QScrollerProperties::operator=(const QScrollerProperties& sp)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             sp: &'l1 ::scroller_properties::ScrollerProperties)
                             -> &'l0 mut ::scroller_properties::ScrollerProperties {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QScrollerProperties_operator_assign(self as *mut ::scroller_properties::ScrollerProperties, sp as *const ::scroller_properties::ScrollerProperties)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QScrollerProperties::operator==(const QScrollerProperties& sp) const```</span>
  ///
  ///
  pub fn op_eq(&self, sp: &::scroller_properties::ScrollerProperties) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QScrollerProperties_operator_eq(self as *const ::scroller_properties::ScrollerProperties,
                                                          sp as *const ::scroller_properties::ScrollerProperties)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QScrollerProperties::operator!=(const QScrollerProperties& sp) const```</span>
  ///
  ///
  pub fn op_neq(&self, sp: &::scroller_properties::ScrollerProperties) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QScrollerProperties_operator_neq(self as *const ::scroller_properties::ScrollerProperties,
                                                           sp as *const ::scroller_properties::ScrollerProperties)
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QScrollerProperties::scrollMetric(QScrollerProperties::ScrollMetric metric) const```</span>
  ///
  ///
  pub fn scroll_metric(&self, metric: ::scroller_properties::ScrollMetric) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QScrollerProperties_scrollMetric_to_output(self as *const ::scroller_properties::ScrollerProperties, metric, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties& sp)```</span>
  ///
  ///
  pub fn set_default_scroller_properties(sp: &::scroller_properties::ScrollerProperties) {
    unsafe { ::ffi::qt_widgets_c_QScrollerProperties_setDefaultScrollerProperties(sp as *const ::scroller_properties::ScrollerProperties) }
  }

  /// C++ method: <span style='color: green;'>```void QScrollerProperties::setScrollMetric(QScrollerProperties::ScrollMetric metric, const QVariant& value)```</span>
  ///
  ///
  pub fn set_scroll_metric(&mut self,
                           metric: ::scroller_properties::ScrollMetric,
                           value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QScrollerProperties_setScrollMetric(self as *mut ::scroller_properties::ScrollerProperties,
                                                              metric,
                                                              value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QScrollerProperties::unsetDefaultScrollerProperties()```</span>
  ///
  ///
  pub fn unset_default_scroller_properties() {
    unsafe { ::ffi::qt_widgets_c_QScrollerProperties_unsetDefaultScrollerProperties() }
  }
}

impl ::cpp_utils::CppDeletable for ::scroller_properties::ScrollerProperties {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QScrollerProperties_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ScrollerProperties::new](../struct.ScrollerProperties.html#method.new) method.
  pub trait ScrollerPropertiesNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::scroller_properties::ScrollerProperties>;
  }
  impl ScrollerPropertiesNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::scroller_properties::ScrollerProperties> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollerProperties_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ScrollerPropertiesNewArgs for &'a ::scroller_properties::ScrollerProperties {
    fn exec(self) -> ::cpp_utils::CppBox<::scroller_properties::ScrollerProperties> {
      let sp = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QScrollerProperties_new_sp(sp as *const ::scroller_properties::ScrollerProperties)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
