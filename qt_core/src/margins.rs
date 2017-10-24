/// C++ type: <span style='color: green;'>```QMargins```</span>
#[repr(C)]
pub struct Margins([u8; ::type_sizes::QT_CORE_MARGINS_MARGINS]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Margins {
  unsafe fn new_uninitialized() -> Margins {
    Margins(::std::mem::uninitialized())
  }
}

impl Margins {
  /// C++ method: <span style='color: green;'>```int QMargins::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMargins_bottom(self as *const ::margins::Margins) }
  }

  /// C++ method: <span style='color: green;'>```bool QMargins::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMargins_isNull(self as *const ::margins::Margins) }
  }

  /// C++ method: <span style='color: green;'>```int QMargins::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMargins_left(self as *const ::margins::Margins) }
  }

  /// C++ method: <span style='color: green;'>```QMargins::QMargins```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMargins::QMargins()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMargins::QMargins(int left, int top, int right, int bottom)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::margins::Margins
    where Args: overloading::MarginsNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMargins::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::margins::Margins) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator+=(const QMargins& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, ::libc::c_int) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator+=(int arg1)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::margins::Margins
    where Args: overloading::MarginsOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMargins::operator/=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_div_assign(&mut self, ::libc::c_double) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator/=(double arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_div_assign(&mut self, ::libc::c_int) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator/=(int arg1)```</span>
  ///
  ///
  pub fn op_div_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::margins::Margins
    where Args: overloading::MarginsOpDivAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMargins::operator*=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, ::libc::c_double) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator*=(double arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, ::libc::c_int) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator*=(int arg1)```</span>
  ///
  ///
  pub fn op_mul_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::margins::Margins
    where Args: overloading::MarginsOpMulAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMargins::operator-=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_sub_assign(&mut self, &'l1 ::margins::Margins) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator-=(const QMargins& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_sub_assign(&mut self, ::libc::c_int) -> &'l0 mut ::margins::Margins```<br>
  /// C++ method: <span style='color: green;'>```QMargins& QMargins::operator-=(int arg1)```</span>
  ///
  ///
  pub fn op_sub_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::margins::Margins
    where Args: overloading::MarginsOpSubAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QMargins::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMargins_right(self as *const ::margins::Margins) }
  }

  /// C++ method: <span style='color: green;'>```void QMargins::setBottom(int bottom)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, bottom: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QMargins_setBottom(self as *mut ::margins::Margins, bottom) }
  }

  /// C++ method: <span style='color: green;'>```void QMargins::setLeft(int left)```</span>
  ///
  ///
  pub fn set_left(&mut self, left: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QMargins_setLeft(self as *mut ::margins::Margins, left) }
  }

  /// C++ method: <span style='color: green;'>```void QMargins::setRight(int right)```</span>
  ///
  ///
  pub fn set_right(&mut self, right: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QMargins_setRight(self as *mut ::margins::Margins, right) }
  }

  /// C++ method: <span style='color: green;'>```void QMargins::setTop(int top)```</span>
  ///
  ///
  pub fn set_top(&mut self, top: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QMargins_setTop(self as *mut ::margins::Margins, top) }
  }

  /// C++ method: <span style='color: green;'>```int QMargins::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMargins_top(self as *const ::margins::Margins) }
  }
}

impl Drop for ::margins::Margins {
  /// C++ method: <span style='color: green;'>```[destructor] void QMargins::~QMargins()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMargins_destructor(self as *mut ::margins::Margins) }
  }
}

