/// C++ type: <span style='color: green;'>```Qt::AlignmentFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AlignmentFlag {
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```AlignLeft = 1```</span>
  /// - <span style='color: green;'>```AlignLeading = 1```</span>
  ///
  Left = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```AlignRight = 2```</span>
  /// - <span style='color: green;'>```AlignTrailing = 2```</span>
  ///
  Right = 2,
  /// C++ enum variant: <span style='color: green;'>```AlignHCenter = 4```</span>
  HCenter = 4,
  /// C++ enum variant: <span style='color: green;'>```AlignJustify = 8```</span>
  Justify = 8,
  /// C++ enum variant: <span style='color: green;'>```AlignAbsolute = 16```</span>
  Absolute = 16,
  /// C++ enum variant: <span style='color: green;'>```AlignHorizontal_Mask = 31```</span>
  HorizontalMask = 31,
  /// C++ enum variant: <span style='color: green;'>```AlignTop = 32```</span>
  Top = 32,
  /// C++ enum variant: <span style='color: green;'>```AlignBottom = 64```</span>
  Bottom = 64,
  /// C++ enum variant: <span style='color: green;'>```AlignVCenter = 128```</span>
  VCenter = 128,
  /// C++ enum variant: <span style='color: green;'>```AlignCenter = 132```</span>
  Center = 132,
  /// C++ enum variant: <span style='color: green;'>```AlignBaseline = 256```</span>
  Baseline = 256,
  /// C++ enum variant: <span style='color: green;'>```AlignVertical_Mask = 480```</span>
  VerticalMask = 480,
}

impl ::flags::FlaggableEnum for AlignmentFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "AlignmentFlag"
  }
}

/// C++ type: <span style='color: green;'>```Qt::AnchorPoint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AnchorPoint {
  /// C++ enum variant: <span style='color: green;'>```AnchorLeft = 0```</span>
  Left = 0,
  /// C++ enum variant: <span style='color: green;'>```AnchorHorizontalCenter = 1```</span>
  HorizontalCenter = 1,
  /// C++ enum variant: <span style='color: green;'>```AnchorRight = 2```</span>
  Right = 2,
  /// C++ enum variant: <span style='color: green;'>```AnchorTop = 3```</span>
  Top = 3,
  /// C++ enum variant: <span style='color: green;'>```AnchorVerticalCenter = 4```</span>
  VerticalCenter = 4,
  /// C++ enum variant: <span style='color: green;'>```AnchorBottom = 5```</span>
  Bottom = 5,
}

/// C++ type: <span style='color: green;'>```Qt::ApplicationAttribute```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ApplicationAttribute {
  /// C++ enum variant: <span style='color: green;'>```AA_ImmediateWidgetCreation = 0```</span>
  ImmediateWidgetCreation = 0,
  /// C++ enum variant: <span style='color: green;'>```AA_MSWindowsUseDirect3DByDefault = 1```</span>
  MSWindowsUseDirect3DByDefault = 1,
  /// C++ enum variant: <span style='color: green;'>```AA_DontShowIconsInMenus = 2```</span>
  DontShowIconsInMenus = 2,
  /// C++ enum variant: <span style='color: green;'>```AA_NativeWindows = 3```</span>
  NativeWindows = 3,
  /// C++ enum variant: <span style='color: green;'>```AA_DontCreateNativeWidgetSiblings = 4```</span>
  DontCreateNativeWidgetSiblings = 4,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```AA_PluginApplication = 5```</span>
  /// - <span style='color: green;'>```AA_MacPluginApplication = 5```</span>
  ///
  PluginApplication = 5,
  /// C++ enum variant: <span style='color: green;'>```AA_DontUseNativeMenuBar = 6```</span>
  DontUseNativeMenuBar = 6,
  /// C++ enum variant: <span style='color: green;'>```AA_MacDontSwapCtrlAndMeta = 7```</span>
  MacDontSwapCtrlAndMeta = 7,
  /// C++ enum variant: <span style='color: green;'>```AA_Use96Dpi = 8```</span>
  Use96Dpi = 8,
  /// C++ enum variant: <span style='color: green;'>```AA_X11InitThreads = 10```</span>
  X11InitThreads = 10,
  /// C++ enum variant: <span style='color: green;'>```AA_SynthesizeTouchForUnhandledMouseEvents = 11```</span>
  SynthesizeTouchForUnhandledMouseEvents = 11,
  /// C++ enum variant: <span style='color: green;'>```AA_SynthesizeMouseForUnhandledTouchEvents = 12```</span>
  SynthesizeMouseForUnhandledTouchEvents = 12,
  /// C++ enum variant: <span style='color: green;'>```AA_UseHighDpiPixmaps = 13```</span>
  UseHighDpiPixmaps = 13,
  /// C++ enum variant: <span style='color: green;'>```AA_ForceRasterWidgets = 14```</span>
  ForceRasterWidgets = 14,
  /// C++ enum variant: <span style='color: green;'>```AA_UseDesktopOpenGL = 15```</span>
  UseDesktopOpenGL = 15,
  /// C++ enum variant: <span style='color: green;'>```AA_UseOpenGLES = 16```</span>
  UseOpenGLES = 16,
  /// C++ enum variant: <span style='color: green;'>```AA_UseSoftwareOpenGL = 17```</span>
  UseSoftwareOpenGL = 17,
  /// C++ enum variant: <span style='color: green;'>```AA_ShareOpenGLContexts = 18```</span>
  ShareOpenGLContexts = 18,
  /// C++ enum variant: <span style='color: green;'>```AA_SetPalette = 19```</span>
  SetPalette = 19,
  /// C++ enum variant: <span style='color: green;'>```AA_EnableHighDpiScaling = 20```</span>
  EnableHighDpiScaling = 20,
  /// C++ enum variant: <span style='color: green;'>```AA_DisableHighDpiScaling = 21```</span>
  DisableHighDpiScaling = 21,
  /// C++ enum variant: <span style='color: green;'>```AA_UseStyleSheetPropagationInWidgetStyles = 22```</span>
  UseStyleSheetPropagationInWidgetStyles = 22,
  /// C++ enum variant: <span style='color: green;'>```AA_DontUseNativeDialogs = 23```</span>
  DontUseNativeDialogs = 23,
  /// C++ enum variant: <span style='color: green;'>```AA_SynthesizeMouseForUnhandledTabletEvents = 24```</span>
  SynthesizeMouseForUnhandledTabletEvents = 24,
  /// C++ enum variant: <span style='color: green;'>```AA_CompressHighFrequencyEvents = 25```</span>
  CompressHighFrequencyEvents = 25,
  /// C++ enum variant: <span style='color: green;'>```AA_DontCheckOpenGLContextThreadAffinity = 26```</span>
  DontCheckOpenGLContextThreadAffinity = 26,
  /// C++ enum variant: <span style='color: green;'>```AA_DisableShaderDiskCache = 27```</span>
  DisableShaderDiskCache = 27,
  /// C++ enum variant: <span style='color: green;'>```AA_AttributeCount = 28```</span>
  AttributeCount = 28,
}

/// C++ type: <span style='color: green;'>```Qt::ApplicationState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ApplicationState {
  /// C++ enum variant: <span style='color: green;'>```ApplicationSuspended = 0```</span>
  Suspended = 0,
  /// C++ enum variant: <span style='color: green;'>```ApplicationHidden = 1```</span>
  Hidden = 1,
  /// C++ enum variant: <span style='color: green;'>```ApplicationInactive = 2```</span>
  Inactive = 2,
  /// C++ enum variant: <span style='color: green;'>```ApplicationActive = 4```</span>
  Active = 4,
}

/// C++ type: <span style='color: green;'>```Qt::ArrowType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ArrowType {
  /// C++ enum variant: <span style='color: green;'>```NoArrow = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```UpArrow = 1```</span>
  Up = 1,
  /// C++ enum variant: <span style='color: green;'>```DownArrow = 2```</span>
  Down = 2,
  /// C++ enum variant: <span style='color: green;'>```LeftArrow = 3```</span>
  Left = 3,
  /// C++ enum variant: <span style='color: green;'>```RightArrow = 4```</span>
  Right = 4,
}

/// C++ type: <span style='color: green;'>```Qt::AspectRatioMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AspectRatioMode {
  /// C++ enum variant: <span style='color: green;'>```IgnoreAspectRatio = 0```</span>
  IgnoreAspectRatio = 0,
  /// C++ enum variant: <span style='color: green;'>```KeepAspectRatio = 1```</span>
  KeepAspectRatio = 1,
  /// C++ enum variant: <span style='color: green;'>```KeepAspectRatioByExpanding = 2```</span>
  KeepAspectRatioByExpanding = 2,
}

/// C++ type: <span style='color: green;'>```Qt::Axis```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Axis {
  /// C++ enum variant: <span style='color: green;'>```XAxis = 0```</span>
  X = 0,
  /// C++ enum variant: <span style='color: green;'>```YAxis = 1```</span>
  Y = 1,
  /// C++ enum variant: <span style='color: green;'>```ZAxis = 2```</span>
  Z = 2,
}

/// C++ type: <span style='color: green;'>```Qt::BGMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BGMode {
  /// C++ enum variant: <span style='color: green;'>```TransparentMode = 0```</span>
  Transparent = 0,
  /// C++ enum variant: <span style='color: green;'>```OpaqueMode = 1```</span>
  Opaque = 1,
}

/// C++ type: <span style='color: green;'>```Qt::BrushStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BrushStyle {
  /// C++ enum variant: <span style='color: green;'>```NoBrush = 0```</span>
  NoBrush = 0,
  /// C++ enum variant: <span style='color: green;'>```SolidPattern = 1```</span>
  SolidPattern = 1,
  /// C++ enum variant: <span style='color: green;'>```Dense1Pattern = 2```</span>
  Dense1Pattern = 2,
  /// C++ enum variant: <span style='color: green;'>```Dense2Pattern = 3```</span>
  Dense2Pattern = 3,
  /// C++ enum variant: <span style='color: green;'>```Dense3Pattern = 4```</span>
  Dense3Pattern = 4,
  /// C++ enum variant: <span style='color: green;'>```Dense4Pattern = 5```</span>
  Dense4Pattern = 5,
  /// C++ enum variant: <span style='color: green;'>```Dense5Pattern = 6```</span>
  Dense5Pattern = 6,
  /// C++ enum variant: <span style='color: green;'>```Dense6Pattern = 7```</span>
  Dense6Pattern = 7,
  /// C++ enum variant: <span style='color: green;'>```Dense7Pattern = 8```</span>
  Dense7Pattern = 8,
  /// C++ enum variant: <span style='color: green;'>```HorPattern = 9```</span>
  HorPattern = 9,
  /// C++ enum variant: <span style='color: green;'>```VerPattern = 10```</span>
  VerPattern = 10,
  /// C++ enum variant: <span style='color: green;'>```CrossPattern = 11```</span>
  CrossPattern = 11,
  /// C++ enum variant: <span style='color: green;'>```BDiagPattern = 12```</span>
  BDiagPattern = 12,
  /// C++ enum variant: <span style='color: green;'>```FDiagPattern = 13```</span>
  FDiagPattern = 13,
  /// C++ enum variant: <span style='color: green;'>```DiagCrossPattern = 14```</span>
  DiagCrossPattern = 14,
  /// C++ enum variant: <span style='color: green;'>```LinearGradientPattern = 15```</span>
  LinearGradientPattern = 15,
  /// C++ enum variant: <span style='color: green;'>```RadialGradientPattern = 16```</span>
  RadialGradientPattern = 16,
  /// C++ enum variant: <span style='color: green;'>```ConicalGradientPattern = 17```</span>
  ConicalGradientPattern = 17,
  /// C++ enum variant: <span style='color: green;'>```TexturePattern = 24```</span>
  TexturePattern = 24,
}

/// C++ type: <span style='color: green;'>```Qt::CaseSensitivity```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CaseSensitivity {
  /// C++ enum variant: <span style='color: green;'>```CaseInsensitive = 0```</span>
  Insensitive = 0,
  /// C++ enum variant: <span style='color: green;'>```CaseSensitive = 1```</span>
  Sensitive = 1,
}

/// C++ type: <span style='color: green;'>```Qt::CheckState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CheckState {
  /// C++ enum variant: <span style='color: green;'>```Unchecked = 0```</span>
  Unchecked = 0,
  /// C++ enum variant: <span style='color: green;'>```PartiallyChecked = 1```</span>
  PartiallyChecked = 1,
  /// C++ enum variant: <span style='color: green;'>```Checked = 2```</span>
  Checked = 2,
}

/// C++ type: <span style='color: green;'>```Qt::ChecksumType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ChecksumType {
  /// C++ enum variant: <span style='color: green;'>```ChecksumIso3309 = 0```</span>
  Iso3309 = 0,
  /// C++ enum variant: <span style='color: green;'>```ChecksumItuV41 = 1```</span>
  ItuV41 = 1,
}

/// C++ type: <span style='color: green;'>```Qt::ClipOperation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ClipOperation {
  /// C++ enum variant: <span style='color: green;'>```NoClip = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```ReplaceClip = 1```</span>
  Replace = 1,
  /// C++ enum variant: <span style='color: green;'>```IntersectClip = 2```</span>
  Intersect = 2,
}

/// C++ type: <span style='color: green;'>```Qt::ConnectionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ConnectionType {
  /// C++ enum variant: <span style='color: green;'>```AutoConnection = 0```</span>
  Auto = 0,
  /// C++ enum variant: <span style='color: green;'>```DirectConnection = 1```</span>
  Direct = 1,
  /// C++ enum variant: <span style='color: green;'>```QueuedConnection = 2```</span>
  Queued = 2,
  /// C++ enum variant: <span style='color: green;'>```BlockingQueuedConnection = 3```</span>
  BlockingQueued = 3,
  /// C++ enum variant: <span style='color: green;'>```UniqueConnection = 128```</span>
  Unique = 128,
}

/// C++ type: <span style='color: green;'>```Qt::ContextMenuPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ContextMenuPolicy {
  /// C++ enum variant: <span style='color: green;'>```NoContextMenu = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```DefaultContextMenu = 1```</span>
  Default = 1,
  /// C++ enum variant: <span style='color: green;'>```ActionsContextMenu = 2```</span>
  Actions = 2,
  /// C++ enum variant: <span style='color: green;'>```CustomContextMenu = 3```</span>
  Custom = 3,
  /// C++ enum variant: <span style='color: green;'>```PreventContextMenu = 4```</span>
  Prevent = 4,
}

/// C++ type: <span style='color: green;'>```Qt::CoordinateSystem```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CoordinateSystem {
  /// C++ enum variant: <span style='color: green;'>```DeviceCoordinates = 0```</span>
  Device = 0,
  /// C++ enum variant: <span style='color: green;'>```LogicalCoordinates = 1```</span>
  Logical = 1,
}

/// C++ type: <span style='color: green;'>```Qt::Corner```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Corner {
  /// C++ enum variant: <span style='color: green;'>```TopLeftCorner = 0```</span>
  TopLeft = 0,
  /// C++ enum variant: <span style='color: green;'>```TopRightCorner = 1```</span>
  TopRight = 1,
  /// C++ enum variant: <span style='color: green;'>```BottomLeftCorner = 2```</span>
  BottomLeft = 2,
  /// C++ enum variant: <span style='color: green;'>```BottomRightCorner = 3```</span>
  BottomRight = 3,
}

/// C++ type: <span style='color: green;'>```Qt::CursorMoveStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CursorMoveStyle {
  /// C++ enum variant: <span style='color: green;'>```LogicalMoveStyle = 0```</span>
  Logical = 0,
  /// C++ enum variant: <span style='color: green;'>```VisualMoveStyle = 1```</span>
  Visual = 1,
}

/// C++ type: <span style='color: green;'>```Qt::CursorShape```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CursorShape {
  /// C++ enum variant: <span style='color: green;'>```ArrowCursor = 0```</span>
  Arrow = 0,
  /// C++ enum variant: <span style='color: green;'>```UpArrowCursor = 1```</span>
  UpArrow = 1,
  /// C++ enum variant: <span style='color: green;'>```CrossCursor = 2```</span>
  Cross = 2,
  /// C++ enum variant: <span style='color: green;'>```WaitCursor = 3```</span>
  Wait = 3,
  /// C++ enum variant: <span style='color: green;'>```IBeamCursor = 4```</span>
  IBeam = 4,
  /// C++ enum variant: <span style='color: green;'>```SizeVerCursor = 5```</span>
  SizeVer = 5,
  /// C++ enum variant: <span style='color: green;'>```SizeHorCursor = 6```</span>
  SizeHor = 6,
  /// C++ enum variant: <span style='color: green;'>```SizeBDiagCursor = 7```</span>
  SizeBDiag = 7,
  /// C++ enum variant: <span style='color: green;'>```SizeFDiagCursor = 8```</span>
  SizeFDiag = 8,
  /// C++ enum variant: <span style='color: green;'>```SizeAllCursor = 9```</span>
  SizeAll = 9,
  /// C++ enum variant: <span style='color: green;'>```BlankCursor = 10```</span>
  Blank = 10,
  /// C++ enum variant: <span style='color: green;'>```SplitVCursor = 11```</span>
  SplitV = 11,
  /// C++ enum variant: <span style='color: green;'>```SplitHCursor = 12```</span>
  SplitH = 12,
  /// C++ enum variant: <span style='color: green;'>```PointingHandCursor = 13```</span>
  PointingHand = 13,
  /// C++ enum variant: <span style='color: green;'>```ForbiddenCursor = 14```</span>
  Forbidden = 14,
  /// C++ enum variant: <span style='color: green;'>```WhatsThisCursor = 15```</span>
  WhatsThis = 15,
  /// C++ enum variant: <span style='color: green;'>```BusyCursor = 16```</span>
  Busy = 16,
  /// C++ enum variant: <span style='color: green;'>```OpenHandCursor = 17```</span>
  OpenHand = 17,
  /// C++ enum variant: <span style='color: green;'>```ClosedHandCursor = 18```</span>
  ClosedHand = 18,
  /// C++ enum variant: <span style='color: green;'>```DragCopyCursor = 19```</span>
  DragCopy = 19,
  /// C++ enum variant: <span style='color: green;'>```DragMoveCursor = 20```</span>
  DragMove = 20,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```DragLinkCursor = 21```</span>
  /// - <span style='color: green;'>```LastCursor = 21```</span>
  ///
  DragLink = 21,
  /// C++ enum variant: <span style='color: green;'>```BitmapCursor = 24```</span>
  Bitmap = 24,
  /// C++ enum variant: <span style='color: green;'>```CustomCursor = 25```</span>
  Custom = 25,
}

