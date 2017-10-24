/// C++ type: <span style='color: green;'>```QSizePolicy::ControlType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ControlType {
  /// C++ enum variant: <span style='color: green;'>```DefaultType = 1```</span>
  DefaultType = 1,
  /// C++ enum variant: <span style='color: green;'>```ButtonBox = 2```</span>
  ButtonBox = 2,
  /// C++ enum variant: <span style='color: green;'>```CheckBox = 4```</span>
  CheckBox = 4,
  /// C++ enum variant: <span style='color: green;'>```ComboBox = 8```</span>
  ComboBox = 8,
  /// C++ enum variant: <span style='color: green;'>```Frame = 16```</span>
  Frame = 16,
  /// C++ enum variant: <span style='color: green;'>```GroupBox = 32```</span>
  GroupBox = 32,
  /// C++ enum variant: <span style='color: green;'>```Label = 64```</span>
  Label = 64,
  /// C++ enum variant: <span style='color: green;'>```Line = 128```</span>
  Line = 128,
  /// C++ enum variant: <span style='color: green;'>```LineEdit = 256```</span>
  LineEdit = 256,
  /// C++ enum variant: <span style='color: green;'>```PushButton = 512```</span>
  PushButton = 512,
  /// C++ enum variant: <span style='color: green;'>```RadioButton = 1024```</span>
  RadioButton = 1024,
  /// C++ enum variant: <span style='color: green;'>```Slider = 2048```</span>
  Slider = 2048,
  /// C++ enum variant: <span style='color: green;'>```SpinBox = 4096```</span>
  SpinBox = 4096,
  /// C++ enum variant: <span style='color: green;'>```TabWidget = 8192```</span>
  TabWidget = 8192,
  /// C++ enum variant: <span style='color: green;'>```ToolButton = 16384```</span>
  ToolButton = 16384,
}

/// C++ type: <span style='color: green;'>```QSizePolicy::Policy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Policy {
  /// C++ enum variant: <span style='color: green;'>```Fixed = 0```</span>
  Fixed = 0,
  /// C++ enum variant: <span style='color: green;'>```Minimum = 1```</span>
  Minimum = 1,
  /// C++ enum variant: <span style='color: green;'>```MinimumExpanding = 3```</span>
  MinimumExpanding = 3,
  /// C++ enum variant: <span style='color: green;'>```Maximum = 4```</span>
  Maximum = 4,
  /// C++ enum variant: <span style='color: green;'>```Preferred = 5```</span>
  Preferred = 5,
  /// C++ enum variant: <span style='color: green;'>```Expanding = 7```</span>
  Expanding = 7,
  /// C++ enum variant: <span style='color: green;'>```Ignored = 13```</span>
  Ignored = 13,
}

/// C++ type: <span style='color: green;'>```QSizePolicy::PolicyFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PolicyFlag {
  /// C++ enum variant: <span style='color: green;'>```GrowFlag = 1```</span>
  Grow = 1,
  /// C++ enum variant: <span style='color: green;'>```ExpandFlag = 2```</span>
  Expand = 2,
  /// C++ enum variant: <span style='color: green;'>```ShrinkFlag = 4```</span>
  Shrink = 4,
  /// C++ enum variant: <span style='color: green;'>```IgnoreFlag = 8```</span>
  Ignore = 8,
}

