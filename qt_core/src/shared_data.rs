/// C++ type: <span style='color: green;'>```QSharedData```</span>
#[repr(C)]
pub struct SharedData([u8; ::type_sizes::QT_CORE_SHARED_DATA_SHARED_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedData {
  unsafe fn new_uninitialized() -> SharedData {
    SharedData(::std::mem::uninitialized())
  }
}

impl SharedData {
  /// C++ method: <span style='color: green;'>```QSharedData::QSharedData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_data::SharedData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedData::QSharedData()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_data::SharedData) -> ::shared_data::SharedData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedData::QSharedData(const QSharedData& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_data::SharedData
    where Args: overloading::SharedDataNewArgs
  {
    args.exec()
  }
}

impl Drop for ::shared_data::SharedData {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedData::~QSharedData()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSharedData_destructor(self as *mut ::shared_data::SharedData) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SharedData::new](../struct.SharedData.html#method.new) method.
  pub trait SharedDataNewArgs {
    fn exec(self) -> ::shared_data::SharedData;
  }
  impl<'a> SharedDataNewArgs for &'a ::shared_data::SharedData {
    fn exec(self) -> ::shared_data::SharedData {
      let arg1 = self;
      {
        let mut object: ::shared_data::SharedData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSharedData_constructor_arg1(arg1 as *const ::shared_data::SharedData, &mut object);
        }
        object
      }
    }
  }
  impl SharedDataNewArgs for () {
    fn exec(self) -> ::shared_data::SharedData {

      {
        let mut object: ::shared_data::SharedData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSharedData_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}