/// C++ type: <span style='color: green;'>```Qt::DateFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DateFormat {
  /// C++ enum variant: <span style='color: green;'>```TextDate = 0```</span>
  TextDate = 0,
  /// C++ enum variant: <span style='color: green;'>```ISODate = 1```</span>
  ISODate = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SystemLocaleDate = 2```</span>
  /// - <span style='color: green;'>```LocalDate = 2```</span>
  ///
  SystemLocaleDate = 2,
  /// C++ enum variant: <span style='color: green;'>```LocaleDate = 3```</span>
  LocaleDate = 3,
  /// C++ enum variant: <span style='color: green;'>```SystemLocaleShortDate = 4```</span>
  SystemLocaleShortDate = 4,
  /// C++ enum variant: <span style='color: green;'>```SystemLocaleLongDate = 5```</span>
  SystemLocaleLongDate = 5,
  /// C++ enum variant: <span style='color: green;'>```DefaultLocaleShortDate = 6```</span>
  DefaultLocaleShortDate = 6,
  /// C++ enum variant: <span style='color: green;'>```DefaultLocaleLongDate = 7```</span>
  DefaultLocaleLongDate = 7,
  /// C++ enum variant: <span style='color: green;'>```RFC2822Date = 8```</span>
  RFC2822Date = 8,
  /// C++ enum variant: <span style='color: green;'>```ISODateWithMs = 9```</span>
  ISODateWithMs = 9,
}

/// C++ type: <span style='color: green;'>```Qt::DayOfWeek```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DayOfWeek {
  /// C++ enum variant: <span style='color: green;'>```Monday = 1```</span>
  Monday = 1,
  /// C++ enum variant: <span style='color: green;'>```Tuesday = 2```</span>
  Tuesday = 2,
  /// C++ enum variant: <span style='color: green;'>```Wednesday = 3```</span>
  Wednesday = 3,
  /// C++ enum variant: <span style='color: green;'>```Thursday = 4```</span>
  Thursday = 4,
  /// C++ enum variant: <span style='color: green;'>```Friday = 5```</span>
  Friday = 5,
  /// C++ enum variant: <span style='color: green;'>```Saturday = 6```</span>
  Saturday = 6,
  /// C++ enum variant: <span style='color: green;'>```Sunday = 7```</span>
  Sunday = 7,
}

/// C++ type: <span style='color: green;'>```Qt::DockWidgetArea```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DockWidgetArea {
  /// C++ enum variant: <span style='color: green;'>```NoDockWidgetArea = 0```</span>
  NoDockWidgetArea = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftDockWidgetArea = 1```</span>
  LeftDockWidgetArea = 1,
  /// C++ enum variant: <span style='color: green;'>```RightDockWidgetArea = 2```</span>
  RightDockWidgetArea = 2,
  /// C++ enum variant: <span style='color: green;'>```TopDockWidgetArea = 4```</span>
  TopDockWidgetArea = 4,
  /// C++ enum variant: <span style='color: green;'>```BottomDockWidgetArea = 8```</span>
  BottomDockWidgetArea = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```DockWidgetArea_Mask = 15```</span>
  /// - <span style='color: green;'>```AllDockWidgetAreas = 15```</span>
  ///
  DockWidgetAreaMask = 15,
}

impl ::flags::FlaggableEnum for DockWidgetArea {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "DockWidgetArea"
  }
}

/// C++ type: <span style='color: green;'>```Qt::DockWidgetAreaSizes```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DockWidgetAreaSizes {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```NDockWidgetAreas = 4```</span>
  NDockWidgetAreas = 4,
}

/// C++ type: <span style='color: green;'>```Qt::DropAction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DropAction {
  /// C++ enum variant: <span style='color: green;'>```IgnoreAction = 0```</span>
  IgnoreAction = 0,
  /// C++ enum variant: <span style='color: green;'>```CopyAction = 1```</span>
  CopyAction = 1,
  /// C++ enum variant: <span style='color: green;'>```MoveAction = 2```</span>
  MoveAction = 2,
  /// C++ enum variant: <span style='color: green;'>```LinkAction = 4```</span>
  LinkAction = 4,
  /// C++ enum variant: <span style='color: green;'>```ActionMask = 255```</span>
  ActionMask = 255,
  /// C++ enum variant: <span style='color: green;'>```TargetMoveAction = 32770```</span>
  TargetMoveAction = 32770,
}

impl ::flags::FlaggableEnum for DropAction {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "DropAction"
  }
}

/// C++ type: <span style='color: green;'>```Qt::Edge```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Edge {
  /// C++ enum variant: <span style='color: green;'>```TopEdge = 1```</span>
  Top = 1,
  /// C++ enum variant: <span style='color: green;'>```LeftEdge = 2```</span>
  Left = 2,
  /// C++ enum variant: <span style='color: green;'>```RightEdge = 4```</span>
  Right = 4,
  /// C++ enum variant: <span style='color: green;'>```BottomEdge = 8```</span>
  Bottom = 8,
}

impl ::flags::FlaggableEnum for Edge {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Edge"
  }
}

/// C++ type: <span style='color: green;'>```Qt::EnterKeyType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum EnterKeyType {
  /// C++ enum variant: <span style='color: green;'>```EnterKeyDefault = 0```</span>
  Default = 0,
  /// C++ enum variant: <span style='color: green;'>```EnterKeyReturn = 1```</span>
  Return = 1,
  /// C++ enum variant: <span style='color: green;'>```EnterKeyDone = 2```</span>
  Done = 2,
  /// C++ enum variant: <span style='color: green;'>```EnterKeyGo = 3```</span>
  Go = 3,
  /// C++ enum variant: <span style='color: green;'>```EnterKeySend = 4```</span>
  Send = 4,
  /// C++ enum variant: <span style='color: green;'>```EnterKeySearch = 5```</span>
  Search = 5,
  /// C++ enum variant: <span style='color: green;'>```EnterKeyNext = 6```</span>
  Next = 6,
  /// C++ enum variant: <span style='color: green;'>```EnterKeyPrevious = 7```</span>
  Previous = 7,
}

/// C++ type: <span style='color: green;'>```Qt::EventPriority```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum EventPriority {
  /// C++ enum variant: <span style='color: green;'>```LowEventPriority = -1```</span>
  Low = -1,
  /// C++ enum variant: <span style='color: green;'>```NormalEventPriority = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```HighEventPriority = 1```</span>
  High = 1,
}

/// C++ type: <span style='color: green;'>```Qt::FillRule```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FillRule {
  /// C++ enum variant: <span style='color: green;'>```OddEvenFill = 0```</span>
  OddEven = 0,
  /// C++ enum variant: <span style='color: green;'>```WindingFill = 1```</span>
  Winding = 1,
}

/// C++ type: <span style='color: green;'>```Qt::FindChildOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FindChildOption {
  /// C++ enum variant: <span style='color: green;'>```FindDirectChildrenOnly = 0```</span>
  DirectChildrenOnly = 0,
  /// C++ enum variant: <span style='color: green;'>```FindChildrenRecursively = 1```</span>
  ChildrenRecursively = 1,
}

/// C++ type: <span style='color: green;'>```Qt::FocusPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FocusPolicy {
  /// C++ enum variant: <span style='color: green;'>```NoFocus = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```TabFocus = 1```</span>
  Tab = 1,
  /// C++ enum variant: <span style='color: green;'>```ClickFocus = 2```</span>
  Click = 2,
  /// C++ enum variant: <span style='color: green;'>```StrongFocus = 11```</span>
  Strong = 11,
  /// C++ enum variant: <span style='color: green;'>```WheelFocus = 15```</span>
  Wheel = 15,
}

/// C++ type: <span style='color: green;'>```Qt::FocusReason```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FocusReason {
  /// C++ enum variant: <span style='color: green;'>```MouseFocusReason = 0```</span>
  Mouse = 0,
  /// C++ enum variant: <span style='color: green;'>```TabFocusReason = 1```</span>
  Tab = 1,
  /// C++ enum variant: <span style='color: green;'>```BacktabFocusReason = 2```</span>
  Backtab = 2,
  /// C++ enum variant: <span style='color: green;'>```ActiveWindowFocusReason = 3```</span>
  ActiveWindow = 3,
  /// C++ enum variant: <span style='color: green;'>```PopupFocusReason = 4```</span>
  Popup = 4,
  /// C++ enum variant: <span style='color: green;'>```ShortcutFocusReason = 5```</span>
  Shortcut = 5,
  /// C++ enum variant: <span style='color: green;'>```MenuBarFocusReason = 6```</span>
  MenuBar = 6,
  /// C++ enum variant: <span style='color: green;'>```OtherFocusReason = 7```</span>
  Other = 7,
  /// C++ enum variant: <span style='color: green;'>```NoFocusReason = 8```</span>
  No = 8,
}

/// C++ type: <span style='color: green;'>```Qt::GestureFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GestureFlag {
  /// C++ enum variant: <span style='color: green;'>```DontStartGestureOnChildren = 1```</span>
  DontStartGestureOnChildren = 1,
  /// C++ enum variant: <span style='color: green;'>```ReceivePartialGestures = 2```</span>
  ReceivePartialGestures = 2,
  /// C++ enum variant: <span style='color: green;'>```IgnoredGesturesPropagateToParent = 4```</span>
  IgnoredGesturesPropagateToParent = 4,
}

/// C++ type: <span style='color: green;'>```Qt::GestureState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GestureState {
  /// C++ enum variant: <span style='color: green;'>```NoGesture = 0```</span>
  NoGesture = 0,
  /// C++ enum variant: <span style='color: green;'>```GestureStarted = 1```</span>
  GestureStarted = 1,
  /// C++ enum variant: <span style='color: green;'>```GestureUpdated = 2```</span>
  GestureUpdated = 2,
  /// C++ enum variant: <span style='color: green;'>```GestureFinished = 3```</span>
  GestureFinished = 3,
  /// C++ enum variant: <span style='color: green;'>```GestureCanceled = 4```</span>
  GestureCanceled = 4,
}

/// C++ type: <span style='color: green;'>```Qt::GestureType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GestureType {
  /// C++ enum variant: <span style='color: green;'>```LastGestureType = -1```</span>
  LastGestureType = -1,
  /// C++ enum variant: <span style='color: green;'>```TapGesture = 1```</span>
  TapGesture = 1,
  /// C++ enum variant: <span style='color: green;'>```TapAndHoldGesture = 2```</span>
  TapAndHoldGesture = 2,
  /// C++ enum variant: <span style='color: green;'>```PanGesture = 3```</span>
  PanGesture = 3,
  /// C++ enum variant: <span style='color: green;'>```PinchGesture = 4```</span>
  PinchGesture = 4,
  /// C++ enum variant: <span style='color: green;'>```SwipeGesture = 5```</span>
  SwipeGesture = 5,
  /// C++ enum variant: <span style='color: green;'>```CustomGesture = 256```</span>
  CustomGesture = 256,
}

/// C++ type: <span style='color: green;'>```Qt::GlobalColor```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GlobalColor {
  /// C++ enum variant: <span style='color: green;'>```color0 = 0```</span>
  Color0 = 0,
  /// C++ enum variant: <span style='color: green;'>```color1 = 1```</span>
  Color1 = 1,
  /// C++ enum variant: <span style='color: green;'>```black = 2```</span>
  Black = 2,
  /// C++ enum variant: <span style='color: green;'>```white = 3```</span>
  White = 3,
  /// C++ enum variant: <span style='color: green;'>```darkGray = 4```</span>
  DarkGray = 4,
  /// C++ enum variant: <span style='color: green;'>```gray = 5```</span>
  Gray = 5,
  /// C++ enum variant: <span style='color: green;'>```lightGray = 6```</span>
  LightGray = 6,
  /// C++ enum variant: <span style='color: green;'>```red = 7```</span>
  Red = 7,
  /// C++ enum variant: <span style='color: green;'>```green = 8```</span>
  Green = 8,
  /// C++ enum variant: <span style='color: green;'>```blue = 9```</span>
  Blue = 9,
  /// C++ enum variant: <span style='color: green;'>```cyan = 10```</span>
  Cyan = 10,
  /// C++ enum variant: <span style='color: green;'>```magenta = 11```</span>
  Magenta = 11,
  /// C++ enum variant: <span style='color: green;'>```yellow = 12```</span>
  Yellow = 12,
  /// C++ enum variant: <span style='color: green;'>```darkRed = 13```</span>
  DarkRed = 13,
  /// C++ enum variant: <span style='color: green;'>```darkGreen = 14```</span>
  DarkGreen = 14,
  /// C++ enum variant: <span style='color: green;'>```darkBlue = 15```</span>
  DarkBlue = 15,
  /// C++ enum variant: <span style='color: green;'>```darkCyan = 16```</span>
  DarkCyan = 16,
  /// C++ enum variant: <span style='color: green;'>```darkMagenta = 17```</span>
  DarkMagenta = 17,
  /// C++ enum variant: <span style='color: green;'>```darkYellow = 18```</span>
  DarkYellow = 18,
  /// C++ enum variant: <span style='color: green;'>```transparent = 19```</span>
  Transparent = 19,
}

/// C++ type: <span style='color: green;'>```Qt::HitTestAccuracy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum HitTestAccuracy {
  /// C++ enum variant: <span style='color: green;'>```ExactHit = 0```</span>
  Exact = 0,
  /// C++ enum variant: <span style='color: green;'>```FuzzyHit = 1```</span>
  Fuzzy = 1,
}

/// C++ type: <span style='color: green;'>```Qt::ImageConversionFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ImageConversionFlag {
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```AutoColor = 0```</span>
  /// - <span style='color: green;'>```ThresholdAlphaDither = 0```</span>
  /// - <span style='color: green;'>```DiffuseDither = 0```</span>
  /// - <span style='color: green;'>```AutoDither = 0```</span>
  ///
  AutoColor = 0,
  /// C++ enum variant: <span style='color: green;'>```MonoOnly = 2```</span>
  MonoOnly = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ColorMode_Mask = 3```</span>
  /// - <span style='color: green;'>```ColorOnly = 3```</span>
  ///
  ColorModeMask = 3,
  /// C++ enum variant: <span style='color: green;'>```OrderedAlphaDither = 4```</span>
  OrderedAlphaDither = 4,
  /// C++ enum variant: <span style='color: green;'>```DiffuseAlphaDither = 8```</span>
  DiffuseAlphaDither = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```AlphaDither_Mask = 12```</span>
  /// - <span style='color: green;'>```NoAlpha = 12```</span>
  ///
  AlphaDitherMask = 12,
  /// C++ enum variant: <span style='color: green;'>```OrderedDither = 16```</span>
  OrderedDither = 16,
  /// C++ enum variant: <span style='color: green;'>```ThresholdDither = 32```</span>
  ThresholdDither = 32,
  /// C++ enum variant: <span style='color: green;'>```Dither_Mask = 48```</span>
  DitherMask = 48,
  /// C++ enum variant: <span style='color: green;'>```PreferDither = 64```</span>
  PreferDither = 64,
  /// C++ enum variant: <span style='color: green;'>```AvoidDither = 128```</span>
  AvoidDither = 128,
  /// C++ enum variant: <span style='color: green;'>```DitherMode_Mask = 192```</span>
  DitherModeMask = 192,
  /// C++ enum variant: <span style='color: green;'>```NoOpaqueDetection = 256```</span>
  NoOpaqueDetection = 256,
  /// C++ enum variant: <span style='color: green;'>```NoFormatConversion = 512```</span>
  NoFormatConversion = 512,
}

impl ::flags::FlaggableEnum for ImageConversionFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ImageConversionFlag"
  }
}

/// C++ type: <span style='color: green;'>```Qt::InputMethodHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputMethodHint {
  /// C++ enum variant: <span style='color: green;'>```ImhExclusiveInputMask = -65536```</span>
  ExclusiveInputMask = -65536,
  /// C++ enum variant: <span style='color: green;'>```ImhNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```ImhHiddenText = 1```</span>
  HiddenText = 1,
  /// C++ enum variant: <span style='color: green;'>```ImhSensitiveData = 2```</span>
  SensitiveData = 2,
  /// C++ enum variant: <span style='color: green;'>```ImhNoAutoUppercase = 4```</span>
  NoAutoUppercase = 4,
  /// C++ enum variant: <span style='color: green;'>```ImhPreferNumbers = 8```</span>
  PreferNumbers = 8,
  /// C++ enum variant: <span style='color: green;'>```ImhPreferUppercase = 16```</span>
  PreferUppercase = 16,
  /// C++ enum variant: <span style='color: green;'>```ImhPreferLowercase = 32```</span>
  PreferLowercase = 32,
  /// C++ enum variant: <span style='color: green;'>```ImhNoPredictiveText = 64```</span>
  NoPredictiveText = 64,
  /// C++ enum variant: <span style='color: green;'>```ImhDate = 128```</span>
  Date = 128,
  /// C++ enum variant: <span style='color: green;'>```ImhTime = 256```</span>
  Time = 256,
  /// C++ enum variant: <span style='color: green;'>```ImhPreferLatin = 512```</span>
  PreferLatin = 512,
  /// C++ enum variant: <span style='color: green;'>```ImhMultiLine = 1024```</span>
  MultiLine = 1024,
  /// C++ enum variant: <span style='color: green;'>```ImhDigitsOnly = 65536```</span>
  DigitsOnly = 65536,
  /// C++ enum variant: <span style='color: green;'>```ImhFormattedNumbersOnly = 131072```</span>
  FormattedNumbersOnly = 131072,
  /// C++ enum variant: <span style='color: green;'>```ImhUppercaseOnly = 262144```</span>
  UppercaseOnly = 262144,
  /// C++ enum variant: <span style='color: green;'>```ImhLowercaseOnly = 524288```</span>
  LowercaseOnly = 524288,
  /// C++ enum variant: <span style='color: green;'>```ImhDialableCharactersOnly = 1048576```</span>
  DialableCharactersOnly = 1048576,
  /// C++ enum variant: <span style='color: green;'>```ImhEmailCharactersOnly = 2097152```</span>
  EmailCharactersOnly = 2097152,
  /// C++ enum variant: <span style='color: green;'>```ImhUrlCharactersOnly = 4194304```</span>
  UrlCharactersOnly = 4194304,
  /// C++ enum variant: <span style='color: green;'>```ImhLatinOnly = 8388608```</span>
  LatinOnly = 8388608,
}

