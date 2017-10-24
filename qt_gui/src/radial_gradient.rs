/// C++ type: <span style='color: green;'>```QRadialGradient```</span>
#[repr(C)]
pub struct RadialGradient(u8);

impl RadialGradient {
  /// C++ method: <span style='color: green;'>```QPointF QRadialGradient::center() const```</span>
  ///
  ///
  pub fn center(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRadialGradient_center_to_output(self as *const ::radial_gradient::RadialGradient,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QRadialGradient::centerRadius() const```</span>
  ///
  ///
  pub fn center_radius(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRadialGradient_centerRadius(self as *const ::radial_gradient::RadialGradient) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QRadialGradient::focalPoint() const```</span>
  ///
  ///
  pub fn focal_point(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRadialGradient_focalPoint_to_output(self as *const ::radial_gradient::RadialGradient,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QRadialGradient::focalRadius() const```</span>
  ///
  ///
  pub fn focal_radius(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRadialGradient_focalRadius(self as *const ::radial_gradient::RadialGradient) }
  }

  /// C++ method: <span style='color: green;'>```QRadialGradient::QRadialGradient```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRadialGradient::QRadialGradient()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, ::libc::c_double, &::qt_core::point_f::PointF, ::libc::c_double)) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRadialGradient::QRadialGradient(const QPointF& center, double centerRadius, const QPointF& focalPoint, double focalRadius)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, ::libc::c_double)) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRadialGradient::QRadialGradient(const QPointF& center, double radius)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, ::libc::c_double, &::qt_core::point_f::PointF)) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRadialGradient::QRadialGradient(const QPointF& center, double radius, const QPointF& focalPoint)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRadialGradient::QRadialGradient(double cx, double cy, double centerRadius, double fx, double fy, double focalRadius)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRadialGradient::QRadialGradient(double cx, double cy, double radius)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRadialGradient::QRadialGradient(double cx, double cy, double radius, double fx, double fy)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>
    where Args: overloading::RadialGradientNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```double QRadialGradient::radius() const```</span>
  ///
  ///
  pub fn radius(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRadialGradient_radius(self as *const ::radial_gradient::RadialGradient) }
  }

  /// C++ method: <span style='color: green;'>```QRadialGradient::setCenter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_center(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRadialGradient::setCenter(const QPointF& center)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_center(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRadialGradient::setCenter(double x, double y)```</span>
  ///
  ///
  pub fn set_center<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::RadialGradientSetCenterArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRadialGradient::setCenterRadius(double radius)```</span>
  ///
  ///
  pub fn set_center_radius(&mut self, radius: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QRadialGradient_setCenterRadius(self as *mut ::radial_gradient::RadialGradient, radius) }
  }

  /// C++ method: <span style='color: green;'>```QRadialGradient::setFocalPoint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_focal_point(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRadialGradient::setFocalPoint(const QPointF& focalPoint)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_focal_point(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRadialGradient::setFocalPoint(double x, double y)```</span>
  ///
  ///
  pub fn set_focal_point<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::RadialGradientSetFocalPointArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRadialGradient::setFocalRadius(double radius)```</span>
  ///
  ///
  pub fn set_focal_radius(&mut self, radius: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QRadialGradient_setFocalRadius(self as *mut ::radial_gradient::RadialGradient, radius) }
  }

  /// C++ method: <span style='color: green;'>```void QRadialGradient::setRadius(double radius)```</span>
  ///
  ///
  pub fn set_radius(&mut self, radius: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QRadialGradient_setRadius(self as *mut ::radial_gradient::RadialGradient, radius) }
  }
}

impl ::cpp_utils::CppDeletable for ::radial_gradient::RadialGradient {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QRadialGradient_delete
  }
}

impl ::cpp_utils::StaticCast<::gradient::Gradient> for ::radial_gradient::RadialGradient {
  fn static_cast_mut(&mut self) -> &mut ::gradient::Gradient {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRadialGradient_G_static_cast_QGradient_ptr(self as *mut ::radial_gradient::RadialGradient)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::gradient::Gradient {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRadialGradient_G_static_cast_QGradient_ptr(self as *const ::radial_gradient::RadialGradient as *mut ::radial_gradient::RadialGradient) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::radial_gradient::RadialGradient> for ::gradient::Gradient {
  unsafe fn static_cast_mut(&mut self) -> &mut ::radial_gradient::RadialGradient {
    let ffi_result =
      ::ffi::qt_gui_c_QRadialGradient_G_static_cast_QRadialGradient_ptr(self as *mut ::gradient::Gradient);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::radial_gradient::RadialGradient {
    let ffi_result = ::ffi::qt_gui_c_QRadialGradient_G_static_cast_QRadialGradient_ptr(self as *const ::gradient::Gradient as *mut ::gradient::Gradient);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::radial_gradient::RadialGradient {
  type Target = ::gradient::Gradient;
  fn deref(&self) -> &::gradient::Gradient {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRadialGradient_G_static_cast_QGradient_ptr(self as *const ::radial_gradient::RadialGradient as *mut ::radial_gradient::RadialGradient) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::radial_gradient::RadialGradient {
  fn deref_mut(&mut self) -> &mut ::gradient::Gradient {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRadialGradient_G_static_cast_QGradient_ptr(self as *mut ::radial_gradient::RadialGradient)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RadialGradient::new](../struct.RadialGradient.html#method.new) method.
  pub trait RadialGradientNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient>;
  }
  impl<'a> RadialGradientNewArgs
    for (&'a ::qt_core::point_f::PointF, ::libc::c_double, &'a ::qt_core::point_f::PointF, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient> {
      let center = self.0;
      let center_radius = self.1;
      let focal_point = self.2;
      let focal_radius = self.3;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRadialGradient_new_center_centerRadius_focalPoint_focalRadius(center as *const ::qt_core::point_f::PointF, center_radius, focal_point as *const ::qt_core::point_f::PointF, focal_radius) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RadialGradientNewArgs for (&'a ::qt_core::point_f::PointF, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient> {
      let center = self.0;
      let radius = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRadialGradient_new_center_radius(center as *const ::qt_core::point_f::PointF, radius)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RadialGradientNewArgs for (&'a ::qt_core::point_f::PointF, ::libc::c_double, &'a ::qt_core::point_f::PointF) {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient> {
      let center = self.0;
      let radius = self.1;
      let focal_point = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QRadialGradient_new_center_radius_focalPoint(center as *const ::qt_core::point_f::PointF, radius, focal_point as *const ::qt_core::point_f::PointF)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RadialGradientNewArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient> {
      let cx = self.0;
      let cy = self.1;
      let center_radius = self.2;
      let fx = self.3;
      let fy = self.4;
      let focal_radius = self.5;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRadialGradient_new_cx_cy_centerRadius_fx_fy_focalRadius(cx,
                                                                                 cy,
                                                                                 center_radius,
                                                                                 fx,
                                                                                 fy,
                                                                                 focal_radius)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RadialGradientNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient> {
      let cx = self.0;
      let cy = self.1;
      let radius = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRadialGradient_new_cx_cy_radius(cx, cy, radius) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RadialGradientNewArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient> {
      let cx = self.0;
      let cy = self.1;
      let radius = self.2;
      let fx = self.3;
      let fy = self.4;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRadialGradient_new_cx_cy_radius_fx_fy(cx, cy, radius, fx, fy) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RadialGradientNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::radial_gradient::RadialGradient> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QRadialGradient_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [RadialGradient::set_center](../struct.RadialGradient.html#method.set_center) method.
  pub trait RadialGradientSetCenterArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::radial_gradient::RadialGradient) -> ();
  }
  impl<'largs> RadialGradientSetCenterArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::radial_gradient::RadialGradient) -> () {
      let center = self;
      unsafe {
        ::ffi::qt_gui_c_QRadialGradient_setCenter_center(original_self as *mut ::radial_gradient::RadialGradient,
                                                         center as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> RadialGradientSetCenterArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::radial_gradient::RadialGradient) -> () {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_gui_c_QRadialGradient_setCenter_x_y(original_self as *mut ::radial_gradient::RadialGradient,
                                                      x,
                                                      y)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RadialGradient::set_focal_point](../struct.RadialGradient.html#method.set_focal_point) method.
  pub trait RadialGradientSetFocalPointArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::radial_gradient::RadialGradient) -> ();
  }
  impl<'largs> RadialGradientSetFocalPointArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::radial_gradient::RadialGradient) -> () {
      let focal_point = self;
      unsafe { ::ffi::qt_gui_c_QRadialGradient_setFocalPoint_focalPoint(original_self as *mut ::radial_gradient::RadialGradient, focal_point as *const ::qt_core::point_f::PointF) }
    }
  }
  impl<'largs> RadialGradientSetFocalPointArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::radial_gradient::RadialGradient) -> () {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_gui_c_QRadialGradient_setFocalPoint_x_y(original_self as *mut ::radial_gradient::RadialGradient,
                                                          x,
                                                          y)
      }
    }
  }
}
