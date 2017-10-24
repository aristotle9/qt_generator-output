/// C++ type: <span style='color: green;'>```QAccessible```</span>
#[repr(C)]
pub struct Accessible(u8);

impl Accessible {
  /// C++ method: <span style='color: green;'>```static QAccessibleInterface* QAccessible::accessibleInterface(unsigned int uniqueId)```</span>
  ///
  ///
  pub fn accessible_interface(unique_id: ::libc::c_uint) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessible_accessibleInterface(unique_id) }
  }

  /// C++ method: <span style='color: green;'>```static void QAccessible::cleanup()```</span>
  ///
  ///
  pub fn cleanup() {
    unsafe { ::ffi::qt_gui_c_QAccessible_cleanup() }
  }

  /// C++ method: <span style='color: green;'>```static void QAccessible::deleteAccessibleInterface(unsigned int uniqueId)```</span>
  ///
  ///
  pub fn delete_accessible_interface(unique_id: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QAccessible_deleteAccessibleInterface(unique_id) }
  }

  /// C++ method: <span style='color: green;'>```static void (*FN_PTR)(QObject*) QAccessible::installRootObjectHandler(void (*FN_PTR)(QObject*) arg1)```</span>
  ///
  ///
  pub unsafe fn install_root_object_handler(arg1: extern "C" fn(*mut ::qt_core::object::Object))
                                            -> extern "C" fn(*mut ::qt_core::object::Object) {
    ::ffi::qt_gui_c_QAccessible_installRootObjectHandler(arg1)
  }

  /// C++ method: <span style='color: green;'>```static void (*FN_PTR)(QAccessibleEvent*) QAccessible::installUpdateHandler(void (*FN_PTR)(QAccessibleEvent*) arg1)```</span>
  ///
  ///
  pub unsafe fn install_update_handler(arg1: extern "C" fn(*mut ::accessible_event::AccessibleEvent))
                                       -> extern "C" fn(*mut ::accessible_event::AccessibleEvent) {
    ::ffi::qt_gui_c_QAccessible_installUpdateHandler(arg1)
  }

  /// C++ method: <span style='color: green;'>```static bool QAccessible::isActive()```</span>
  ///
  ///
  pub fn is_active() -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessible_isActive() }
  }

  /// C++ method: <span style='color: green;'>```static QPair<int, int> QAccessible::qAccessibleTextBoundaryHelper(const QTextCursor& cursor, QAccessible::TextBoundaryType boundaryType)```</span>
  ///
  ///
  pub fn q_accessible_text_boundary_helper(cursor: &::text_cursor::TextCursor,
                                           boundary_type: ::accessible::TextBoundaryType)
                                           -> ::pair::PairCIntCInt {
    {
      let mut object: ::pair::PairCIntCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessible_qAccessibleTextBoundaryHelper_to_output(cursor as *const ::text_cursor::TextCursor, boundary_type, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QAccessibleInterface* QAccessible::queryAccessibleInterface(QObject* arg1)```</span>
  ///
  ///
  pub unsafe fn query_accessible_interface(arg1: *mut ::qt_core::object::Object)
                                           -> *mut ::accessible_interface::AccessibleInterface {
    ::ffi::qt_gui_c_QAccessible_queryAccessibleInterface(arg1)
  }

  /// C++ method: <span style='color: green;'>```static unsigned int QAccessible::registerAccessibleInterface(QAccessibleInterface* iface)```</span>
  ///
  ///
  pub unsafe fn register_accessible_interface(iface: *mut ::accessible_interface::AccessibleInterface)
                                              -> ::libc::c_uint {
    ::ffi::qt_gui_c_QAccessible_registerAccessibleInterface(iface)
  }

  /// C++ method: <span style='color: green;'>```static void QAccessible::setActive(bool active)```</span>
  ///
  ///
  pub fn set_active(active: bool) {
    unsafe { ::ffi::qt_gui_c_QAccessible_setActive(active) }
  }

  /// C++ method: <span style='color: green;'>```static void QAccessible::setRootObject(QObject* object)```</span>
  ///
  ///
  pub unsafe fn set_root_object(object: *mut ::qt_core::object::Object) {
    ::ffi::qt_gui_c_QAccessible_setRootObject(object)
  }

  /// C++ method: <span style='color: green;'>```static unsigned int QAccessible::uniqueId(QAccessibleInterface* iface)```</span>
  ///
  ///
  pub unsafe fn unique_id(iface: *mut ::accessible_interface::AccessibleInterface) -> ::libc::c_uint {
    ::ffi::qt_gui_c_QAccessible_uniqueId(iface)
  }

  /// C++ method: <span style='color: green;'>```static void QAccessible::updateAccessibility(QAccessibleEvent* event)```</span>
  ///
  ///
  pub unsafe fn update_accessibility(event: *mut ::accessible_event::AccessibleEvent) {
    ::ffi::qt_gui_c_QAccessible_updateAccessibility(event)
  }
}

impl ::cpp_utils::CppDeletable for ::accessible::Accessible {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessible_delete
  }
}

/// C++ type: <span style='color: green;'>```QAccessible::Event```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Event {
  /// C++ enum variant: <span style='color: green;'>```SoundPlayed = 1```</span>
  SoundPlayed = 1,
  /// C++ enum variant: <span style='color: green;'>```Alert = 2```</span>
  Alert = 2,
  /// C++ enum variant: <span style='color: green;'>```ForegroundChanged = 3```</span>
  ForegroundChanged = 3,
  /// C++ enum variant: <span style='color: green;'>```MenuStart = 4```</span>
  MenuStart = 4,
  /// C++ enum variant: <span style='color: green;'>```MenuEnd = 5```</span>
  MenuEnd = 5,
  /// C++ enum variant: <span style='color: green;'>```PopupMenuStart = 6```</span>
  PopupMenuStart = 6,
  /// C++ enum variant: <span style='color: green;'>```PopupMenuEnd = 7```</span>
  PopupMenuEnd = 7,
  /// C++ enum variant: <span style='color: green;'>```ContextHelpStart = 12```</span>
  ContextHelpStart = 12,
  /// C++ enum variant: <span style='color: green;'>```ContextHelpEnd = 13```</span>
  ContextHelpEnd = 13,
  /// C++ enum variant: <span style='color: green;'>```DragDropStart = 14```</span>
  DragDropStart = 14,
  /// C++ enum variant: <span style='color: green;'>```DragDropEnd = 15```</span>
  DragDropEnd = 15,
  /// C++ enum variant: <span style='color: green;'>```DialogStart = 16```</span>
  DialogStart = 16,
  /// C++ enum variant: <span style='color: green;'>```DialogEnd = 17```</span>
  DialogEnd = 17,
  /// C++ enum variant: <span style='color: green;'>```ScrollingStart = 18```</span>
  ScrollingStart = 18,
  /// C++ enum variant: <span style='color: green;'>```ScrollingEnd = 19```</span>
  ScrollingEnd = 19,
  /// C++ enum variant: <span style='color: green;'>```MenuCommand = 24```</span>
  MenuCommand = 24,
  /// C++ enum variant: <span style='color: green;'>```ActionChanged = 257```</span>
  ActionChanged = 257,
  /// C++ enum variant: <span style='color: green;'>```ActiveDescendantChanged = 258```</span>
  ActiveDescendantChanged = 258,
  /// C++ enum variant: <span style='color: green;'>```AttributeChanged = 259```</span>
  AttributeChanged = 259,
  /// C++ enum variant: <span style='color: green;'>```DocumentContentChanged = 260```</span>
  DocumentContentChanged = 260,
  /// C++ enum variant: <span style='color: green;'>```DocumentLoadComplete = 261```</span>
  DocumentLoadComplete = 261,
  /// C++ enum variant: <span style='color: green;'>```DocumentLoadStopped = 262```</span>
  DocumentLoadStopped = 262,
  /// C++ enum variant: <span style='color: green;'>```DocumentReload = 263```</span>
  DocumentReload = 263,
  /// C++ enum variant: <span style='color: green;'>```HyperlinkEndIndexChanged = 264```</span>
  HyperlinkEndIndexChanged = 264,
  /// C++ enum variant: <span style='color: green;'>```HyperlinkNumberOfAnchorsChanged = 265```</span>
  HyperlinkNumberOfAnchorsChanged = 265,
  /// C++ enum variant: <span style='color: green;'>```HyperlinkSelectedLinkChanged = 266```</span>
  HyperlinkSelectedLinkChanged = 266,
  /// C++ enum variant: <span style='color: green;'>```HypertextLinkActivated = 267```</span>
  HypertextLinkActivated = 267,
  /// C++ enum variant: <span style='color: green;'>```HypertextLinkSelected = 268```</span>
  HypertextLinkSelected = 268,
  /// C++ enum variant: <span style='color: green;'>```HyperlinkStartIndexChanged = 269```</span>
  HyperlinkStartIndexChanged = 269,
  /// C++ enum variant: <span style='color: green;'>```HypertextChanged = 270```</span>
  HypertextChanged = 270,
  /// C++ enum variant: <span style='color: green;'>```HypertextNLinksChanged = 271```</span>
  HypertextNLinksChanged = 271,
  /// C++ enum variant: <span style='color: green;'>```ObjectAttributeChanged = 272```</span>
  ObjectAttributeChanged = 272,
  /// C++ enum variant: <span style='color: green;'>```PageChanged = 273```</span>
  PageChanged = 273,
  /// C++ enum variant: <span style='color: green;'>```SectionChanged = 274```</span>
  SectionChanged = 274,
  /// C++ enum variant: <span style='color: green;'>```TableCaptionChanged = 275```</span>
  TableCaptionChanged = 275,
  /// C++ enum variant: <span style='color: green;'>```TableColumnDescriptionChanged = 276```</span>
  TableColumnDescriptionChanged = 276,
  /// C++ enum variant: <span style='color: green;'>```TableColumnHeaderChanged = 277```</span>
  TableColumnHeaderChanged = 277,
  /// C++ enum variant: <span style='color: green;'>```TableModelChanged = 278```</span>
  TableModelChanged = 278,
  /// C++ enum variant: <span style='color: green;'>```TableRowDescriptionChanged = 279```</span>
  TableRowDescriptionChanged = 279,
  /// C++ enum variant: <span style='color: green;'>```TableRowHeaderChanged = 280```</span>
  TableRowHeaderChanged = 280,
  /// C++ enum variant: <span style='color: green;'>```TableSummaryChanged = 281```</span>
  TableSummaryChanged = 281,
  /// C++ enum variant: <span style='color: green;'>```TextAttributeChanged = 282```</span>
  TextAttributeChanged = 282,
  /// C++ enum variant: <span style='color: green;'>```TextCaretMoved = 283```</span>
  TextCaretMoved = 283,
  /// C++ enum variant: <span style='color: green;'>```TextColumnChanged = 285```</span>
  TextColumnChanged = 285,
  /// C++ enum variant: <span style='color: green;'>```TextInserted = 286```</span>
  TextInserted = 286,
  /// C++ enum variant: <span style='color: green;'>```TextRemoved = 287```</span>
  TextRemoved = 287,
  /// C++ enum variant: <span style='color: green;'>```TextUpdated = 288```</span>
  TextUpdated = 288,
  /// C++ enum variant: <span style='color: green;'>```TextSelectionChanged = 289```</span>
  TextSelectionChanged = 289,
  /// C++ enum variant: <span style='color: green;'>```VisibleDataChanged = 290```</span>
  VisibleDataChanged = 290,
  /// C++ enum variant: <span style='color: green;'>```ObjectCreated = 32768```</span>
  ObjectCreated = 32768,
  /// C++ enum variant: <span style='color: green;'>```ObjectDestroyed = 32769```</span>
  ObjectDestroyed = 32769,
  /// C++ enum variant: <span style='color: green;'>```ObjectShow = 32770```</span>
  ObjectShow = 32770,
  /// C++ enum variant: <span style='color: green;'>```ObjectHide = 32771```</span>
  ObjectHide = 32771,
  /// C++ enum variant: <span style='color: green;'>```ObjectReorder = 32772```</span>
  ObjectReorder = 32772,
  /// C++ enum variant: <span style='color: green;'>```Focus = 32773```</span>
  Focus = 32773,
  /// C++ enum variant: <span style='color: green;'>```Selection = 32774```</span>
  Selection = 32774,
  /// C++ enum variant: <span style='color: green;'>```SelectionAdd = 32775```</span>
  SelectionAdd = 32775,
  /// C++ enum variant: <span style='color: green;'>```SelectionRemove = 32776```</span>
  SelectionRemove = 32776,
  /// C++ enum variant: <span style='color: green;'>```SelectionWithin = 32777```</span>
  SelectionWithin = 32777,
  /// C++ enum variant: <span style='color: green;'>```StateChanged = 32778```</span>
  StateChanged = 32778,
  /// C++ enum variant: <span style='color: green;'>```LocationChanged = 32779```</span>
  LocationChanged = 32779,
  /// C++ enum variant: <span style='color: green;'>```NameChanged = 32780```</span>
  NameChanged = 32780,
  /// C++ enum variant: <span style='color: green;'>```DescriptionChanged = 32781```</span>
  DescriptionChanged = 32781,
  /// C++ enum variant: <span style='color: green;'>```ValueChanged = 32782```</span>
  ValueChanged = 32782,
  /// C++ enum variant: <span style='color: green;'>```ParentChanged = 32783```</span>
  ParentChanged = 32783,
  /// C++ enum variant: <span style='color: green;'>```HelpChanged = 32928```</span>
  HelpChanged = 32928,
  /// C++ enum variant: <span style='color: green;'>```DefaultActionChanged = 32944```</span>
  DefaultActionChanged = 32944,
  /// C++ enum variant: <span style='color: green;'>```AcceleratorChanged = 32960```</span>
  AcceleratorChanged = 32960,
  /// C++ enum variant: <span style='color: green;'>```InvalidEvent = 32961```</span>
  InvalidEvent = 32961,
}

/// C++ type: <span style='color: green;'>```QAccessible::InterfaceType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InterfaceType {
  /// C++ enum variant: <span style='color: green;'>```TextInterface = 0```</span>
  Text = 0,
  /// C++ enum variant: <span style='color: green;'>```EditableTextInterface = 1```</span>
  EditableText = 1,
  /// C++ enum variant: <span style='color: green;'>```ValueInterface = 2```</span>
  Value = 2,
  /// C++ enum variant: <span style='color: green;'>```ActionInterface = 3```</span>
  Action = 3,
  /// C++ enum variant: <span style='color: green;'>```ImageInterface = 4```</span>
  Image = 4,
  /// C++ enum variant: <span style='color: green;'>```TableInterface = 5```</span>
  Table = 5,
  /// C++ enum variant: <span style='color: green;'>```TableCellInterface = 6```</span>
  TableCell = 6,
}

/// C++ type: <span style='color: green;'>```QAccessible::RelationFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RelationFlag {
  /// C++ enum variant: <span style='color: green;'>```AllRelations = -1```</span>
  AllRelations = -1,
  /// C++ enum variant: <span style='color: green;'>```Label = 1```</span>
  Label = 1,
  /// C++ enum variant: <span style='color: green;'>```Labelled = 2```</span>
  Labelled = 2,
  /// C++ enum variant: <span style='color: green;'>```Controller = 4```</span>
  Controller = 4,
  /// C++ enum variant: <span style='color: green;'>```Controlled = 8```</span>
  Controlled = 8,
}

/// C++ type: <span style='color: green;'>```QAccessible::Role```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Role {
  /// C++ enum variant: <span style='color: green;'>```NoRole = 0```</span>
  NoRole = 0,
  /// C++ enum variant: <span style='color: green;'>```TitleBar = 1```</span>
  TitleBar = 1,
  /// C++ enum variant: <span style='color: green;'>```MenuBar = 2```</span>
  MenuBar = 2,
  /// C++ enum variant: <span style='color: green;'>```ScrollBar = 3```</span>
  ScrollBar = 3,
  /// C++ enum variant: <span style='color: green;'>```Grip = 4```</span>
  Grip = 4,
  /// C++ enum variant: <span style='color: green;'>```Sound = 5```</span>
  Sound = 5,
  /// C++ enum variant: <span style='color: green;'>```Cursor = 6```</span>
  Cursor = 6,
  /// C++ enum variant: <span style='color: green;'>```Caret = 7```</span>
  Caret = 7,
  /// C++ enum variant: <span style='color: green;'>```AlertMessage = 8```</span>
  AlertMessage = 8,
  /// C++ enum variant: <span style='color: green;'>```Window = 9```</span>
  Window = 9,
  /// C++ enum variant: <span style='color: green;'>```Client = 10```</span>
  Client = 10,
  /// C++ enum variant: <span style='color: green;'>```PopupMenu = 11```</span>
  PopupMenu = 11,
  /// C++ enum variant: <span style='color: green;'>```MenuItem = 12```</span>
  MenuItem = 12,
  /// C++ enum variant: <span style='color: green;'>```ToolTip = 13```</span>
  ToolTip = 13,
  /// C++ enum variant: <span style='color: green;'>```Application = 14```</span>
  Application = 14,
  /// C++ enum variant: <span style='color: green;'>```Document = 15```</span>
  Document = 15,
  /// C++ enum variant: <span style='color: green;'>```Pane = 16```</span>
  Pane = 16,
  /// C++ enum variant: <span style='color: green;'>```Chart = 17```</span>
  Chart = 17,
  /// C++ enum variant: <span style='color: green;'>```Dialog = 18```</span>
  Dialog = 18,
  /// C++ enum variant: <span style='color: green;'>```Border = 19```</span>
  Border = 19,
  /// C++ enum variant: <span style='color: green;'>```Grouping = 20```</span>
  Grouping = 20,
  /// C++ enum variant: <span style='color: green;'>```Separator = 21```</span>
  Separator = 21,
  /// C++ enum variant: <span style='color: green;'>```ToolBar = 22```</span>
  ToolBar = 22,
  /// C++ enum variant: <span style='color: green;'>```StatusBar = 23```</span>
  StatusBar = 23,
  /// C++ enum variant: <span style='color: green;'>```Table = 24```</span>
  Table = 24,
  /// C++ enum variant: <span style='color: green;'>```ColumnHeader = 25```</span>
  ColumnHeader = 25,
  /// C++ enum variant: <span style='color: green;'>```RowHeader = 26```</span>
  RowHeader = 26,
  /// C++ enum variant: <span style='color: green;'>```Column = 27```</span>
  Column = 27,
  /// C++ enum variant: <span style='color: green;'>```Row = 28```</span>
  Row = 28,
  /// C++ enum variant: <span style='color: green;'>```Cell = 29```</span>
  Cell = 29,
  /// C++ enum variant: <span style='color: green;'>```Link = 30```</span>
  Link = 30,
  /// C++ enum variant: <span style='color: green;'>```HelpBalloon = 31```</span>
  HelpBalloon = 31,
  /// C++ enum variant: <span style='color: green;'>```Assistant = 32```</span>
  Assistant = 32,
  /// C++ enum variant: <span style='color: green;'>```List = 33```</span>
  List = 33,
  /// C++ enum variant: <span style='color: green;'>```ListItem = 34```</span>
  ListItem = 34,
  /// C++ enum variant: <span style='color: green;'>```Tree = 35```</span>
  Tree = 35,
  /// C++ enum variant: <span style='color: green;'>```TreeItem = 36```</span>
  TreeItem = 36,
  /// C++ enum variant: <span style='color: green;'>```PageTab = 37```</span>
  PageTab = 37,
  /// C++ enum variant: <span style='color: green;'>```PropertyPage = 38```</span>
  PropertyPage = 38,
  /// C++ enum variant: <span style='color: green;'>```Indicator = 39```</span>
  Indicator = 39,
  /// C++ enum variant: <span style='color: green;'>```Graphic = 40```</span>
  Graphic = 40,
  /// C++ enum variant: <span style='color: green;'>```StaticText = 41```</span>
  StaticText = 41,
  /// C++ enum variant: <span style='color: green;'>```EditableText = 42```</span>
  EditableText = 42,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Button = 43```</span>
  /// - <span style='color: green;'>```PushButton = 43```</span>
  ///
  Button = 43,
  /// C++ enum variant: <span style='color: green;'>```CheckBox = 44```</span>
  CheckBox = 44,
  /// C++ enum variant: <span style='color: green;'>```RadioButton = 45```</span>
  RadioButton = 45,
  /// C++ enum variant: <span style='color: green;'>```ComboBox = 46```</span>
  ComboBox = 46,
  /// C++ enum variant: <span style='color: green;'>```ProgressBar = 48```</span>
  ProgressBar = 48,
  /// C++ enum variant: <span style='color: green;'>```Dial = 49```</span>
  Dial = 49,
  /// C++ enum variant: <span style='color: green;'>```HotkeyField = 50```</span>
  HotkeyField = 50,
  /// C++ enum variant: <span style='color: green;'>```Slider = 51```</span>
  Slider = 51,
  /// C++ enum variant: <span style='color: green;'>```SpinBox = 52```</span>
  SpinBox = 52,
  /// C++ enum variant: <span style='color: green;'>```Canvas = 53```</span>
  Canvas = 53,
  /// C++ enum variant: <span style='color: green;'>```Animation = 54```</span>
  Animation = 54,
  /// C++ enum variant: <span style='color: green;'>```Equation = 55```</span>
  Equation = 55,
  /// C++ enum variant: <span style='color: green;'>```ButtonDropDown = 56```</span>
  ButtonDropDown = 56,
  /// C++ enum variant: <span style='color: green;'>```ButtonMenu = 57```</span>
  ButtonMenu = 57,
  /// C++ enum variant: <span style='color: green;'>```ButtonDropGrid = 58```</span>
  ButtonDropGrid = 58,
  /// C++ enum variant: <span style='color: green;'>```Whitespace = 59```</span>
  Whitespace = 59,
  /// C++ enum variant: <span style='color: green;'>```PageTabList = 60```</span>
  PageTabList = 60,
  /// C++ enum variant: <span style='color: green;'>```Clock = 61```</span>
  Clock = 61,
  /// C++ enum variant: <span style='color: green;'>```Splitter = 62```</span>
  Splitter = 62,
  /// C++ enum variant: <span style='color: green;'>```LayeredPane = 128```</span>
  LayeredPane = 128,
  /// C++ enum variant: <span style='color: green;'>```Terminal = 129```</span>
  Terminal = 129,
  /// C++ enum variant: <span style='color: green;'>```Desktop = 130```</span>
  Desktop = 130,
  /// C++ enum variant: <span style='color: green;'>```Paragraph = 131```</span>
  Paragraph = 131,
  /// C++ enum variant: <span style='color: green;'>```WebDocument = 132```</span>
  WebDocument = 132,
  /// C++ enum variant: <span style='color: green;'>```Section = 133```</span>
  Section = 133,
  /// C++ enum variant: <span style='color: green;'>```ColorChooser = 1028```</span>
  ColorChooser = 1028,
  /// C++ enum variant: <span style='color: green;'>```Footer = 1038```</span>
  Footer = 1038,
  /// C++ enum variant: <span style='color: green;'>```Form = 1040```</span>
  Form = 1040,
  /// C++ enum variant: <span style='color: green;'>```Heading = 1044```</span>
  Heading = 1044,
  /// C++ enum variant: <span style='color: green;'>```Note = 1051```</span>
  Note = 1051,
  /// C++ enum variant: <span style='color: green;'>```ComplementaryContent = 1068```</span>
  ComplementaryContent = 1068,
  /// C++ enum variant: <span style='color: green;'>```UserRole = 65535```</span>
  UserRole = 65535,
}

/// C++ type: <span style='color: green;'>```QAccessible::State```</span>
#[repr(C)]
pub struct State([u8; ::type_sizes::QT_GUI_ACCESSIBLE_STATE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for State {
  unsafe fn new_uninitialized() -> State {
    State(::std::mem::uninitialized())
  }
}

impl State {
  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::active() const```</span>
  ///
  ///
  pub fn active(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_active(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::animated() const```</span>
  ///
  ///
  pub fn animated(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_animated(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::busy() const```</span>
  ///
  ///
  pub fn busy(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_busy(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::checkStateMixed() const```</span>
  ///
  ///
  pub fn check_state_mixed(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_checkStateMixed(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::checkable() const```</span>
  ///
  ///
  pub fn checkable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_checkable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::checked() const```</span>
  ///
  ///
  pub fn checked(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_checked(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::collapsed() const```</span>
  ///
  ///
  pub fn collapsed(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_collapsed(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::defaultButton() const```</span>
  ///
  ///
  pub fn default_button(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_defaultButton(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::disabled() const```</span>
  ///
  ///
  pub fn disabled(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_disabled(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::editable() const```</span>
  ///
  ///
  pub fn editable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_editable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::expandable() const```</span>
  ///
  ///
  pub fn expandable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_expandable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::expanded() const```</span>
  ///
  ///
  pub fn expanded(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_expanded(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::extSelectable() const```</span>
  ///
  ///
  pub fn ext_selectable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_extSelectable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::focusable() const```</span>
  ///
  ///
  pub fn focusable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_focusable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::focused() const```</span>
  ///
  ///
  pub fn focused(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_focused(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::hasPopup() const```</span>
  ///
  ///
  pub fn has_popup(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_hasPopup(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::hotTracked() const```</span>
  ///
  ///
  pub fn hot_tracked(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_hotTracked(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::invalid() const```</span>
  ///
  ///
  pub fn invalid(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_invalid(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::invisible() const```</span>
  ///
  ///
  pub fn invisible(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_invisible(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::linked() const```</span>
  ///
  ///
  pub fn linked(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_linked(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::marqueed() const```</span>
  ///
  ///
  pub fn marqueed(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_marqueed(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::modal() const```</span>
  ///
  ///
  pub fn modal(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_modal(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::movable() const```</span>
  ///
  ///
  pub fn movable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_movable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::multiLine() const```</span>
  ///
  ///
  pub fn multi_line(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_multiLine(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::multiSelectable() const```</span>
  ///
  ///
  pub fn multi_selectable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_multiSelectable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAccessible::State::State()```</span>
  ///
  ///
  pub fn new() -> ::accessible::State {
    {
      let mut object: ::accessible::State =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessible_State_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::offscreen() const```</span>
  ///
  ///
  pub fn offscreen(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_offscreen(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::passwordEdit() const```</span>
  ///
  ///
  pub fn password_edit(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_passwordEdit(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::pressed() const```</span>
  ///
  ///
  pub fn pressed(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_pressed(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::readOnly() const```</span>
  ///
  ///
  pub fn read_only(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_readOnly(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::searchEdit() const```</span>
  ///
  ///
  pub fn search_edit(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_searchEdit(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::selectable() const```</span>
  ///
  ///
  pub fn selectable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_selectable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::selectableText() const```</span>
  ///
  ///
  pub fn selectable_text(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_selectableText(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::selected() const```</span>
  ///
  ///
  pub fn selected(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_selected(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::selfVoicing() const```</span>
  ///
  ///
  pub fn self_voicing(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_selfVoicing(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_active(quint64 value)```</span>
  ///
  ///
  pub fn set_active(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_active(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_animated(quint64 value)```</span>
  ///
  ///
  pub fn set_animated(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_animated(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_busy(quint64 value)```</span>
  ///
  ///
  pub fn set_busy(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_busy(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_checkStateMixed(quint64 value)```</span>
  ///
  ///
  pub fn set_check_state_mixed(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_checkStateMixed(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_checkable(quint64 value)```</span>
  ///
  ///
  pub fn set_checkable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_checkable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_checked(quint64 value)```</span>
  ///
  ///
  pub fn set_checked(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_checked(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_collapsed(quint64 value)```</span>
  ///
  ///
  pub fn set_collapsed(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_collapsed(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_defaultButton(quint64 value)```</span>
  ///
  ///
  pub fn set_default_button(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_defaultButton(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_disabled(quint64 value)```</span>
  ///
  ///
  pub fn set_disabled(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_disabled(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_editable(quint64 value)```</span>
  ///
  ///
  pub fn set_editable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_editable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_expandable(quint64 value)```</span>
  ///
  ///
  pub fn set_expandable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_expandable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_expanded(quint64 value)```</span>
  ///
  ///
  pub fn set_expanded(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_expanded(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_extSelectable(quint64 value)```</span>
  ///
  ///
  pub fn set_ext_selectable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_extSelectable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_focusable(quint64 value)```</span>
  ///
  ///
  pub fn set_focusable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_focusable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_focused(quint64 value)```</span>
  ///
  ///
  pub fn set_focused(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_focused(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_hasPopup(quint64 value)```</span>
  ///
  ///
  pub fn set_has_popup(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_hasPopup(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_hotTracked(quint64 value)```</span>
  ///
  ///
  pub fn set_hot_tracked(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_hotTracked(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_invalid(quint64 value)```</span>
  ///
  ///
  pub fn set_invalid(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_invalid(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_invisible(quint64 value)```</span>
  ///
  ///
  pub fn set_invisible(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_invisible(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_linked(quint64 value)```</span>
  ///
  ///
  pub fn set_linked(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_linked(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_marqueed(quint64 value)```</span>
  ///
  ///
  pub fn set_marqueed(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_marqueed(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_modal(quint64 value)```</span>
  ///
  ///
  pub fn set_modal(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_modal(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_movable(quint64 value)```</span>
  ///
  ///
  pub fn set_movable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_movable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_multiLine(quint64 value)```</span>
  ///
  ///
  pub fn set_multi_line(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_multiLine(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_multiSelectable(quint64 value)```</span>
  ///
  ///
  pub fn set_multi_selectable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_multiSelectable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_offscreen(quint64 value)```</span>
  ///
  ///
  pub fn set_offscreen(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_offscreen(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_passwordEdit(quint64 value)```</span>
  ///
  ///
  pub fn set_password_edit(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_passwordEdit(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_pressed(quint64 value)```</span>
  ///
  ///
  pub fn set_pressed(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_pressed(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_readOnly(quint64 value)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_readOnly(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_searchEdit(quint64 value)```</span>
  ///
  ///
  pub fn set_search_edit(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_searchEdit(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_selectable(quint64 value)```</span>
  ///
  ///
  pub fn set_selectable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_selectable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_selectableText(quint64 value)```</span>
  ///
  ///
  pub fn set_selectable_text(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_selectableText(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_selected(quint64 value)```</span>
  ///
  ///
  pub fn set_selected(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_selected(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_selfVoicing(quint64 value)```</span>
  ///
  ///
  pub fn set_self_voicing(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_selfVoicing(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_sizeable(quint64 value)```</span>
  ///
  ///
  pub fn set_sizeable(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_sizeable(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_supportsAutoCompletion(quint64 value)```</span>
  ///
  ///
  pub fn set_supports_auto_completion(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_supportsAutoCompletion(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessible::State::set_traversed(quint64 value)```</span>
  ///
  ///
  pub fn set_traversed(&mut self, value: u64) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_set_traversed(self as *mut ::accessible::State, value) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::sizeable() const```</span>
  ///
  ///
  pub fn sizeable(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_sizeable(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::supportsAutoCompletion() const```</span>
  ///
  ///
  pub fn supports_auto_completion(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_supportsAutoCompletion(self as *const ::accessible::State) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QAccessible::State::traversed() const```</span>
  ///
  ///
  pub fn traversed(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_traversed(self as *const ::accessible::State) }
  }
}

impl Drop for ::accessible::State {
  /// C++ method: <span style='color: green;'>```[destructor] void QAccessible::State::~QAccessible::State()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QAccessible_State_destructor(self as *mut ::accessible::State) }
  }
}

/// C++ type: <span style='color: green;'>```QAccessible::Text```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Text {
  /// C++ enum variant: <span style='color: green;'>```Name = 0```</span>
  Name = 0,
  /// C++ enum variant: <span style='color: green;'>```Description = 1```</span>
  Description = 1,
  /// C++ enum variant: <span style='color: green;'>```Value = 2```</span>
  Value = 2,
  /// C++ enum variant: <span style='color: green;'>```Help = 3```</span>
  Help = 3,
  /// C++ enum variant: <span style='color: green;'>```Accelerator = 4```</span>
  Accelerator = 4,
  /// C++ enum variant: <span style='color: green;'>```DebugDescription = 5```</span>
  DebugDescription = 5,
  /// C++ enum variant: <span style='color: green;'>```UserText = 65535```</span>
  UserText = 65535,
}

/// C++ type: <span style='color: green;'>```QAccessible::TextBoundaryType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextBoundaryType {
  /// C++ enum variant: <span style='color: green;'>```CharBoundary = 0```</span>
  Char = 0,
  /// C++ enum variant: <span style='color: green;'>```WordBoundary = 1```</span>
  Word = 1,
  /// C++ enum variant: <span style='color: green;'>```SentenceBoundary = 2```</span>
  Sentence = 2,
  /// C++ enum variant: <span style='color: green;'>```ParagraphBoundary = 3```</span>
  Paragraph = 3,
  /// C++ enum variant: <span style='color: green;'>```LineBoundary = 4```</span>
  Line = 4,
  /// C++ enum variant: <span style='color: green;'>```NoBoundary = 5```</span>
  No = 5,
}

/// C++ method: <span style='color: green;'>```const char* qAccessibleEventString(QAccessible::Event event)```</span>
///
///
pub fn accessible_event_string(event: &::accessible::Event) -> *const ::libc::c_char {
  unsafe { ::ffi::qt_gui_c_QAccessible_G_qAccessibleEventString(event as *const ::accessible::Event) }
}

/// C++ method: <span style='color: green;'>```QString qAccessibleLocalizedActionDescription(const QString& actionName)```</span>
///
///
pub fn accessible_localized_action_description(action_name: &::qt_core::string::String) -> ::qt_core::string::String {
  {
    let mut object: ::qt_core::string::String =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QAccessible_G_qAccessibleLocalizedActionDescription_to_output(action_name as *const ::qt_core::string::String, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```const char* qAccessibleRoleString(QAccessible::Role role)```</span>
///
///
pub fn accessible_role_string(role: &::accessible::Role) -> *const ::libc::c_char {
  unsafe { ::ffi::qt_gui_c_QAccessible_G_qAccessibleRoleString(role as *const ::accessible::Role) }
}

/// C++ method: <span style='color: green;'>```bool operator==(const QAccessible::State& first, const QAccessible::State& second)```</span>
///
///
pub fn op_eq(first: &::accessible::State, second: &::accessible::State) -> bool {
  unsafe {
    ::ffi::qt_gui_c_QAccessible_G_operator_eq(first as *const ::accessible::State,
                                              second as *const ::accessible::State)
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug d, const QAccessibleEvent& ev)```</span>
///
///
pub fn op_shl(d: &::qt_core::debug::Debug, ev: &::accessible_event::AccessibleEvent) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QAccessible_G_operator_shl_to_output_d_ev(d as *const ::qt_core::debug::Debug,
                                                                ev as *const ::accessible_event::AccessibleEvent,
                                                                &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug d, const QAccessibleInterface* iface)```</span>
///
///
pub unsafe fn op_shl_unsafe(d: &::qt_core::debug::Debug,
                            iface: *const ::accessible_interface::AccessibleInterface)
                            -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_gui_c_QAccessible_G_operator_shl_to_output_d_iface(d as *const ::qt_core::debug::Debug,
                                                                 iface,
                                                                 &mut object);
    object
  }
}
