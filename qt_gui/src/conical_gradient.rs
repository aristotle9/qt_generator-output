/// C++ type: <span style='color: green;'>```QConicalGradient```</span>
#[repr(C)]
pub struct ConicalGradient(u8);

impl ConicalGradient {
  /// C++ method: <span style='color: green;'>```double QConicalGradient::angle() const```</span>
  ///
  ///
  pub fn angle(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QConicalGradient_angle(self as *const ::conical_gradient::ConicalGradient) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QConicalGradient::center() const```</span>
  ///
  ///
  pub fn center(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QConicalGradient_center_to_output(self as *const ::conical_gradient::ConicalGradient,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QConicalGradient::QConicalGradient```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QConicalGradient::QConicalGradient()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, ::libc::c_double)) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QConicalGradient::QConicalGradient(const QPointF& center, double startAngle)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QConicalGradient::QConicalGradient(double cx, double cy, double startAngle)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient>
    where Args: overloading::ConicalGradientNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QConicalGradient::setAngle(double angle)```</span>
  ///
  ///
  pub fn set_angle(&mut self, angle: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QConicalGradient_setAngle(self as *mut ::conical_gradient::ConicalGradient, angle) }
  }

  /// C++ method: <span style='color: green;'>```QConicalGradient::setCenter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_center(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QConicalGradient::setCenter(const QPointF& center)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_center(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QConicalGradient::setCenter(double x, double y)```</span>
  ///
  ///
  pub fn set_center<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ConicalGradientSetCenterArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::conical_gradient::ConicalGradient {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QConicalGradient_delete
  }
}

impl ::cpp_utils::StaticCast<::gradient::Gradient> for ::conical_gradient::ConicalGradient {
  fn static_cast_mut(&mut self) -> &mut ::gradient::Gradient {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QConicalGradient_G_static_cast_QGradient_ptr(self as *mut ::conical_gradient::ConicalGradient)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::gradient::Gradient {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QConicalGradient_G_static_cast_QGradient_ptr(self as *const ::conical_gradient::ConicalGradient as *mut ::conical_gradient::ConicalGradient) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::conical_gradient::ConicalGradient> for ::gradient::Gradient {
  unsafe fn static_cast_mut(&mut self) -> &mut ::conical_gradient::ConicalGradient {
    let ffi_result =
      ::ffi::qt_gui_c_QConicalGradient_G_static_cast_QConicalGradient_ptr(self as *mut ::gradient::Gradient);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::conical_gradient::ConicalGradient {
    let ffi_result = ::ffi::qt_gui_c_QConicalGradient_G_static_cast_QConicalGradient_ptr(self as *const ::gradient::Gradient as *mut ::gradient::Gradient);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::conical_gradient::ConicalGradient {
  type Target = ::gradient::Gradient;
  fn deref(&self) -> &::gradient::Gradient {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QConicalGradient_G_static_cast_QGradient_ptr(self as *const ::conical_gradient::ConicalGradient as *mut ::conical_gradient::ConicalGradient) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::conical_gradient::ConicalGradient {
  fn deref_mut(&mut self) -> &mut ::gradient::Gradient {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QConicalGradient_G_static_cast_QGradient_ptr(self as *mut ::conical_gradient::ConicalGradient)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ConicalGradient::new](../struct.ConicalGradient.html#method.new) method.
  pub trait ConicalGradientNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient>;
  }
  impl<'a> ConicalGradientNewArgs for (&'a ::qt_core::point_f::PointF, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient> {
      let center = self.0;
      let start_angle = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QConicalGradient_new_center_startAngle(center as *const ::qt_core::point_f::PointF,
                                                                 start_angle)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ConicalGradientNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient> {
      let cx = self.0;
      let cy = self.1;
      let start_angle = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QConicalGradient_new_cx_cy_startAngle(cx, cy, start_angle) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ConicalGradientNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::conical_gradient::ConicalGradient> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QConicalGradient_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [ConicalGradient::set_center](../struct.ConicalGradient.html#method.set_center) method.
  pub trait ConicalGradientSetCenterArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::conical_gradient::ConicalGradient) -> ();
  }
  impl<'largs> ConicalGradientSetCenterArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::conical_gradient::ConicalGradient) -> () {
      let center = self;
      unsafe {
        ::ffi::qt_gui_c_QConicalGradient_setCenter_center(original_self as *mut ::conical_gradient::ConicalGradient,
                                                          center as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> ConicalGradientSetCenterArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::conical_gradient::ConicalGradient) -> () {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_gui_c_QConicalGradient_setCenter_x_y(original_self as *mut ::conical_gradient::ConicalGradient,
                                                       x,
                                                       y)
      }
    }
  }
}
