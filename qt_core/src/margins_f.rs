/// C++ type: <span style='color: green;'>```QMarginsF```</span>
#[repr(C)]
pub struct MarginsF([u8; ::type_sizes::QT_CORE_MARGINS_F_MARGINS_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MarginsF {
  unsafe fn new_uninitialized() -> MarginsF {
    MarginsF(::std::mem::uninitialized())
  }
}

impl MarginsF {
  /// C++ method: <span style='color: green;'>```double QMarginsF::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QMarginsF_bottom(self as *const ::margins_f::MarginsF) }
  }

  /// C++ method: <span style='color: green;'>```bool QMarginsF::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMarginsF_isNull(self as *const ::margins_f::MarginsF) }
  }

  /// C++ method: <span style='color: green;'>```double QMarginsF::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QMarginsF_left(self as *const ::margins_f::MarginsF) }
  }

  /// C++ method: <span style='color: green;'>```QMarginsF::QMarginsF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMarginsF::QMarginsF()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::margins::Margins) -> ::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMarginsF::QMarginsF(const QMargins& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMarginsF::QMarginsF(double left, double top, double right, double bottom)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::margins_f::MarginsF
    where Args: overloading::MarginsFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMarginsF::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::margins_f::MarginsF) -> &'l0 mut ::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```QMarginsF& QMarginsF::operator+=(const QMarginsF& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, ::libc::c_double) -> &'l0 mut ::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```QMarginsF& QMarginsF::operator+=(double addend)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::margins_f::MarginsF
    where Args: overloading::MarginsFOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMarginsF& QMarginsF::operator/=(double divisor)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, divisor: ::libc::c_double) -> &'l0 mut ::margins_f::MarginsF {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QMarginsF_operator_div_assign(self as *mut ::margins_f::MarginsF, divisor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMarginsF& QMarginsF::operator*=(double factor)```</span>
  ///
  ///
  pub fn op_mul_assign<'l0>(&'l0 mut self, factor: ::libc::c_double) -> &'l0 mut ::margins_f::MarginsF {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QMarginsF_operator_mul_assign(self as *mut ::margins_f::MarginsF, factor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMarginsF::operator-=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_sub_assign(&mut self, &'l1 ::margins_f::MarginsF) -> &'l0 mut ::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```QMarginsF& QMarginsF::operator-=(const QMarginsF& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_sub_assign(&mut self, ::libc::c_double) -> &'l0 mut ::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```QMarginsF& QMarginsF::operator-=(double subtrahend)```</span>
  ///
  ///
  pub fn op_sub_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::margins_f::MarginsF
    where Args: overloading::MarginsFOpSubAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QMarginsF::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QMarginsF_right(self as *const ::margins_f::MarginsF) }
  }

  /// C++ method: <span style='color: green;'>```void QMarginsF::setBottom(double bottom)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, bottom: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QMarginsF_setBottom(self as *mut ::margins_f::MarginsF, bottom) }
  }

  /// C++ method: <span style='color: green;'>```void QMarginsF::setLeft(double left)```</span>
  ///
  ///
  pub fn set_left(&mut self, left: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QMarginsF_setLeft(self as *mut ::margins_f::MarginsF, left) }
  }

  /// C++ method: <span style='color: green;'>```void QMarginsF::setRight(double right)```</span>
  ///
  ///
  pub fn set_right(&mut self, right: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QMarginsF_setRight(self as *mut ::margins_f::MarginsF, right) }
  }

  /// C++ method: <span style='color: green;'>```void QMarginsF::setTop(double top)```</span>
  ///
  ///
  pub fn set_top(&mut self, top: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QMarginsF_setTop(self as *mut ::margins_f::MarginsF, top) }
  }

  /// C++ method: <span style='color: green;'>```QMargins QMarginsF::toMargins() const```</span>
  ///
  ///
  pub fn to_margins(&self) -> ::margins::Margins {
    {
      let mut object: ::margins::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMarginsF_toMargins_to_output(self as *const ::margins_f::MarginsF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QMarginsF::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QMarginsF_top(self as *const ::margins_f::MarginsF) }
  }
}

impl Drop for ::margins_f::MarginsF {
  /// C++ method: <span style='color: green;'>```[destructor] void QMarginsF::~QMarginsF()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMarginsF_destructor(self as *mut ::margins_f::MarginsF) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MarginsF::new](../struct.MarginsF.html#method.new) method.
  pub trait MarginsFNewArgs {
    fn exec(self) -> ::margins_f::MarginsF;
  }
  impl MarginsFNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::margins_f::MarginsF {
      let left = self.0;
      let top = self.1;
      let right = self.2;
      let bottom = self.3;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMarginsF_constructor_left_top_right_bottom(left, top, right, bottom, &mut object);
        }
        object
      }
    }
  }
  impl<'a> MarginsFNewArgs for &'a ::margins::Margins {
    fn exec(self) -> ::margins_f::MarginsF {
      let margins = self;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMarginsF_constructor_margins(margins as *const ::margins::Margins, &mut object);
        }
        object
      }
    }
  }
  impl MarginsFNewArgs for () {
    fn exec(self) -> ::margins_f::MarginsF {

      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMarginsF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MarginsF::op_add_assign](../struct.MarginsF.html#method.op_add_assign) method.
  pub trait MarginsFOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::margins_f::MarginsF) -> &'largs mut ::margins_f::MarginsF;
  }
  impl<'largs> MarginsFOpAddAssignArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::margins_f::MarginsF) -> &'largs mut ::margins_f::MarginsF {
      let addend = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMarginsF_operator_add_assign_addend(original_self as *mut ::margins_f::MarginsF, addend)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> MarginsFOpAddAssignArgs<'largs> for &'largs ::margins_f::MarginsF {
    fn exec(self, original_self: &'largs mut ::margins_f::MarginsF) -> &'largs mut ::margins_f::MarginsF {
      let margins = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMarginsF_operator_add_assign_margins(original_self as *mut ::margins_f::MarginsF,
                                                                 margins as *const ::margins_f::MarginsF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [MarginsF::op_sub_assign](../struct.MarginsF.html#method.op_sub_assign) method.
  pub trait MarginsFOpSubAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::margins_f::MarginsF) -> &'largs mut ::margins_f::MarginsF;
  }
  impl<'largs> MarginsFOpSubAssignArgs<'largs> for &'largs ::margins_f::MarginsF {
    fn exec(self, original_self: &'largs mut ::margins_f::MarginsF) -> &'largs mut ::margins_f::MarginsF {
      let margins = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMarginsF_operator_sub_assign_margins(original_self as *mut ::margins_f::MarginsF,
                                                                 margins as *const ::margins_f::MarginsF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> MarginsFOpSubAssignArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::margins_f::MarginsF) -> &'largs mut ::margins_f::MarginsF {
      let subtrahend = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMarginsF_operator_sub_assign_subtrahend(original_self as *mut ::margins_f::MarginsF,
                                                                    subtrahend)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