/// C++ method: <span style='color: green;'>```operator+```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_add((&::margins::Margins, ::libc::c_int)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator+(const QMargins& lhs, int rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_add((&::margins::Margins, &::margins::Margins)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator+(const QMargins& m1, const QMargins& m2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_add((::libc::c_int, &::margins::Margins)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator+(int lhs, const QMargins& rhs)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_add((&::margins_f::MarginsF, &::margins_f::MarginsF)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator+(const QMarginsF& lhs, const QMarginsF& rhs)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_add((&::margins_f::MarginsF, ::libc::c_double)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator+(const QMarginsF& lhs, double rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_add((::libc::c_double, &::margins_f::MarginsF)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator+(double lhs, const QMarginsF& rhs)```</span>
///
///
pub fn op_add<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpAddArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator/```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_div((&::margins::Margins, ::libc::c_double)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator/(const QMargins& margins, double divisor)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_div((&::margins::Margins, ::libc::c_int)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator/(const QMargins& margins, int divisor)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_div((&::margins_f::MarginsF, ::libc::c_double)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator/(const QMarginsF& lhs, double divisor)```</span>
///
///
pub fn op_div<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpDivArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator==(const QMarginsF& lhs, const QMarginsF& rhs)```</span>
///
///
pub fn op_eq(lhs: &::margins_f::MarginsF, rhs: &::margins_f::MarginsF) -> bool {
  unsafe {
    ::ffi::qt_core_c_QMargins_G_operator_eq(lhs as *const ::margins_f::MarginsF,
                                            rhs as *const ::margins_f::MarginsF)
  }
}

/// C++ method: <span style='color: green;'>```operator*```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_mul((&::margins::Margins, ::libc::c_double)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator*(const QMargins& margins, double factor)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_mul((&::margins::Margins, ::libc::c_int)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator*(const QMargins& margins, int factor)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_mul((::libc::c_double, &::margins::Margins)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator*(double factor, const QMargins& margins)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_mul((::libc::c_int, &::margins::Margins)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator*(int factor, const QMargins& margins)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_mul((&::margins_f::MarginsF, ::libc::c_double)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator*(const QMarginsF& lhs, double rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_mul((::libc::c_double, &::margins_f::MarginsF)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator*(double lhs, const QMarginsF& rhs)```</span>
///
///
pub fn op_mul<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpMulArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator-```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_neg(&::margins::Margins) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator-(const QMargins& margins)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_neg(&::margins_f::MarginsF) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator-(const QMarginsF& margins)```</span>
///
///
pub fn op_neg<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpNegArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator!=(const QMarginsF& lhs, const QMarginsF& rhs)```</span>
///
///
pub fn op_neq(lhs: &::margins_f::MarginsF, rhs: &::margins_f::MarginsF) -> bool {
  unsafe {
    ::ffi::qt_core_c_QMargins_G_operator_neq(lhs as *const ::margins_f::MarginsF,
                                             rhs as *const ::margins_f::MarginsF)
  }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::margins::Margins)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QMargins& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::margins_f::MarginsF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QMarginsF& arg2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::margins::Margins)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QMargins& arg2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::margins_f::MarginsF)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QMarginsF& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator>>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::margins::Margins)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QMargins& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::margins_f::MarginsF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QMarginsF& arg2)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator-```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_sub((&::margins::Margins, ::libc::c_int)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator-(const QMargins& lhs, int rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_sub((&::margins::Margins, &::margins::Margins)) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator-(const QMargins& m1, const QMargins& m2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_sub((&::margins_f::MarginsF, &::margins_f::MarginsF)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator-(const QMarginsF& lhs, const QMarginsF& rhs)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_sub((&::margins_f::MarginsF, ::libc::c_double)) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator-(const QMarginsF& lhs, double rhs)```</span>
///
///
pub fn op_sub<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpSubArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator+```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_unary_plus(&::margins::Margins) -> ::margins::Margins```<br>
/// C++ method: <span style='color: green;'>```QMargins operator+(const QMargins& margins)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_unary_plus(&::margins_f::MarginsF) -> ::margins_f::MarginsF```<br>
/// C++ method: <span style='color: green;'>```QMarginsF operator+(const QMarginsF& margins)```</span>
///
///
pub fn op_unary_plus<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpUnaryPlusArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Margins::new](../struct.Margins.html#method.new) method.
  pub trait MarginsNewArgs {
    fn exec(self) -> ::margins::Margins;
  }
  impl MarginsNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::margins::Margins {
      let left = self.0;
      let top = self.1;
      let right = self.2;
      let bottom = self.3;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_constructor_left_top_right_bottom(left, top, right, bottom, &mut object);
        }
        object
      }
    }
  }
  impl MarginsNewArgs for () {
    fn exec(self) -> ::margins::Margins {

      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Margins::op_add_assign](../struct.Margins.html#method.op_add_assign) method.
  pub trait MarginsOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins;
  }
  impl<'largs> MarginsOpAddAssignArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QMargins_operator_add_assign_arg1(original_self as *mut ::margins::Margins, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> MarginsOpAddAssignArgs<'largs> for &'largs ::margins::Margins {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let margins = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QMargins_operator_add_assign_margins(original_self as *mut ::margins::Margins,
                                                              margins as *const ::margins::Margins)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Margins::op_div_assign](../struct.Margins.html#method.op_div_assign) method.
  pub trait MarginsOpDivAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins;
  }
  impl<'largs> MarginsOpDivAssignArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QMargins_operator_div_assign_double(original_self as *mut ::margins::Margins, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> MarginsOpDivAssignArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QMargins_operator_div_assign_int(original_self as *mut ::margins::Margins, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Margins::op_mul_assign](../struct.Margins.html#method.op_mul_assign) method.
  pub trait MarginsOpMulAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins;
  }
  impl<'largs> MarginsOpMulAssignArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QMargins_operator_mul_assign_double(original_self as *mut ::margins::Margins, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> MarginsOpMulAssignArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QMargins_operator_mul_assign_int(original_self as *mut ::margins::Margins, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Margins::op_sub_assign](../struct.Margins.html#method.op_sub_assign) method.
  pub trait MarginsOpSubAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins;
  }
  impl<'largs> MarginsOpSubAssignArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QMargins_operator_sub_assign_arg1(original_self as *mut ::margins::Margins, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> MarginsOpSubAssignArgs<'largs> for &'largs ::margins::Margins {
    fn exec(self, original_self: &'largs mut ::margins::Margins) -> &'largs mut ::margins::Margins {
      let margins = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QMargins_operator_sub_assign_margins(original_self as *mut ::margins::Margins,
                                                              margins as *const ::margins::Margins)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [op_add](../fn.op_add.html) method.
  pub trait OpAddArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpAddArgs for (&'a ::margins_f::MarginsF, &'a ::margins_f::MarginsF) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_add_to_output_QMarginsF_QMarginsF(lhs as *const ::margins_f::MarginsF,
                                                                                 rhs as *const ::margins_f::MarginsF,
                                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::margins_f::MarginsF, ::libc::c_double) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_add_to_output_QMarginsF_double(lhs as *const ::margins_f::MarginsF,
                                                                              rhs,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::margins::Margins, &'a ::margins::Margins) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let m1 = self.0;
      let m2 = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_add_to_output_QMargins_QMargins(m1 as *const ::margins::Margins,
                                                                               m2 as *const ::margins::Margins,
                                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::margins::Margins, ::libc::c_int) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_add_to_output_QMargins_int(lhs as *const ::margins::Margins,
                                                                          rhs,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (::libc::c_double, &'a ::margins_f::MarginsF) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_add_to_output_double_QMarginsF(lhs,
                                                                              rhs as *const ::margins_f::MarginsF,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (::libc::c_int, &'a ::margins::Margins) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_add_to_output_int_QMargins(lhs,
                                                                          rhs as *const ::margins::Margins,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_div](../fn.op_div.html) method.
  pub trait OpDivArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpDivArgs for (&'a ::margins_f::MarginsF, ::libc::c_double) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let divisor = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_div_to_output_QMarginsF_double(lhs as *const ::margins_f::MarginsF,
                                                                              divisor,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpDivArgs for (&'a ::margins::Margins, ::libc::c_double) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let margins = self.0;
      let divisor = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_div_to_output_QMargins_double(margins as *const ::margins::Margins,
                                                                             divisor,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpDivArgs for (&'a ::margins::Margins, ::libc::c_int) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let margins = self.0;
      let divisor = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_div_to_output_QMargins_int(margins as *const ::margins::Margins,
                                                                          divisor,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_mul](../fn.op_mul.html) method.
  pub trait OpMulArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpMulArgs for (&'a ::margins_f::MarginsF, ::libc::c_double) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_mul_to_output_QMarginsF_double(lhs as *const ::margins_f::MarginsF,
                                                                              rhs,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::margins::Margins, ::libc::c_double) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let margins = self.0;
      let factor = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_mul_to_output_QMargins_double(margins as *const ::margins::Margins,
                                                                             factor,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::margins::Margins, ::libc::c_int) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let margins = self.0;
      let factor = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_mul_to_output_QMargins_int(margins as *const ::margins::Margins,
                                                                          factor,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (::libc::c_double, &'a ::margins::Margins) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let factor = self.0;
      let margins = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_mul_to_output_double_QMargins(factor,
                                                                             margins as *const ::margins::Margins,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (::libc::c_double, &'a ::margins_f::MarginsF) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_mul_to_output_double_QMarginsF(lhs,
                                                                              rhs as *const ::margins_f::MarginsF,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (::libc::c_int, &'a ::margins::Margins) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let factor = self.0;
      let margins = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_mul_to_output_int_QMargins(factor,
                                                                          margins as *const ::margins::Margins,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_neg](../fn.op_neg.html) method.
  pub trait OpNegArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpNegArgs for &'a ::margins::Margins {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let margins = self;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_neg_to_output_QMargins(margins as *const ::margins::Margins,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpNegArgs for &'a ::margins_f::MarginsF {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let margins = self;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_neg_to_output_QMarginsF(margins as *const ::margins_f::MarginsF,
                                                                       &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::margins::Margins) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_shl_QDataStream_QMargins(arg1 as *mut ::data_stream::DataStream,
                                                                        arg2 as *const ::margins::Margins)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::margins_f::MarginsF) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_shl_QDataStream_QMarginsF(arg1 as *mut ::data_stream::DataStream,
                                                                         arg2 as *const ::margins_f::MarginsF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::margins::Margins) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_shl_to_output_QDebug_QMargins(arg1 as *const ::debug::Debug,
                                                                             arg2 as *const ::margins::Margins,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::margins_f::MarginsF) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_shl_to_output_QDebug_QMarginsF(arg1 as *const ::debug::Debug,
                                                                              arg2 as *const ::margins_f::MarginsF,
                                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shr](../fn.op_shr.html) method.
  pub trait OpShrArgs<'largs> {
    fn exec(self) -> &'largs mut ::data_stream::DataStream;
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::margins::Margins) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_shr_QDataStream_QMargins(arg1 as *mut ::data_stream::DataStream,
                                                                        arg2 as *mut ::margins::Margins)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::margins_f::MarginsF) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_shr_QDataStream_QMarginsF(arg1 as *mut ::data_stream::DataStream,
                                                                         arg2 as *mut ::margins_f::MarginsF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [op_sub](../fn.op_sub.html) method.
  pub trait OpSubArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpSubArgs for (&'a ::margins_f::MarginsF, &'a ::margins_f::MarginsF) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_sub_to_output_QMarginsF_QMarginsF(lhs as *const ::margins_f::MarginsF,
                                                                                 rhs as *const ::margins_f::MarginsF,
                                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpSubArgs for (&'a ::margins_f::MarginsF, ::libc::c_double) {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_sub_to_output_QMarginsF_double(lhs as *const ::margins_f::MarginsF,
                                                                              rhs,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpSubArgs for (&'a ::margins::Margins, &'a ::margins::Margins) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let m1 = self.0;
      let m2 = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_sub_to_output_QMargins_QMargins(m1 as *const ::margins::Margins,
                                                                               m2 as *const ::margins::Margins,
                                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpSubArgs for (&'a ::margins::Margins, ::libc::c_int) {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_sub_to_output_QMargins_int(lhs as *const ::margins::Margins,
                                                                          rhs,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_unary_plus](../fn.op_unary_plus.html) method.
  pub trait OpUnaryPlusArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpUnaryPlusArgs for &'a ::margins::Margins {
    type ReturnType = ::margins::Margins;
    fn exec(self) -> ::margins::Margins {
      let margins = self;
      {
        let mut object: ::margins::Margins =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_unary_plus_to_output_QMargins(margins as *const ::margins::Margins,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpUnaryPlusArgs for &'a ::margins_f::MarginsF {
    type ReturnType = ::margins_f::MarginsF;
    fn exec(self) -> ::margins_f::MarginsF {
      let margins = self;
      {
        let mut object: ::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMargins_G_operator_unary_plus_to_output_QMarginsF(margins as *const ::margins_f::MarginsF, &mut object);
        }
        object
      }
    }
  }
}
