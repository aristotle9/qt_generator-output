/// C++ method: <span style='color: green;'>```qCountLeadingZeroBits```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn count_leading_zero_bits0(u16) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountLeadingZeroBits(quint16 v)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn count_leading_zero_bits0(u32) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountLeadingZeroBits(quint32 v)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn count_leading_zero_bits0(u64) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountLeadingZeroBits(quint64 v)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn count_leading_zero_bits0(u8) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountLeadingZeroBits(quint8 v)```</span>
///
///
pub fn count_leading_zero_bits0<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::CountLeadingZeroBits0Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```unsigned int qCountLeadingZeroBits(unsigned long v)```</span>
///
///
pub fn count_leading_zero_bits1(v: ::libc::c_ulong) -> ::libc::c_uint {
  unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountLeadingZeroBits_unsigned_long(v) }
}

/// C++ method: <span style='color: green;'>```qCountTrailingZeroBits```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn count_trailing_zero_bits0(u16) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountTrailingZeroBits(quint16 v)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn count_trailing_zero_bits0(u32) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountTrailingZeroBits(quint32 v)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn count_trailing_zero_bits0(u64) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountTrailingZeroBits(quint64 v)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn count_trailing_zero_bits0(u8) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qCountTrailingZeroBits(quint8 v)```</span>
///
///
pub fn count_trailing_zero_bits0<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::CountTrailingZeroBits0Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```unsigned int qCountTrailingZeroBits(unsigned long v)```</span>
///
///
pub fn count_trailing_zero_bits1(v: ::libc::c_ulong) -> ::libc::c_uint {
  unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountTrailingZeroBits_unsigned_long(v) }
}

/// C++ method: <span style='color: green;'>```qPopulationCount```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn population_count0(u16) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qPopulationCount(quint16 v)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn population_count0(u32) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qPopulationCount(quint32 v)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn population_count0(u64) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qPopulationCount(quint64 v)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn population_count0(u8) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qPopulationCount(quint8 v)```</span>
///
///
pub fn population_count0<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::PopulationCount0Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```unsigned int qPopulationCount(unsigned long v)```</span>
///
///
pub fn population_count1(v: ::libc::c_ulong) -> ::libc::c_uint {
  unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qPopulationCount_unsigned_long(v) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [count_leading_zero_bits0](../fn.count_leading_zero_bits0.html) method.
  pub trait CountLeadingZeroBits0Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl CountLeadingZeroBits0Args for u16 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountLeadingZeroBits_quint16(v) }
    }
  }
  impl CountLeadingZeroBits0Args for u32 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountLeadingZeroBits_quint32(v) }
    }
  }
  impl CountLeadingZeroBits0Args for u64 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountLeadingZeroBits_quint64(v) }
    }
  }
  impl CountLeadingZeroBits0Args for u8 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountLeadingZeroBits_quint8(v) }
    }
  }
  /// This trait represents a set of arguments accepted by [count_trailing_zero_bits0](../fn.count_trailing_zero_bits0.html) method.
  pub trait CountTrailingZeroBits0Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl CountTrailingZeroBits0Args for u16 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountTrailingZeroBits_quint16(v) }
    }
  }
  impl CountTrailingZeroBits0Args for u32 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountTrailingZeroBits_quint32(v) }
    }
  }
  impl CountTrailingZeroBits0Args for u64 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountTrailingZeroBits_quint64(v) }
    }
  }
  impl CountTrailingZeroBits0Args for u8 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qCountTrailingZeroBits_quint8(v) }
    }
  }
  /// This trait represents a set of arguments accepted by [population_count0](../fn.population_count0.html) method.
  pub trait PopulationCount0Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl PopulationCount0Args for u16 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qPopulationCount_quint16(v) }
    }
  }
  impl PopulationCount0Args for u32 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qPopulationCount_quint32(v) }
    }
  }
  impl PopulationCount0Args for u64 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qPopulationCount_quint64(v) }
    }
  }
  impl PopulationCount0Args for u8 {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtAlgorithms_G_qPopulationCount_quint8(v) }
    }
  }
}