/// C++ type: <span style='color: green;'>```QSizePolicy```</span>
#[repr(C)]
pub struct SizePolicy([u8; ::type_sizes::QT_WIDGETS_SIZE_POLICY_SIZE_POLICY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SizePolicy {
  unsafe fn new_uninitialized() -> SizePolicy {
    SizePolicy(::std::mem::uninitialized())
  }
}

impl SizePolicy {
  /// C++ method: <span style='color: green;'>```QVariant QSizePolicy::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSizePolicy_convert_to_QVariant_to_output(self as *const ::size_policy::SizePolicy,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizePolicy::ControlType QSizePolicy::controlType() const```</span>
  ///
  ///
  pub fn control_type(&self) -> ::size_policy::ControlType {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_controlType(self as *const ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```bool QSizePolicy::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_hasHeightForWidth(self as *const ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```bool QSizePolicy::hasWidthForHeight() const```</span>
  ///
  ///
  pub fn has_width_for_height(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_hasWidthForHeight(self as *const ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```QSizePolicy::Policy QSizePolicy::horizontalPolicy() const```</span>
  ///
  ///
  pub fn horizontal_policy(&self) -> ::size_policy::Policy {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_horizontalPolicy(self as *const ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```int QSizePolicy::horizontalStretch() const```</span>
  ///
  ///
  pub fn horizontal_stretch(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_horizontalStretch(self as *const ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```QSizePolicy::QSizePolicy```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::size_policy::SizePolicy```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSizePolicy::QSizePolicy()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::size_policy::Policy, ::size_policy::Policy)) -> ::size_policy::SizePolicy```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSizePolicy::QSizePolicy(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::size_policy::Policy, ::size_policy::Policy, ::size_policy::ControlType)) -> ::size_policy::SizePolicy```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSizePolicy::QSizePolicy(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical, QSizePolicy::ControlType type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::size_policy::SizePolicy
    where Args: overloading::SizePolicyNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QSizePolicy::operator==(const QSizePolicy& s) const```</span>
  ///
  ///
  pub fn op_eq(&self, s: &::size_policy::SizePolicy) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QSizePolicy_operator_eq(self as *const ::size_policy::SizePolicy,
                                                  s as *const ::size_policy::SizePolicy)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSizePolicy::operator!=(const QSizePolicy& s) const```</span>
  ///
  ///
  pub fn op_neq(&self, s: &::size_policy::SizePolicy) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QSizePolicy_operator_neq(self as *const ::size_policy::SizePolicy,
                                                   s as *const ::size_policy::SizePolicy)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSizePolicy::retainSizeWhenHidden() const```</span>
  ///
  ///
  pub fn retain_size_when_hidden(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_retainSizeWhenHidden(self as *const ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setControlType(QSizePolicy::ControlType type)```</span>
  ///
  ///
  pub fn set_control_type(&mut self, type_: ::size_policy::ControlType) {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_setControlType(self as *mut ::size_policy::SizePolicy, type_) }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setHeightForWidth(bool b)```</span>
  ///
  ///
  pub fn set_height_for_width(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_setHeightForWidth(self as *mut ::size_policy::SizePolicy, b) }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setHorizontalPolicy(QSizePolicy::Policy d)```</span>
  ///
  ///
  pub fn set_horizontal_policy(&mut self, d: ::size_policy::Policy) {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_setHorizontalPolicy(self as *mut ::size_policy::SizePolicy, d) }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setHorizontalStretch(int stretchFactor)```</span>
  ///
  ///
  pub fn set_horizontal_stretch(&mut self, stretch_factor: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QSizePolicy_setHorizontalStretch(self as *mut ::size_policy::SizePolicy, stretch_factor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setRetainSizeWhenHidden(bool retainSize)```</span>
  ///
  ///
  pub fn set_retain_size_when_hidden(&mut self, retain_size: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QSizePolicy_setRetainSizeWhenHidden(self as *mut ::size_policy::SizePolicy, retain_size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setVerticalPolicy(QSizePolicy::Policy d)```</span>
  ///
  ///
  pub fn set_vertical_policy(&mut self, d: ::size_policy::Policy) {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_setVerticalPolicy(self as *mut ::size_policy::SizePolicy, d) }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setVerticalStretch(int stretchFactor)```</span>
  ///
  ///
  pub fn set_vertical_stretch(&mut self, stretch_factor: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QSizePolicy_setVerticalStretch(self as *mut ::size_policy::SizePolicy, stretch_factor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::setWidthForHeight(bool b)```</span>
  ///
  ///
  pub fn set_width_for_height(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_setWidthForHeight(self as *mut ::size_policy::SizePolicy, b) }
  }

  /// C++ method: <span style='color: green;'>```void QSizePolicy::transpose()```</span>
  ///
  ///
  pub fn transpose(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_transpose(self as *mut ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```QSizePolicy QSizePolicy::transposed() const```</span>
  ///
  ///
  pub fn transposed(&self) -> ::size_policy::SizePolicy {
    {
      let mut object: ::size_policy::SizePolicy =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSizePolicy_transposed_to_output(self as *const ::size_policy::SizePolicy, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizePolicy::Policy QSizePolicy::verticalPolicy() const```</span>
  ///
  ///
  pub fn vertical_policy(&self) -> ::size_policy::Policy {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_verticalPolicy(self as *const ::size_policy::SizePolicy) }
  }

  /// C++ method: <span style='color: green;'>```int QSizePolicy::verticalStretch() const```</span>
  ///
  ///
  pub fn vertical_stretch(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_verticalStretch(self as *const ::size_policy::SizePolicy) }
  }
}

impl Drop for ::size_policy::SizePolicy {
  /// C++ method: <span style='color: green;'>```[destructor] void QSizePolicy::~QSizePolicy()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QSizePolicy_destructor(self as *mut ::size_policy::SizePolicy) }
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::size_policy::SizePolicy) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(QSizePolicy key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::size_policy::SizePolicy, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(QSizePolicy key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QSizePolicy& arg2)```</span>
///
///
pub fn op_shl(dbg: &::qt_core::debug::Debug, arg2: &::size_policy::SizePolicy) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_widgets_c_QSizePolicy_G_operator_shl_to_output(dbg as *const ::qt_core::debug::Debug,
                                                               arg2 as *const ::size_policy::SizePolicy,
                                                               &mut object);
    }
    object
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SizePolicy::new](../struct.SizePolicy.html#method.new) method.
  pub trait SizePolicyNewArgs {
    fn exec(self) -> ::size_policy::SizePolicy;
  }
  impl SizePolicyNewArgs for (::size_policy::Policy, ::size_policy::Policy) {
    fn exec(self) -> ::size_policy::SizePolicy {
      let horizontal = self.0;
      let vertical = self.1;
      {
        let mut object: ::size_policy::SizePolicy =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QSizePolicy_constructor_horizontal_vertical(horizontal, vertical, &mut object);
        }
        object
      }
    }
  }
  impl SizePolicyNewArgs for (::size_policy::Policy, ::size_policy::Policy, ::size_policy::ControlType) {
    fn exec(self) -> ::size_policy::SizePolicy {
      let horizontal = self.0;
      let vertical = self.1;
      let type_ = self.2;
      {
        let mut object: ::size_policy::SizePolicy =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QSizePolicy_constructor_horizontal_vertical_type(horizontal,
                                                                               vertical,
                                                                               type_,
                                                                               &mut object);
        }
        object
      }
    }
  }
  impl SizePolicyNewArgs for () {
    fn exec(self) -> ::size_policy::SizePolicy {

      {
        let mut object: ::size_policy::SizePolicy =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QSizePolicy_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::size_policy::SizePolicy {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_widgets_c_QSizePolicy_G_qHash_key(key as *const ::size_policy::SizePolicy) }
    }
  }
  impl<'a> HashArgs for (&'a ::size_policy::SizePolicy, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_widgets_c_QSizePolicy_G_qHash_key_seed(key as *const ::size_policy::SizePolicy, seed) }
    }
  }
}