impl ::flags::FlaggableEnum for InputMethodHint {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "InputMethodHint"
  }
}

/// C++ type: <span style='color: green;'>```Qt::InputMethodQuery```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputMethodQuery {
  /// C++ enum variant: <span style='color: green;'>```ImPlatformData = -2147483648```</span>
  PlatformData = -2147483648,
  /// C++ enum variant: <span style='color: green;'>```ImQueryAll = -1```</span>
  QueryAll = -1,
  /// C++ enum variant: <span style='color: green;'>```ImEnabled = 1```</span>
  Enabled = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ImCursorRectangle = 2```</span>
  /// - <span style='color: green;'>```ImMicroFocus = 2```</span>
  ///
  CursorRectangle = 2,
  /// C++ enum variant: <span style='color: green;'>```ImFont = 4```</span>
  Font = 4,
  /// C++ enum variant: <span style='color: green;'>```ImCursorPosition = 8```</span>
  CursorPosition = 8,
  /// C++ enum variant: <span style='color: green;'>```ImSurroundingText = 16```</span>
  SurroundingText = 16,
  /// C++ enum variant: <span style='color: green;'>```ImCurrentSelection = 32```</span>
  CurrentSelection = 32,
  /// C++ enum variant: <span style='color: green;'>```ImMaximumTextLength = 64```</span>
  MaximumTextLength = 64,
  /// C++ enum variant: <span style='color: green;'>```ImAnchorPosition = 128```</span>
  AnchorPosition = 128,
  /// C++ enum variant: <span style='color: green;'>```ImHints = 256```</span>
  Hints = 256,
  /// C++ enum variant: <span style='color: green;'>```ImPreferredLanguage = 512```</span>
  PreferredLanguage = 512,
  /// C++ enum variant: <span style='color: green;'>```ImAbsolutePosition = 1024```</span>
  AbsolutePosition = 1024,
  /// C++ enum variant: <span style='color: green;'>```ImTextBeforeCursor = 2048```</span>
  TextBeforeCursor = 2048,
  /// C++ enum variant: <span style='color: green;'>```ImTextAfterCursor = 4096```</span>
  TextAfterCursor = 4096,
  /// C++ enum variant: <span style='color: green;'>```ImEnterKeyType = 8192```</span>
  EnterKeyType = 8192,
  /// C++ enum variant: <span style='color: green;'>```ImAnchorRectangle = 16384```</span>
  AnchorRectangle = 16384,
  /// C++ enum variant: <span style='color: green;'>```ImQueryInput = 16570```</span>
  QueryInput = 16570,
  /// C++ enum variant: <span style='color: green;'>```ImInputItemClipRectangle = 32768```</span>
  InputItemClipRectangle = 32768,
}

impl ::flags::FlaggableEnum for InputMethodQuery {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "InputMethodQuery"
  }
}

/// C++ type: <span style='color: green;'>```Qt::ItemDataRole```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemDataRole {
  /// C++ enum variant: <span style='color: green;'>```DisplayRole = 0```</span>
  Display = 0,
  /// C++ enum variant: <span style='color: green;'>```DecorationRole = 1```</span>
  Decoration = 1,
  /// C++ enum variant: <span style='color: green;'>```EditRole = 2```</span>
  Edit = 2,
  /// C++ enum variant: <span style='color: green;'>```ToolTipRole = 3```</span>
  ToolTip = 3,
  /// C++ enum variant: <span style='color: green;'>```StatusTipRole = 4```</span>
  StatusTip = 4,
  /// C++ enum variant: <span style='color: green;'>```WhatsThisRole = 5```</span>
  WhatsThis = 5,
  /// C++ enum variant: <span style='color: green;'>```FontRole = 6```</span>
  Font = 6,
  /// C++ enum variant: <span style='color: green;'>```TextAlignmentRole = 7```</span>
  TextAlignment = 7,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```BackgroundColorRole = 8```</span>
  /// - <span style='color: green;'>```BackgroundRole = 8```</span>
  ///
  BackgroundColor = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```TextColorRole = 9```</span>
  /// - <span style='color: green;'>```ForegroundRole = 9```</span>
  ///
  TextColor = 9,
  /// C++ enum variant: <span style='color: green;'>```CheckStateRole = 10```</span>
  CheckState = 10,
  /// C++ enum variant: <span style='color: green;'>```AccessibleTextRole = 11```</span>
  AccessibleText = 11,
  /// C++ enum variant: <span style='color: green;'>```AccessibleDescriptionRole = 12```</span>
  AccessibleDescription = 12,
  /// C++ enum variant: <span style='color: green;'>```SizeHintRole = 13```</span>
  SizeHint = 13,
  /// C++ enum variant: <span style='color: green;'>```InitialSortOrderRole = 14```</span>
  InitialSortOrder = 14,
  /// C++ enum variant: <span style='color: green;'>```DisplayPropertyRole = 27```</span>
  DisplayProperty = 27,
  /// C++ enum variant: <span style='color: green;'>```DecorationPropertyRole = 28```</span>
  DecorationProperty = 28,
  /// C++ enum variant: <span style='color: green;'>```ToolTipPropertyRole = 29```</span>
  ToolTipProperty = 29,
  /// C++ enum variant: <span style='color: green;'>```StatusTipPropertyRole = 30```</span>
  StatusTipProperty = 30,
  /// C++ enum variant: <span style='color: green;'>```WhatsThisPropertyRole = 31```</span>
  WhatsThisProperty = 31,
  /// C++ enum variant: <span style='color: green;'>```UserRole = 256```</span>
  User = 256,
}

/// C++ type: <span style='color: green;'>```Qt::ItemFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemFlag {
  /// C++ enum variant: <span style='color: green;'>```NoItemFlags = 0```</span>
  NoItemFlags = 0,
  /// C++ enum variant: <span style='color: green;'>```ItemIsSelectable = 1```</span>
  ItemIsSelectable = 1,
  /// C++ enum variant: <span style='color: green;'>```ItemIsEditable = 2```</span>
  ItemIsEditable = 2,
  /// C++ enum variant: <span style='color: green;'>```ItemIsDragEnabled = 4```</span>
  ItemIsDragEnabled = 4,
  /// C++ enum variant: <span style='color: green;'>```ItemIsDropEnabled = 8```</span>
  ItemIsDropEnabled = 8,
  /// C++ enum variant: <span style='color: green;'>```ItemIsUserCheckable = 16```</span>
  ItemIsUserCheckable = 16,
  /// C++ enum variant: <span style='color: green;'>```ItemIsEnabled = 32```</span>
  ItemIsEnabled = 32,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ItemIsAutoTristate = 64```</span>
  /// - <span style='color: green;'>```ItemIsTristate = 64```</span>
  ///
  ItemIsAutoTristate = 64,
  /// C++ enum variant: <span style='color: green;'>```ItemNeverHasChildren = 128```</span>
  ItemNeverHasChildren = 128,
  /// C++ enum variant: <span style='color: green;'>```ItemIsUserTristate = 256```</span>
  ItemIsUserTristate = 256,
}

impl ::flags::FlaggableEnum for ItemFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ItemFlag"
  }
}

/// C++ type: <span style='color: green;'>```Qt::ItemSelectionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemSelectionMode {
  /// C++ enum variant: <span style='color: green;'>```ContainsItemShape = 0```</span>
  ContainsItemShape = 0,
  /// C++ enum variant: <span style='color: green;'>```IntersectsItemShape = 1```</span>
  IntersectsItemShape = 1,
  /// C++ enum variant: <span style='color: green;'>```ContainsItemBoundingRect = 2```</span>
  ContainsItemBoundingRect = 2,
  /// C++ enum variant: <span style='color: green;'>```IntersectsItemBoundingRect = 3```</span>
  IntersectsItemBoundingRect = 3,
}

/// C++ type: <span style='color: green;'>```Qt::ItemSelectionOperation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemSelectionOperation {
  /// C++ enum variant: <span style='color: green;'>```ReplaceSelection = 0```</span>
  Replace = 0,
  /// C++ enum variant: <span style='color: green;'>```AddToSelection = 1```</span>
  AddTo = 1,
}

