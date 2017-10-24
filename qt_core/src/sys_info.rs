/// C++ type: <span style='color: green;'>```QSysInfo::Endian```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Endian {
  /// C++ enum variant: <span style='color: green;'>```BigEndian = 0```</span>
  Big = 0,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```LittleEndian = 1```</span>
  /// - <span style='color: green;'>```ByteOrder = 1```</span>
  ///
  Little = 1,
}

/// C++ type: <span style='color: green;'>```QSysInfo::MacVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MacVersion {
  /// C++ enum variant: <span style='color: green;'>```MV_Unknown = 0```</span>
  VUnknown = 0,
  /// C++ enum variant: <span style='color: green;'>```MV_9 = 1```</span>
  V9 = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_0 = 2```</span>
  /// - <span style='color: green;'>```MV_CHEETAH = 2```</span>
  ///
  V100 = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_1 = 3```</span>
  /// - <span style='color: green;'>```MV_PUMA = 3```</span>
  ///
  V101 = 3,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_2 = 4```</span>
  /// - <span style='color: green;'>```MV_JAGUAR = 4```</span>
  ///
  V102 = 4,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_3 = 5```</span>
  /// - <span style='color: green;'>```MV_PANTHER = 5```</span>
  ///
  V103 = 5,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_4 = 6```</span>
  /// - <span style='color: green;'>```MV_TIGER = 6```</span>
  ///
  V104 = 6,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_5 = 7```</span>
  /// - <span style='color: green;'>```MV_LEOPARD = 7```</span>
  ///
  V105 = 7,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_6 = 8```</span>
  /// - <span style='color: green;'>```MV_SNOWLEOPARD = 8```</span>
  ///
  V106 = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_7 = 9```</span>
  /// - <span style='color: green;'>```MV_LION = 9```</span>
  ///
  V107 = 9,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_8 = 10```</span>
  /// - <span style='color: green;'>```MV_MOUNTAINLION = 10```</span>
  ///
  V108 = 10,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_9 = 11```</span>
  /// - <span style='color: green;'>```MV_MAVERICKS = 11```</span>
  ///
  V109 = 11,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_10 = 12```</span>
  /// - <span style='color: green;'>```MV_YOSEMITE = 12```</span>
  ///
  V1010 = 12,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_11 = 13```</span>
  /// - <span style='color: green;'>```MV_ELCAPITAN = 13```</span>
  ///
  V1011 = 13,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MV_10_12 = 14```</span>
  /// - <span style='color: green;'>```MV_SIERRA = 14```</span>
  ///
  V1012 = 14,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS = 256```</span>
  VIOS = 256,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_4_3 = 323```</span>
  VIOS43 = 323,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_5_0 = 336```</span>
  VIOS50 = 336,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_5_1 = 337```</span>
  VIOS51 = 337,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_6_0 = 352```</span>
  VIOS60 = 352,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_6_1 = 353```</span>
  VIOS61 = 353,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_7_0 = 368```</span>
  VIOS70 = 368,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_7_1 = 369```</span>
  VIOS71 = 369,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_8_0 = 384```</span>
  VIOS80 = 384,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_8_1 = 385```</span>
  VIOS81 = 385,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_8_2 = 386```</span>
  VIOS82 = 386,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_8_3 = 387```</span>
  VIOS83 = 387,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_8_4 = 388```</span>
  VIOS84 = 388,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_9_0 = 400```</span>
  VIOS90 = 400,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_9_1 = 401```</span>
  VIOS91 = 401,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_9_2 = 402```</span>
  VIOS92 = 402,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_9_3 = 403```</span>
  VIOS93 = 403,
  /// C++ enum variant: <span style='color: green;'>```MV_IOS_10_0 = 416```</span>
  VIOS100 = 416,
  /// C++ enum variant: <span style='color: green;'>```MV_TVOS = 512```</span>
  VTVOS = 512,
  /// C++ enum variant: <span style='color: green;'>```MV_TVOS_9_0 = 656```</span>
  VTVOS90 = 656,
  /// C++ enum variant: <span style='color: green;'>```MV_TVOS_9_1 = 657```</span>
  VTVOS91 = 657,
  /// C++ enum variant: <span style='color: green;'>```MV_TVOS_9_2 = 658```</span>
  VTVOS92 = 658,
  /// C++ enum variant: <span style='color: green;'>```MV_TVOS_10_0 = 672```</span>
  VTVOS100 = 672,
  /// C++ enum variant: <span style='color: green;'>```MV_WATCHOS = 1024```</span>
  VWATCHOS = 1024,
  /// C++ enum variant: <span style='color: green;'>```MV_WATCHOS_2_0 = 1056```</span>
  VWATCHOS20 = 1056,
  /// C++ enum variant: <span style='color: green;'>```MV_WATCHOS_2_1 = 1057```</span>
  VWATCHOS21 = 1057,
  /// C++ enum variant: <span style='color: green;'>```MV_WATCHOS_2_2 = 1058```</span>
  VWATCHOS22 = 1058,
  /// C++ enum variant: <span style='color: green;'>```MV_WATCHOS_3_0 = 1072```</span>
  VWATCHOS30 = 1072,
  /// C++ enum variant: <span style='color: green;'>```MV_None = 65535```</span>
  VNone = 65535,
}

