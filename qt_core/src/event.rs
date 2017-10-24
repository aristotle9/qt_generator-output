/// C++ type: <span style='color: green;'>```QEvent```</span>
#[repr(C)]
pub struct Event(u8);

impl Event {
  /// C++ method: <span style='color: green;'>```void QEvent::accept()```</span>
  ///
  ///
  pub fn accept(&mut self) {
    unsafe { ::ffi::qt_core_c_QEvent_accept(self as *mut ::event::Event) }
  }

  /// C++ method: <span style='color: green;'>```void QEvent::ignore()```</span>
  ///
  ///
  pub fn ignore(&mut self) {
    unsafe { ::ffi::qt_core_c_QEvent_ignore(self as *mut ::event::Event) }
  }

  /// C++ method: <span style='color: green;'>```bool QEvent::isAccepted() const```</span>
  ///
  ///
  pub fn is_accepted(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QEvent_isAccepted(self as *const ::event::Event) }
  }

  /// C++ method: <span style='color: green;'>```QEvent::QEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(::event::Type) -> ::cpp_utils::CppBox<::event::Event>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEvent::QEvent(QEvent::Type type)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::event::Event) -> ::cpp_utils::CppBox<::event::Event>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEvent::QEvent(const QEvent& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::event::Event>
    where Args: overloading::EventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QEvent& QEvent::operator=(const QEvent& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::event::Event) -> &'l0 mut ::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QEvent_operator_assign(self as *mut ::event::Event, other as *const ::event::Event) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QEvent::registerEventType```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn register_event_type(()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QEvent::registerEventType()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn register_event_type(::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QEvent::registerEventType(int hint = ?)```</span>
  ///
  ///
  pub fn register_event_type<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::EventRegisterEventTypeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QEvent::setAccepted(bool accepted)```</span>
  ///
  ///
  pub fn set_accepted(&mut self, accepted: bool) {
    unsafe { ::ffi::qt_core_c_QEvent_setAccepted(self as *mut ::event::Event, accepted) }
  }

  /// C++ method: <span style='color: green;'>```bool QEvent::spontaneous() const```</span>
  ///
  ///
  pub fn spontaneous(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QEvent_spontaneous(self as *const ::event::Event) }
  }

  /// C++ method: <span style='color: green;'>```QEvent::Type QEvent::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::event::Type {
    unsafe { ::ffi::qt_core_c_QEvent_type(self as *const ::event::Event) }
  }
}