/// C++ type: <span style='color: green;'>```Qt::Key```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Key {
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Key_Space = 32```</span>
  /// - <span style='color: green;'>```Key_Any = 32```</span>
  ///
  KeySpace = 32,
  /// C++ enum variant: <span style='color: green;'>```Key_Exclam = 33```</span>
  KeyExclam = 33,
  /// C++ enum variant: <span style='color: green;'>```Key_QuoteDbl = 34```</span>
  KeyQuoteDbl = 34,
  /// C++ enum variant: <span style='color: green;'>```Key_NumberSign = 35```</span>
  KeyNumberSign = 35,
  /// C++ enum variant: <span style='color: green;'>```Key_Dollar = 36```</span>
  KeyDollar = 36,
  /// C++ enum variant: <span style='color: green;'>```Key_Percent = 37```</span>
  KeyPercent = 37,
  /// C++ enum variant: <span style='color: green;'>```Key_Ampersand = 38```</span>
  KeyAmpersand = 38,
  /// C++ enum variant: <span style='color: green;'>```Key_Apostrophe = 39```</span>
  KeyApostrophe = 39,
  /// C++ enum variant: <span style='color: green;'>```Key_ParenLeft = 40```</span>
  KeyParenLeft = 40,
  /// C++ enum variant: <span style='color: green;'>```Key_ParenRight = 41```</span>
  KeyParenRight = 41,
  /// C++ enum variant: <span style='color: green;'>```Key_Asterisk = 42```</span>
  KeyAsterisk = 42,
  /// C++ enum variant: <span style='color: green;'>```Key_Plus = 43```</span>
  KeyPlus = 43,
  /// C++ enum variant: <span style='color: green;'>```Key_Comma = 44```</span>
  KeyComma = 44,
  /// C++ enum variant: <span style='color: green;'>```Key_Minus = 45```</span>
  KeyMinus = 45,
  /// C++ enum variant: <span style='color: green;'>```Key_Period = 46```</span>
  KeyPeriod = 46,
  /// C++ enum variant: <span style='color: green;'>```Key_Slash = 47```</span>
  KeySlash = 47,
  /// C++ enum variant: <span style='color: green;'>```Key_0 = 48```</span>
  Key0 = 48,
  /// C++ enum variant: <span style='color: green;'>```Key_1 = 49```</span>
  Key1 = 49,
  /// C++ enum variant: <span style='color: green;'>```Key_2 = 50```</span>
  Key2 = 50,
  /// C++ enum variant: <span style='color: green;'>```Key_3 = 51```</span>
  Key3 = 51,
  /// C++ enum variant: <span style='color: green;'>```Key_4 = 52```</span>
  Key4 = 52,
  /// C++ enum variant: <span style='color: green;'>```Key_5 = 53```</span>
  Key5 = 53,
  /// C++ enum variant: <span style='color: green;'>```Key_6 = 54```</span>
  Key6 = 54,
  /// C++ enum variant: <span style='color: green;'>```Key_7 = 55```</span>
  Key7 = 55,
  /// C++ enum variant: <span style='color: green;'>```Key_8 = 56```</span>
  Key8 = 56,
  /// C++ enum variant: <span style='color: green;'>```Key_9 = 57```</span>
  Key9 = 57,
  /// C++ enum variant: <span style='color: green;'>```Key_Colon = 58```</span>
  KeyColon = 58,
  /// C++ enum variant: <span style='color: green;'>```Key_Semicolon = 59```</span>
  KeySemicolon = 59,
  /// C++ enum variant: <span style='color: green;'>```Key_Less = 60```</span>
  KeyLess = 60,
  /// C++ enum variant: <span style='color: green;'>```Key_Equal = 61```</span>
  KeyEqual = 61,
  /// C++ enum variant: <span style='color: green;'>```Key_Greater = 62```</span>
  KeyGreater = 62,
  /// C++ enum variant: <span style='color: green;'>```Key_Question = 63```</span>
  KeyQuestion = 63,
  /// C++ enum variant: <span style='color: green;'>```Key_At = 64```</span>
  KeyAt = 64,
  /// C++ enum variant: <span style='color: green;'>```Key_A = 65```</span>
  KeyA = 65,
  /// C++ enum variant: <span style='color: green;'>```Key_B = 66```</span>
  KeyB = 66,
  /// C++ enum variant: <span style='color: green;'>```Key_C = 67```</span>
  KeyC = 67,
  /// C++ enum variant: <span style='color: green;'>```Key_D = 68```</span>
  KeyD = 68,
  /// C++ enum variant: <span style='color: green;'>```Key_E = 69```</span>
  KeyE = 69,
  /// C++ enum variant: <span style='color: green;'>```Key_F = 70```</span>
  KeyF = 70,
  /// C++ enum variant: <span style='color: green;'>```Key_G = 71```</span>
  KeyG = 71,
  /// C++ enum variant: <span style='color: green;'>```Key_H = 72```</span>
  KeyH = 72,
  /// C++ enum variant: <span style='color: green;'>```Key_I = 73```</span>
  KeyI = 73,
  /// C++ enum variant: <span style='color: green;'>```Key_J = 74```</span>
  KeyJ = 74,
  /// C++ enum variant: <span style='color: green;'>```Key_K = 75```</span>
  KeyK = 75,
  /// C++ enum variant: <span style='color: green;'>```Key_L = 76```</span>
  KeyL = 76,
  /// C++ enum variant: <span style='color: green;'>```Key_M = 77```</span>
  KeyM = 77,
  /// C++ enum variant: <span style='color: green;'>```Key_N = 78```</span>
  KeyN = 78,
  /// C++ enum variant: <span style='color: green;'>```Key_O = 79```</span>
  KeyO = 79,
  /// C++ enum variant: <span style='color: green;'>```Key_P = 80```</span>
  KeyP = 80,
  /// C++ enum variant: <span style='color: green;'>```Key_Q = 81```</span>
  KeyQ = 81,
  /// C++ enum variant: <span style='color: green;'>```Key_R = 82```</span>
  KeyR = 82,
  /// C++ enum variant: <span style='color: green;'>```Key_S = 83```</span>
  KeyS = 83,
  /// C++ enum variant: <span style='color: green;'>```Key_T = 84```</span>
  KeyT = 84,
  /// C++ enum variant: <span style='color: green;'>```Key_U = 85```</span>
  KeyU = 85,
  /// C++ enum variant: <span style='color: green;'>```Key_V = 86```</span>
  KeyV = 86,
  /// C++ enum variant: <span style='color: green;'>```Key_W = 87```</span>
  KeyW = 87,
  /// C++ enum variant: <span style='color: green;'>```Key_X = 88```</span>
  KeyX = 88,
  /// C++ enum variant: <span style='color: green;'>```Key_Y = 89```</span>
  KeyY = 89,
  /// C++ enum variant: <span style='color: green;'>```Key_Z = 90```</span>
  KeyZ = 90,
  /// C++ enum variant: <span style='color: green;'>```Key_BracketLeft = 91```</span>
  KeyBracketLeft = 91,
  /// C++ enum variant: <span style='color: green;'>```Key_Backslash = 92```</span>
  KeyBackslash = 92,
  /// C++ enum variant: <span style='color: green;'>```Key_BracketRight = 93```</span>
  KeyBracketRight = 93,
  /// C++ enum variant: <span style='color: green;'>```Key_AsciiCircum = 94```</span>
  KeyAsciiCircum = 94,
  /// C++ enum variant: <span style='color: green;'>```Key_Underscore = 95```</span>
  KeyUnderscore = 95,
  /// C++ enum variant: <span style='color: green;'>```Key_QuoteLeft = 96```</span>
  KeyQuoteLeft = 96,
  /// C++ enum variant: <span style='color: green;'>```Key_BraceLeft = 123```</span>
  KeyBraceLeft = 123,
  /// C++ enum variant: <span style='color: green;'>```Key_Bar = 124```</span>
  KeyBar = 124,
  /// C++ enum variant: <span style='color: green;'>```Key_BraceRight = 125```</span>
  KeyBraceRight = 125,
  /// C++ enum variant: <span style='color: green;'>```Key_AsciiTilde = 126```</span>
  KeyAsciiTilde = 126,
  /// C++ enum variant: <span style='color: green;'>```Key_nobreakspace = 160```</span>
  KeyNobreakspace = 160,
  /// C++ enum variant: <span style='color: green;'>```Key_exclamdown = 161```</span>
  KeyExclamdown = 161,
  /// C++ enum variant: <span style='color: green;'>```Key_cent = 162```</span>
  KeyCent = 162,
  /// C++ enum variant: <span style='color: green;'>```Key_sterling = 163```</span>
  KeySterling = 163,
  /// C++ enum variant: <span style='color: green;'>```Key_currency = 164```</span>
  KeyCurrency = 164,
  /// C++ enum variant: <span style='color: green;'>```Key_yen = 165```</span>
  KeyYen = 165,
  /// C++ enum variant: <span style='color: green;'>```Key_brokenbar = 166```</span>
  KeyBrokenbar = 166,
  /// C++ enum variant: <span style='color: green;'>```Key_section = 167```</span>
  KeySection = 167,
  /// C++ enum variant: <span style='color: green;'>```Key_diaeresis = 168```</span>
  KeyDiaeresis = 168,
  /// C++ enum variant: <span style='color: green;'>```Key_copyright = 169```</span>
  KeyCopyright = 169,
  /// C++ enum variant: <span style='color: green;'>```Key_ordfeminine = 170```</span>
  KeyOrdfeminine = 170,
  /// C++ enum variant: <span style='color: green;'>```Key_guillemotleft = 171```</span>
  KeyGuillemotleft = 171,
  /// C++ enum variant: <span style='color: green;'>```Key_notsign = 172```</span>
  KeyNotsign = 172,
  /// C++ enum variant: <span style='color: green;'>```Key_hyphen = 173```</span>
  KeyHyphen = 173,
  /// C++ enum variant: <span style='color: green;'>```Key_registered = 174```</span>
  KeyRegistered = 174,
  /// C++ enum variant: <span style='color: green;'>```Key_macron = 175```</span>
  KeyMacron = 175,
  /// C++ enum variant: <span style='color: green;'>```Key_degree = 176```</span>
  KeyDegree = 176,
  /// C++ enum variant: <span style='color: green;'>```Key_plusminus = 177```</span>
  KeyPlusminus = 177,
  /// C++ enum variant: <span style='color: green;'>```Key_twosuperior = 178```</span>
  KeyTwosuperior = 178,
  /// C++ enum variant: <span style='color: green;'>```Key_threesuperior = 179```</span>
  KeyThreesuperior = 179,
  /// C++ enum variant: <span style='color: green;'>```Key_acute = 180```</span>
  KeyAcute = 180,
  /// C++ enum variant: <span style='color: green;'>```Key_mu = 181```</span>
  KeyMu = 181,
  /// C++ enum variant: <span style='color: green;'>```Key_paragraph = 182```</span>
  KeyParagraph = 182,
  /// C++ enum variant: <span style='color: green;'>```Key_periodcentered = 183```</span>
  KeyPeriodcentered = 183,
  /// C++ enum variant: <span style='color: green;'>```Key_cedilla = 184```</span>
  KeyCedilla = 184,
  /// C++ enum variant: <span style='color: green;'>```Key_onesuperior = 185```</span>
  KeyOnesuperior = 185,
  /// C++ enum variant: <span style='color: green;'>```Key_masculine = 186```</span>
  KeyMasculine = 186,
  /// C++ enum variant: <span style='color: green;'>```Key_guillemotright = 187```</span>
  KeyGuillemotright = 187,
  /// C++ enum variant: <span style='color: green;'>```Key_onequarter = 188```</span>
  KeyOnequarter = 188,
  /// C++ enum variant: <span style='color: green;'>```Key_onehalf = 189```</span>
  KeyOnehalf = 189,
  /// C++ enum variant: <span style='color: green;'>```Key_threequarters = 190```</span>
  KeyThreequarters = 190,
  /// C++ enum variant: <span style='color: green;'>```Key_questiondown = 191```</span>
  KeyQuestiondown = 191,
  /// C++ enum variant: <span style='color: green;'>```Key_Agrave = 192```</span>
  KeyAgrave = 192,
  /// C++ enum variant: <span style='color: green;'>```Key_Aacute = 193```</span>
  KeyAacute = 193,
  /// C++ enum variant: <span style='color: green;'>```Key_Acircumflex = 194```</span>
  KeyAcircumflex = 194,
  /// C++ enum variant: <span style='color: green;'>```Key_Atilde = 195```</span>
  KeyAtilde = 195,
  /// C++ enum variant: <span style='color: green;'>```Key_Adiaeresis = 196```</span>
  KeyAdiaeresis = 196,
  /// C++ enum variant: <span style='color: green;'>```Key_Aring = 197```</span>
  KeyAring = 197,
  /// C++ enum variant: <span style='color: green;'>```Key_AE = 198```</span>
  KeyAE = 198,
  /// C++ enum variant: <span style='color: green;'>```Key_Ccedilla = 199```</span>
  KeyCcedilla = 199,
  /// C++ enum variant: <span style='color: green;'>```Key_Egrave = 200```</span>
  KeyEgrave = 200,
  /// C++ enum variant: <span style='color: green;'>```Key_Eacute = 201```</span>
  KeyEacute = 201,
  /// C++ enum variant: <span style='color: green;'>```Key_Ecircumflex = 202```</span>
  KeyEcircumflex = 202,
  /// C++ enum variant: <span style='color: green;'>```Key_Ediaeresis = 203```</span>
  KeyEdiaeresis = 203,
  /// C++ enum variant: <span style='color: green;'>```Key_Igrave = 204```</span>
  KeyIgrave = 204,
  /// C++ enum variant: <span style='color: green;'>```Key_Iacute = 205```</span>
  KeyIacute = 205,
  /// C++ enum variant: <span style='color: green;'>```Key_Icircumflex = 206```</span>
  KeyIcircumflex = 206,
  /// C++ enum variant: <span style='color: green;'>```Key_Idiaeresis = 207```</span>
  KeyIdiaeresis = 207,
  /// C++ enum variant: <span style='color: green;'>```Key_ETH = 208```</span>
  KeyETH = 208,
  /// C++ enum variant: <span style='color: green;'>```Key_Ntilde = 209```</span>
  KeyNtilde = 209,
  /// C++ enum variant: <span style='color: green;'>```Key_Ograve = 210```</span>
  KeyOgrave = 210,
  /// C++ enum variant: <span style='color: green;'>```Key_Oacute = 211```</span>
  KeyOacute = 211,
  /// C++ enum variant: <span style='color: green;'>```Key_Ocircumflex = 212```</span>
  KeyOcircumflex = 212,
  /// C++ enum variant: <span style='color: green;'>```Key_Otilde = 213```</span>
  KeyOtilde = 213,
  /// C++ enum variant: <span style='color: green;'>```Key_Odiaeresis = 214```</span>
  KeyOdiaeresis = 214,
  /// C++ enum variant: <span style='color: green;'>```Key_multiply = 215```</span>
  KeyMultiply = 215,
  /// C++ enum variant: <span style='color: green;'>```Key_Ooblique = 216```</span>
  KeyOoblique = 216,
  /// C++ enum variant: <span style='color: green;'>```Key_Ugrave = 217```</span>
  KeyUgrave = 217,
  /// C++ enum variant: <span style='color: green;'>```Key_Uacute = 218```</span>
  KeyUacute = 218,
  /// C++ enum variant: <span style='color: green;'>```Key_Ucircumflex = 219```</span>
  KeyUcircumflex = 219,
  /// C++ enum variant: <span style='color: green;'>```Key_Udiaeresis = 220```</span>
  KeyUdiaeresis = 220,
  /// C++ enum variant: <span style='color: green;'>```Key_Yacute = 221```</span>
  KeyYacute = 221,
  /// C++ enum variant: <span style='color: green;'>```Key_THORN = 222```</span>
  KeyTHORN = 222,
  /// C++ enum variant: <span style='color: green;'>```Key_ssharp = 223```</span>
  KeySsharp = 223,
  /// C++ enum variant: <span style='color: green;'>```Key_division = 247```</span>
  KeyDivision = 247,
  /// C++ enum variant: <span style='color: green;'>```Key_ydiaeresis = 255```</span>
  KeyYdiaeresis = 255,
  /// C++ enum variant: <span style='color: green;'>```Key_Escape = 16777216```</span>
  KeyEscape = 16777216,
  /// C++ enum variant: <span style='color: green;'>```Key_Tab = 16777217```</span>
  KeyTab = 16777217,
  /// C++ enum variant: <span style='color: green;'>```Key_Backtab = 16777218```</span>
  KeyBacktab = 16777218,
  /// C++ enum variant: <span style='color: green;'>```Key_Backspace = 16777219```</span>
  KeyBackspace = 16777219,
  /// C++ enum variant: <span style='color: green;'>```Key_Return = 16777220```</span>
  KeyReturn = 16777220,
  /// C++ enum variant: <span style='color: green;'>```Key_Enter = 16777221```</span>
  KeyEnter = 16777221,
  /// C++ enum variant: <span style='color: green;'>```Key_Insert = 16777222```</span>
  KeyInsert = 16777222,
  /// C++ enum variant: <span style='color: green;'>```Key_Delete = 16777223```</span>
  KeyDelete = 16777223,
  /// C++ enum variant: <span style='color: green;'>```Key_Pause = 16777224```</span>
  KeyPause = 16777224,
  /// C++ enum variant: <span style='color: green;'>```Key_Print = 16777225```</span>
  KeyPrint = 16777225,
  /// C++ enum variant: <span style='color: green;'>```Key_SysReq = 16777226```</span>
  KeySysReq = 16777226,
  /// C++ enum variant: <span style='color: green;'>```Key_Clear = 16777227```</span>
  KeyClear = 16777227,
  /// C++ enum variant: <span style='color: green;'>```Key_Home = 16777232```</span>
  KeyHome = 16777232,
  /// C++ enum variant: <span style='color: green;'>```Key_End = 16777233```</span>
  KeyEnd = 16777233,
  /// C++ enum variant: <span style='color: green;'>```Key_Left = 16777234```</span>
  KeyLeft = 16777234,
  /// C++ enum variant: <span style='color: green;'>```Key_Up = 16777235```</span>
  KeyUp = 16777235,
  /// C++ enum variant: <span style='color: green;'>```Key_Right = 16777236```</span>
  KeyRight = 16777236,
  /// C++ enum variant: <span style='color: green;'>```Key_Down = 16777237```</span>
  KeyDown = 16777237,
  /// C++ enum variant: <span style='color: green;'>```Key_PageUp = 16777238```</span>
  KeyPageUp = 16777238,
  /// C++ enum variant: <span style='color: green;'>```Key_PageDown = 16777239```</span>
  KeyPageDown = 16777239,
  /// C++ enum variant: <span style='color: green;'>```Key_Shift = 16777248```</span>
  KeyShift = 16777248,
  /// C++ enum variant: <span style='color: green;'>```Key_Control = 16777249```</span>
  KeyControl = 16777249,
  /// C++ enum variant: <span style='color: green;'>```Key_Meta = 16777250```</span>
  KeyMeta = 16777250,
  /// C++ enum variant: <span style='color: green;'>```Key_Alt = 16777251```</span>
  KeyAlt = 16777251,
  /// C++ enum variant: <span style='color: green;'>```Key_CapsLock = 16777252```</span>
  KeyCapsLock = 16777252,
  /// C++ enum variant: <span style='color: green;'>```Key_NumLock = 16777253```</span>
  KeyNumLock = 16777253,
  /// C++ enum variant: <span style='color: green;'>```Key_ScrollLock = 16777254```</span>
  KeyScrollLock = 16777254,
  /// C++ enum variant: <span style='color: green;'>```Key_F1 = 16777264```</span>
  KeyF1 = 16777264,
  /// C++ enum variant: <span style='color: green;'>```Key_F2 = 16777265```</span>
  KeyF2 = 16777265,
  /// C++ enum variant: <span style='color: green;'>```Key_F3 = 16777266```</span>
  KeyF3 = 16777266,
  /// C++ enum variant: <span style='color: green;'>```Key_F4 = 16777267```</span>
  KeyF4 = 16777267,
  /// C++ enum variant: <span style='color: green;'>```Key_F5 = 16777268```</span>
  KeyF5 = 16777268,
  /// C++ enum variant: <span style='color: green;'>```Key_F6 = 16777269```</span>
  KeyF6 = 16777269,
  /// C++ enum variant: <span style='color: green;'>```Key_F7 = 16777270```</span>
  KeyF7 = 16777270,
  /// C++ enum variant: <span style='color: green;'>```Key_F8 = 16777271```</span>
  KeyF8 = 16777271,
  /// C++ enum variant: <span style='color: green;'>```Key_F9 = 16777272```</span>
  KeyF9 = 16777272,
  /// C++ enum variant: <span style='color: green;'>```Key_F10 = 16777273```</span>
  KeyF10 = 16777273,
  /// C++ enum variant: <span style='color: green;'>```Key_F11 = 16777274```</span>
  KeyF11 = 16777274,
  /// C++ enum variant: <span style='color: green;'>```Key_F12 = 16777275```</span>
  KeyF12 = 16777275,
  /// C++ enum variant: <span style='color: green;'>```Key_F13 = 16777276```</span>
  KeyF13 = 16777276,
  /// C++ enum variant: <span style='color: green;'>```Key_F14 = 16777277```</span>
  KeyF14 = 16777277,
  /// C++ enum variant: <span style='color: green;'>```Key_F15 = 16777278```</span>
  KeyF15 = 16777278,
  /// C++ enum variant: <span style='color: green;'>```Key_F16 = 16777279```</span>
  KeyF16 = 16777279,
  /// C++ enum variant: <span style='color: green;'>```Key_F17 = 16777280```</span>
  KeyF17 = 16777280,
  /// C++ enum variant: <span style='color: green;'>```Key_F18 = 16777281```</span>
  KeyF18 = 16777281,
  /// C++ enum variant: <span style='color: green;'>```Key_F19 = 16777282```</span>
  KeyF19 = 16777282,
  /// C++ enum variant: <span style='color: green;'>```Key_F20 = 16777283```</span>
  KeyF20 = 16777283,
  /// C++ enum variant: <span style='color: green;'>```Key_F21 = 16777284```</span>
  KeyF21 = 16777284,
  /// C++ enum variant: <span style='color: green;'>```Key_F22 = 16777285```</span>
  KeyF22 = 16777285,
  /// C++ enum variant: <span style='color: green;'>```Key_F23 = 16777286```</span>
  KeyF23 = 16777286,
  /// C++ enum variant: <span style='color: green;'>```Key_F24 = 16777287```</span>
  KeyF24 = 16777287,
  /// C++ enum variant: <span style='color: green;'>```Key_F25 = 16777288```</span>
  KeyF25 = 16777288,
  /// C++ enum variant: <span style='color: green;'>```Key_F26 = 16777289```</span>
  KeyF26 = 16777289,
  /// C++ enum variant: <span style='color: green;'>```Key_F27 = 16777290```</span>
  KeyF27 = 16777290,
  /// C++ enum variant: <span style='color: green;'>```Key_F28 = 16777291```</span>
  KeyF28 = 16777291,
  /// C++ enum variant: <span style='color: green;'>```Key_F29 = 16777292```</span>
  KeyF29 = 16777292,
  /// C++ enum variant: <span style='color: green;'>```Key_F30 = 16777293```</span>
  KeyF30 = 16777293,
  /// C++ enum variant: <span style='color: green;'>```Key_F31 = 16777294```</span>
  KeyF31 = 16777294,
  /// C++ enum variant: <span style='color: green;'>```Key_F32 = 16777295```</span>
  KeyF32 = 16777295,
  /// C++ enum variant: <span style='color: green;'>```Key_F33 = 16777296```</span>
  KeyF33 = 16777296,
  /// C++ enum variant: <span style='color: green;'>```Key_F34 = 16777297```</span>
  KeyF34 = 16777297,
  /// C++ enum variant: <span style='color: green;'>```Key_F35 = 16777298```</span>
  KeyF35 = 16777298,
  /// C++ enum variant: <span style='color: green;'>```Key_Super_L = 16777299```</span>
  KeySuperL = 16777299,
  /// C++ enum variant: <span style='color: green;'>```Key_Super_R = 16777300```</span>
  KeySuperR = 16777300,
  /// C++ enum variant: <span style='color: green;'>```Key_Menu = 16777301```</span>
  KeyMenu = 16777301,
  /// C++ enum variant: <span style='color: green;'>```Key_Hyper_L = 16777302```</span>
  KeyHyperL = 16777302,
  /// C++ enum variant: <span style='color: green;'>```Key_Hyper_R = 16777303```</span>
  KeyHyperR = 16777303,
  /// C++ enum variant: <span style='color: green;'>```Key_Help = 16777304```</span>
  KeyHelp = 16777304,
  /// C++ enum variant: <span style='color: green;'>```Key_Direction_L = 16777305```</span>
  KeyDirectionL = 16777305,
  /// C++ enum variant: <span style='color: green;'>```Key_Direction_R = 16777312```</span>
  KeyDirectionR = 16777312,
  /// C++ enum variant: <span style='color: green;'>```Key_Back = 16777313```</span>
  KeyBack = 16777313,
  /// C++ enum variant: <span style='color: green;'>```Key_Forward = 16777314```</span>
  KeyForward = 16777314,
  /// C++ enum variant: <span style='color: green;'>```Key_Stop = 16777315```</span>
  KeyStop = 16777315,
  /// C++ enum variant: <span style='color: green;'>```Key_Refresh = 16777316```</span>
  KeyRefresh = 16777316,
  /// C++ enum variant: <span style='color: green;'>```Key_VolumeDown = 16777328```</span>
  KeyVolumeDown = 16777328,
  /// C++ enum variant: <span style='color: green;'>```Key_VolumeMute = 16777329```</span>
  KeyVolumeMute = 16777329,
  /// C++ enum variant: <span style='color: green;'>```Key_VolumeUp = 16777330```</span>
  KeyVolumeUp = 16777330,
  /// C++ enum variant: <span style='color: green;'>```Key_BassBoost = 16777331```</span>
  KeyBassBoost = 16777331,
  /// C++ enum variant: <span style='color: green;'>```Key_BassUp = 16777332```</span>
  KeyBassUp = 16777332,
  /// C++ enum variant: <span style='color: green;'>```Key_BassDown = 16777333```</span>
  KeyBassDown = 16777333,
  /// C++ enum variant: <span style='color: green;'>```Key_TrebleUp = 16777334```</span>
  KeyTrebleUp = 16777334,
  /// C++ enum variant: <span style='color: green;'>```Key_TrebleDown = 16777335```</span>
  KeyTrebleDown = 16777335,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaPlay = 16777344```</span>
  KeyMediaPlay = 16777344,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaStop = 16777345```</span>
  KeyMediaStop = 16777345,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaPrevious = 16777346```</span>
  KeyMediaPrevious = 16777346,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaNext = 16777347```</span>
  KeyMediaNext = 16777347,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaRecord = 16777348```</span>
  KeyMediaRecord = 16777348,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaPause = 16777349```</span>
  KeyMediaPause = 16777349,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaTogglePlayPause = 16777350```</span>
  KeyMediaTogglePlayPause = 16777350,
  /// C++ enum variant: <span style='color: green;'>```Key_HomePage = 16777360```</span>
  KeyHomePage = 16777360,
  /// C++ enum variant: <span style='color: green;'>```Key_Favorites = 16777361```</span>
  KeyFavorites = 16777361,
  /// C++ enum variant: <span style='color: green;'>```Key_Search = 16777362```</span>
  KeySearch = 16777362,
  /// C++ enum variant: <span style='color: green;'>```Key_Standby = 16777363```</span>
  KeyStandby = 16777363,
  /// C++ enum variant: <span style='color: green;'>```Key_OpenUrl = 16777364```</span>
  KeyOpenUrl = 16777364,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchMail = 16777376```</span>
  KeyLaunchMail = 16777376,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchMedia = 16777377```</span>
  KeyLaunchMedia = 16777377,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch0 = 16777378```</span>
  KeyLaunch0 = 16777378,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch1 = 16777379```</span>
  KeyLaunch1 = 16777379,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch2 = 16777380```</span>
  KeyLaunch2 = 16777380,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch3 = 16777381```</span>
  KeyLaunch3 = 16777381,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch4 = 16777382```</span>
  KeyLaunch4 = 16777382,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch5 = 16777383```</span>
  KeyLaunch5 = 16777383,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch6 = 16777384```</span>
  KeyLaunch6 = 16777384,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch7 = 16777385```</span>
  KeyLaunch7 = 16777385,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch8 = 16777386```</span>
  KeyLaunch8 = 16777386,
  /// C++ enum variant: <span style='color: green;'>```Key_Launch9 = 16777387```</span>
  KeyLaunch9 = 16777387,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchA = 16777388```</span>
  KeyLaunchA = 16777388,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchB = 16777389```</span>
  KeyLaunchB = 16777389,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchC = 16777390```</span>
  KeyLaunchC = 16777390,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchD = 16777391```</span>
  KeyLaunchD = 16777391,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchE = 16777392```</span>
  KeyLaunchE = 16777392,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchF = 16777393```</span>
  KeyLaunchF = 16777393,
  /// C++ enum variant: <span style='color: green;'>```Key_MonBrightnessUp = 16777394```</span>
  KeyMonBrightnessUp = 16777394,
  /// C++ enum variant: <span style='color: green;'>```Key_MonBrightnessDown = 16777395```</span>
  KeyMonBrightnessDown = 16777395,
  /// C++ enum variant: <span style='color: green;'>```Key_KeyboardLightOnOff = 16777396```</span>
  KeyKeyboardLightOnOff = 16777396,
  /// C++ enum variant: <span style='color: green;'>```Key_KeyboardBrightnessUp = 16777397```</span>
  KeyKeyboardBrightnessUp = 16777397,
  /// C++ enum variant: <span style='color: green;'>```Key_KeyboardBrightnessDown = 16777398```</span>
  KeyKeyboardBrightnessDown = 16777398,
  /// C++ enum variant: <span style='color: green;'>```Key_PowerOff = 16777399```</span>
  KeyPowerOff = 16777399,
  /// C++ enum variant: <span style='color: green;'>```Key_WakeUp = 16777400```</span>
  KeyWakeUp = 16777400,
  /// C++ enum variant: <span style='color: green;'>```Key_Eject = 16777401```</span>
  KeyEject = 16777401,
  /// C++ enum variant: <span style='color: green;'>```Key_ScreenSaver = 16777402```</span>
  KeyScreenSaver = 16777402,
  /// C++ enum variant: <span style='color: green;'>```Key_WWW = 16777403```</span>
  KeyWWW = 16777403,
  /// C++ enum variant: <span style='color: green;'>```Key_Memo = 16777404```</span>
  KeyMemo = 16777404,
  /// C++ enum variant: <span style='color: green;'>```Key_LightBulb = 16777405```</span>
  KeyLightBulb = 16777405,
  /// C++ enum variant: <span style='color: green;'>```Key_Shop = 16777406```</span>
  KeyShop = 16777406,
  /// C++ enum variant: <span style='color: green;'>```Key_History = 16777407```</span>
  KeyHistory = 16777407,
  /// C++ enum variant: <span style='color: green;'>```Key_AddFavorite = 16777408```</span>
  KeyAddFavorite = 16777408,
  /// C++ enum variant: <span style='color: green;'>```Key_HotLinks = 16777409```</span>
  KeyHotLinks = 16777409,
  /// C++ enum variant: <span style='color: green;'>```Key_BrightnessAdjust = 16777410```</span>
  KeyBrightnessAdjust = 16777410,
  /// C++ enum variant: <span style='color: green;'>```Key_Finance = 16777411```</span>
  KeyFinance = 16777411,
  /// C++ enum variant: <span style='color: green;'>```Key_Community = 16777412```</span>
  KeyCommunity = 16777412,
  /// C++ enum variant: <span style='color: green;'>```Key_AudioRewind = 16777413```</span>
  KeyAudioRewind = 16777413,
  /// C++ enum variant: <span style='color: green;'>```Key_BackForward = 16777414```</span>
  KeyBackForward = 16777414,
  /// C++ enum variant: <span style='color: green;'>```Key_ApplicationLeft = 16777415```</span>
  KeyApplicationLeft = 16777415,
  /// C++ enum variant: <span style='color: green;'>```Key_ApplicationRight = 16777416```</span>
  KeyApplicationRight = 16777416,
  /// C++ enum variant: <span style='color: green;'>```Key_Book = 16777417```</span>
  KeyBook = 16777417,
  /// C++ enum variant: <span style='color: green;'>```Key_CD = 16777418```</span>
  KeyCD = 16777418,
  /// C++ enum variant: <span style='color: green;'>```Key_Calculator = 16777419```</span>
  KeyCalculator = 16777419,
  /// C++ enum variant: <span style='color: green;'>```Key_ToDoList = 16777420```</span>
  KeyToDoList = 16777420,
  /// C++ enum variant: <span style='color: green;'>```Key_ClearGrab = 16777421```</span>
  KeyClearGrab = 16777421,
  /// C++ enum variant: <span style='color: green;'>```Key_Close = 16777422```</span>
  KeyClose = 16777422,
  /// C++ enum variant: <span style='color: green;'>```Key_Copy = 16777423```</span>
  KeyCopy = 16777423,
  /// C++ enum variant: <span style='color: green;'>```Key_Cut = 16777424```</span>
  KeyCut = 16777424,
  /// C++ enum variant: <span style='color: green;'>```Key_Display = 16777425```</span>
  KeyDisplay = 16777425,
  /// C++ enum variant: <span style='color: green;'>```Key_DOS = 16777426```</span>
  KeyDOS = 16777426,
  /// C++ enum variant: <span style='color: green;'>```Key_Documents = 16777427```</span>
  KeyDocuments = 16777427,
  /// C++ enum variant: <span style='color: green;'>```Key_Excel = 16777428```</span>
  KeyExcel = 16777428,
  /// C++ enum variant: <span style='color: green;'>```Key_Explorer = 16777429```</span>
  KeyExplorer = 16777429,
  /// C++ enum variant: <span style='color: green;'>```Key_Game = 16777430```</span>
  KeyGame = 16777430,
  /// C++ enum variant: <span style='color: green;'>```Key_Go = 16777431```</span>
  KeyGo = 16777431,
  /// C++ enum variant: <span style='color: green;'>```Key_iTouch = 16777432```</span>
  KeyITouch = 16777432,
  /// C++ enum variant: <span style='color: green;'>```Key_LogOff = 16777433```</span>
  KeyLogOff = 16777433,
  /// C++ enum variant: <span style='color: green;'>```Key_Market = 16777434```</span>
  KeyMarket = 16777434,
  /// C++ enum variant: <span style='color: green;'>```Key_Meeting = 16777435```</span>
  KeyMeeting = 16777435,
  /// C++ enum variant: <span style='color: green;'>```Key_MenuKB = 16777436```</span>
  KeyMenuKB = 16777436,
  /// C++ enum variant: <span style='color: green;'>```Key_MenuPB = 16777437```</span>
  KeyMenuPB = 16777437,
  /// C++ enum variant: <span style='color: green;'>```Key_MySites = 16777438```</span>
  KeyMySites = 16777438,
  /// C++ enum variant: <span style='color: green;'>```Key_News = 16777439```</span>
  KeyNews = 16777439,
  /// C++ enum variant: <span style='color: green;'>```Key_OfficeHome = 16777440```</span>
  KeyOfficeHome = 16777440,
  /// C++ enum variant: <span style='color: green;'>```Key_Option = 16777441```</span>
  KeyOption = 16777441,
  /// C++ enum variant: <span style='color: green;'>```Key_Paste = 16777442```</span>
  KeyPaste = 16777442,
  /// C++ enum variant: <span style='color: green;'>```Key_Phone = 16777443```</span>
  KeyPhone = 16777443,
  /// C++ enum variant: <span style='color: green;'>```Key_Calendar = 16777444```</span>
  KeyCalendar = 16777444,
  /// C++ enum variant: <span style='color: green;'>```Key_Reply = 16777445```</span>
  KeyReply = 16777445,
  /// C++ enum variant: <span style='color: green;'>```Key_Reload = 16777446```</span>
  KeyReload = 16777446,
  /// C++ enum variant: <span style='color: green;'>```Key_RotateWindows = 16777447```</span>
  KeyRotateWindows = 16777447,
  /// C++ enum variant: <span style='color: green;'>```Key_RotationPB = 16777448```</span>
  KeyRotationPB = 16777448,
  /// C++ enum variant: <span style='color: green;'>```Key_RotationKB = 16777449```</span>
  KeyRotationKB = 16777449,
  /// C++ enum variant: <span style='color: green;'>```Key_Save = 16777450```</span>
  KeySave = 16777450,
  /// C++ enum variant: <span style='color: green;'>```Key_Send = 16777451```</span>
  KeySend = 16777451,
  /// C++ enum variant: <span style='color: green;'>```Key_Spell = 16777452```</span>
  KeySpell = 16777452,
  /// C++ enum variant: <span style='color: green;'>```Key_SplitScreen = 16777453```</span>
  KeySplitScreen = 16777453,
  /// C++ enum variant: <span style='color: green;'>```Key_Support = 16777454```</span>
  KeySupport = 16777454,
  /// C++ enum variant: <span style='color: green;'>```Key_TaskPane = 16777455```</span>
  KeyTaskPane = 16777455,
  /// C++ enum variant: <span style='color: green;'>```Key_Terminal = 16777456```</span>
  KeyTerminal = 16777456,
  /// C++ enum variant: <span style='color: green;'>```Key_Tools = 16777457```</span>
  KeyTools = 16777457,
  /// C++ enum variant: <span style='color: green;'>```Key_Travel = 16777458```</span>
  KeyTravel = 16777458,
  /// C++ enum variant: <span style='color: green;'>```Key_Video = 16777459```</span>
  KeyVideo = 16777459,
  /// C++ enum variant: <span style='color: green;'>```Key_Word = 16777460```</span>
  KeyWord = 16777460,
  /// C++ enum variant: <span style='color: green;'>```Key_Xfer = 16777461```</span>
  KeyXfer = 16777461,
  /// C++ enum variant: <span style='color: green;'>```Key_ZoomIn = 16777462```</span>
  KeyZoomIn = 16777462,
  /// C++ enum variant: <span style='color: green;'>```Key_ZoomOut = 16777463```</span>
  KeyZoomOut = 16777463,
  /// C++ enum variant: <span style='color: green;'>```Key_Away = 16777464```</span>
  KeyAway = 16777464,
  /// C++ enum variant: <span style='color: green;'>```Key_Messenger = 16777465```</span>
  KeyMessenger = 16777465,
  /// C++ enum variant: <span style='color: green;'>```Key_WebCam = 16777466```</span>
  KeyWebCam = 16777466,
  /// C++ enum variant: <span style='color: green;'>```Key_MailForward = 16777467```</span>
  KeyMailForward = 16777467,
  /// C++ enum variant: <span style='color: green;'>```Key_Pictures = 16777468```</span>
  KeyPictures = 16777468,
  /// C++ enum variant: <span style='color: green;'>```Key_Music = 16777469```</span>
  KeyMusic = 16777469,
  /// C++ enum variant: <span style='color: green;'>```Key_Battery = 16777470```</span>
  KeyBattery = 16777470,
  /// C++ enum variant: <span style='color: green;'>```Key_Bluetooth = 16777471```</span>
  KeyBluetooth = 16777471,
  /// C++ enum variant: <span style='color: green;'>```Key_WLAN = 16777472```</span>
  KeyWLAN = 16777472,
  /// C++ enum variant: <span style='color: green;'>```Key_UWB = 16777473```</span>
  KeyUWB = 16777473,
  /// C++ enum variant: <span style='color: green;'>```Key_AudioForward = 16777474```</span>
  KeyAudioForward = 16777474,
  /// C++ enum variant: <span style='color: green;'>```Key_AudioRepeat = 16777475```</span>
  KeyAudioRepeat = 16777475,
  /// C++ enum variant: <span style='color: green;'>```Key_AudioRandomPlay = 16777476```</span>
  KeyAudioRandomPlay = 16777476,
  /// C++ enum variant: <span style='color: green;'>```Key_Subtitle = 16777477```</span>
  KeySubtitle = 16777477,
  /// C++ enum variant: <span style='color: green;'>```Key_AudioCycleTrack = 16777478```</span>
  KeyAudioCycleTrack = 16777478,
  /// C++ enum variant: <span style='color: green;'>```Key_Time = 16777479```</span>
  KeyTime = 16777479,
  /// C++ enum variant: <span style='color: green;'>```Key_Hibernate = 16777480```</span>
  KeyHibernate = 16777480,
  /// C++ enum variant: <span style='color: green;'>```Key_View = 16777481```</span>
  KeyView = 16777481,
  /// C++ enum variant: <span style='color: green;'>```Key_TopMenu = 16777482```</span>
  KeyTopMenu = 16777482,
  /// C++ enum variant: <span style='color: green;'>```Key_PowerDown = 16777483```</span>
  KeyPowerDown = 16777483,
  /// C++ enum variant: <span style='color: green;'>```Key_Suspend = 16777484```</span>
  KeySuspend = 16777484,
  /// C++ enum variant: <span style='color: green;'>```Key_ContrastAdjust = 16777485```</span>
  KeyContrastAdjust = 16777485,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchG = 16777486```</span>
  KeyLaunchG = 16777486,
  /// C++ enum variant: <span style='color: green;'>```Key_LaunchH = 16777487```</span>
  KeyLaunchH = 16777487,
  /// C++ enum variant: <span style='color: green;'>```Key_TouchpadToggle = 16777488```</span>
  KeyTouchpadToggle = 16777488,
  /// C++ enum variant: <span style='color: green;'>```Key_TouchpadOn = 16777489```</span>
  KeyTouchpadOn = 16777489,
  /// C++ enum variant: <span style='color: green;'>```Key_TouchpadOff = 16777490```</span>
  KeyTouchpadOff = 16777490,
  /// C++ enum variant: <span style='color: green;'>```Key_MicMute = 16777491```</span>
  KeyMicMute = 16777491,
  /// C++ enum variant: <span style='color: green;'>```Key_Red = 16777492```</span>
  KeyRed = 16777492,
  /// C++ enum variant: <span style='color: green;'>```Key_Green = 16777493```</span>
  KeyGreen = 16777493,
  /// C++ enum variant: <span style='color: green;'>```Key_Yellow = 16777494```</span>
  KeyYellow = 16777494,
  /// C++ enum variant: <span style='color: green;'>```Key_Blue = 16777495```</span>
  KeyBlue = 16777495,
  /// C++ enum variant: <span style='color: green;'>```Key_ChannelUp = 16777496```</span>
  KeyChannelUp = 16777496,
  /// C++ enum variant: <span style='color: green;'>```Key_ChannelDown = 16777497```</span>
  KeyChannelDown = 16777497,
  /// C++ enum variant: <span style='color: green;'>```Key_Guide = 16777498```</span>
  KeyGuide = 16777498,
  /// C++ enum variant: <span style='color: green;'>```Key_Info = 16777499```</span>
  KeyInfo = 16777499,
  /// C++ enum variant: <span style='color: green;'>```Key_Settings = 16777500```</span>
  KeySettings = 16777500,
  /// C++ enum variant: <span style='color: green;'>```Key_MicVolumeUp = 16777501```</span>
  KeyMicVolumeUp = 16777501,
  /// C++ enum variant: <span style='color: green;'>```Key_MicVolumeDown = 16777502```</span>
  KeyMicVolumeDown = 16777502,
  /// C++ enum variant: <span style='color: green;'>```Key_New = 16777504```</span>
  KeyNew = 16777504,
  /// C++ enum variant: <span style='color: green;'>```Key_Open = 16777505```</span>
  KeyOpen = 16777505,
  /// C++ enum variant: <span style='color: green;'>```Key_Find = 16777506```</span>
  KeyFind = 16777506,
  /// C++ enum variant: <span style='color: green;'>```Key_Undo = 16777507```</span>
  KeyUndo = 16777507,
  /// C++ enum variant: <span style='color: green;'>```Key_Redo = 16777508```</span>
  KeyRedo = 16777508,
  /// C++ enum variant: <span style='color: green;'>```Key_AltGr = 16781571```</span>
  KeyAltGr = 16781571,
  /// C++ enum variant: <span style='color: green;'>```Key_Multi_key = 16781600```</span>
  KeyMultiKey = 16781600,
  /// C++ enum variant: <span style='color: green;'>```Key_Kanji = 16781601```</span>
  KeyKanji = 16781601,
  /// C++ enum variant: <span style='color: green;'>```Key_Muhenkan = 16781602```</span>
  KeyMuhenkan = 16781602,
  /// C++ enum variant: <span style='color: green;'>```Key_Henkan = 16781603```</span>
  KeyHenkan = 16781603,
  /// C++ enum variant: <span style='color: green;'>```Key_Romaji = 16781604```</span>
  KeyRomaji = 16781604,
  /// C++ enum variant: <span style='color: green;'>```Key_Hiragana = 16781605```</span>
  KeyHiragana = 16781605,
  /// C++ enum variant: <span style='color: green;'>```Key_Katakana = 16781606```</span>
  KeyKatakana = 16781606,
  /// C++ enum variant: <span style='color: green;'>```Key_Hiragana_Katakana = 16781607```</span>
  KeyHiraganaKatakana = 16781607,
  /// C++ enum variant: <span style='color: green;'>```Key_Zenkaku = 16781608```</span>
  KeyZenkaku = 16781608,
  /// C++ enum variant: <span style='color: green;'>```Key_Hankaku = 16781609```</span>
  KeyHankaku = 16781609,
  /// C++ enum variant: <span style='color: green;'>```Key_Zenkaku_Hankaku = 16781610```</span>
  KeyZenkakuHankaku = 16781610,
  /// C++ enum variant: <span style='color: green;'>```Key_Touroku = 16781611```</span>
  KeyTouroku = 16781611,
  /// C++ enum variant: <span style='color: green;'>```Key_Massyo = 16781612```</span>
  KeyMassyo = 16781612,
  /// C++ enum variant: <span style='color: green;'>```Key_Kana_Lock = 16781613```</span>
  KeyKanaLock = 16781613,
  /// C++ enum variant: <span style='color: green;'>```Key_Kana_Shift = 16781614```</span>
  KeyKanaShift = 16781614,
  /// C++ enum variant: <span style='color: green;'>```Key_Eisu_Shift = 16781615```</span>
  KeyEisuShift = 16781615,
  /// C++ enum variant: <span style='color: green;'>```Key_Eisu_toggle = 16781616```</span>
  KeyEisuToggle = 16781616,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul = 16781617```</span>
  KeyHangul = 16781617,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_Start = 16781618```</span>
  KeyHangulStart = 16781618,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_End = 16781619```</span>
  KeyHangulEnd = 16781619,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_Hanja = 16781620```</span>
  KeyHangulHanja = 16781620,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_Jamo = 16781621```</span>
  KeyHangulJamo = 16781621,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_Romaja = 16781622```</span>
  KeyHangulRomaja = 16781622,
  /// C++ enum variant: <span style='color: green;'>```Key_Codeinput = 16781623```</span>
  KeyCodeinput = 16781623,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_Jeonja = 16781624```</span>
  KeyHangulJeonja = 16781624,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_Banja = 16781625```</span>
  KeyHangulBanja = 16781625,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_PreHanja = 16781626```</span>
  KeyHangulPreHanja = 16781626,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_PostHanja = 16781627```</span>
  KeyHangulPostHanja = 16781627,
  /// C++ enum variant: <span style='color: green;'>```Key_SingleCandidate = 16781628```</span>
  KeySingleCandidate = 16781628,
  /// C++ enum variant: <span style='color: green;'>```Key_MultipleCandidate = 16781629```</span>
  KeyMultipleCandidate = 16781629,
  /// C++ enum variant: <span style='color: green;'>```Key_PreviousCandidate = 16781630```</span>
  KeyPreviousCandidate = 16781630,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangul_Special = 16781631```</span>
  KeyHangulSpecial = 16781631,
  /// C++ enum variant: <span style='color: green;'>```Key_Mode_switch = 16781694```</span>
  KeyModeSwitch = 16781694,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Grave = 16781904```</span>
  KeyDeadGrave = 16781904,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Acute = 16781905```</span>
  KeyDeadAcute = 16781905,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Circumflex = 16781906```</span>
  KeyDeadCircumflex = 16781906,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Tilde = 16781907```</span>
  KeyDeadTilde = 16781907,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Macron = 16781908```</span>
  KeyDeadMacron = 16781908,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Breve = 16781909```</span>
  KeyDeadBreve = 16781909,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Abovedot = 16781910```</span>
  KeyDeadAbovedot = 16781910,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Diaeresis = 16781911```</span>
  KeyDeadDiaeresis = 16781911,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Abovering = 16781912```</span>
  KeyDeadAbovering = 16781912,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Doubleacute = 16781913```</span>
  KeyDeadDoubleacute = 16781913,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Caron = 16781914```</span>
  KeyDeadCaron = 16781914,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Cedilla = 16781915```</span>
  KeyDeadCedilla = 16781915,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Ogonek = 16781916```</span>
  KeyDeadOgonek = 16781916,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Iota = 16781917```</span>
  KeyDeadIota = 16781917,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Voiced_Sound = 16781918```</span>
  KeyDeadVoicedSound = 16781918,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Semivoiced_Sound = 16781919```</span>
  KeyDeadSemivoicedSound = 16781919,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Belowdot = 16781920```</span>
  KeyDeadBelowdot = 16781920,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Hook = 16781921```</span>
  KeyDeadHook = 16781921,
  /// C++ enum variant: <span style='color: green;'>```Key_Dead_Horn = 16781922```</span>
  KeyDeadHorn = 16781922,
  /// C++ enum variant: <span style='color: green;'>```Key_MediaLast = 16842751```</span>
  KeyMediaLast = 16842751,
  /// C++ enum variant: <span style='color: green;'>```Key_Select = 16842752```</span>
  KeySelect = 16842752,
  /// C++ enum variant: <span style='color: green;'>```Key_Yes = 16842753```</span>
  KeyYes = 16842753,
  /// C++ enum variant: <span style='color: green;'>```Key_No = 16842754```</span>
  KeyNo = 16842754,
  /// C++ enum variant: <span style='color: green;'>```Key_Cancel = 16908289```</span>
  KeyCancel = 16908289,
  /// C++ enum variant: <span style='color: green;'>```Key_Printer = 16908290```</span>
  KeyPrinter = 16908290,
  /// C++ enum variant: <span style='color: green;'>```Key_Execute = 16908291```</span>
  KeyExecute = 16908291,
  /// C++ enum variant: <span style='color: green;'>```Key_Sleep = 16908292```</span>
  KeySleep = 16908292,
  /// C++ enum variant: <span style='color: green;'>```Key_Play = 16908293```</span>
  KeyPlay = 16908293,
  /// C++ enum variant: <span style='color: green;'>```Key_Zoom = 16908294```</span>
  KeyZoom = 16908294,
  /// C++ enum variant: <span style='color: green;'>```Key_Exit = 16908298```</span>
  KeyExit = 16908298,
  /// C++ enum variant: <span style='color: green;'>```Key_Context1 = 17825792```</span>
  KeyContext1 = 17825792,
  /// C++ enum variant: <span style='color: green;'>```Key_Context2 = 17825793```</span>
  KeyContext2 = 17825793,
  /// C++ enum variant: <span style='color: green;'>```Key_Context3 = 17825794```</span>
  KeyContext3 = 17825794,
  /// C++ enum variant: <span style='color: green;'>```Key_Context4 = 17825795```</span>
  KeyContext4 = 17825795,
  /// C++ enum variant: <span style='color: green;'>```Key_Call = 17825796```</span>
  KeyCall = 17825796,
  /// C++ enum variant: <span style='color: green;'>```Key_Hangup = 17825797```</span>
  KeyHangup = 17825797,
  /// C++ enum variant: <span style='color: green;'>```Key_Flip = 17825798```</span>
  KeyFlip = 17825798,
  /// C++ enum variant: <span style='color: green;'>```Key_ToggleCallHangup = 17825799```</span>
  KeyToggleCallHangup = 17825799,
  /// C++ enum variant: <span style='color: green;'>```Key_VoiceDial = 17825800```</span>
  KeyVoiceDial = 17825800,
  /// C++ enum variant: <span style='color: green;'>```Key_LastNumberRedial = 17825801```</span>
  KeyLastNumberRedial = 17825801,
  /// C++ enum variant: <span style='color: green;'>```Key_Camera = 17825824```</span>
  KeyCamera = 17825824,
  /// C++ enum variant: <span style='color: green;'>```Key_CameraFocus = 17825825```</span>
  KeyCameraFocus = 17825825,
  /// C++ enum variant: <span style='color: green;'>```Key_unknown = 33554431```</span>
  KeyUnknown = 33554431,
}

/// C++ type: <span style='color: green;'>```Qt::KeyboardModifier```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum KeyboardModifier {
  /// C++ enum variant: <span style='color: green;'>```KeyboardModifierMask = -33554432```</span>
  KeyboardModifierMask = -33554432,
  /// C++ enum variant: <span style='color: green;'>```NoModifier = 0```</span>
  NoModifier = 0,
  /// C++ enum variant: <span style='color: green;'>```ShiftModifier = 33554432```</span>
  ShiftModifier = 33554432,
  /// C++ enum variant: <span style='color: green;'>```ControlModifier = 67108864```</span>
  ControlModifier = 67108864,
  /// C++ enum variant: <span style='color: green;'>```AltModifier = 134217728```</span>
  AltModifier = 134217728,
  /// C++ enum variant: <span style='color: green;'>```MetaModifier = 268435456```</span>
  MetaModifier = 268435456,
  /// C++ enum variant: <span style='color: green;'>```KeypadModifier = 536870912```</span>
  KeypadModifier = 536870912,
  /// C++ enum variant: <span style='color: green;'>```GroupSwitchModifier = 1073741824```</span>
  GroupSwitchModifier = 1073741824,
}

impl ::flags::FlaggableEnum for KeyboardModifier {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "KeyboardModifier"
  }
}

/// C++ type: <span style='color: green;'>```Qt::LayoutDirection```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LayoutDirection {
  /// C++ enum variant: <span style='color: green;'>```LeftToRight = 0```</span>
  LeftToRight = 0,
  /// C++ enum variant: <span style='color: green;'>```RightToLeft = 1```</span>
  RightToLeft = 1,
  /// C++ enum variant: <span style='color: green;'>```LayoutDirectionAuto = 2```</span>
  LayoutDirectionAuto = 2,
}

/// C++ type: <span style='color: green;'>```Qt::MaskMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MaskMode {
  /// C++ enum variant: <span style='color: green;'>```MaskInColor = 0```</span>
  In = 0,
  /// C++ enum variant: <span style='color: green;'>```MaskOutColor = 1```</span>
  Out = 1,
}

