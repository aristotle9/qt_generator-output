/// C++ type: <span style='color: green;'>```QLinearGradient```</span>
#[repr(C)]
pub struct LinearGradient(u8);

impl LinearGradient {
  /// C++ method: <span style='color: green;'>```QPointF QLinearGradient::finalStop() const```</span>
  ///
  ///
  pub fn final_stop(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_finalStop_to_output(self as *const ::linear_gradient::LinearGradient,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLinearGradient::QLinearGradient```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLinearGradient::QLinearGradient()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, &::qt_core::point_f::PointF)) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLinearGradient::QLinearGradient(const QPointF& start, const QPointF& finalStop)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLinearGradient::QLinearGradient(double xStart, double yStart, double xFinalStop, double yFinalStop)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient>
    where Args: overloading::LinearGradientNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QLinearGradient::setFinalStop```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_final_stop(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLinearGradient::setFinalStop(const QPointF& stop)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_final_stop(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLinearGradient::setFinalStop(double x, double y)```</span>
  ///
  ///
  pub fn set_final_stop<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LinearGradientSetFinalStopArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLinearGradient::setStart```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_start(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLinearGradient::setStart(const QPointF& start)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_start(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLinearGradient::setStart(double x, double y)```</span>
  ///
  ///
  pub fn set_start<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LinearGradientSetStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPointF QLinearGradient::start() const```</span>
  ///
  ///
  pub fn start(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_start_to_output(self as *const ::linear_gradient::LinearGradient,
                                                        &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::linear_gradient::LinearGradient {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QLinearGradient_delete
  }
}

impl ::cpp_utils::StaticCast<::gradient::Gradient> for ::linear_gradient::LinearGradient {
  fn static_cast_mut(&mut self) -> &mut ::gradient::Gradient {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_G_static_cast_QGradient_ptr(self as *mut ::linear_gradient::LinearGradient)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::gradient::Gradient {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QLinearGradient_G_static_cast_QGradient_ptr(self as *const ::linear_gradient::LinearGradient as *mut ::linear_gradient::LinearGradient) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::linear_gradient::LinearGradient> for ::gradient::Gradient {
  unsafe fn static_cast_mut(&mut self) -> &mut ::linear_gradient::LinearGradient {
    let ffi_result =
      ::ffi::qt_gui_c_QLinearGradient_G_static_cast_QLinearGradient_ptr(self as *mut ::gradient::Gradient);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::linear_gradient::LinearGradient {
    let ffi_result = ::ffi::qt_gui_c_QLinearGradient_G_static_cast_QLinearGradient_ptr(self as *const ::gradient::Gradient as *mut ::gradient::Gradient);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::linear_gradient::LinearGradient {
  type Target = ::gradient::Gradient;
  fn deref(&self) -> &::gradient::Gradient {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QLinearGradient_G_static_cast_QGradient_ptr(self as *const ::linear_gradient::LinearGradient as *mut ::linear_gradient::LinearGradient) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::linear_gradient::LinearGradient {
  fn deref_mut(&mut self) -> &mut ::gradient::Gradient {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_G_static_cast_QGradient_ptr(self as *mut ::linear_gradient::LinearGradient)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [LinearGradient::new](../struct.LinearGradient.html#method.new) method.
  pub trait LinearGradientNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient>;
  }
  impl LinearGradientNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QLinearGradient_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> LinearGradientNewArgs for (&'a ::qt_core::point_f::PointF, &'a ::qt_core::point_f::PointF) {
    fn exec(self) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient> {
      let start = self.0;
      let final_stop = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QLinearGradient_new_start_finalStop(start as *const ::qt_core::point_f::PointF,
                                                              final_stop as *const ::qt_core::point_f::PointF)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl LinearGradientNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::linear_gradient::LinearGradient> {
      let x_start = self.0;
      let y_start = self.1;
      let x_final_stop = self.2;
      let y_final_stop = self.3;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QLinearGradient_new_xStart_yStart_xFinalStop_yFinalStop(x_start,
                                                                                y_start,
                                                                                x_final_stop,
                                                                                y_final_stop)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [LinearGradient::set_final_stop](../struct.LinearGradient.html#method.set_final_stop) method.
  pub trait LinearGradientSetFinalStopArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::linear_gradient::LinearGradient) -> ();
  }
  impl<'largs> LinearGradientSetFinalStopArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::linear_gradient::LinearGradient) -> () {
      let stop = self;
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_setFinalStop_stop(original_self as *mut ::linear_gradient::LinearGradient,
                                                          stop as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> LinearGradientSetFinalStopArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::linear_gradient::LinearGradient) -> () {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_setFinalStop_x_y(original_self as *mut ::linear_gradient::LinearGradient,
                                                         x,
                                                         y)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [LinearGradient::set_start](../struct.LinearGradient.html#method.set_start) method.
  pub trait LinearGradientSetStartArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::linear_gradient::LinearGradient) -> ();
  }
  impl<'largs> LinearGradientSetStartArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::linear_gradient::LinearGradient) -> () {
      let start = self;
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_setStart_start(original_self as *mut ::linear_gradient::LinearGradient,
                                                       start as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> LinearGradientSetStartArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::linear_gradient::LinearGradient) -> () {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_gui_c_QLinearGradient_setStart_x_y(original_self as *mut ::linear_gradient::LinearGradient,
                                                     x,
                                                     y)
      }
    }
  }
}