/// C++ type: <span style='color: green;'>```QSysInfo::Sizes```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Sizes {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```WordSize = 64```</span>
  WordSize = 64,
}

/// C++ type: <span style='color: green;'>```QSysInfo```</span>
#[repr(C)]
pub struct SysInfo(u8);

impl SysInfo {
  /// C++ method: <span style='color: green;'>```static QString QSysInfo::buildAbi()```</span>
  ///
  ///
  pub fn build_abi() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_buildAbi_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::buildCpuArchitecture()```</span>
  ///
  ///
  pub fn build_cpu_architecture() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_buildCpuArchitecture_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::currentCpuArchitecture()```</span>
  ///
  ///
  pub fn current_cpu_architecture() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_currentCpuArchitecture_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::kernelType()```</span>
  ///
  ///
  pub fn kernel_type() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_kernelType_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::kernelVersion()```</span>
  ///
  ///
  pub fn kernel_version() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_kernelVersion_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QSysInfo::MacVersion QSysInfo::macVersion()```</span>
  ///
  ///
  pub fn mac_version() -> ::sys_info::MacVersion {
    unsafe { ::ffi::qt_core_c_QSysInfo_macVersion() }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::machineHostName()```</span>
  ///
  ///
  pub fn machine_host_name() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_machineHostName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::prettyProductName()```</span>
  ///
  ///
  pub fn pretty_product_name() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_prettyProductName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::productType()```</span>
  ///
  ///
  pub fn product_type() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_productType_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSysInfo::productVersion()```</span>
  ///
  ///
  pub fn product_version() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSysInfo_productVersion_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QSysInfo::WinVersion QSysInfo::windowsVersion()```</span>
  ///
  ///
  pub fn windows_version() -> ::sys_info::WinVersion {
    unsafe { ::ffi::qt_core_c_QSysInfo_windowsVersion() }
  }
}

impl ::cpp_utils::CppDeletable for ::sys_info::SysInfo {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSysInfo_delete
  }
}

/// C++ type: <span style='color: green;'>```QSysInfo::WinVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WinVersion {
  /// C++ enum variant: <span style='color: green;'>```WV_None = 0```</span>
  VNone = 0,
  /// C++ enum variant: <span style='color: green;'>```WV_32s = 1```</span>
  V32S = 1,
  /// C++ enum variant: <span style='color: green;'>```WV_95 = 2```</span>
  V95 = 2,
  /// C++ enum variant: <span style='color: green;'>```WV_98 = 3```</span>
  V98 = 3,
  /// C++ enum variant: <span style='color: green;'>```WV_Me = 4```</span>
  VMe = 4,
  /// C++ enum variant: <span style='color: green;'>```WV_DOS_based = 15```</span>
  VDOSBased = 15,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_NT = 16```</span>
  /// - <span style='color: green;'>```WV_4_0 = 16```</span>
  ///
  VNT = 16,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_2000 = 32```</span>
  /// - <span style='color: green;'>```WV_5_0 = 32```</span>
  ///
  V2000 = 32,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_XP = 48```</span>
  /// - <span style='color: green;'>```WV_5_1 = 48```</span>
  ///
  VXP = 48,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_2003 = 64```</span>
  /// - <span style='color: green;'>```WV_5_2 = 64```</span>
  ///
  V2003 = 64,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_VISTA = 128```</span>
  /// - <span style='color: green;'>```WV_6_0 = 128```</span>
  ///
  VVISTA = 128,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_WINDOWS7 = 144```</span>
  /// - <span style='color: green;'>```WV_6_1 = 144```</span>
  ///
  VWINDOWS7 = 144,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_WINDOWS8 = 160```</span>
  /// - <span style='color: green;'>```WV_6_2 = 160```</span>
  ///
  VWINDOWS8 = 160,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_WINDOWS8_1 = 176```</span>
  /// - <span style='color: green;'>```WV_6_3 = 176```</span>
  ///
  VWINDOWS81 = 176,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WV_WINDOWS10 = 192```</span>
  /// - <span style='color: green;'>```WV_10_0 = 192```</span>
  ///
  VWINDOWS10 = 192,
  /// C++ enum variant: <span style='color: green;'>```WV_NT_based = 240```</span>
  VNTBased = 240,
  /// C++ enum variant: <span style='color: green;'>```WV_CE = 256```</span>
  VCE = 256,
  /// C++ enum variant: <span style='color: green;'>```WV_CENET = 512```</span>
  VCENET = 512,
  /// C++ enum variant: <span style='color: green;'>```WV_CE_5 = 768```</span>
  VCE5 = 768,
  /// C++ enum variant: <span style='color: green;'>```WV_CE_6 = 1024```</span>
  VCE6 = 1024,
  /// C++ enum variant: <span style='color: green;'>```WV_CE_based = 3840```</span>
  VCEBased = 3840,
}