/// C++ type: <span style='color: green;'>```Qt::MatchFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MatchFlag {
  /// C++ enum variant: <span style='color: green;'>```MatchExactly = 0```</span>
  Exactly = 0,
  /// C++ enum variant: <span style='color: green;'>```MatchContains = 1```</span>
  Contains = 1,
  /// C++ enum variant: <span style='color: green;'>```MatchStartsWith = 2```</span>
  StartsWith = 2,
  /// C++ enum variant: <span style='color: green;'>```MatchEndsWith = 3```</span>
  EndsWith = 3,
  /// C++ enum variant: <span style='color: green;'>```MatchRegExp = 4```</span>
  RegExp = 4,
  /// C++ enum variant: <span style='color: green;'>```MatchWildcard = 5```</span>
  Wildcard = 5,
  /// C++ enum variant: <span style='color: green;'>```MatchFixedString = 8```</span>
  FixedString = 8,
  /// C++ enum variant: <span style='color: green;'>```MatchCaseSensitive = 16```</span>
  CaseSensitive = 16,
  /// C++ enum variant: <span style='color: green;'>```MatchWrap = 32```</span>
  Wrap = 32,
  /// C++ enum variant: <span style='color: green;'>```MatchRecursive = 64```</span>
  Recursive = 64,
}