impl ::cpp_utils::CppDeletable for ::event::Event {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QEvent::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Timer = 1```</span>
  Timer = 1,
  /// C++ enum variant: <span style='color: green;'>```MouseButtonPress = 2```</span>
  MouseButtonPress = 2,
  /// C++ enum variant: <span style='color: green;'>```MouseButtonRelease = 3```</span>
  MouseButtonRelease = 3,
  /// C++ enum variant: <span style='color: green;'>```MouseButtonDblClick = 4```</span>
  MouseButtonDblClick = 4,
  /// C++ enum variant: <span style='color: green;'>```MouseMove = 5```</span>
  MouseMove = 5,
  /// C++ enum variant: <span style='color: green;'>```KeyPress = 6```</span>
  KeyPress = 6,
  /// C++ enum variant: <span style='color: green;'>```KeyRelease = 7```</span>
  KeyRelease = 7,
  /// C++ enum variant: <span style='color: green;'>```FocusIn = 8```</span>
  FocusIn = 8,
  /// C++ enum variant: <span style='color: green;'>```FocusOut = 9```</span>
  FocusOut = 9,
  /// C++ enum variant: <span style='color: green;'>```Enter = 10```</span>
  Enter = 10,
  /// C++ enum variant: <span style='color: green;'>```Leave = 11```</span>
  Leave = 11,
  /// C++ enum variant: <span style='color: green;'>```Paint = 12```</span>
  Paint = 12,
  /// C++ enum variant: <span style='color: green;'>```Move = 13```</span>
  Move = 13,
  /// C++ enum variant: <span style='color: green;'>```Resize = 14```</span>
  Resize = 14,
  /// C++ enum variant: <span style='color: green;'>```Create = 15```</span>
  Create = 15,
  /// C++ enum variant: <span style='color: green;'>```Destroy = 16```</span>
  Destroy = 16,
  /// C++ enum variant: <span style='color: green;'>```Show = 17```</span>
  Show = 17,
  /// C++ enum variant: <span style='color: green;'>```Hide = 18```</span>
  Hide = 18,
  /// C++ enum variant: <span style='color: green;'>```Close = 19```</span>
  Close = 19,
  /// C++ enum variant: <span style='color: green;'>```Quit = 20```</span>
  Quit = 20,
  /// C++ enum variant: <span style='color: green;'>```ParentChange = 21```</span>
  ParentChange = 21,
  /// C++ enum variant: <span style='color: green;'>```ThreadChange = 22```</span>
  ThreadChange = 22,
  /// C++ enum variant: <span style='color: green;'>```FocusAboutToChange = 23```</span>
  FocusAboutToChange = 23,
  /// C++ enum variant: <span style='color: green;'>```WindowActivate = 24```</span>
  WindowActivate = 24,
  /// C++ enum variant: <span style='color: green;'>```WindowDeactivate = 25```</span>
  WindowDeactivate = 25,
  /// C++ enum variant: <span style='color: green;'>```ShowToParent = 26```</span>
  ShowToParent = 26,
  /// C++ enum variant: <span style='color: green;'>```HideToParent = 27```</span>
  HideToParent = 27,
  /// C++ enum variant: <span style='color: green;'>```Wheel = 31```</span>
  Wheel = 31,
  /// C++ enum variant: <span style='color: green;'>```WindowTitleChange = 33```</span>
  WindowTitleChange = 33,
  /// C++ enum variant: <span style='color: green;'>```WindowIconChange = 34```</span>
  WindowIconChange = 34,
  /// C++ enum variant: <span style='color: green;'>```ApplicationWindowIconChange = 35```</span>
  ApplicationWindowIconChange = 35,
  /// C++ enum variant: <span style='color: green;'>```ApplicationFontChange = 36```</span>
  ApplicationFontChange = 36,
  /// C++ enum variant: <span style='color: green;'>```ApplicationLayoutDirectionChange = 37```</span>
  ApplicationLayoutDirectionChange = 37,
  /// C++ enum variant: <span style='color: green;'>```ApplicationPaletteChange = 38```</span>
  ApplicationPaletteChange = 38,
  /// C++ enum variant: <span style='color: green;'>```PaletteChange = 39```</span>
  PaletteChange = 39,
  /// C++ enum variant: <span style='color: green;'>```Clipboard = 40```</span>
  Clipboard = 40,
  /// C++ enum variant: <span style='color: green;'>```Speech = 42```</span>
  Speech = 42,
  /// C++ enum variant: <span style='color: green;'>```MetaCall = 43```</span>
  MetaCall = 43,
  /// C++ enum variant: <span style='color: green;'>```SockAct = 50```</span>
  SockAct = 50,
  /// C++ enum variant: <span style='color: green;'>```ShortcutOverride = 51```</span>
  ShortcutOverride = 51,
  /// C++ enum variant: <span style='color: green;'>```DeferredDelete = 52```</span>
  DeferredDelete = 52,
  /// C++ enum variant: <span style='color: green;'>```DragEnter = 60```</span>
  DragEnter = 60,
  /// C++ enum variant: <span style='color: green;'>```DragMove = 61```</span>
  DragMove = 61,
  /// C++ enum variant: <span style='color: green;'>```DragLeave = 62```</span>
  DragLeave = 62,
  /// C++ enum variant: <span style='color: green;'>```Drop = 63```</span>
  Drop = 63,
  /// C++ enum variant: <span style='color: green;'>```DragResponse = 64```</span>
  DragResponse = 64,
  /// C++ enum variant: <span style='color: green;'>```ChildAdded = 68```</span>
  ChildAdded = 68,
  /// C++ enum variant: <span style='color: green;'>```ChildPolished = 69```</span>
  ChildPolished = 69,
  /// C++ enum variant: <span style='color: green;'>```ChildRemoved = 71```</span>
  ChildRemoved = 71,
  /// C++ enum variant: <span style='color: green;'>```ShowWindowRequest = 73```</span>
  ShowWindowRequest = 73,
  /// C++ enum variant: <span style='color: green;'>```PolishRequest = 74```</span>
  PolishRequest = 74,
  /// C++ enum variant: <span style='color: green;'>```Polish = 75```</span>
  Polish = 75,
  /// C++ enum variant: <span style='color: green;'>```LayoutRequest = 76```</span>
  LayoutRequest = 76,
  /// C++ enum variant: <span style='color: green;'>```UpdateRequest = 77```</span>
  UpdateRequest = 77,
  /// C++ enum variant: <span style='color: green;'>```UpdateLater = 78```</span>
  UpdateLater = 78,
  /// C++ enum variant: <span style='color: green;'>```EmbeddingControl = 79```</span>
  EmbeddingControl = 79,
  /// C++ enum variant: <span style='color: green;'>```ActivateControl = 80```</span>
  ActivateControl = 80,
  /// C++ enum variant: <span style='color: green;'>```DeactivateControl = 81```</span>
  DeactivateControl = 81,
  /// C++ enum variant: <span style='color: green;'>```ContextMenu = 82```</span>
  ContextMenu = 82,
  /// C++ enum variant: <span style='color: green;'>```InputMethod = 83```</span>
  InputMethod = 83,
  /// C++ enum variant: <span style='color: green;'>```TabletMove = 87```</span>
  TabletMove = 87,
  /// C++ enum variant: <span style='color: green;'>```LocaleChange = 88```</span>
  LocaleChange = 88,
  /// C++ enum variant: <span style='color: green;'>```LanguageChange = 89```</span>
  LanguageChange = 89,
  /// C++ enum variant: <span style='color: green;'>```LayoutDirectionChange = 90```</span>
  LayoutDirectionChange = 90,
  /// C++ enum variant: <span style='color: green;'>```Style = 91```</span>
  Style = 91,
  /// C++ enum variant: <span style='color: green;'>```TabletPress = 92```</span>
  TabletPress = 92,
  /// C++ enum variant: <span style='color: green;'>```TabletRelease = 93```</span>
  TabletRelease = 93,
  /// C++ enum variant: <span style='color: green;'>```OkRequest = 94```</span>
  OkRequest = 94,
  /// C++ enum variant: <span style='color: green;'>```HelpRequest = 95```</span>
  HelpRequest = 95,
  /// C++ enum variant: <span style='color: green;'>```IconDrag = 96```</span>
  IconDrag = 96,
  /// C++ enum variant: <span style='color: green;'>```FontChange = 97```</span>
  FontChange = 97,
  /// C++ enum variant: <span style='color: green;'>```EnabledChange = 98```</span>
  EnabledChange = 98,
  /// C++ enum variant: <span style='color: green;'>```ActivationChange = 99```</span>
  ActivationChange = 99,
  /// C++ enum variant: <span style='color: green;'>```StyleChange = 100```</span>
  StyleChange = 100,
  /// C++ enum variant: <span style='color: green;'>```IconTextChange = 101```</span>
  IconTextChange = 101,
  /// C++ enum variant: <span style='color: green;'>```ModifiedChange = 102```</span>
  ModifiedChange = 102,
  /// C++ enum variant: <span style='color: green;'>```WindowBlocked = 103```</span>
  WindowBlocked = 103,
  /// C++ enum variant: <span style='color: green;'>```WindowUnblocked = 104```</span>
  WindowUnblocked = 104,
  /// C++ enum variant: <span style='color: green;'>```WindowStateChange = 105```</span>
  WindowStateChange = 105,
  /// C++ enum variant: <span style='color: green;'>```ReadOnlyChange = 106```</span>
  ReadOnlyChange = 106,
  /// C++ enum variant: <span style='color: green;'>```MouseTrackingChange = 109```</span>
  MouseTrackingChange = 109,
  /// C++ enum variant: <span style='color: green;'>```ToolTip = 110```</span>
  ToolTip = 110,
  /// C++ enum variant: <span style='color: green;'>```WhatsThis = 111```</span>
  WhatsThis = 111,
  /// C++ enum variant: <span style='color: green;'>```StatusTip = 112```</span>
  StatusTip = 112,
  /// C++ enum variant: <span style='color: green;'>```ActionChanged = 113```</span>
  ActionChanged = 113,
  /// C++ enum variant: <span style='color: green;'>```ActionAdded = 114```</span>
  ActionAdded = 114,
  /// C++ enum variant: <span style='color: green;'>```ActionRemoved = 115```</span>
  ActionRemoved = 115,
  /// C++ enum variant: <span style='color: green;'>```FileOpen = 116```</span>
  FileOpen = 116,
  /// C++ enum variant: <span style='color: green;'>```Shortcut = 117```</span>
  Shortcut = 117,
  /// C++ enum variant: <span style='color: green;'>```WhatsThisClicked = 118```</span>
  WhatsThisClicked = 118,
  /// C++ enum variant: <span style='color: green;'>```ToolBarChange = 120```</span>
  ToolBarChange = 120,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ApplicationActivate = 121```</span>
  /// - <span style='color: green;'>```ApplicationActivated = 121```</span>
  ///
  ApplicationActivate = 121,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```ApplicationDeactivate = 122```</span>
  /// - <span style='color: green;'>```ApplicationDeactivated = 122```</span>
  ///
  ApplicationDeactivate = 122,
  /// C++ enum variant: <span style='color: green;'>```QueryWhatsThis = 123```</span>
  QueryWhatsThis = 123,
  /// C++ enum variant: <span style='color: green;'>```EnterWhatsThisMode = 124```</span>
  EnterWhatsThisMode = 124,
  /// C++ enum variant: <span style='color: green;'>```LeaveWhatsThisMode = 125```</span>
  LeaveWhatsThisMode = 125,
  /// C++ enum variant: <span style='color: green;'>```ZOrderChange = 126```</span>
  ZOrderChange = 126,
  /// C++ enum variant: <span style='color: green;'>```HoverEnter = 127```</span>
  HoverEnter = 127,
  /// C++ enum variant: <span style='color: green;'>```HoverLeave = 128```</span>
  HoverLeave = 128,
  /// C++ enum variant: <span style='color: green;'>```HoverMove = 129```</span>
  HoverMove = 129,
  /// C++ enum variant: <span style='color: green;'>```ParentAboutToChange = 131```</span>
  ParentAboutToChange = 131,
  /// C++ enum variant: <span style='color: green;'>```WinEventAct = 132```</span>
  WinEventAct = 132,
  /// C++ enum variant: <span style='color: green;'>```AcceptDropsChange = 152```</span>
  AcceptDropsChange = 152,
  /// C++ enum variant: <span style='color: green;'>```ZeroTimerEvent = 154```</span>
  ZeroTimerEvent = 154,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneMouseMove = 155```</span>
  GraphicsSceneMouseMove = 155,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneMousePress = 156```</span>
  GraphicsSceneMousePress = 156,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneMouseRelease = 157```</span>
  GraphicsSceneMouseRelease = 157,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneMouseDoubleClick = 158```</span>
  GraphicsSceneMouseDoubleClick = 158,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneContextMenu = 159```</span>
  GraphicsSceneContextMenu = 159,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneHoverEnter = 160```</span>
  GraphicsSceneHoverEnter = 160,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneHoverMove = 161```</span>
  GraphicsSceneHoverMove = 161,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneHoverLeave = 162```</span>
  GraphicsSceneHoverLeave = 162,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneHelp = 163```</span>
  GraphicsSceneHelp = 163,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneDragEnter = 164```</span>
  GraphicsSceneDragEnter = 164,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneDragMove = 165```</span>
  GraphicsSceneDragMove = 165,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneDragLeave = 166```</span>
  GraphicsSceneDragLeave = 166,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneDrop = 167```</span>
  GraphicsSceneDrop = 167,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneWheel = 168```</span>
  GraphicsSceneWheel = 168,
  /// C++ enum variant: <span style='color: green;'>```KeyboardLayoutChange = 169```</span>
  KeyboardLayoutChange = 169,
  /// C++ enum variant: <span style='color: green;'>```DynamicPropertyChange = 170```</span>
  DynamicPropertyChange = 170,
  /// C++ enum variant: <span style='color: green;'>```TabletEnterProximity = 171```</span>
  TabletEnterProximity = 171,
  /// C++ enum variant: <span style='color: green;'>```TabletLeaveProximity = 172```</span>
  TabletLeaveProximity = 172,
  /// C++ enum variant: <span style='color: green;'>```NonClientAreaMouseMove = 173```</span>
  NonClientAreaMouseMove = 173,
  /// C++ enum variant: <span style='color: green;'>```NonClientAreaMouseButtonPress = 174```</span>
  NonClientAreaMouseButtonPress = 174,
  /// C++ enum variant: <span style='color: green;'>```NonClientAreaMouseButtonRelease = 175```</span>
  NonClientAreaMouseButtonRelease = 175,
  /// C++ enum variant: <span style='color: green;'>```NonClientAreaMouseButtonDblClick = 176```</span>
  NonClientAreaMouseButtonDblClick = 176,
  /// C++ enum variant: <span style='color: green;'>```MacSizeChange = 177```</span>
  MacSizeChange = 177,
  /// C++ enum variant: <span style='color: green;'>```ContentsRectChange = 178```</span>
  ContentsRectChange = 178,
  /// C++ enum variant: <span style='color: green;'>```MacGLWindowChange = 179```</span>
  MacGLWindowChange = 179,
  /// C++ enum variant: <span style='color: green;'>```FutureCallOut = 180```</span>
  FutureCallOut = 180,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneResize = 181```</span>
  GraphicsSceneResize = 181,
  /// C++ enum variant: <span style='color: green;'>```GraphicsSceneMove = 182```</span>
  GraphicsSceneMove = 182,
  /// C++ enum variant: <span style='color: green;'>```CursorChange = 183```</span>
  CursorChange = 183,
  /// C++ enum variant: <span style='color: green;'>```ToolTipChange = 184```</span>
  ToolTipChange = 184,
  /// C++ enum variant: <span style='color: green;'>```NetworkReplyUpdated = 185```</span>
  NetworkReplyUpdated = 185,
  /// C++ enum variant: <span style='color: green;'>```GrabMouse = 186```</span>
  GrabMouse = 186,
  /// C++ enum variant: <span style='color: green;'>```UngrabMouse = 187```</span>
  UngrabMouse = 187,
  /// C++ enum variant: <span style='color: green;'>```GrabKeyboard = 188```</span>
  GrabKeyboard = 188,
  /// C++ enum variant: <span style='color: green;'>```UngrabKeyboard = 189```</span>
  UngrabKeyboard = 189,
  /// C++ enum variant: <span style='color: green;'>```MacGLClearDrawable = 191```</span>
  MacGLClearDrawable = 191,
  /// C++ enum variant: <span style='color: green;'>```StateMachineSignal = 192```</span>
  StateMachineSignal = 192,
  /// C++ enum variant: <span style='color: green;'>```StateMachineWrapped = 193```</span>
  StateMachineWrapped = 193,
  /// C++ enum variant: <span style='color: green;'>```TouchBegin = 194```</span>
  TouchBegin = 194,
  /// C++ enum variant: <span style='color: green;'>```TouchUpdate = 195```</span>
  TouchUpdate = 195,
  /// C++ enum variant: <span style='color: green;'>```TouchEnd = 196```</span>
  TouchEnd = 196,
  /// C++ enum variant: <span style='color: green;'>```NativeGesture = 197```</span>
  NativeGesture = 197,
  /// C++ enum variant: <span style='color: green;'>```Gesture = 198```</span>
  Gesture = 198,
  /// C++ enum variant: <span style='color: green;'>```RequestSoftwareInputPanel = 199```</span>
  RequestSoftwareInputPanel = 199,
  /// C++ enum variant: <span style='color: green;'>```CloseSoftwareInputPanel = 200```</span>
  CloseSoftwareInputPanel = 200,
  /// C++ enum variant: <span style='color: green;'>```GestureOverride = 202```</span>
  GestureOverride = 202,
  /// C++ enum variant: <span style='color: green;'>```WinIdChange = 203```</span>
  WinIdChange = 203,
  /// C++ enum variant: <span style='color: green;'>```ScrollPrepare = 204```</span>
  ScrollPrepare = 204,
  /// C++ enum variant: <span style='color: green;'>```Scroll = 205```</span>
  Scroll = 205,
  /// C++ enum variant: <span style='color: green;'>```Expose = 206```</span>
  Expose = 206,
  /// C++ enum variant: <span style='color: green;'>```InputMethodQuery = 207```</span>
  InputMethodQuery = 207,
  /// C++ enum variant: <span style='color: green;'>```OrientationChange = 208```</span>
  OrientationChange = 208,
  /// C++ enum variant: <span style='color: green;'>```TouchCancel = 209```</span>
  TouchCancel = 209,
  /// C++ enum variant: <span style='color: green;'>```ThemeChange = 210```</span>
  ThemeChange = 210,
  /// C++ enum variant: <span style='color: green;'>```SockClose = 211```</span>
  SockClose = 211,
  /// C++ enum variant: <span style='color: green;'>```PlatformPanel = 212```</span>
  PlatformPanel = 212,
  /// C++ enum variant: <span style='color: green;'>```StyleAnimationUpdate = 213```</span>
  StyleAnimationUpdate = 213,
  /// C++ enum variant: <span style='color: green;'>```ApplicationStateChange = 214```</span>
  ApplicationStateChange = 214,
  /// C++ enum variant: <span style='color: green;'>```WindowChangeInternal = 215```</span>
  WindowChangeInternal = 215,
  /// C++ enum variant: <span style='color: green;'>```ScreenChangeInternal = 216```</span>
  ScreenChangeInternal = 216,
  /// C++ enum variant: <span style='color: green;'>```PlatformSurface = 217```</span>
  PlatformSurface = 217,
  /// C++ enum variant: <span style='color: green;'>```Pointer = 218```</span>
  Pointer = 218,
  /// C++ enum variant: <span style='color: green;'>```TabletTrackingChange = 219```</span>
  TabletTrackingChange = 219,
  /// C++ enum variant: <span style='color: green;'>```User = 1000```</span>
  User = 1000,
  /// C++ enum variant: <span style='color: green;'>```MaxUser = 65535```</span>
  MaxUser = 65535,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Event::new](../struct.Event.html#method.new) method.
  pub trait EventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::event::Event>;
  }
  impl<'a> EventNewArgs for &'a ::event::Event {
    fn exec(self) -> ::cpp_utils::CppBox<::event::Event> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QEvent_new_other(other as *const ::event::Event) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl EventNewArgs for ::event::Type {
    fn exec(self) -> ::cpp_utils::CppBox<::event::Event> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QEvent_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Event::register_event_type](../struct.Event.html#method.register_event_type) method.
  pub trait EventRegisterEventTypeArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl EventRegisterEventTypeArgs for ::libc::c_int {
    fn exec(self) -> ::libc::c_int {
      let hint = self;
      unsafe { ::ffi::qt_core_c_QEvent_registerEventType_hint(hint) }
    }
  }
  impl EventRegisterEventTypeArgs for () {
    fn exec(self) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QEvent_registerEventType_no_args() }
    }
  }
}
