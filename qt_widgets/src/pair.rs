/// C++ type: <span style='color: green;'>```QPair<double, double>```</span>
#[repr(C)]
pub struct PairCDoubleCDouble([u8; ::type_sizes::QT_WIDGETS_PAIR_PAIR_C_DOUBLE_C_DOUBLE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairCDoubleCDouble {
  unsafe fn new_uninitialized() -> PairCDoubleCDouble {
    PairCDoubleCDouble(::std::mem::uninitialized())
  }
}

impl PairCDoubleCDouble {
  /// C++ method: <span style='color: green;'>```QPair<double, double>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, double>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::libc::c_double, &::libc::c_double)) -> ::pair::PairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, double>::QPair(const double& t1, const double& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairCDoubleCDouble
    where Args: overloading::PairCDoubleCDoubleNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<double, double>::swap(QPair<double, double>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairCDoubleCDouble) {
    unsafe {
      ::ffi::qt_widgets_c_QPair_double_double_swap(self as *mut ::pair::PairCDoubleCDouble,
                                                   other as *mut ::pair::PairCDoubleCDouble)
    }
  }
}

impl Drop for ::pair::PairCDoubleCDouble {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<double, double>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPair_double_double_destructor(self as *mut ::pair::PairCDoubleCDouble) }
  }
}

/// C++ type: <span style='color: green;'>```QPair<double, QPointF>```</span>
#[repr(C)]
pub struct PairCDoubleQtCorePointF([u8; ::type_sizes::QT_WIDGETS_PAIR_PAIR_C_DOUBLE_QT_CORE_POINT_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PairCDoubleQtCorePointF {
  unsafe fn new_uninitialized() -> PairCDoubleQtCorePointF {
    PairCDoubleQtCorePointF(::std::mem::uninitialized())
  }
}

impl PairCDoubleQtCorePointF {
  /// C++ method: <span style='color: green;'>```QPair<double, QPointF>::QPair```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pair::PairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, QPointF>::QPair()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::libc::c_double, &::qt_core::point_f::PointF)) -> ::pair::PairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPair<double, QPointF>::QPair(const double& t1, const QPointF& t2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pair::PairCDoubleQtCorePointF
    where Args: overloading::PairCDoubleQtCorePointFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPair<double, QPointF>::swap(QPair<double, QPointF>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pair::PairCDoubleQtCorePointF) {
    unsafe {
      ::ffi::qt_widgets_c_QPair_double_QPointF_swap(self as *mut ::pair::PairCDoubleQtCorePointF,
                                                    other as *mut ::pair::PairCDoubleQtCorePointF)
    }
  }
}

impl Drop for ::pair::PairCDoubleQtCorePointF {
  /// C++ method: <span style='color: green;'>```[destructor] void QPair<double, QPointF>::~QPair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPair_double_QPointF_destructor(self as *mut ::pair::PairCDoubleQtCorePointF) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PairCDoubleCDouble::new](../struct.PairCDoubleCDouble.html#method.new) method.
  pub trait PairCDoubleCDoubleNewArgs {
    fn exec(self) -> ::pair::PairCDoubleCDouble;
  }
  impl PairCDoubleCDoubleNewArgs for () {
    fn exec(self) -> ::pair::PairCDoubleCDouble {

      {
        let mut object: ::pair::PairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPair_double_double_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairCDoubleCDoubleNewArgs for (&'a ::libc::c_double, &'a ::libc::c_double) {
    fn exec(self) -> ::pair::PairCDoubleCDouble {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPair_double_double_constructor_t1_t2(t1 as *const ::libc::c_double,
                                                                    t2 as *const ::libc::c_double,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PairCDoubleQtCorePointF::new](../struct.PairCDoubleQtCorePointF.html#method.new) method.
  pub trait PairCDoubleQtCorePointFNewArgs {
    fn exec(self) -> ::pair::PairCDoubleQtCorePointF;
  }
  impl PairCDoubleQtCorePointFNewArgs for () {
    fn exec(self) -> ::pair::PairCDoubleQtCorePointF {

      {
        let mut object: ::pair::PairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPair_double_QPointF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PairCDoubleQtCorePointFNewArgs for (&'a ::libc::c_double, &'a ::qt_core::point_f::PointF) {
    fn exec(self) -> ::pair::PairCDoubleQtCorePointF {
      let t1 = self.0;
      let t2 = self.1;
      {
        let mut object: ::pair::PairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPair_double_QPointF_constructor_t1_t2(t1 as *const ::libc::c_double,
                                                                     t2 as *const ::qt_core::point_f::PointF,
                                                                     &mut object);
        }
        object
      }
    }
  }
}