impl ::flags::FlaggableEnum for MatchFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "MatchFlag"
  }
}

/// C++ type: <span style='color: green;'>```Qt::Modifier```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Modifier {
  /// C++ enum variant: <span style='color: green;'>```MODIFIER_MASK = -33554432```</span>
  MODIFIERMASK = -33554432,
  /// C++ enum variant: <span style='color: green;'>```UNICODE_ACCEL = 0```</span>
  UNICODEACCEL = 0,
  /// C++ enum variant: <span style='color: green;'>```SHIFT = 33554432```</span>
  SHIFT = 33554432,
  /// C++ enum variant: <span style='color: green;'>```CTRL = 67108864```</span>
  CTRL = 67108864,
  /// C++ enum variant: <span style='color: green;'>```ALT = 134217728```</span>
  ALT = 134217728,
  /// C++ enum variant: <span style='color: green;'>```META = 268435456```</span>
  META = 268435456,
}

/// C++ type: <span style='color: green;'>```Qt::MouseButton```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MouseButton {
  /// C++ enum variant: <span style='color: green;'>```MouseButtonMask = -1```</span>
  MouseButtonMask = -1,
  /// C++ enum variant: <span style='color: green;'>```NoButton = 0```</span>
  NoButton = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftButton = 1```</span>
  LeftButton = 1,
  /// C++ enum variant: <span style='color: green;'>```RightButton = 2```</span>
  RightButton = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```MidButton = 4```</span>
  /// - <span style='color: green;'>```MiddleButton = 4```</span>
  ///
  MidButton = 4,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```BackButton = 8```</span>
  /// - <span style='color: green;'>```XButton1 = 8```</span>
  /// - <span style='color: green;'>```ExtraButton1 = 8```</span>
  ///
  BackButton = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ForwardButton = 16```</span>
  /// - <span style='color: green;'>```XButton2 = 16```</span>
  /// - <span style='color: green;'>```ExtraButton2 = 16```</span>
  ///
  ForwardButton = 16,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```TaskButton = 32```</span>
  /// - <span style='color: green;'>```ExtraButton3 = 32```</span>
  ///
  TaskButton = 32,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton4 = 64```</span>
  ExtraButton4 = 64,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton5 = 128```</span>
  ExtraButton5 = 128,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton6 = 256```</span>
  ExtraButton6 = 256,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton7 = 512```</span>
  ExtraButton7 = 512,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton8 = 1024```</span>
  ExtraButton8 = 1024,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton9 = 2048```</span>
  ExtraButton9 = 2048,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton10 = 4096```</span>
  ExtraButton10 = 4096,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton11 = 8192```</span>
  ExtraButton11 = 8192,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton12 = 16384```</span>
  ExtraButton12 = 16384,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton13 = 32768```</span>
  ExtraButton13 = 32768,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton14 = 65536```</span>
  ExtraButton14 = 65536,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton15 = 131072```</span>
  ExtraButton15 = 131072,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton16 = 262144```</span>
  ExtraButton16 = 262144,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton17 = 524288```</span>
  ExtraButton17 = 524288,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton18 = 1048576```</span>
  ExtraButton18 = 1048576,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton19 = 2097152```</span>
  ExtraButton19 = 2097152,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton20 = 4194304```</span>
  ExtraButton20 = 4194304,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton21 = 8388608```</span>
  ExtraButton21 = 8388608,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton22 = 16777216```</span>
  ExtraButton22 = 16777216,
  /// C++ enum variant: <span style='color: green;'>```ExtraButton23 = 33554432```</span>
  ExtraButton23 = 33554432,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ExtraButton24 = 67108864```</span>
  /// - <span style='color: green;'>```MaxMouseButton = 67108864```</span>
  ///
  ExtraButton24 = 67108864,
  /// C++ enum variant: <span style='color: green;'>```AllButtons = 134217727```</span>
  AllButtons = 134217727,
}

impl ::flags::FlaggableEnum for MouseButton {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "MouseButton"
  }
}

/// C++ type: <span style='color: green;'>```Qt::MouseEventFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MouseEventFlag {
  /// C++ enum variant: <span style='color: green;'>```MouseEventCreatedDoubleClick = 1```</span>
  CreatedDoubleClick = 1,
  /// C++ enum variant: <span style='color: green;'>```MouseEventFlagMask = 255```</span>
  FlagMask = 255,
}

/// C++ type: <span style='color: green;'>```Qt::MouseEventSource```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MouseEventSource {
  /// C++ enum variant: <span style='color: green;'>```MouseEventNotSynthesized = 0```</span>
  NotSynthesized = 0,
  /// C++ enum variant: <span style='color: green;'>```MouseEventSynthesizedBySystem = 1```</span>
  SynthesizedBySystem = 1,
  /// C++ enum variant: <span style='color: green;'>```MouseEventSynthesizedByQt = 2```</span>
  SynthesizedByQt = 2,
  /// C++ enum variant: <span style='color: green;'>```MouseEventSynthesizedByApplication = 3```</span>
  SynthesizedByApplication = 3,
}

/// C++ type: <span style='color: green;'>```Qt::NativeGestureType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum NativeGestureType {
  /// C++ enum variant: <span style='color: green;'>```BeginNativeGesture = 0```</span>
  Begin = 0,
  /// C++ enum variant: <span style='color: green;'>```EndNativeGesture = 1```</span>
  End = 1,
  /// C++ enum variant: <span style='color: green;'>```PanNativeGesture = 2```</span>
  Pan = 2,
  /// C++ enum variant: <span style='color: green;'>```ZoomNativeGesture = 3```</span>
  Zoom = 3,
  /// C++ enum variant: <span style='color: green;'>```SmartZoomNativeGesture = 4```</span>
  SmartZoom = 4,
  /// C++ enum variant: <span style='color: green;'>```RotateNativeGesture = 5```</span>
  Rotate = 5,
  /// C++ enum variant: <span style='color: green;'>```SwipeNativeGesture = 6```</span>
  Swipe = 6,
}

/// C++ type: <span style='color: green;'>```Qt::NavigationMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum NavigationMode {
  /// C++ enum variant: <span style='color: green;'>```NavigationModeNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```NavigationModeKeypadTabOrder = 1```</span>
  KeypadTabOrder = 1,
  /// C++ enum variant: <span style='color: green;'>```NavigationModeKeypadDirectional = 2```</span>
  KeypadDirectional = 2,
  /// C++ enum variant: <span style='color: green;'>```NavigationModeCursorAuto = 3```</span>
  CursorAuto = 3,
  /// C++ enum variant: <span style='color: green;'>```NavigationModeCursorForceVisible = 4```</span>
  CursorForceVisible = 4,
}

/// C++ type: <span style='color: green;'>```Qt::Orientation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Orientation {
  /// C++ enum variant: <span style='color: green;'>```Horizontal = 1```</span>
  Horizontal = 1,
  /// C++ enum variant: <span style='color: green;'>```Vertical = 2```</span>
  Vertical = 2,
}

impl ::flags::FlaggableEnum for Orientation {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Orientation"
  }
}

/// C++ type: <span style='color: green;'>```Qt::PenCapStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PenCapStyle {
  /// C++ enum variant: <span style='color: green;'>```FlatCap = 0```</span>
  FlatCap = 0,
  /// C++ enum variant: <span style='color: green;'>```SquareCap = 16```</span>
  SquareCap = 16,
  /// C++ enum variant: <span style='color: green;'>```RoundCap = 32```</span>
  RoundCap = 32,
  /// C++ enum variant: <span style='color: green;'>```MPenCapStyle = 48```</span>
  MPenCapStyle = 48,
}

/// C++ type: <span style='color: green;'>```Qt::PenJoinStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PenJoinStyle {
  /// C++ enum variant: <span style='color: green;'>```MiterJoin = 0```</span>
  MiterJoin = 0,
  /// C++ enum variant: <span style='color: green;'>```BevelJoin = 64```</span>
  BevelJoin = 64,
  /// C++ enum variant: <span style='color: green;'>```RoundJoin = 128```</span>
  RoundJoin = 128,
  /// C++ enum variant: <span style='color: green;'>```SvgMiterJoin = 256```</span>
  SvgMiterJoin = 256,
  /// C++ enum variant: <span style='color: green;'>```MPenJoinStyle = 448```</span>
  MPenJoinStyle = 448,
}

/// C++ type: <span style='color: green;'>```Qt::PenStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PenStyle {
  /// C++ enum variant: <span style='color: green;'>```NoPen = 0```</span>
  NoPen = 0,
  /// C++ enum variant: <span style='color: green;'>```SolidLine = 1```</span>
  SolidLine = 1,
  /// C++ enum variant: <span style='color: green;'>```DashLine = 2```</span>
  DashLine = 2,
  /// C++ enum variant: <span style='color: green;'>```DotLine = 3```</span>
  DotLine = 3,
  /// C++ enum variant: <span style='color: green;'>```DashDotLine = 4```</span>
  DashDotLine = 4,
  /// C++ enum variant: <span style='color: green;'>```DashDotDotLine = 5```</span>
  DashDotDotLine = 5,
  /// C++ enum variant: <span style='color: green;'>```CustomDashLine = 6```</span>
  CustomDashLine = 6,
  /// C++ enum variant: <span style='color: green;'>```MPenStyle = 15```</span>
  MPenStyle = 15,
}

/// C++ type: <span style='color: green;'>```Qt::ScreenOrientation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScreenOrientation {
  /// C++ enum variant: <span style='color: green;'>```PrimaryOrientation = 0```</span>
  Primary = 0,
  /// C++ enum variant: <span style='color: green;'>```PortraitOrientation = 1```</span>
  Portrait = 1,
  /// C++ enum variant: <span style='color: green;'>```LandscapeOrientation = 2```</span>
  Landscape = 2,
  /// C++ enum variant: <span style='color: green;'>```InvertedPortraitOrientation = 4```</span>
  InvertedPortrait = 4,
  /// C++ enum variant: <span style='color: green;'>```InvertedLandscapeOrientation = 8```</span>
  InvertedLandscape = 8,
}

impl ::flags::FlaggableEnum for ScreenOrientation {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ScreenOrientation"
  }
}

/// C++ type: <span style='color: green;'>```Qt::ScrollBarPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScrollBarPolicy {
  /// C++ enum variant: <span style='color: green;'>```ScrollBarAsNeeded = 0```</span>
  AsNeeded = 0,
  /// C++ enum variant: <span style='color: green;'>```ScrollBarAlwaysOff = 1```</span>
  AlwaysOff = 1,
  /// C++ enum variant: <span style='color: green;'>```ScrollBarAlwaysOn = 2```</span>
  AlwaysOn = 2,
}

/// C++ type: <span style='color: green;'>```Qt::ScrollPhase```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScrollPhase {
  /// C++ enum variant: <span style='color: green;'>```NoScrollPhase = 0```</span>
  NoScrollPhase = 0,
  /// C++ enum variant: <span style='color: green;'>```ScrollBegin = 1```</span>
  ScrollBegin = 1,
  /// C++ enum variant: <span style='color: green;'>```ScrollUpdate = 2```</span>
  ScrollUpdate = 2,
  /// C++ enum variant: <span style='color: green;'>```ScrollEnd = 3```</span>
  ScrollEnd = 3,
}

/// C++ type: <span style='color: green;'>```Qt::ShortcutContext```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ShortcutContext {
  /// C++ enum variant: <span style='color: green;'>```WidgetShortcut = 0```</span>
  Widget = 0,
  /// C++ enum variant: <span style='color: green;'>```WindowShortcut = 1```</span>
  Window = 1,
  /// C++ enum variant: <span style='color: green;'>```ApplicationShortcut = 2```</span>
  Application = 2,
  /// C++ enum variant: <span style='color: green;'>```WidgetWithChildrenShortcut = 3```</span>
  WidgetWithChildren = 3,
}

/// C++ type: <span style='color: green;'>```Qt::SizeHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SizeHint {
  /// C++ enum variant: <span style='color: green;'>```MinimumSize = 0```</span>
  MinimumSize = 0,
  /// C++ enum variant: <span style='color: green;'>```PreferredSize = 1```</span>
  PreferredSize = 1,
  /// C++ enum variant: <span style='color: green;'>```MaximumSize = 2```</span>
  MaximumSize = 2,
  /// C++ enum variant: <span style='color: green;'>```MinimumDescent = 3```</span>
  MinimumDescent = 3,
  /// C++ enum variant: <span style='color: green;'>```NSizeHints = 4```</span>
  NSizeHints = 4,
}

/// C++ type: <span style='color: green;'>```Qt::SizeMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SizeMode {
  /// C++ enum variant: <span style='color: green;'>```AbsoluteSize = 0```</span>
  Absolute = 0,
  /// C++ enum variant: <span style='color: green;'>```RelativeSize = 1```</span>
  Relative = 1,
}

/// C++ type: <span style='color: green;'>```Qt::SortOrder```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SortOrder {
  /// C++ enum variant: <span style='color: green;'>```AscendingOrder = 0```</span>
  Ascending = 0,
  /// C++ enum variant: <span style='color: green;'>```DescendingOrder = 1```</span>
  Descending = 1,
}

/// C++ type: <span style='color: green;'>```Qt::TabFocusBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabFocusBehavior {
  /// C++ enum variant: <span style='color: green;'>```NoTabFocus = 0```</span>
  NoTabFocus = 0,
  /// C++ enum variant: <span style='color: green;'>```TabFocusTextControls = 1```</span>
  TabFocusTextControls = 1,
  /// C++ enum variant: <span style='color: green;'>```TabFocusListControls = 2```</span>
  TabFocusListControls = 2,
  /// C++ enum variant: <span style='color: green;'>```TabFocusAllControls = 255```</span>
  TabFocusAllControls = 255,
}

/// C++ type: <span style='color: green;'>```Qt::TextElideMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextElideMode {
  /// C++ enum variant: <span style='color: green;'>```ElideLeft = 0```</span>
  Left = 0,
  /// C++ enum variant: <span style='color: green;'>```ElideRight = 1```</span>
  Right = 1,
  /// C++ enum variant: <span style='color: green;'>```ElideMiddle = 2```</span>
  Middle = 2,
  /// C++ enum variant: <span style='color: green;'>```ElideNone = 3```</span>
  None = 3,
}

/// C++ type: <span style='color: green;'>```Qt::TextFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextFlag {
  /// C++ enum variant: <span style='color: green;'>```TextSingleLine = 256```</span>
  SingleLine = 256,
  /// C++ enum variant: <span style='color: green;'>```TextDontClip = 512```</span>
  DontClip = 512,
  /// C++ enum variant: <span style='color: green;'>```TextExpandTabs = 1024```</span>
  ExpandTabs = 1024,
  /// C++ enum variant: <span style='color: green;'>```TextShowMnemonic = 2048```</span>
  ShowMnemonic = 2048,
  /// C++ enum variant: <span style='color: green;'>```TextWordWrap = 4096```</span>
  WordWrap = 4096,
  /// C++ enum variant: <span style='color: green;'>```TextWrapAnywhere = 8192```</span>
  WrapAnywhere = 8192,
  /// C++ enum variant: <span style='color: green;'>```TextDontPrint = 16384```</span>
  DontPrint = 16384,
  /// C++ enum variant: <span style='color: green;'>```TextHideMnemonic = 32768```</span>
  HideMnemonic = 32768,
  /// C++ enum variant: <span style='color: green;'>```TextJustificationForced = 65536```</span>
  JustificationForced = 65536,
  /// C++ enum variant: <span style='color: green;'>```TextForceLeftToRight = 131072```</span>
  ForceLeftToRight = 131072,
  /// C++ enum variant: <span style='color: green;'>```TextForceRightToLeft = 262144```</span>
  ForceRightToLeft = 262144,
  /// C++ enum variant: <span style='color: green;'>```TextLongestVariant = 524288```</span>
  LongestVariant = 524288,
  /// C++ enum variant: <span style='color: green;'>```TextBypassShaping = 1048576```</span>
  BypassShaping = 1048576,
  /// C++ enum variant: <span style='color: green;'>```TextIncludeTrailingSpaces = 134217728```</span>
  IncludeTrailingSpaces = 134217728,
}

/// C++ type: <span style='color: green;'>```Qt::TextFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextFormat {
  /// C++ enum variant: <span style='color: green;'>```PlainText = 0```</span>
  Plain = 0,
  /// C++ enum variant: <span style='color: green;'>```RichText = 1```</span>
  Rich = 1,
  /// C++ enum variant: <span style='color: green;'>```AutoText = 2```</span>
  Auto = 2,
}

/// C++ type: <span style='color: green;'>```Qt::TextInteractionFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextInteractionFlag {
  /// C++ enum variant: <span style='color: green;'>```NoTextInteraction = 0```</span>
  NoTextInteraction = 0,
  /// C++ enum variant: <span style='color: green;'>```TextSelectableByMouse = 1```</span>
  TextSelectableByMouse = 1,
  /// C++ enum variant: <span style='color: green;'>```TextSelectableByKeyboard = 2```</span>
  TextSelectableByKeyboard = 2,
  /// C++ enum variant: <span style='color: green;'>```LinksAccessibleByMouse = 4```</span>
  LinksAccessibleByMouse = 4,
  /// C++ enum variant: <span style='color: green;'>```LinksAccessibleByKeyboard = 8```</span>
  LinksAccessibleByKeyboard = 8,
  /// C++ enum variant: <span style='color: green;'>```TextBrowserInteraction = 13```</span>
  TextBrowserInteraction = 13,
  /// C++ enum variant: <span style='color: green;'>```TextEditable = 16```</span>
  TextEditable = 16,
  /// C++ enum variant: <span style='color: green;'>```TextEditorInteraction = 19```</span>
  TextEditorInteraction = 19,
}

impl ::flags::FlaggableEnum for TextInteractionFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "TextInteractionFlag"
  }
}

/// C++ type: <span style='color: green;'>```Qt::TileRule```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TileRule {
  /// C++ enum variant: <span style='color: green;'>```StretchTile = 0```</span>
  Stretch = 0,
  /// C++ enum variant: <span style='color: green;'>```RepeatTile = 1```</span>
  Repeat = 1,
  /// C++ enum variant: <span style='color: green;'>```RoundTile = 2```</span>
  Round = 2,
}

/// C++ type: <span style='color: green;'>```Qt::TimeSpec```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TimeSpec {
  /// C++ enum variant: <span style='color: green;'>```LocalTime = 0```</span>
  LocalTime = 0,
  /// C++ enum variant: <span style='color: green;'>```UTC = 1```</span>
  UTC = 1,
  /// C++ enum variant: <span style='color: green;'>```OffsetFromUTC = 2```</span>
  OffsetFromUTC = 2,
  /// C++ enum variant: <span style='color: green;'>```TimeZone = 3```</span>
  TimeZone = 3,
}

/// C++ type: <span style='color: green;'>```Qt::TimerType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TimerType {
  /// C++ enum variant: <span style='color: green;'>```PreciseTimer = 0```</span>
  Precise = 0,
  /// C++ enum variant: <span style='color: green;'>```CoarseTimer = 1```</span>
  Coarse = 1,
  /// C++ enum variant: <span style='color: green;'>```VeryCoarseTimer = 2```</span>
  VeryCoarse = 2,
}

/// C++ type: <span style='color: green;'>```Qt::ToolBarArea```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ToolBarArea {
  /// C++ enum variant: <span style='color: green;'>```NoToolBarArea = 0```</span>
  NoToolBarArea = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftToolBarArea = 1```</span>
  LeftToolBarArea = 1,
  /// C++ enum variant: <span style='color: green;'>```RightToolBarArea = 2```</span>
  RightToolBarArea = 2,
  /// C++ enum variant: <span style='color: green;'>```TopToolBarArea = 4```</span>
  TopToolBarArea = 4,
  /// C++ enum variant: <span style='color: green;'>```BottomToolBarArea = 8```</span>
  BottomToolBarArea = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ToolBarArea_Mask = 15```</span>
  /// - <span style='color: green;'>```AllToolBarAreas = 15```</span>
  ///
  ToolBarAreaMask = 15,
}

impl ::flags::FlaggableEnum for ToolBarArea {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ToolBarArea"
  }
}

/// C++ type: <span style='color: green;'>```Qt::ToolBarAreaSizes```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ToolBarAreaSizes {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```NToolBarAreas = 4```</span>
  NToolBarAreas = 4,
}

/// C++ type: <span style='color: green;'>```Qt::ToolButtonStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ToolButtonStyle {
  /// C++ enum variant: <span style='color: green;'>```ToolButtonIconOnly = 0```</span>
  IconOnly = 0,
  /// C++ enum variant: <span style='color: green;'>```ToolButtonTextOnly = 1```</span>
  TextOnly = 1,
  /// C++ enum variant: <span style='color: green;'>```ToolButtonTextBesideIcon = 2```</span>
  TextBesideIcon = 2,
  /// C++ enum variant: <span style='color: green;'>```ToolButtonTextUnderIcon = 3```</span>
  TextUnderIcon = 3,
  /// C++ enum variant: <span style='color: green;'>```ToolButtonFollowStyle = 4```</span>
  FollowStyle = 4,
}

/// C++ type: <span style='color: green;'>```Qt::TouchPointState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TouchPointState {
  /// C++ enum variant: <span style='color: green;'>```TouchPointPressed = 1```</span>
  Pressed = 1,
  /// C++ enum variant: <span style='color: green;'>```TouchPointMoved = 2```</span>
  Moved = 2,
  /// C++ enum variant: <span style='color: green;'>```TouchPointStationary = 4```</span>
  Stationary = 4,
  /// C++ enum variant: <span style='color: green;'>```TouchPointReleased = 8```</span>
  Released = 8,
}

impl ::flags::FlaggableEnum for TouchPointState {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "TouchPointState"
  }
}

/// C++ type: <span style='color: green;'>```Qt::TransformationMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TransformationMode {
  /// C++ enum variant: <span style='color: green;'>```FastTransformation = 0```</span>
  Fast = 0,
  /// C++ enum variant: <span style='color: green;'>```SmoothTransformation = 1```</span>
  Smooth = 1,
}

/// C++ type: <span style='color: green;'>```Qt::UIEffect```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UIEffect {
  /// C++ enum variant: <span style='color: green;'>```UI_General = 0```</span>
  General = 0,
  /// C++ enum variant: <span style='color: green;'>```UI_AnimateMenu = 1```</span>
  AnimateMenu = 1,
  /// C++ enum variant: <span style='color: green;'>```UI_FadeMenu = 2```</span>
  FadeMenu = 2,
  /// C++ enum variant: <span style='color: green;'>```UI_AnimateCombo = 3```</span>
  AnimateCombo = 3,
  /// C++ enum variant: <span style='color: green;'>```UI_AnimateTooltip = 4```</span>
  AnimateTooltip = 4,
  /// C++ enum variant: <span style='color: green;'>```UI_FadeTooltip = 5```</span>
  FadeTooltip = 5,
  /// C++ enum variant: <span style='color: green;'>```UI_AnimateToolBox = 6```</span>
  AnimateToolBox = 6,
}

/// C++ type: <span style='color: green;'>```Qt::WhiteSpaceMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WhiteSpaceMode {
  /// C++ enum variant: <span style='color: green;'>```WhiteSpaceModeUndefined = -1```</span>
  ModeUndefined = -1,
  /// C++ enum variant: <span style='color: green;'>```WhiteSpaceNormal = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```WhiteSpacePre = 1```</span>
  Pre = 1,
  /// C++ enum variant: <span style='color: green;'>```WhiteSpaceNoWrap = 2```</span>
  NoWrap = 2,
}

/// C++ type: <span style='color: green;'>```Qt::WidgetAttribute```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WidgetAttribute {
  /// C++ enum variant: <span style='color: green;'>```WA_Disabled = 0```</span>
  Disabled = 0,
  /// C++ enum variant: <span style='color: green;'>```WA_UnderMouse = 1```</span>
  UnderMouse = 1,
  /// C++ enum variant: <span style='color: green;'>```WA_MouseTracking = 2```</span>
  MouseTracking = 2,
  /// C++ enum variant: <span style='color: green;'>```WA_ContentsPropagated = 3```</span>
  ContentsPropagated = 3,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WA_OpaquePaintEvent = 4```</span>
  /// - <span style='color: green;'>```WA_NoBackground = 4```</span>
  ///
  OpaquePaintEvent = 4,
  /// C++ enum variant: <span style='color: green;'>```WA_StaticContents = 5```</span>
  StaticContents = 5,
  /// C++ enum variant: <span style='color: green;'>```WA_LaidOut = 7```</span>
  LaidOut = 7,
  /// C++ enum variant: <span style='color: green;'>```WA_PaintOnScreen = 8```</span>
  PaintOnScreen = 8,
  /// C++ enum variant: <span style='color: green;'>```WA_NoSystemBackground = 9```</span>
  NoSystemBackground = 9,
  /// C++ enum variant: <span style='color: green;'>```WA_UpdatesDisabled = 10```</span>
  UpdatesDisabled = 10,
  /// C++ enum variant: <span style='color: green;'>```WA_Mapped = 11```</span>
  Mapped = 11,
  /// C++ enum variant: <span style='color: green;'>```WA_MacNoClickThrough = 12```</span>
  MacNoClickThrough = 12,
  /// C++ enum variant: <span style='color: green;'>```WA_InputMethodEnabled = 14```</span>
  InputMethodEnabled = 14,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_Visible = 15```</span>
  WStateVisible = 15,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_Hidden = 16```</span>
  WStateHidden = 16,
  /// C++ enum variant: <span style='color: green;'>```WA_ForceDisabled = 32```</span>
  ForceDisabled = 32,
  /// C++ enum variant: <span style='color: green;'>```WA_KeyCompression = 33```</span>
  KeyCompression = 33,
  /// C++ enum variant: <span style='color: green;'>```WA_PendingMoveEvent = 34```</span>
  PendingMoveEvent = 34,
  /// C++ enum variant: <span style='color: green;'>```WA_PendingResizeEvent = 35```</span>
  PendingResizeEvent = 35,
  /// C++ enum variant: <span style='color: green;'>```WA_SetPalette = 36```</span>
  SetPalette = 36,
  /// C++ enum variant: <span style='color: green;'>```WA_SetFont = 37```</span>
  SetFont = 37,
  /// C++ enum variant: <span style='color: green;'>```WA_SetCursor = 38```</span>
  SetCursor = 38,
  /// C++ enum variant: <span style='color: green;'>```WA_NoChildEventsFromChildren = 39```</span>
  NoChildEventsFromChildren = 39,
  /// C++ enum variant: <span style='color: green;'>```WA_WindowModified = 41```</span>
  WindowModified = 41,
  /// C++ enum variant: <span style='color: green;'>```WA_Resized = 42```</span>
  Resized = 42,
  /// C++ enum variant: <span style='color: green;'>```WA_Moved = 43```</span>
  Moved = 43,
  /// C++ enum variant: <span style='color: green;'>```WA_PendingUpdate = 44```</span>
  PendingUpdate = 44,
  /// C++ enum variant: <span style='color: green;'>```WA_InvalidSize = 45```</span>
  InvalidSize = 45,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WA_MacBrushedMetal = 46```</span>
  /// - <span style='color: green;'>```WA_MacMetalStyle = 46```</span>
  ///
  MacBrushedMetal = 46,
  /// C++ enum variant: <span style='color: green;'>```WA_CustomWhatsThis = 47```</span>
  CustomWhatsThis = 47,
  /// C++ enum variant: <span style='color: green;'>```WA_LayoutOnEntireRect = 48```</span>
  LayoutOnEntireRect = 48,
  /// C++ enum variant: <span style='color: green;'>```WA_OutsideWSRange = 49```</span>
  OutsideWSRange = 49,
  /// C++ enum variant: <span style='color: green;'>```WA_GrabbedShortcut = 50```</span>
  GrabbedShortcut = 50,
  /// C++ enum variant: <span style='color: green;'>```WA_TransparentForMouseEvents = 51```</span>
  TransparentForMouseEvents = 51,
  /// C++ enum variant: <span style='color: green;'>```WA_PaintUnclipped = 52```</span>
  PaintUnclipped = 52,
  /// C++ enum variant: <span style='color: green;'>```WA_SetWindowIcon = 53```</span>
  SetWindowIcon = 53,
  /// C++ enum variant: <span style='color: green;'>```WA_NoMouseReplay = 54```</span>
  NoMouseReplay = 54,
  /// C++ enum variant: <span style='color: green;'>```WA_DeleteOnClose = 55```</span>
  DeleteOnClose = 55,
  /// C++ enum variant: <span style='color: green;'>```WA_RightToLeft = 56```</span>
  RightToLeft = 56,
  /// C++ enum variant: <span style='color: green;'>```WA_SetLayoutDirection = 57```</span>
  SetLayoutDirection = 57,
  /// C++ enum variant: <span style='color: green;'>```WA_NoChildEventsForParent = 58```</span>
  NoChildEventsForParent = 58,
  /// C++ enum variant: <span style='color: green;'>```WA_ForceUpdatesDisabled = 59```</span>
  ForceUpdatesDisabled = 59,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_Created = 60```</span>
  WStateCreated = 60,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_CompressKeys = 61```</span>
  WStateCompressKeys = 61,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_InPaintEvent = 62```</span>
  WStateInPaintEvent = 62,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_Reparented = 63```</span>
  WStateReparented = 63,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_ConfigPending = 64```</span>
  WStateConfigPending = 64,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_Polished = 66```</span>
  WStatePolished = 66,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_DND = 67```</span>
  WStateDND = 67,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_OwnSizePolicy = 68```</span>
  WStateOwnSizePolicy = 68,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_ExplicitShowHide = 69```</span>
  WStateExplicitShowHide = 69,
  /// C++ enum variant: <span style='color: green;'>```WA_ShowModal = 70```</span>
  ShowModal = 70,
  /// C++ enum variant: <span style='color: green;'>```WA_MouseNoMask = 71```</span>
  MouseNoMask = 71,
  /// C++ enum variant: <span style='color: green;'>```WA_GroupLeader = 72```</span>
  GroupLeader = 72,
  /// C++ enum variant: <span style='color: green;'>```WA_NoMousePropagation = 73```</span>
  NoMousePropagation = 73,
  /// C++ enum variant: <span style='color: green;'>```WA_Hover = 74```</span>
  Hover = 74,
  /// C++ enum variant: <span style='color: green;'>```WA_InputMethodTransparent = 75```</span>
  InputMethodTransparent = 75,
  /// C++ enum variant: <span style='color: green;'>```WA_QuitOnClose = 76```</span>
  QuitOnClose = 76,
  /// C++ enum variant: <span style='color: green;'>```WA_KeyboardFocusChange = 77```</span>
  KeyboardFocusChange = 77,
  /// C++ enum variant: <span style='color: green;'>```WA_AcceptDrops = 78```</span>
  AcceptDrops = 78,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WA_DropSiteRegistered = 79```</span>
  /// - <span style='color: green;'>```WA_ForceAcceptDrops = 79```</span>
  ///
  DropSiteRegistered = 79,
  /// C++ enum variant: <span style='color: green;'>```WA_WindowPropagation = 80```</span>
  WindowPropagation = 80,
  /// C++ enum variant: <span style='color: green;'>```WA_NoX11EventCompression = 81```</span>
  NoX11EventCompression = 81,
  /// C++ enum variant: <span style='color: green;'>```WA_TintedBackground = 82```</span>
  TintedBackground = 82,
  /// C++ enum variant: <span style='color: green;'>```WA_X11OpenGLOverlay = 83```</span>
  X11OpenGLOverlay = 83,
  /// C++ enum variant: <span style='color: green;'>```WA_AlwaysShowToolTips = 84```</span>
  AlwaysShowToolTips = 84,
  /// C++ enum variant: <span style='color: green;'>```WA_MacOpaqueSizeGrip = 85```</span>
  MacOpaqueSizeGrip = 85,
  /// C++ enum variant: <span style='color: green;'>```WA_SetStyle = 86```</span>
  SetStyle = 86,
  /// C++ enum variant: <span style='color: green;'>```WA_SetLocale = 87```</span>
  SetLocale = 87,
  /// C++ enum variant: <span style='color: green;'>```WA_MacShowFocusRect = 88```</span>
  MacShowFocusRect = 88,
  /// C++ enum variant: <span style='color: green;'>```WA_MacNormalSize = 89```</span>
  MacNormalSize = 89,
  /// C++ enum variant: <span style='color: green;'>```WA_MacSmallSize = 90```</span>
  MacSmallSize = 90,
  /// C++ enum variant: <span style='color: green;'>```WA_MacMiniSize = 91```</span>
  MacMiniSize = 91,
  /// C++ enum variant: <span style='color: green;'>```WA_LayoutUsesWidgetRect = 92```</span>
  LayoutUsesWidgetRect = 92,
  /// C++ enum variant: <span style='color: green;'>```WA_StyledBackground = 93```</span>
  StyledBackground = 93,
  /// C++ enum variant: <span style='color: green;'>```WA_MSWindowsUseDirect3D = 94```</span>
  MSWindowsUseDirect3D = 94,
  /// C++ enum variant: <span style='color: green;'>```WA_CanHostQMdiSubWindowTitleBar = 95```</span>
  CanHostQMdiSubWindowTitleBar = 95,
  /// C++ enum variant: <span style='color: green;'>```WA_MacAlwaysShowToolWindow = 96```</span>
  MacAlwaysShowToolWindow = 96,
  /// C++ enum variant: <span style='color: green;'>```WA_StyleSheet = 97```</span>
  StyleSheet = 97,
  /// C++ enum variant: <span style='color: green;'>```WA_ShowWithoutActivating = 98```</span>
  ShowWithoutActivating = 98,
  /// C++ enum variant: <span style='color: green;'>```WA_X11BypassTransientForHint = 99```</span>
  X11BypassTransientForHint = 99,
  /// C++ enum variant: <span style='color: green;'>```WA_NativeWindow = 100```</span>
  NativeWindow = 100,
  /// C++ enum variant: <span style='color: green;'>```WA_DontCreateNativeAncestors = 101```</span>
  DontCreateNativeAncestors = 101,
  /// C++ enum variant: <span style='color: green;'>```WA_MacVariableSize = 102```</span>
  MacVariableSize = 102,
  /// C++ enum variant: <span style='color: green;'>```WA_DontShowOnScreen = 103```</span>
  DontShowOnScreen = 103,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeDesktop = 104```</span>
  X11NetWmWindowTypeDesktop = 104,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeDock = 105```</span>
  X11NetWmWindowTypeDock = 105,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeToolBar = 106```</span>
  X11NetWmWindowTypeToolBar = 106,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeMenu = 107```</span>
  X11NetWmWindowTypeMenu = 107,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeUtility = 108```</span>
  X11NetWmWindowTypeUtility = 108,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeSplash = 109```</span>
  X11NetWmWindowTypeSplash = 109,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeDialog = 110```</span>
  X11NetWmWindowTypeDialog = 110,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeDropDownMenu = 111```</span>
  X11NetWmWindowTypeDropDownMenu = 111,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypePopupMenu = 112```</span>
  X11NetWmWindowTypePopupMenu = 112,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeToolTip = 113```</span>
  X11NetWmWindowTypeToolTip = 113,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeNotification = 114```</span>
  X11NetWmWindowTypeNotification = 114,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeCombo = 115```</span>
  X11NetWmWindowTypeCombo = 115,
  /// C++ enum variant: <span style='color: green;'>```WA_X11NetWmWindowTypeDND = 116```</span>
  X11NetWmWindowTypeDND = 116,
  /// C++ enum variant: <span style='color: green;'>```WA_MacFrameworkScaled = 117```</span>
  MacFrameworkScaled = 117,
  /// C++ enum variant: <span style='color: green;'>```WA_SetWindowModality = 118```</span>
  SetWindowModality = 118,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_WindowOpacitySet = 119```</span>
  WStateWindowOpacitySet = 119,
  /// C++ enum variant: <span style='color: green;'>```WA_TranslucentBackground = 120```</span>
  TranslucentBackground = 120,
  /// C++ enum variant: <span style='color: green;'>```WA_AcceptTouchEvents = 121```</span>
  AcceptTouchEvents = 121,
  /// C++ enum variant: <span style='color: green;'>```WA_WState_AcceptedTouchBeginEvent = 122```</span>
  WStateAcceptedTouchBeginEvent = 122,
  /// C++ enum variant: <span style='color: green;'>```WA_TouchPadAcceptSingleTouchEvents = 123```</span>
  TouchPadAcceptSingleTouchEvents = 123,
  /// C++ enum variant: <span style='color: green;'>```WA_X11DoNotAcceptFocus = 126```</span>
  X11DoNotAcceptFocus = 126,
  /// C++ enum variant: <span style='color: green;'>```WA_MacNoShadow = 127```</span>
  MacNoShadow = 127,
  /// C++ enum variant: <span style='color: green;'>```WA_AlwaysStackOnTop = 128```</span>
  AlwaysStackOnTop = 128,
  /// C++ enum variant: <span style='color: green;'>```WA_TabletTracking = 129```</span>
  TabletTracking = 129,
  /// C++ enum variant: <span style='color: green;'>```WA_AttributeCount = 130```</span>
  AttributeCount = 130,
}

/// C++ type: <span style='color: green;'>```Qt::WindowFrameSection```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WindowFrameSection {
  /// C++ enum variant: <span style='color: green;'>```NoSection = 0```</span>
  NoSection = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftSection = 1```</span>
  LeftSection = 1,
  /// C++ enum variant: <span style='color: green;'>```TopLeftSection = 2```</span>
  TopLeftSection = 2,
  /// C++ enum variant: <span style='color: green;'>```TopSection = 3```</span>
  TopSection = 3,
  /// C++ enum variant: <span style='color: green;'>```TopRightSection = 4```</span>
  TopRightSection = 4,
  /// C++ enum variant: <span style='color: green;'>```RightSection = 5```</span>
  RightSection = 5,
  /// C++ enum variant: <span style='color: green;'>```BottomRightSection = 6```</span>
  BottomRightSection = 6,
  /// C++ enum variant: <span style='color: green;'>```BottomSection = 7```</span>
  BottomSection = 7,
  /// C++ enum variant: <span style='color: green;'>```BottomLeftSection = 8```</span>
  BottomLeftSection = 8,
  /// C++ enum variant: <span style='color: green;'>```TitleBarArea = 9```</span>
  TitleBarArea = 9,
}

/// C++ type: <span style='color: green;'>```Qt::WindowModality```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WindowModality {
  /// C++ enum variant: <span style='color: green;'>```NonModal = 0```</span>
  Non = 0,
  /// C++ enum variant: <span style='color: green;'>```WindowModal = 1```</span>
  Window = 1,
  /// C++ enum variant: <span style='color: green;'>```ApplicationModal = 2```</span>
  Application = 2,
}

/// C++ type: <span style='color: green;'>```Qt::WindowState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WindowState {
  /// C++ enum variant: <span style='color: green;'>```WindowNoState = 0```</span>
  NoState = 0,
  /// C++ enum variant: <span style='color: green;'>```WindowMinimized = 1```</span>
  Minimized = 1,
  /// C++ enum variant: <span style='color: green;'>```WindowMaximized = 2```</span>
  Maximized = 2,
  /// C++ enum variant: <span style='color: green;'>```WindowFullScreen = 4```</span>
  FullScreen = 4,
  /// C++ enum variant: <span style='color: green;'>```WindowActive = 8```</span>
  Active = 8,
}

impl ::flags::FlaggableEnum for WindowState {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "WindowState"
  }
}

/// C++ type: <span style='color: green;'>```Qt::WindowType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WindowType {
  /// C++ enum variant: <span style='color: green;'>```WindowFullscreenButtonHint = -2147483648```</span>
  WindowFullscreenButtonHint = -2147483648,
  /// C++ enum variant: <span style='color: green;'>```Widget = 0```</span>
  Widget = 0,
  /// C++ enum variant: <span style='color: green;'>```Window = 1```</span>
  Window = 1,
  /// C++ enum variant: <span style='color: green;'>```Dialog = 3```</span>
  Dialog = 3,
  /// C++ enum variant: <span style='color: green;'>```Sheet = 5```</span>
  Sheet = 5,
  /// C++ enum variant: <span style='color: green;'>```Drawer = 7```</span>
  Drawer = 7,
  /// C++ enum variant: <span style='color: green;'>```Popup = 9```</span>
  Popup = 9,
  /// C++ enum variant: <span style='color: green;'>```Tool = 11```</span>
  Tool = 11,
  /// C++ enum variant: <span style='color: green;'>```ToolTip = 13```</span>
  ToolTip = 13,
  /// C++ enum variant: <span style='color: green;'>```SplashScreen = 15```</span>
  SplashScreen = 15,
  /// C++ enum variant: <span style='color: green;'>```Desktop = 17```</span>
  Desktop = 17,
  /// C++ enum variant: <span style='color: green;'>```SubWindow = 18```</span>
  SubWindow = 18,
  /// C++ enum variant: <span style='color: green;'>```ForeignWindow = 33```</span>
  ForeignWindow = 33,
  /// C++ enum variant: <span style='color: green;'>```CoverWindow = 65```</span>
  CoverWindow = 65,
  /// C++ enum variant: <span style='color: green;'>```WindowType_Mask = 255```</span>
  WindowTypeMask = 255,
  /// C++ enum variant: <span style='color: green;'>```MSWindowsFixedSizeDialogHint = 256```</span>
  MSWindowsFixedSizeDialogHint = 256,
  /// C++ enum variant: <span style='color: green;'>```MSWindowsOwnDC = 512```</span>
  MSWindowsOwnDC = 512,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```BypassWindowManagerHint = 1024```</span>
  /// - <span style='color: green;'>```X11BypassWindowManagerHint = 1024```</span>
  ///
  BypassWindowManagerHint = 1024,
  /// C++ enum variant: <span style='color: green;'>```FramelessWindowHint = 2048```</span>
  FramelessWindowHint = 2048,
  /// C++ enum variant: <span style='color: green;'>```WindowTitleHint = 4096```</span>
  WindowTitleHint = 4096,
  /// C++ enum variant: <span style='color: green;'>```WindowSystemMenuHint = 8192```</span>
  WindowSystemMenuHint = 8192,
  /// C++ enum variant: <span style='color: green;'>```WindowMinimizeButtonHint = 16384```</span>
  WindowMinimizeButtonHint = 16384,
  /// C++ enum variant: <span style='color: green;'>```WindowMaximizeButtonHint = 32768```</span>
  WindowMaximizeButtonHint = 32768,
  /// C++ enum variant: <span style='color: green;'>```WindowMinMaxButtonsHint = 49152```</span>
  WindowMinMaxButtonsHint = 49152,
  /// C++ enum variant: <span style='color: green;'>```WindowContextHelpButtonHint = 65536```</span>
  WindowContextHelpButtonHint = 65536,
  /// C++ enum variant: <span style='color: green;'>```WindowShadeButtonHint = 131072```</span>
  WindowShadeButtonHint = 131072,
  /// C++ enum variant: <span style='color: green;'>```WindowStaysOnTopHint = 262144```</span>
  WindowStaysOnTopHint = 262144,
  /// C++ enum variant: <span style='color: green;'>```WindowTransparentForInput = 524288```</span>
  WindowTransparentForInput = 524288,
  /// C++ enum variant: <span style='color: green;'>```WindowOverridesSystemGestures = 1048576```</span>
  WindowOverridesSystemGestures = 1048576,
  /// C++ enum variant: <span style='color: green;'>```WindowDoesNotAcceptFocus = 2097152```</span>
  WindowDoesNotAcceptFocus = 2097152,
  /// C++ enum variant: <span style='color: green;'>```MaximizeUsingFullscreenGeometryHint = 4194304```</span>
  MaximizeUsingFullscreenGeometryHint = 4194304,
  /// C++ enum variant: <span style='color: green;'>```CustomizeWindowHint = 33554432```</span>
  CustomizeWindowHint = 33554432,
  /// C++ enum variant: <span style='color: green;'>```WindowStaysOnBottomHint = 67108864```</span>
  WindowStaysOnBottomHint = 67108864,
  /// C++ enum variant: <span style='color: green;'>```WindowCloseButtonHint = 134217728```</span>
  WindowCloseButtonHint = 134217728,
  /// C++ enum variant: <span style='color: green;'>```MacWindowToolBarButtonHint = 268435456```</span>
  MacWindowToolBarButtonHint = 268435456,
  /// C++ enum variant: <span style='color: green;'>```BypassGraphicsProxyWidget = 536870912```</span>
  BypassGraphicsProxyWidget = 536870912,
  /// C++ enum variant: <span style='color: green;'>```NoDropShadowWindowHint = 1073741824```</span>
  NoDropShadowWindowHint = 1073741824,
}

impl ::flags::FlaggableEnum for WindowType {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "WindowType"
  }
}
