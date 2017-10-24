/// C++ type: <span style='color: green;'>```QStyle::ComplexControl```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ComplexControl {
  /// C++ enum variant: <span style='color: green;'>```CC_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```CC_SpinBox = 0```</span>
  SpinBox = 0,
  /// C++ enum variant: <span style='color: green;'>```CC_ComboBox = 1```</span>
  ComboBox = 1,
  /// C++ enum variant: <span style='color: green;'>```CC_ScrollBar = 2```</span>
  ScrollBar = 2,
  /// C++ enum variant: <span style='color: green;'>```CC_Slider = 3```</span>
  Slider = 3,
  /// C++ enum variant: <span style='color: green;'>```CC_ToolButton = 4```</span>
  ToolButton = 4,
  /// C++ enum variant: <span style='color: green;'>```CC_TitleBar = 5```</span>
  TitleBar = 5,
  /// C++ enum variant: <span style='color: green;'>```CC_Dial = 6```</span>
  Dial = 6,
  /// C++ enum variant: <span style='color: green;'>```CC_GroupBox = 7```</span>
  GroupBox = 7,
  /// C++ enum variant: <span style='color: green;'>```CC_MdiControls = 8```</span>
  MdiControls = 8,
}

/// C++ type: <span style='color: green;'>```QStyle::ContentsType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ContentsType {
  /// C++ enum variant: <span style='color: green;'>```CT_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```CT_PushButton = 0```</span>
  PushButton = 0,
  /// C++ enum variant: <span style='color: green;'>```CT_CheckBox = 1```</span>
  CheckBox = 1,
  /// C++ enum variant: <span style='color: green;'>```CT_RadioButton = 2```</span>
  RadioButton = 2,
  /// C++ enum variant: <span style='color: green;'>```CT_ToolButton = 3```</span>
  ToolButton = 3,
  /// C++ enum variant: <span style='color: green;'>```CT_ComboBox = 4```</span>
  ComboBox = 4,
  /// C++ enum variant: <span style='color: green;'>```CT_Splitter = 5```</span>
  Splitter = 5,
  /// C++ enum variant: <span style='color: green;'>```CT_ProgressBar = 6```</span>
  ProgressBar = 6,
  /// C++ enum variant: <span style='color: green;'>```CT_MenuItem = 7```</span>
  MenuItem = 7,
  /// C++ enum variant: <span style='color: green;'>```CT_MenuBarItem = 8```</span>
  MenuBarItem = 8,
  /// C++ enum variant: <span style='color: green;'>```CT_MenuBar = 9```</span>
  MenuBar = 9,
  /// C++ enum variant: <span style='color: green;'>```CT_Menu = 10```</span>
  Menu = 10,
  /// C++ enum variant: <span style='color: green;'>```CT_TabBarTab = 11```</span>
  TabBarTab = 11,
  /// C++ enum variant: <span style='color: green;'>```CT_Slider = 12```</span>
  Slider = 12,
  /// C++ enum variant: <span style='color: green;'>```CT_ScrollBar = 13```</span>
  ScrollBar = 13,
  /// C++ enum variant: <span style='color: green;'>```CT_LineEdit = 14```</span>
  LineEdit = 14,
  /// C++ enum variant: <span style='color: green;'>```CT_SpinBox = 15```</span>
  SpinBox = 15,
  /// C++ enum variant: <span style='color: green;'>```CT_SizeGrip = 16```</span>
  SizeGrip = 16,
  /// C++ enum variant: <span style='color: green;'>```CT_TabWidget = 17```</span>
  TabWidget = 17,
  /// C++ enum variant: <span style='color: green;'>```CT_DialogButtons = 18```</span>
  DialogButtons = 18,
  /// C++ enum variant: <span style='color: green;'>```CT_HeaderSection = 19```</span>
  HeaderSection = 19,
  /// C++ enum variant: <span style='color: green;'>```CT_GroupBox = 20```</span>
  GroupBox = 20,
  /// C++ enum variant: <span style='color: green;'>```CT_MdiControls = 21```</span>
  MdiControls = 21,
  /// C++ enum variant: <span style='color: green;'>```CT_ItemViewItem = 22```</span>
  ItemViewItem = 22,
}

/// C++ type: <span style='color: green;'>```QStyle::ControlElement```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ControlElement {
  /// C++ enum variant: <span style='color: green;'>```CE_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```CE_PushButton = 0```</span>
  PushButton = 0,
  /// C++ enum variant: <span style='color: green;'>```CE_PushButtonBevel = 1```</span>
  PushButtonBevel = 1,
  /// C++ enum variant: <span style='color: green;'>```CE_PushButtonLabel = 2```</span>
  PushButtonLabel = 2,
  /// C++ enum variant: <span style='color: green;'>```CE_CheckBox = 3```</span>
  CheckBox = 3,
  /// C++ enum variant: <span style='color: green;'>```CE_CheckBoxLabel = 4```</span>
  CheckBoxLabel = 4,
  /// C++ enum variant: <span style='color: green;'>```CE_RadioButton = 5```</span>
  RadioButton = 5,
  /// C++ enum variant: <span style='color: green;'>```CE_RadioButtonLabel = 6```</span>
  RadioButtonLabel = 6,
  /// C++ enum variant: <span style='color: green;'>```CE_TabBarTab = 7```</span>
  TabBarTab = 7,
  /// C++ enum variant: <span style='color: green;'>```CE_TabBarTabShape = 8```</span>
  TabBarTabShape = 8,
  /// C++ enum variant: <span style='color: green;'>```CE_TabBarTabLabel = 9```</span>
  TabBarTabLabel = 9,
  /// C++ enum variant: <span style='color: green;'>```CE_ProgressBar = 10```</span>
  ProgressBar = 10,
  /// C++ enum variant: <span style='color: green;'>```CE_ProgressBarGroove = 11```</span>
  ProgressBarGroove = 11,
  /// C++ enum variant: <span style='color: green;'>```CE_ProgressBarContents = 12```</span>
  ProgressBarContents = 12,
  /// C++ enum variant: <span style='color: green;'>```CE_ProgressBarLabel = 13```</span>
  ProgressBarLabel = 13,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuItem = 14```</span>
  MenuItem = 14,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuScroller = 15```</span>
  MenuScroller = 15,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuVMargin = 16```</span>
  MenuVMargin = 16,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuHMargin = 17```</span>
  MenuHMargin = 17,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuTearoff = 18```</span>
  MenuTearoff = 18,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuEmptyArea = 19```</span>
  MenuEmptyArea = 19,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuBarItem = 20```</span>
  MenuBarItem = 20,
  /// C++ enum variant: <span style='color: green;'>```CE_MenuBarEmptyArea = 21```</span>
  MenuBarEmptyArea = 21,
  /// C++ enum variant: <span style='color: green;'>```CE_ToolButtonLabel = 22```</span>
  ToolButtonLabel = 22,
  /// C++ enum variant: <span style='color: green;'>```CE_Header = 23```</span>
  Header = 23,
  /// C++ enum variant: <span style='color: green;'>```CE_HeaderSection = 24```</span>
  HeaderSection = 24,
  /// C++ enum variant: <span style='color: green;'>```CE_HeaderLabel = 25```</span>
  HeaderLabel = 25,
  /// C++ enum variant: <span style='color: green;'>```CE_ToolBoxTab = 26```</span>
  ToolBoxTab = 26,
  /// C++ enum variant: <span style='color: green;'>```CE_SizeGrip = 27```</span>
  SizeGrip = 27,
  /// C++ enum variant: <span style='color: green;'>```CE_Splitter = 28```</span>
  Splitter = 28,
  /// C++ enum variant: <span style='color: green;'>```CE_RubberBand = 29```</span>
  RubberBand = 29,
  /// C++ enum variant: <span style='color: green;'>```CE_DockWidgetTitle = 30```</span>
  DockWidgetTitle = 30,
  /// C++ enum variant: <span style='color: green;'>```CE_ScrollBarAddLine = 31```</span>
  ScrollBarAddLine = 31,
  /// C++ enum variant: <span style='color: green;'>```CE_ScrollBarSubLine = 32```</span>
  ScrollBarSubLine = 32,
  /// C++ enum variant: <span style='color: green;'>```CE_ScrollBarAddPage = 33```</span>
  ScrollBarAddPage = 33,
  /// C++ enum variant: <span style='color: green;'>```CE_ScrollBarSubPage = 34```</span>
  ScrollBarSubPage = 34,
  /// C++ enum variant: <span style='color: green;'>```CE_ScrollBarSlider = 35```</span>
  ScrollBarSlider = 35,
  /// C++ enum variant: <span style='color: green;'>```CE_ScrollBarFirst = 36```</span>
  ScrollBarFirst = 36,
  /// C++ enum variant: <span style='color: green;'>```CE_ScrollBarLast = 37```</span>
  ScrollBarLast = 37,
  /// C++ enum variant: <span style='color: green;'>```CE_FocusFrame = 38```</span>
  FocusFrame = 38,
  /// C++ enum variant: <span style='color: green;'>```CE_ComboBoxLabel = 39```</span>
  ComboBoxLabel = 39,
  /// C++ enum variant: <span style='color: green;'>```CE_ToolBar = 40```</span>
  ToolBar = 40,
  /// C++ enum variant: <span style='color: green;'>```CE_ToolBoxTabShape = 41```</span>
  ToolBoxTabShape = 41,
  /// C++ enum variant: <span style='color: green;'>```CE_ToolBoxTabLabel = 42```</span>
  ToolBoxTabLabel = 42,
  /// C++ enum variant: <span style='color: green;'>```CE_HeaderEmptyArea = 43```</span>
  HeaderEmptyArea = 43,
  /// C++ enum variant: <span style='color: green;'>```CE_ColumnViewGrip = 44```</span>
  ColumnViewGrip = 44,
  /// C++ enum variant: <span style='color: green;'>```CE_ItemViewItem = 45```</span>
  ItemViewItem = 45,
  /// C++ enum variant: <span style='color: green;'>```CE_ShapedFrame = 46```</span>
  ShapedFrame = 46,
}

/// C++ type: <span style='color: green;'>```QStyle::PixelMetric```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PixelMetric {
  /// C++ enum variant: <span style='color: green;'>```PM_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```PM_ButtonMargin = 0```</span>
  ButtonMargin = 0,
  /// C++ enum variant: <span style='color: green;'>```PM_ButtonDefaultIndicator = 1```</span>
  ButtonDefaultIndicator = 1,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuButtonIndicator = 2```</span>
  MenuButtonIndicator = 2,
  /// C++ enum variant: <span style='color: green;'>```PM_ButtonShiftHorizontal = 3```</span>
  ButtonShiftHorizontal = 3,
  /// C++ enum variant: <span style='color: green;'>```PM_ButtonShiftVertical = 4```</span>
  ButtonShiftVertical = 4,
  /// C++ enum variant: <span style='color: green;'>```PM_DefaultFrameWidth = 5```</span>
  DefaultFrameWidth = 5,
  /// C++ enum variant: <span style='color: green;'>```PM_SpinBoxFrameWidth = 6```</span>
  SpinBoxFrameWidth = 6,
  /// C++ enum variant: <span style='color: green;'>```PM_ComboBoxFrameWidth = 7```</span>
  ComboBoxFrameWidth = 7,
  /// C++ enum variant: <span style='color: green;'>```PM_MaximumDragDistance = 8```</span>
  MaximumDragDistance = 8,
  /// C++ enum variant: <span style='color: green;'>```PM_ScrollBarExtent = 9```</span>
  ScrollBarExtent = 9,
  /// C++ enum variant: <span style='color: green;'>```PM_ScrollBarSliderMin = 10```</span>
  ScrollBarSliderMin = 10,
  /// C++ enum variant: <span style='color: green;'>```PM_SliderThickness = 11```</span>
  SliderThickness = 11,
  /// C++ enum variant: <span style='color: green;'>```PM_SliderControlThickness = 12```</span>
  SliderControlThickness = 12,
  /// C++ enum variant: <span style='color: green;'>```PM_SliderLength = 13```</span>
  SliderLength = 13,
  /// C++ enum variant: <span style='color: green;'>```PM_SliderTickmarkOffset = 14```</span>
  SliderTickmarkOffset = 14,
  /// C++ enum variant: <span style='color: green;'>```PM_SliderSpaceAvailable = 15```</span>
  SliderSpaceAvailable = 15,
  /// C++ enum variant: <span style='color: green;'>```PM_DockWidgetSeparatorExtent = 16```</span>
  DockWidgetSeparatorExtent = 16,
  /// C++ enum variant: <span style='color: green;'>```PM_DockWidgetHandleExtent = 17```</span>
  DockWidgetHandleExtent = 17,
  /// C++ enum variant: <span style='color: green;'>```PM_DockWidgetFrameWidth = 18```</span>
  DockWidgetFrameWidth = 18,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarTabOverlap = 19```</span>
  TabBarTabOverlap = 19,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarTabHSpace = 20```</span>
  TabBarTabHSpace = 20,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarTabVSpace = 21```</span>
  TabBarTabVSpace = 21,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarBaseHeight = 22```</span>
  TabBarBaseHeight = 22,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarBaseOverlap = 23```</span>
  TabBarBaseOverlap = 23,
  /// C++ enum variant: <span style='color: green;'>```PM_ProgressBarChunkWidth = 24```</span>
  ProgressBarChunkWidth = 24,
  /// C++ enum variant: <span style='color: green;'>```PM_SplitterWidth = 25```</span>
  SplitterWidth = 25,
  /// C++ enum variant: <span style='color: green;'>```PM_TitleBarHeight = 26```</span>
  TitleBarHeight = 26,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuScrollerHeight = 27```</span>
  MenuScrollerHeight = 27,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuHMargin = 28```</span>
  MenuHMargin = 28,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuVMargin = 29```</span>
  MenuVMargin = 29,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuPanelWidth = 30```</span>
  MenuPanelWidth = 30,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuTearoffHeight = 31```</span>
  MenuTearoffHeight = 31,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuDesktopFrameWidth = 32```</span>
  MenuDesktopFrameWidth = 32,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuBarPanelWidth = 33```</span>
  MenuBarPanelWidth = 33,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuBarItemSpacing = 34```</span>
  MenuBarItemSpacing = 34,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuBarVMargin = 35```</span>
  MenuBarVMargin = 35,
  /// C++ enum variant: <span style='color: green;'>```PM_MenuBarHMargin = 36```</span>
  MenuBarHMargin = 36,
  /// C++ enum variant: <span style='color: green;'>```PM_IndicatorWidth = 37```</span>
  IndicatorWidth = 37,
  /// C++ enum variant: <span style='color: green;'>```PM_IndicatorHeight = 38```</span>
  IndicatorHeight = 38,
  /// C++ enum variant: <span style='color: green;'>```PM_ExclusiveIndicatorWidth = 39```</span>
  ExclusiveIndicatorWidth = 39,
  /// C++ enum variant: <span style='color: green;'>```PM_ExclusiveIndicatorHeight = 40```</span>
  ExclusiveIndicatorHeight = 40,
  /// C++ enum variant: <span style='color: green;'>```PM_DialogButtonsSeparator = 41```</span>
  DialogButtonsSeparator = 41,
  /// C++ enum variant: <span style='color: green;'>```PM_DialogButtonsButtonWidth = 42```</span>
  DialogButtonsButtonWidth = 42,
  /// C++ enum variant: <span style='color: green;'>```PM_DialogButtonsButtonHeight = 43```</span>
  DialogButtonsButtonHeight = 43,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```PM_MdiSubWindowFrameWidth = 44```</span>
  /// - <span style='color: green;'>```PM_MDIFrameWidth = 44```</span>
  ///
  MdiSubWindowFrameWidth = 44,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```PM_MdiSubWindowMinimizedWidth = 45```</span>
  /// - <span style='color: green;'>```PM_MDIMinimizedWidth = 45```</span>
  ///
  MdiSubWindowMinimizedWidth = 45,
  /// C++ enum variant: <span style='color: green;'>```PM_HeaderMargin = 46```</span>
  HeaderMargin = 46,
  /// C++ enum variant: <span style='color: green;'>```PM_HeaderMarkSize = 47```</span>
  HeaderMarkSize = 47,
  /// C++ enum variant: <span style='color: green;'>```PM_HeaderGripMargin = 48```</span>
  HeaderGripMargin = 48,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarTabShiftHorizontal = 49```</span>
  TabBarTabShiftHorizontal = 49,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarTabShiftVertical = 50```</span>
  TabBarTabShiftVertical = 50,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarScrollButtonWidth = 51```</span>
  TabBarScrollButtonWidth = 51,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolBarFrameWidth = 52```</span>
  ToolBarFrameWidth = 52,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolBarHandleExtent = 53```</span>
  ToolBarHandleExtent = 53,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolBarItemSpacing = 54```</span>
  ToolBarItemSpacing = 54,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolBarItemMargin = 55```</span>
  ToolBarItemMargin = 55,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolBarSeparatorExtent = 56```</span>
  ToolBarSeparatorExtent = 56,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolBarExtensionExtent = 57```</span>
  ToolBarExtensionExtent = 57,
  /// C++ enum variant: <span style='color: green;'>```PM_SpinBoxSliderHeight = 58```</span>
  SpinBoxSliderHeight = 58,
  /// C++ enum variant: <span style='color: green;'>```PM_DefaultTopLevelMargin = 59```</span>
  DefaultTopLevelMargin = 59,
  /// C++ enum variant: <span style='color: green;'>```PM_DefaultChildMargin = 60```</span>
  DefaultChildMargin = 60,
  /// C++ enum variant: <span style='color: green;'>```PM_DefaultLayoutSpacing = 61```</span>
  DefaultLayoutSpacing = 61,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolBarIconSize = 62```</span>
  ToolBarIconSize = 62,
  /// C++ enum variant: <span style='color: green;'>```PM_ListViewIconSize = 63```</span>
  ListViewIconSize = 63,
  /// C++ enum variant: <span style='color: green;'>```PM_IconViewIconSize = 64```</span>
  IconViewIconSize = 64,
  /// C++ enum variant: <span style='color: green;'>```PM_SmallIconSize = 65```</span>
  SmallIconSize = 65,
  /// C++ enum variant: <span style='color: green;'>```PM_LargeIconSize = 66```</span>
  LargeIconSize = 66,
  /// C++ enum variant: <span style='color: green;'>```PM_FocusFrameVMargin = 67```</span>
  FocusFrameVMargin = 67,
  /// C++ enum variant: <span style='color: green;'>```PM_FocusFrameHMargin = 68```</span>
  FocusFrameHMargin = 68,
  /// C++ enum variant: <span style='color: green;'>```PM_ToolTipLabelFrameWidth = 69```</span>
  ToolTipLabelFrameWidth = 69,
  /// C++ enum variant: <span style='color: green;'>```PM_CheckBoxLabelSpacing = 70```</span>
  CheckBoxLabelSpacing = 70,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBarIconSize = 71```</span>
  TabBarIconSize = 71,
  /// C++ enum variant: <span style='color: green;'>```PM_SizeGripSize = 72```</span>
  SizeGripSize = 72,
  /// C++ enum variant: <span style='color: green;'>```PM_DockWidgetTitleMargin = 73```</span>
  DockWidgetTitleMargin = 73,
  /// C++ enum variant: <span style='color: green;'>```PM_MessageBoxIconSize = 74```</span>
  MessageBoxIconSize = 74,
  /// C++ enum variant: <span style='color: green;'>```PM_ButtonIconSize = 75```</span>
  ButtonIconSize = 75,
  /// C++ enum variant: <span style='color: green;'>```PM_DockWidgetTitleBarButtonMargin = 76```</span>
  DockWidgetTitleBarButtonMargin = 76,
  /// C++ enum variant: <span style='color: green;'>```PM_RadioButtonLabelSpacing = 77```</span>
  RadioButtonLabelSpacing = 77,
  /// C++ enum variant: <span style='color: green;'>```PM_LayoutLeftMargin = 78```</span>
  LayoutLeftMargin = 78,
  /// C++ enum variant: <span style='color: green;'>```PM_LayoutTopMargin = 79```</span>
  LayoutTopMargin = 79,
  /// C++ enum variant: <span style='color: green;'>```PM_LayoutRightMargin = 80```</span>
  LayoutRightMargin = 80,
  /// C++ enum variant: <span style='color: green;'>```PM_LayoutBottomMargin = 81```</span>
  LayoutBottomMargin = 81,
  /// C++ enum variant: <span style='color: green;'>```PM_LayoutHorizontalSpacing = 82```</span>
  LayoutHorizontalSpacing = 82,
  /// C++ enum variant: <span style='color: green;'>```PM_LayoutVerticalSpacing = 83```</span>
  LayoutVerticalSpacing = 83,
  /// C++ enum variant: <span style='color: green;'>```PM_TabBar_ScrollButtonOverlap = 84```</span>
  TabBarScrollButtonOverlap = 84,
  /// C++ enum variant: <span style='color: green;'>```PM_TextCursorWidth = 85```</span>
  TextCursorWidth = 85,
  /// C++ enum variant: <span style='color: green;'>```PM_TabCloseIndicatorWidth = 86```</span>
  TabCloseIndicatorWidth = 86,
  /// C++ enum variant: <span style='color: green;'>```PM_TabCloseIndicatorHeight = 87```</span>
  TabCloseIndicatorHeight = 87,
  /// C++ enum variant: <span style='color: green;'>```PM_ScrollView_ScrollBarSpacing = 88```</span>
  ScrollViewScrollBarSpacing = 88,
  /// C++ enum variant: <span style='color: green;'>```PM_ScrollView_ScrollBarOverlap = 89```</span>
  ScrollViewScrollBarOverlap = 89,
  /// C++ enum variant: <span style='color: green;'>```PM_SubMenuOverlap = 90```</span>
  SubMenuOverlap = 90,
  /// C++ enum variant: <span style='color: green;'>```PM_TreeViewIndentation = 91```</span>
  TreeViewIndentation = 91,
  /// C++ enum variant: <span style='color: green;'>```PM_HeaderDefaultSectionSizeHorizontal = 92```</span>
  HeaderDefaultSectionSizeHorizontal = 92,
  /// C++ enum variant: <span style='color: green;'>```PM_HeaderDefaultSectionSizeVertical = 93```</span>
  HeaderDefaultSectionSizeVertical = 93,
  /// C++ enum variant: <span style='color: green;'>```PM_TitleBarButtonIconSize = 94```</span>
  TitleBarButtonIconSize = 94,
  /// C++ enum variant: <span style='color: green;'>```PM_TitleBarButtonSize = 95```</span>
  TitleBarButtonSize = 95,
}

/// C++ type: <span style='color: green;'>```QStyle::PrimitiveElement```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PrimitiveElement {
  /// C++ enum variant: <span style='color: green;'>```PE_Frame = 0```</span>
  Frame = 0,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameDefaultButton = 1```</span>
  FrameDefaultButton = 1,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameDockWidget = 2```</span>
  FrameDockWidget = 2,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameFocusRect = 3```</span>
  FrameFocusRect = 3,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameGroupBox = 4```</span>
  FrameGroupBox = 4,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameLineEdit = 5```</span>
  FrameLineEdit = 5,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameMenu = 6```</span>
  FrameMenu = 6,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```PE_FrameStatusBar = 7```</span>
  /// - <span style='color: green;'>```PE_FrameStatusBarItem = 7```</span>
  ///
  FrameStatusBar = 7,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameTabWidget = 8```</span>
  FrameTabWidget = 8,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameWindow = 9```</span>
  FrameWindow = 9,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameButtonBevel = 10```</span>
  FrameButtonBevel = 10,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameButtonTool = 11```</span>
  FrameButtonTool = 11,
  /// C++ enum variant: <span style='color: green;'>```PE_FrameTabBarBase = 12```</span>
  FrameTabBarBase = 12,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelButtonCommand = 13```</span>
  PanelButtonCommand = 13,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelButtonBevel = 14```</span>
  PanelButtonBevel = 14,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelButtonTool = 15```</span>
  PanelButtonTool = 15,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelMenuBar = 16```</span>
  PanelMenuBar = 16,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelToolBar = 17```</span>
  PanelToolBar = 17,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelLineEdit = 18```</span>
  PanelLineEdit = 18,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorArrowDown = 19```</span>
  IndicatorArrowDown = 19,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorArrowLeft = 20```</span>
  IndicatorArrowLeft = 20,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorArrowRight = 21```</span>
  IndicatorArrowRight = 21,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorArrowUp = 22```</span>
  IndicatorArrowUp = 22,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorBranch = 23```</span>
  IndicatorBranch = 23,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorButtonDropDown = 24```</span>
  IndicatorButtonDropDown = 24,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```PE_IndicatorViewItemCheck = 25```</span>
  /// - <span style='color: green;'>```PE_IndicatorItemViewItemCheck = 25```</span>
  ///
  IndicatorViewItemCheck = 25,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorCheckBox = 26```</span>
  IndicatorCheckBox = 26,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorDockWidgetResizeHandle = 27```</span>
  IndicatorDockWidgetResizeHandle = 27,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorHeaderArrow = 28```</span>
  IndicatorHeaderArrow = 28,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorMenuCheckMark = 29```</span>
  IndicatorMenuCheckMark = 29,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorProgressChunk = 30```</span>
  IndicatorProgressChunk = 30,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorRadioButton = 31```</span>
  IndicatorRadioButton = 31,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorSpinDown = 32```</span>
  IndicatorSpinDown = 32,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorSpinMinus = 33```</span>
  IndicatorSpinMinus = 33,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorSpinPlus = 34```</span>
  IndicatorSpinPlus = 34,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorSpinUp = 35```</span>
  IndicatorSpinUp = 35,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorToolBarHandle = 36```</span>
  IndicatorToolBarHandle = 36,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorToolBarSeparator = 37```</span>
  IndicatorToolBarSeparator = 37,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelTipLabel = 38```</span>
  PanelTipLabel = 38,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```PE_IndicatorTabTear = 39```</span>
  /// - <span style='color: green;'>```PE_IndicatorTabTearLeft = 39```</span>
  ///
  IndicatorTabTear = 39,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelScrollAreaCorner = 40```</span>
  PanelScrollAreaCorner = 40,
  /// C++ enum variant: <span style='color: green;'>```PE_Widget = 41```</span>
  Widget = 41,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorColumnViewArrow = 42```</span>
  IndicatorColumnViewArrow = 42,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorItemViewItemDrop = 43```</span>
  IndicatorItemViewItemDrop = 43,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelItemViewItem = 44```</span>
  PanelItemViewItem = 44,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelItemViewRow = 45```</span>
  PanelItemViewRow = 45,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelStatusBar = 46```</span>
  PanelStatusBar = 46,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorTabClose = 47```</span>
  IndicatorTabClose = 47,
  /// C++ enum variant: <span style='color: green;'>```PE_PanelMenu = 48```</span>
  PanelMenu = 48,
  /// C++ enum variant: <span style='color: green;'>```PE_IndicatorTabTearRight = 49```</span>
  IndicatorTabTearRight = 49,
  /// C++ enum variant: <span style='color: green;'>```PE_CustomBase = 251658240```</span>
  CustomBase = 251658240,
}

/// C++ type: <span style='color: green;'>```QStyle::RequestSoftwareInputPanel```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RequestSoftwareInputPanel {
  /// C++ enum variant: <span style='color: green;'>```RSIP_OnMouseClickAndAlreadyFocused = 0```</span>
  RSIPOnMouseClickAndAlreadyFocused = 0,
  /// C++ enum variant: <span style='color: green;'>```RSIP_OnMouseClick = 1```</span>
  RSIPOnMouseClick = 1,
}

/// C++ type: <span style='color: green;'>```QStyle::StandardPixmap```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StandardPixmap {
  /// C++ enum variant: <span style='color: green;'>```SP_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarMenuButton = 0```</span>
  TitleBarMenuButton = 0,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarMinButton = 1```</span>
  TitleBarMinButton = 1,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarMaxButton = 2```</span>
  TitleBarMaxButton = 2,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarCloseButton = 3```</span>
  TitleBarCloseButton = 3,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarNormalButton = 4```</span>
  TitleBarNormalButton = 4,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarShadeButton = 5```</span>
  TitleBarShadeButton = 5,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarUnshadeButton = 6```</span>
  TitleBarUnshadeButton = 6,
  /// C++ enum variant: <span style='color: green;'>```SP_TitleBarContextHelpButton = 7```</span>
  TitleBarContextHelpButton = 7,
  /// C++ enum variant: <span style='color: green;'>```SP_DockWidgetCloseButton = 8```</span>
  DockWidgetCloseButton = 8,
  /// C++ enum variant: <span style='color: green;'>```SP_MessageBoxInformation = 9```</span>
  MessageBoxInformation = 9,
  /// C++ enum variant: <span style='color: green;'>```SP_MessageBoxWarning = 10```</span>
  MessageBoxWarning = 10,
  /// C++ enum variant: <span style='color: green;'>```SP_MessageBoxCritical = 11```</span>
  MessageBoxCritical = 11,
  /// C++ enum variant: <span style='color: green;'>```SP_MessageBoxQuestion = 12```</span>
  MessageBoxQuestion = 12,
  /// C++ enum variant: <span style='color: green;'>```SP_DesktopIcon = 13```</span>
  DesktopIcon = 13,
  /// C++ enum variant: <span style='color: green;'>```SP_TrashIcon = 14```</span>
  TrashIcon = 14,
  /// C++ enum variant: <span style='color: green;'>```SP_ComputerIcon = 15```</span>
  ComputerIcon = 15,
  /// C++ enum variant: <span style='color: green;'>```SP_DriveFDIcon = 16```</span>
  DriveFDIcon = 16,
  /// C++ enum variant: <span style='color: green;'>```SP_DriveHDIcon = 17```</span>
  DriveHDIcon = 17,
  /// C++ enum variant: <span style='color: green;'>```SP_DriveCDIcon = 18```</span>
  DriveCDIcon = 18,
  /// C++ enum variant: <span style='color: green;'>```SP_DriveDVDIcon = 19```</span>
  DriveDVDIcon = 19,
  /// C++ enum variant: <span style='color: green;'>```SP_DriveNetIcon = 20```</span>
  DriveNetIcon = 20,
  /// C++ enum variant: <span style='color: green;'>```SP_DirOpenIcon = 21```</span>
  DirOpenIcon = 21,
  /// C++ enum variant: <span style='color: green;'>```SP_DirClosedIcon = 22```</span>
  DirClosedIcon = 22,
  /// C++ enum variant: <span style='color: green;'>```SP_DirLinkIcon = 23```</span>
  DirLinkIcon = 23,
  /// C++ enum variant: <span style='color: green;'>```SP_DirLinkOpenIcon = 24```</span>
  DirLinkOpenIcon = 24,
  /// C++ enum variant: <span style='color: green;'>```SP_FileIcon = 25```</span>
  FileIcon = 25,
  /// C++ enum variant: <span style='color: green;'>```SP_FileLinkIcon = 26```</span>
  FileLinkIcon = 26,
  /// C++ enum variant: <span style='color: green;'>```SP_ToolBarHorizontalExtensionButton = 27```</span>
  ToolBarHorizontalExtensionButton = 27,
  /// C++ enum variant: <span style='color: green;'>```SP_ToolBarVerticalExtensionButton = 28```</span>
  ToolBarVerticalExtensionButton = 28,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogStart = 29```</span>
  FileDialogStart = 29,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogEnd = 30```</span>
  FileDialogEnd = 30,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogToParent = 31```</span>
  FileDialogToParent = 31,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogNewFolder = 32```</span>
  FileDialogNewFolder = 32,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogDetailedView = 33```</span>
  FileDialogDetailedView = 33,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogInfoView = 34```</span>
  FileDialogInfoView = 34,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogContentsView = 35```</span>
  FileDialogContentsView = 35,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogListView = 36```</span>
  FileDialogListView = 36,
  /// C++ enum variant: <span style='color: green;'>```SP_FileDialogBack = 37```</span>
  FileDialogBack = 37,
  /// C++ enum variant: <span style='color: green;'>```SP_DirIcon = 38```</span>
  DirIcon = 38,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogOkButton = 39```</span>
  DialogOkButton = 39,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogCancelButton = 40```</span>
  DialogCancelButton = 40,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogHelpButton = 41```</span>
  DialogHelpButton = 41,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogOpenButton = 42```</span>
  DialogOpenButton = 42,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogSaveButton = 43```</span>
  DialogSaveButton = 43,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogCloseButton = 44```</span>
  DialogCloseButton = 44,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogApplyButton = 45```</span>
  DialogApplyButton = 45,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogResetButton = 46```</span>
  DialogResetButton = 46,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogDiscardButton = 47```</span>
  DialogDiscardButton = 47,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogYesButton = 48```</span>
  DialogYesButton = 48,
  /// C++ enum variant: <span style='color: green;'>```SP_DialogNoButton = 49```</span>
  DialogNoButton = 49,
  /// C++ enum variant: <span style='color: green;'>```SP_ArrowUp = 50```</span>
  ArrowUp = 50,
  /// C++ enum variant: <span style='color: green;'>```SP_ArrowDown = 51```</span>
  ArrowDown = 51,
  /// C++ enum variant: <span style='color: green;'>```SP_ArrowLeft = 52```</span>
  ArrowLeft = 52,
  /// C++ enum variant: <span style='color: green;'>```SP_ArrowRight = 53```</span>
  ArrowRight = 53,
  /// C++ enum variant: <span style='color: green;'>```SP_ArrowBack = 54```</span>
  ArrowBack = 54,
  /// C++ enum variant: <span style='color: green;'>```SP_ArrowForward = 55```</span>
  ArrowForward = 55,
  /// C++ enum variant: <span style='color: green;'>```SP_DirHomeIcon = 56```</span>
  DirHomeIcon = 56,
  /// C++ enum variant: <span style='color: green;'>```SP_CommandLink = 57```</span>
  CommandLink = 57,
  /// C++ enum variant: <span style='color: green;'>```SP_VistaShield = 58```</span>
  VistaShield = 58,
  /// C++ enum variant: <span style='color: green;'>```SP_BrowserReload = 59```</span>
  BrowserReload = 59,
  /// C++ enum variant: <span style='color: green;'>```SP_BrowserStop = 60```</span>
  BrowserStop = 60,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaPlay = 61```</span>
  MediaPlay = 61,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaStop = 62```</span>
  MediaStop = 62,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaPause = 63```</span>
  MediaPause = 63,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaSkipForward = 64```</span>
  MediaSkipForward = 64,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaSkipBackward = 65```</span>
  MediaSkipBackward = 65,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaSeekForward = 66```</span>
  MediaSeekForward = 66,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaSeekBackward = 67```</span>
  MediaSeekBackward = 67,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaVolume = 68```</span>
  MediaVolume = 68,
  /// C++ enum variant: <span style='color: green;'>```SP_MediaVolumeMuted = 69```</span>
  MediaVolumeMuted = 69,
  /// C++ enum variant: <span style='color: green;'>```SP_LineEditClearButton = 70```</span>
  LineEditClearButton = 70,
}

/// C++ type: <span style='color: green;'>```QStyle::StateFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StateFlag {
  /// C++ enum variant: <span style='color: green;'>```State_None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```State_Enabled = 1```</span>
  Enabled = 1,
  /// C++ enum variant: <span style='color: green;'>```State_Raised = 2```</span>
  Raised = 2,
  /// C++ enum variant: <span style='color: green;'>```State_Sunken = 4```</span>
  Sunken = 4,
  /// C++ enum variant: <span style='color: green;'>```State_Off = 8```</span>
  Off = 8,
  /// C++ enum variant: <span style='color: green;'>```State_NoChange = 16```</span>
  NoChange = 16,
  /// C++ enum variant: <span style='color: green;'>```State_On = 32```</span>
  On = 32,
  /// C++ enum variant: <span style='color: green;'>```State_DownArrow = 64```</span>
  DownArrow = 64,
  /// C++ enum variant: <span style='color: green;'>```State_Horizontal = 128```</span>
  Horizontal = 128,
  /// C++ enum variant: <span style='color: green;'>```State_HasFocus = 256```</span>
  HasFocus = 256,
  /// C++ enum variant: <span style='color: green;'>```State_Top = 512```</span>
  Top = 512,
  /// C++ enum variant: <span style='color: green;'>```State_Bottom = 1024```</span>
  Bottom = 1024,
  /// C++ enum variant: <span style='color: green;'>```State_FocusAtBorder = 2048```</span>
  FocusAtBorder = 2048,
  /// C++ enum variant: <span style='color: green;'>```State_AutoRaise = 4096```</span>
  AutoRaise = 4096,
  /// C++ enum variant: <span style='color: green;'>```State_MouseOver = 8192```</span>
  MouseOver = 8192,
  /// C++ enum variant: <span style='color: green;'>```State_UpArrow = 16384```</span>
  UpArrow = 16384,
  /// C++ enum variant: <span style='color: green;'>```State_Selected = 32768```</span>
  Selected = 32768,
  /// C++ enum variant: <span style='color: green;'>```State_Active = 65536```</span>
  Active = 65536,
  /// C++ enum variant: <span style='color: green;'>```State_Window = 131072```</span>
  Window = 131072,
  /// C++ enum variant: <span style='color: green;'>```State_Open = 262144```</span>
  Open = 262144,
  /// C++ enum variant: <span style='color: green;'>```State_Children = 524288```</span>
  Children = 524288,
  /// C++ enum variant: <span style='color: green;'>```State_Item = 1048576```</span>
  Item = 1048576,
  /// C++ enum variant: <span style='color: green;'>```State_Sibling = 2097152```</span>
  Sibling = 2097152,
  /// C++ enum variant: <span style='color: green;'>```State_Editing = 4194304```</span>
  Editing = 4194304,
  /// C++ enum variant: <span style='color: green;'>```State_KeyboardFocusChange = 8388608```</span>
  KeyboardFocusChange = 8388608,
  /// C++ enum variant: <span style='color: green;'>```State_ReadOnly = 33554432```</span>
  ReadOnly = 33554432,
  /// C++ enum variant: <span style='color: green;'>```State_Small = 67108864```</span>
  Small = 67108864,
  /// C++ enum variant: <span style='color: green;'>```State_Mini = 134217728```</span>
  Mini = 134217728,
}

/// C++ type: <span style='color: green;'>```QStyle```</span>
#[repr(C)]
pub struct Style(u8);

impl Style {
  /// C++ method: <span style='color: green;'>```QStyle::drawComplexControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QStyle::drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QStyle::drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_complex_control<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::StyleDrawComplexControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStyle::drawControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_control(&self, (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QStyle::drawControl(QStyle::ControlElement element, const QStyleOption* opt, QPainter* p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_control(&self, (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QStyle::drawControl(QStyle::ControlElement element, const QStyleOption* opt, QPainter* p, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_control<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::StyleDrawControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QStyle::drawItemPixmap(QPainter* painter, const QRect& rect, int alignment, const QPixmap& pixmap) const```</span>
  ///
  ///
  pub unsafe fn draw_item_pixmap(&self,
                                 painter: *mut ::qt_gui::painter::Painter,
                                 rect: &::qt_core::rect::Rect,
                                 alignment: ::libc::c_int,
                                 pixmap: &::qt_gui::pixmap::Pixmap) {
    ::ffi::qt_widgets_c_QStyle_drawItemPixmap(self as *const ::style::Style,
                                              painter,
                                              rect as *const ::qt_core::rect::Rect,
                                              alignment,
                                              pixmap as *const ::qt_gui::pixmap::Pixmap)
  }

  /// C++ method: <span style='color: green;'>```QStyle::drawItemText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_item_text(&self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, ::libc::c_int, &::qt_gui::palette::Palette, bool, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStyle::drawItemText(QPainter* painter, const QRect& rect, int flags, const QPalette& pal, bool enabled, const QString& text) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_item_text(&self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, ::libc::c_int, &::qt_gui::palette::Palette, bool, &::qt_core::string::String, &::qt_gui::palette::ColorRole)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStyle::drawItemText(QPainter* painter, const QRect& rect, int flags, const QPalette& pal, bool enabled, const QString& text, QPalette::ColorRole textRole = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_item_text<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::StyleDrawItemTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStyle::drawPrimitive```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_primitive(&self, (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QStyle::drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_primitive(&self, (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QStyle::drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_primitive<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::StyleDrawPrimitiveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```pure virtual QPixmap QStyle::generatedIconPixmap(QIcon::Mode iconMode, const QPixmap& pixmap, const QStyleOption* opt) const```</span>
  ///
  ///
  pub unsafe fn generated_icon_pixmap(&self,
                                      icon_mode: &::qt_gui::icon::Mode,
                                      pixmap: &::qt_gui::pixmap::Pixmap,
                                      opt: *const ::style_option::StyleOption)
                                      -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result = ::ffi::qt_widgets_c_QStyle_generatedIconPixmap_as_ptr(self as *const ::style::Style,
                                                                           icon_mode as *const ::qt_gui::icon::Mode,
                                                                           pixmap as *const ::qt_gui::pixmap::Pixmap,
                                                                           opt);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QStyle::hitTestComplexControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn hit_test_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &::qt_core::point::Point)) -> ::style::SubControl```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QStyle::SubControl QStyle::hitTestComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint& pt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn hit_test_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &::qt_core::point::Point, *const ::widget::Widget)) -> ::style::SubControl```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QStyle::SubControl QStyle::hitTestComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint& pt, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn hit_test_complex_control<'largs, Args>(&'largs self, args: Args) -> ::style::SubControl
    where Args: overloading::StyleHitTestComplexControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QRect QStyle::itemPixmapRect(const QRect& r, int flags, const QPixmap& pixmap) const```</span>
  ///
  ///
  pub fn item_pixmap_rect(&self,
                          r: &::qt_core::rect::Rect,
                          flags: ::libc::c_int,
                          pixmap: &::qt_gui::pixmap::Pixmap)
                          -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStyle_itemPixmapRect_to_output(self as *const ::style::Style,
                                                            r as *const ::qt_core::rect::Rect,
                                                            flags,
                                                            pixmap as *const ::qt_gui::pixmap::Pixmap,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QStyle::itemTextRect(const QFontMetrics& fm, const QRect& r, int flags, bool enabled, const QString& text) const```</span>
  ///
  ///
  pub fn item_text_rect(&self,
                        fm: &::qt_gui::font_metrics::FontMetrics,
                        r: &::qt_core::rect::Rect,
                        flags: ::libc::c_int,
                        enabled: bool,
                        text: &::qt_core::string::String)
                        -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStyle_itemTextRect_to_output(self as *const ::style::Style,
                                                          fm as *const ::qt_gui::font_metrics::FontMetrics,
                                                          r as *const ::qt_core::rect::Rect,
                                                          flags,
                                                          enabled,
                                                          text as *const ::qt_core::string::String,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation) const```</span>
  ///
  ///
  pub fn layout_spacing(&self,
                        control1: &::size_policy::ControlType,
                        control2: &::size_policy::ControlType,
                        orientation: &::qt_core::qt::Orientation)
                        -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation(self as *const ::style::Style, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation) }
  }

  /// C++ method: <span style='color: green;'>```QStyle::layoutSpacing```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn layout_spacing_unsafe(&self, (&::size_policy::ControlType, &::size_policy::ControlType, &::qt_core::qt::Orientation, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn layout_spacing_unsafe(&self, (&::size_policy::ControlType, &::size_policy::ControlType, &::qt_core::qt::Orientation, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn layout_spacing_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StyleLayoutSpacingUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStyle::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QStyle_metaObject(self as *const ::style::Style) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::pixelMetric(QStyle::PixelMetric metric) const```</span>
  ///
  ///
  pub fn pixel_metric(&self, metric: ::style::PixelMetric) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyle_pixelMetric_metric(self as *const ::style::Style, metric) }
  }

  /// C++ method: <span style='color: green;'>```QStyle::pixelMetric```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixel_metric_unsafe(&self, (::style::PixelMetric, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::pixelMetric(QStyle::PixelMetric metric, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixel_metric_unsafe(&self, (::style::PixelMetric, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::pixelMetric(QStyle::PixelMetric metric, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn pixel_metric_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StylePixelMetricUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QStyle::polish(QPalette& palette)```</span>
  ///
  ///
  pub fn polish(&mut self, palette: &mut ::qt_gui::palette::Palette) {
    unsafe {
      ::ffi::qt_widgets_c_QStyle_polish_palette(self as *mut ::style::Style,
                                                palette as *mut ::qt_gui::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```QStyle::polish```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn polish_unsafe(&mut self, *mut ::application::Application) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStyle::polish(QApplication* application)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn polish_unsafe(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStyle::polish(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn polish_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StylePolishUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QStyle* QStyle::proxy() const```</span>
  ///
  ///
  pub fn proxy(&self) -> *const ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QStyle_proxy(self as *const ::style::Style) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QStyle::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStyle_qt_metacall(self as *mut ::style::Style,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QStyle::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QStyle_qt_metacast(self as *mut ::style::Style, arg1)
  }

  /// C++ method: <span style='color: green;'>```QStyle::sizeFromContents```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn size_from_contents(&self, (::style::ContentsType, *const ::style_option::StyleOption, &::qt_core::size::Size)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QSize QStyle::sizeFromContents(QStyle::ContentsType ct, const QStyleOption* opt, const QSize& contentsSize) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn size_from_contents(&self, (::style::ContentsType, *const ::style_option::StyleOption, &::qt_core::size::Size, *const ::widget::Widget)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QSize QStyle::sizeFromContents(QStyle::ContentsType ct, const QStyleOption* opt, const QSize& contentsSize, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn size_from_contents<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size::Size
    where Args: overloading::StyleSizeFromContentsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStyle::sliderPositionFromValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn slider_position_from_value((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStyle::sliderPositionFromValue(int min, int max, int val, int space)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn slider_position_from_value((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, bool)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown = ?)```</span>
  ///
  ///
  pub fn slider_position_from_value<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::StyleSliderPositionFromValueArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStyle::sliderValueFromPosition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn slider_value_from_position((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn slider_value_from_position((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, bool)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown = ?)```</span>
  ///
  ///
  pub fn slider_value_from_position<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::StyleSliderValueFromPositionArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```pure virtual QIcon QStyle::standardIcon(QStyle::StandardPixmap standardIcon) const```</span>
  ///
  ///
  pub fn standard_icon(&self, standard_icon: ::style::StandardPixmap) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStyle_standardIcon_to_output_standardIcon(self as *const ::style::Style,
                                                                       standard_icon,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStyle::standardIcon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn standard_icon_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption)) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QIcon QStyle::standardIcon(QStyle::StandardPixmap standardIcon, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn standard_icon_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QIcon QStyle::standardIcon(QStyle::StandardPixmap standardIcon, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn standard_icon_unsafe<'largs, Args>(&'largs self, args: Args) -> ::qt_gui::icon::Icon
    where Args: overloading::StyleStandardIconUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QPalette QStyle::standardPalette() const```</span>
  ///
  ///
  pub fn standard_palette(&self) -> ::qt_gui::palette::Palette {
    {
      let mut object: ::qt_gui::palette::Palette =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStyle_standardPalette_to_output(self as *const ::style::Style, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QPixmap QStyle::standardPixmap(QStyle::StandardPixmap standardPixmap) const```</span>
  ///
  ///
  pub fn standard_pixmap(&self,
                         standard_pixmap: ::style::StandardPixmap)
                         -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap(self as *const ::style::Style, standard_pixmap)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QStyle::standardPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn standard_pixmap_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption)) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QPixmap QStyle::standardPixmap(QStyle::StandardPixmap standardPixmap, const QStyleOption* opt = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn standard_pixmap_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QPixmap QStyle::standardPixmap(QStyle::StandardPixmap standardPixmap, const QStyleOption* opt = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn standard_pixmap_unsafe<'largs, Args>(&'largs self,
                                                     args: Args)
                                                     -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>
    where Args: overloading::StyleStandardPixmapUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::styleHint(QStyle::StyleHint stylehint) const```</span>
  ///
  ///
  pub fn style_hint(&self, stylehint: ::style::StyleHint) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyle_styleHint_stylehint(self as *const ::style::Style, stylehint) }
  }

  /// C++ method: <span style='color: green;'>```QStyle::styleHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::styleHint(QStyle::StyleHint stylehint, const QStyleOption* opt = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::styleHint(QStyle::StyleHint stylehint, const QStyleOption* opt = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget, *mut ::style_hint_return::StyleHintReturn)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QStyle::styleHint(QStyle::StyleHint stylehint, const QStyleOption* opt = ?, const QWidget* widget = ?, QStyleHintReturn* returnData = ?) const```</span>
  ///
  ///
  pub unsafe fn style_hint_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StyleStyleHintUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStyle::subControlRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sub_control_rect(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, ::style::SubControl)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QRect QStyle::subControlRect(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sub_control_rect(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, ::style::SubControl, *const ::widget::Widget)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QRect QStyle::subControlRect(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn sub_control_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::StyleSubControlRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStyle::subElementRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sub_element_rect(&self, (::style::SubElement, *const ::style_option::StyleOption)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QRect QStyle::subElementRect(QStyle::SubElement subElement, const QStyleOption* option) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sub_element_rect(&self, (::style::SubElement, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QRect QStyle::subElementRect(QStyle::SubElement subElement, const QStyleOption* option, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn sub_element_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::StyleSubElementRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QStyle::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStyle_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStyle::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStyle_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStyle::unpolish```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn unpolish(&mut self, *mut ::application::Application) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStyle::unpolish(QApplication* application)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn unpolish(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStyle::unpolish(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn unpolish<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StyleUnpolishArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QPoint QStyle::visualPos(Qt::LayoutDirection direction, const QRect& boundingRect, const QPoint& logicalPos)```</span>
  ///
  ///
  pub fn visual_pos(direction: &::qt_core::qt::LayoutDirection,
                    bounding_rect: &::qt_core::rect::Rect,
                    logical_pos: &::qt_core::point::Point)
                    -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStyle_visualPos_to_output(direction as *const ::qt_core::qt::LayoutDirection,
                                                       bounding_rect as *const ::qt_core::rect::Rect,
                                                       logical_pos as *const ::qt_core::point::Point,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QRect QStyle::visualRect(Qt::LayoutDirection direction, const QRect& boundingRect, const QRect& logicalRect)```</span>
  ///
  ///
  pub fn visual_rect(direction: &::qt_core::qt::LayoutDirection,
                     bounding_rect: &::qt_core::rect::Rect,
                     logical_rect: &::qt_core::rect::Rect)
                     -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStyle_visualRect_to_output(direction as *const ::qt_core::qt::LayoutDirection,
                                                        bounding_rect as *const ::qt_core::rect::Rect,
                                                        logical_rect as *const ::qt_core::rect::Rect,
                                                        &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style::Style {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyle_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Style`.
  pub struct Signals<'a>(&'a ::style::Style);
  /// Represents a built-in Qt signal `QStyle::objectNameChanged`.
  ///
  /// An object of this type can be created from `Style` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Style` object.
  pub struct ObjectNameChanged<'a>(&'a ::style::Style);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QStyle::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::style::Style {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QStyle::StyleHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleHint {
  /// C++ enum variant: <span style='color: green;'>```SH_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```SH_EtchDisabledText = 0```</span>
  EtchDisabledText = 0,
  /// C++ enum variant: <span style='color: green;'>```SH_DitherDisabledText = 1```</span>
  DitherDisabledText = 1,
  /// C++ enum variant: <span style='color: green;'>```SH_ScrollBar_MiddleClickAbsolutePosition = 2```</span>
  ScrollBarMiddleClickAbsolutePosition = 2,
  /// C++ enum variant: <span style='color: green;'>```SH_ScrollBar_ScrollWhenPointerLeavesControl = 3```</span>
  ScrollBarScrollWhenPointerLeavesControl = 3,
  /// C++ enum variant: <span style='color: green;'>```SH_TabBar_SelectMouseType = 4```</span>
  TabBarSelectMouseType = 4,
  /// C++ enum variant: <span style='color: green;'>```SH_TabBar_Alignment = 5```</span>
  TabBarAlignment = 5,
  /// C++ enum variant: <span style='color: green;'>```SH_Header_ArrowAlignment = 6```</span>
  HeaderArrowAlignment = 6,
  /// C++ enum variant: <span style='color: green;'>```SH_Slider_SnapToValue = 7```</span>
  SliderSnapToValue = 7,
  /// C++ enum variant: <span style='color: green;'>```SH_Slider_SloppyKeyEvents = 8```</span>
  SliderSloppyKeyEvents = 8,
  /// C++ enum variant: <span style='color: green;'>```SH_ProgressDialog_CenterCancelButton = 9```</span>
  ProgressDialogCenterCancelButton = 9,
  /// C++ enum variant: <span style='color: green;'>```SH_ProgressDialog_TextLabelAlignment = 10```</span>
  ProgressDialogTextLabelAlignment = 10,
  /// C++ enum variant: <span style='color: green;'>```SH_PrintDialog_RightAlignButtons = 11```</span>
  PrintDialogRightAlignButtons = 11,
  /// C++ enum variant: <span style='color: green;'>```SH_MainWindow_SpaceBelowMenuBar = 12```</span>
  MainWindowSpaceBelowMenuBar = 12,
  /// C++ enum variant: <span style='color: green;'>```SH_FontDialog_SelectAssociatedText = 13```</span>
  FontDialogSelectAssociatedText = 13,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_AllowActiveAndDisabled = 14```</span>
  MenuAllowActiveAndDisabled = 14,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SpaceActivatesItem = 15```</span>
  MenuSpaceActivatesItem = 15,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SubMenuPopupDelay = 16```</span>
  MenuSubMenuPopupDelay = 16,
  /// C++ enum variant: <span style='color: green;'>```SH_ScrollView_FrameOnlyAroundContents = 17```</span>
  ScrollViewFrameOnlyAroundContents = 17,
  /// C++ enum variant: <span style='color: green;'>```SH_MenuBar_AltKeyNavigation = 18```</span>
  MenuBarAltKeyNavigation = 18,
  /// C++ enum variant: <span style='color: green;'>```SH_ComboBox_ListMouseTracking = 19```</span>
  ComboBoxListMouseTracking = 19,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_MouseTracking = 20```</span>
  MenuMouseTracking = 20,
  /// C++ enum variant: <span style='color: green;'>```SH_MenuBar_MouseTracking = 21```</span>
  MenuBarMouseTracking = 21,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_ChangeHighlightOnFocus = 22```</span>
  ItemViewChangeHighlightOnFocus = 22,
  /// C++ enum variant: <span style='color: green;'>```SH_Widget_ShareActivation = 23```</span>
  WidgetShareActivation = 23,
  /// C++ enum variant: <span style='color: green;'>```SH_Workspace_FillSpaceOnMaximize = 24```</span>
  WorkspaceFillSpaceOnMaximize = 24,
  /// C++ enum variant: <span style='color: green;'>```SH_ComboBox_Popup = 25```</span>
  ComboBoxPopup = 25,
  /// C++ enum variant: <span style='color: green;'>```SH_TitleBar_NoBorder = 26```</span>
  TitleBarNoBorder = 26,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SH_Slider_StopMouseOverSlider = 27```</span>
  /// - <span style='color: green;'>```SH_ScrollBar_StopMouseOverSlider = 27```</span>
  ///
  SliderStopMouseOverSlider = 27,
  /// C++ enum variant: <span style='color: green;'>```SH_BlinkCursorWhenTextSelected = 28```</span>
  BlinkCursorWhenTextSelected = 28,
  /// C++ enum variant: <span style='color: green;'>```SH_RichText_FullWidthSelection = 29```</span>
  RichTextFullWidthSelection = 29,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_Scrollable = 30```</span>
  MenuScrollable = 30,
  /// C++ enum variant: <span style='color: green;'>```SH_GroupBox_TextLabelVerticalAlignment = 31```</span>
  GroupBoxTextLabelVerticalAlignment = 31,
  /// C++ enum variant: <span style='color: green;'>```SH_GroupBox_TextLabelColor = 32```</span>
  GroupBoxTextLabelColor = 32,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SloppySubMenus = 33```</span>
  MenuSloppySubMenus = 33,
  /// C++ enum variant: <span style='color: green;'>```SH_Table_GridLineColor = 34```</span>
  TableGridLineColor = 34,
  /// C++ enum variant: <span style='color: green;'>```SH_LineEdit_PasswordCharacter = 35```</span>
  LineEditPasswordCharacter = 35,
  /// C++ enum variant: <span style='color: green;'>```SH_DialogButtons_DefaultButton = 36```</span>
  DialogButtonsDefaultButton = 36,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolBox_SelectedPageTitleBold = 37```</span>
  ToolBoxSelectedPageTitleBold = 37,
  /// C++ enum variant: <span style='color: green;'>```SH_TabBar_PreferNoArrows = 38```</span>
  TabBarPreferNoArrows = 38,
  /// C++ enum variant: <span style='color: green;'>```SH_ScrollBar_LeftClickAbsolutePosition = 39```</span>
  ScrollBarLeftClickAbsolutePosition = 39,
  /// C++ enum variant: <span style='color: green;'>```SH_ListViewExpand_SelectMouseType = 40```</span>
  ListViewExpandSelectMouseType = 40,
  /// C++ enum variant: <span style='color: green;'>```SH_UnderlineShortcut = 41```</span>
  UnderlineShortcut = 41,
  /// C++ enum variant: <span style='color: green;'>```SH_SpinBox_AnimateButton = 42```</span>
  SpinBoxAnimateButton = 42,
  /// C++ enum variant: <span style='color: green;'>```SH_SpinBox_KeyPressAutoRepeatRate = 43```</span>
  SpinBoxKeyPressAutoRepeatRate = 43,
  /// C++ enum variant: <span style='color: green;'>```SH_SpinBox_ClickAutoRepeatRate = 44```</span>
  SpinBoxClickAutoRepeatRate = 44,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_FillScreenWithScroll = 45```</span>
  MenuFillScreenWithScroll = 45,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolTipLabel_Opacity = 46```</span>
  ToolTipLabelOpacity = 46,
  /// C++ enum variant: <span style='color: green;'>```SH_DrawMenuBarSeparator = 47```</span>
  DrawMenuBarSeparator = 47,
  /// C++ enum variant: <span style='color: green;'>```SH_TitleBar_ModifyNotification = 48```</span>
  TitleBarModifyNotification = 48,
  /// C++ enum variant: <span style='color: green;'>```SH_Button_FocusPolicy = 49```</span>
  ButtonFocusPolicy = 49,
  /// C++ enum variant: <span style='color: green;'>```SH_MessageBox_UseBorderForButtonSpacing = 50```</span>
  MessageBoxUseBorderForButtonSpacing = 50,
  /// C++ enum variant: <span style='color: green;'>```SH_TitleBar_AutoRaise = 51```</span>
  TitleBarAutoRaise = 51,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolButton_PopupDelay = 52```</span>
  ToolButtonPopupDelay = 52,
  /// C++ enum variant: <span style='color: green;'>```SH_FocusFrame_Mask = 53```</span>
  FocusFrameMask = 53,
  /// C++ enum variant: <span style='color: green;'>```SH_RubberBand_Mask = 54```</span>
  RubberBandMask = 54,
  /// C++ enum variant: <span style='color: green;'>```SH_WindowFrame_Mask = 55```</span>
  WindowFrameMask = 55,
  /// C++ enum variant: <span style='color: green;'>```SH_SpinControls_DisableOnBounds = 56```</span>
  SpinControlsDisableOnBounds = 56,
  /// C++ enum variant: <span style='color: green;'>```SH_Dial_BackgroundRole = 57```</span>
  DialBackgroundRole = 57,
  /// C++ enum variant: <span style='color: green;'>```SH_ComboBox_LayoutDirection = 58```</span>
  ComboBoxLayoutDirection = 58,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_EllipsisLocation = 59```</span>
  ItemViewEllipsisLocation = 59,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_ShowDecorationSelected = 60```</span>
  ItemViewShowDecorationSelected = 60,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_ActivateItemOnSingleClick = 61```</span>
  ItemViewActivateItemOnSingleClick = 61,
  /// C++ enum variant: <span style='color: green;'>```SH_ScrollBar_ContextMenu = 62```</span>
  ScrollBarContextMenu = 62,
  /// C++ enum variant: <span style='color: green;'>```SH_ScrollBar_RollBetweenButtons = 63```</span>
  ScrollBarRollBetweenButtons = 63,
  /// C++ enum variant: <span style='color: green;'>```SH_Slider_AbsoluteSetButtons = 64```</span>
  SliderAbsoluteSetButtons = 64,
  /// C++ enum variant: <span style='color: green;'>```SH_Slider_PageSetButtons = 65```</span>
  SliderPageSetButtons = 65,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_KeyboardSearch = 66```</span>
  MenuKeyboardSearch = 66,
  /// C++ enum variant: <span style='color: green;'>```SH_TabBar_ElideMode = 67```</span>
  TabBarElideMode = 67,
  /// C++ enum variant: <span style='color: green;'>```SH_DialogButtonLayout = 68```</span>
  DialogButtonLayout = 68,
  /// C++ enum variant: <span style='color: green;'>```SH_ComboBox_PopupFrameStyle = 69```</span>
  ComboBoxPopupFrameStyle = 69,
  /// C++ enum variant: <span style='color: green;'>```SH_MessageBox_TextInteractionFlags = 70```</span>
  MessageBoxTextInteractionFlags = 70,
  /// C++ enum variant: <span style='color: green;'>```SH_DialogButtonBox_ButtonsHaveIcons = 71```</span>
  DialogButtonBoxButtonsHaveIcons = 71,
  /// C++ enum variant: <span style='color: green;'>```SH_SpellCheckUnderlineStyle = 72```</span>
  SpellCheckUnderlineStyle = 72,
  /// C++ enum variant: <span style='color: green;'>```SH_MessageBox_CenterButtons = 73```</span>
  MessageBoxCenterButtons = 73,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SelectionWrap = 74```</span>
  MenuSelectionWrap = 74,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_MovementWithoutUpdatingSelection = 75```</span>
  ItemViewMovementWithoutUpdatingSelection = 75,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolTip_Mask = 76```</span>
  ToolTipMask = 76,
  /// C++ enum variant: <span style='color: green;'>```SH_FocusFrame_AboveWidget = 77```</span>
  FocusFrameAboveWidget = 77,
  /// C++ enum variant: <span style='color: green;'>```SH_TextControl_FocusIndicatorTextCharFormat = 78```</span>
  TextControlFocusIndicatorTextCharFormat = 78,
  /// C++ enum variant: <span style='color: green;'>```SH_WizardStyle = 79```</span>
  WizardStyle = 79,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_ArrowKeysNavigateIntoChildren = 80```</span>
  ItemViewArrowKeysNavigateIntoChildren = 80,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_Mask = 81```</span>
  MenuMask = 81,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_FlashTriggeredItem = 82```</span>
  MenuFlashTriggeredItem = 82,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_FadeOutOnHide = 83```</span>
  MenuFadeOutOnHide = 83,
  /// C++ enum variant: <span style='color: green;'>```SH_SpinBox_ClickAutoRepeatThreshold = 84```</span>
  SpinBoxClickAutoRepeatThreshold = 84,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_PaintAlternatingRowColorsForEmptyArea = 85```</span>
  ItemViewPaintAlternatingRowColorsForEmptyArea = 85,
  /// C++ enum variant: <span style='color: green;'>```SH_FormLayoutWrapPolicy = 86```</span>
  FormLayoutWrapPolicy = 86,
  /// C++ enum variant: <span style='color: green;'>```SH_TabWidget_DefaultTabPosition = 87```</span>
  TabWidgetDefaultTabPosition = 87,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolBar_Movable = 88```</span>
  ToolBarMovable = 88,
  /// C++ enum variant: <span style='color: green;'>```SH_FormLayoutFieldGrowthPolicy = 89```</span>
  FormLayoutFieldGrowthPolicy = 89,
  /// C++ enum variant: <span style='color: green;'>```SH_FormLayoutFormAlignment = 90```</span>
  FormLayoutFormAlignment = 90,
  /// C++ enum variant: <span style='color: green;'>```SH_FormLayoutLabelAlignment = 91```</span>
  FormLayoutLabelAlignment = 91,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_DrawDelegateFrame = 92```</span>
  ItemViewDrawDelegateFrame = 92,
  /// C++ enum variant: <span style='color: green;'>```SH_TabBar_CloseButtonPosition = 93```</span>
  TabBarCloseButtonPosition = 93,
  /// C++ enum variant: <span style='color: green;'>```SH_DockWidget_ButtonsHaveFrame = 94```</span>
  DockWidgetButtonsHaveFrame = 94,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolButtonStyle = 95```</span>
  ToolButtonStyle = 95,
  /// C++ enum variant: <span style='color: green;'>```SH_RequestSoftwareInputPanel = 96```</span>
  RequestSoftwareInputPanel = 96,
  /// C++ enum variant: <span style='color: green;'>```SH_ScrollBar_Transient = 97```</span>
  ScrollBarTransient = 97,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SupportsSections = 98```</span>
  MenuSupportsSections = 98,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolTip_WakeUpDelay = 99```</span>
  ToolTipWakeUpDelay = 99,
  /// C++ enum variant: <span style='color: green;'>```SH_ToolTip_FallAsleepDelay = 100```</span>
  ToolTipFallAsleepDelay = 100,
  /// C++ enum variant: <span style='color: green;'>```SH_Widget_Animate = 101```</span>
  WidgetAnimate = 101,
  /// C++ enum variant: <span style='color: green;'>```SH_Splitter_OpaqueResize = 102```</span>
  SplitterOpaqueResize = 102,
  /// C++ enum variant: <span style='color: green;'>```SH_ComboBox_UseNativePopup = 103```</span>
  ComboBoxUseNativePopup = 103,
  /// C++ enum variant: <span style='color: green;'>```SH_LineEdit_PasswordMaskDelay = 104```</span>
  LineEditPasswordMaskDelay = 104,
  /// C++ enum variant: <span style='color: green;'>```SH_TabBar_ChangeCurrentDelay = 105```</span>
  TabBarChangeCurrentDelay = 105,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SubMenuUniDirection = 106```</span>
  MenuSubMenuUniDirection = 106,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SubMenuUniDirectionFailCount = 107```</span>
  MenuSubMenuUniDirectionFailCount = 107,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SubMenuSloppySelectOtherActions = 108```</span>
  MenuSubMenuSloppySelectOtherActions = 108,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SubMenuSloppyCloseTimeout = 109```</span>
  MenuSubMenuSloppyCloseTimeout = 109,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SubMenuResetWhenReenteringParent = 110```</span>
  MenuSubMenuResetWhenReenteringParent = 110,
  /// C++ enum variant: <span style='color: green;'>```SH_Menu_SubMenuDontStartSloppyOnLeave = 111```</span>
  MenuSubMenuDontStartSloppyOnLeave = 111,
  /// C++ enum variant: <span style='color: green;'>```SH_ItemView_ScrollMode = 112```</span>
  ItemViewScrollMode = 112,
}

/// C++ type: <span style='color: green;'>```QStyle::SubControl```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SubControl {
  /// C++ enum variant: <span style='color: green;'>```SC_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```SC_All = -1```</span>
  All = -1,
  /// C++ enum variant: <span style='color: green;'>```SC_None = 0```</span>
  None = 0,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarAddLine = 1```</span>
  /// - <span style='color: green;'>```SC_SpinBoxUp = 1```</span>
  /// - <span style='color: green;'>```SC_ComboBoxFrame = 1```</span>
  /// - <span style='color: green;'>```SC_SliderGroove = 1```</span>
  /// - <span style='color: green;'>```SC_ToolButton = 1```</span>
  /// - <span style='color: green;'>```SC_TitleBarSysMenu = 1```</span>
  /// - <span style='color: green;'>```SC_DialGroove = 1```</span>
  /// - <span style='color: green;'>```SC_GroupBoxCheckBox = 1```</span>
  /// - <span style='color: green;'>```SC_MdiMinButton = 1```</span>
  ///
  ScrollBarAddLine = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarSubLine = 2```</span>
  /// - <span style='color: green;'>```SC_SpinBoxDown = 2```</span>
  /// - <span style='color: green;'>```SC_ComboBoxEditField = 2```</span>
  /// - <span style='color: green;'>```SC_SliderHandle = 2```</span>
  /// - <span style='color: green;'>```SC_ToolButtonMenu = 2```</span>
  /// - <span style='color: green;'>```SC_TitleBarMinButton = 2```</span>
  /// - <span style='color: green;'>```SC_DialHandle = 2```</span>
  /// - <span style='color: green;'>```SC_GroupBoxLabel = 2```</span>
  /// - <span style='color: green;'>```SC_MdiNormalButton = 2```</span>
  ///
  ScrollBarSubLine = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarAddPage = 4```</span>
  /// - <span style='color: green;'>```SC_SpinBoxFrame = 4```</span>
  /// - <span style='color: green;'>```SC_ComboBoxArrow = 4```</span>
  /// - <span style='color: green;'>```SC_SliderTickmarks = 4```</span>
  /// - <span style='color: green;'>```SC_TitleBarMaxButton = 4```</span>
  /// - <span style='color: green;'>```SC_DialTickmarks = 4```</span>
  /// - <span style='color: green;'>```SC_GroupBoxContents = 4```</span>
  /// - <span style='color: green;'>```SC_MdiCloseButton = 4```</span>
  ///
  ScrollBarAddPage = 4,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarSubPage = 8```</span>
  /// - <span style='color: green;'>```SC_SpinBoxEditField = 8```</span>
  /// - <span style='color: green;'>```SC_ComboBoxListBoxPopup = 8```</span>
  /// - <span style='color: green;'>```SC_TitleBarCloseButton = 8```</span>
  /// - <span style='color: green;'>```SC_GroupBoxFrame = 8```</span>
  ///
  ScrollBarSubPage = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarFirst = 16```</span>
  /// - <span style='color: green;'>```SC_TitleBarNormalButton = 16```</span>
  ///
  ScrollBarFirst = 16,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarLast = 32```</span>
  /// - <span style='color: green;'>```SC_TitleBarShadeButton = 32```</span>
  ///
  ScrollBarLast = 32,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarSlider = 64```</span>
  /// - <span style='color: green;'>```SC_TitleBarUnshadeButton = 64```</span>
  ///
  ScrollBarSlider = 64,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SC_ScrollBarGroove = 128```</span>
  /// - <span style='color: green;'>```SC_TitleBarContextHelpButton = 128```</span>
  ///
  ScrollBarGroove = 128,
  /// C++ enum variant: <span style='color: green;'>```SC_TitleBarLabel = 256```</span>
  TitleBarLabel = 256,
}

/// C++ type: <span style='color: green;'>```QStyle::SubElement```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SubElement {
  /// C++ enum variant: <span style='color: green;'>```SE_CustomBase = -268435456```</span>
  CustomBase = -268435456,
  /// C++ enum variant: <span style='color: green;'>```SE_PushButtonContents = 0```</span>
  PushButtonContents = 0,
  /// C++ enum variant: <span style='color: green;'>```SE_PushButtonFocusRect = 1```</span>
  PushButtonFocusRect = 1,
  /// C++ enum variant: <span style='color: green;'>```SE_CheckBoxIndicator = 2```</span>
  CheckBoxIndicator = 2,
  /// C++ enum variant: <span style='color: green;'>```SE_CheckBoxContents = 3```</span>
  CheckBoxContents = 3,
  /// C++ enum variant: <span style='color: green;'>```SE_CheckBoxFocusRect = 4```</span>
  CheckBoxFocusRect = 4,
  /// C++ enum variant: <span style='color: green;'>```SE_CheckBoxClickRect = 5```</span>
  CheckBoxClickRect = 5,
  /// C++ enum variant: <span style='color: green;'>```SE_RadioButtonIndicator = 6```</span>
  RadioButtonIndicator = 6,
  /// C++ enum variant: <span style='color: green;'>```SE_RadioButtonContents = 7```</span>
  RadioButtonContents = 7,
  /// C++ enum variant: <span style='color: green;'>```SE_RadioButtonFocusRect = 8```</span>
  RadioButtonFocusRect = 8,
  /// C++ enum variant: <span style='color: green;'>```SE_RadioButtonClickRect = 9```</span>
  RadioButtonClickRect = 9,
  /// C++ enum variant: <span style='color: green;'>```SE_ComboBoxFocusRect = 10```</span>
  ComboBoxFocusRect = 10,
  /// C++ enum variant: <span style='color: green;'>```SE_SliderFocusRect = 11```</span>
  SliderFocusRect = 11,
  /// C++ enum variant: <span style='color: green;'>```SE_ProgressBarGroove = 12```</span>
  ProgressBarGroove = 12,
  /// C++ enum variant: <span style='color: green;'>```SE_ProgressBarContents = 13```</span>
  ProgressBarContents = 13,
  /// C++ enum variant: <span style='color: green;'>```SE_ProgressBarLabel = 14```</span>
  ProgressBarLabel = 14,
  /// C++ enum variant: <span style='color: green;'>```SE_ToolBoxTabContents = 15```</span>
  ToolBoxTabContents = 15,
  /// C++ enum variant: <span style='color: green;'>```SE_HeaderLabel = 16```</span>
  HeaderLabel = 16,
  /// C++ enum variant: <span style='color: green;'>```SE_HeaderArrow = 17```</span>
  HeaderArrow = 17,
  /// C++ enum variant: <span style='color: green;'>```SE_TabWidgetTabBar = 18```</span>
  TabWidgetTabBar = 18,
  /// C++ enum variant: <span style='color: green;'>```SE_TabWidgetTabPane = 19```</span>
  TabWidgetTabPane = 19,
  /// C++ enum variant: <span style='color: green;'>```SE_TabWidgetTabContents = 20```</span>
  TabWidgetTabContents = 20,
  /// C++ enum variant: <span style='color: green;'>```SE_TabWidgetLeftCorner = 21```</span>
  TabWidgetLeftCorner = 21,
  /// C++ enum variant: <span style='color: green;'>```SE_TabWidgetRightCorner = 22```</span>
  TabWidgetRightCorner = 22,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SE_ViewItemCheckIndicator = 23```</span>
  /// - <span style='color: green;'>```SE_ItemViewItemCheckIndicator = 23```</span>
  ///
  ViewItemCheckIndicator = 23,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```SE_TabBarTearIndicator = 24```</span>
  /// - <span style='color: green;'>```SE_TabBarTearIndicatorLeft = 24```</span>
  ///
  TabBarTearIndicator = 24,
  /// C++ enum variant: <span style='color: green;'>```SE_TreeViewDisclosureItem = 25```</span>
  TreeViewDisclosureItem = 25,
  /// C++ enum variant: <span style='color: green;'>```SE_LineEditContents = 26```</span>
  LineEditContents = 26,
  /// C++ enum variant: <span style='color: green;'>```SE_FrameContents = 27```</span>
  FrameContents = 27,
  /// C++ enum variant: <span style='color: green;'>```SE_DockWidgetCloseButton = 28```</span>
  DockWidgetCloseButton = 28,
  /// C++ enum variant: <span style='color: green;'>```SE_DockWidgetFloatButton = 29```</span>
  DockWidgetFloatButton = 29,
  /// C++ enum variant: <span style='color: green;'>```SE_DockWidgetTitleBarText = 30```</span>
  DockWidgetTitleBarText = 30,
  /// C++ enum variant: <span style='color: green;'>```SE_DockWidgetIcon = 31```</span>
  DockWidgetIcon = 31,
  /// C++ enum variant: <span style='color: green;'>```SE_CheckBoxLayoutItem = 32```</span>
  CheckBoxLayoutItem = 32,
  /// C++ enum variant: <span style='color: green;'>```SE_ComboBoxLayoutItem = 33```</span>
  ComboBoxLayoutItem = 33,
  /// C++ enum variant: <span style='color: green;'>```SE_DateTimeEditLayoutItem = 34```</span>
  DateTimeEditLayoutItem = 34,
  /// C++ enum variant: <span style='color: green;'>```SE_DialogButtonBoxLayoutItem = 35```</span>
  DialogButtonBoxLayoutItem = 35,
  /// C++ enum variant: <span style='color: green;'>```SE_LabelLayoutItem = 36```</span>
  LabelLayoutItem = 36,
  /// C++ enum variant: <span style='color: green;'>```SE_ProgressBarLayoutItem = 37```</span>
  ProgressBarLayoutItem = 37,
  /// C++ enum variant: <span style='color: green;'>```SE_PushButtonLayoutItem = 38```</span>
  PushButtonLayoutItem = 38,
  /// C++ enum variant: <span style='color: green;'>```SE_RadioButtonLayoutItem = 39```</span>
  RadioButtonLayoutItem = 39,
  /// C++ enum variant: <span style='color: green;'>```SE_SliderLayoutItem = 40```</span>
  SliderLayoutItem = 40,
  /// C++ enum variant: <span style='color: green;'>```SE_SpinBoxLayoutItem = 41```</span>
  SpinBoxLayoutItem = 41,
  /// C++ enum variant: <span style='color: green;'>```SE_ToolButtonLayoutItem = 42```</span>
  ToolButtonLayoutItem = 42,
  /// C++ enum variant: <span style='color: green;'>```SE_FrameLayoutItem = 43```</span>
  FrameLayoutItem = 43,
  /// C++ enum variant: <span style='color: green;'>```SE_GroupBoxLayoutItem = 44```</span>
  GroupBoxLayoutItem = 44,
  /// C++ enum variant: <span style='color: green;'>```SE_TabWidgetLayoutItem = 45```</span>
  TabWidgetLayoutItem = 45,
  /// C++ enum variant: <span style='color: green;'>```SE_ItemViewItemDecoration = 46```</span>
  ItemViewItemDecoration = 46,
  /// C++ enum variant: <span style='color: green;'>```SE_ItemViewItemText = 47```</span>
  ItemViewItemText = 47,
  /// C++ enum variant: <span style='color: green;'>```SE_ItemViewItemFocusRect = 48```</span>
  ItemViewItemFocusRect = 48,
  /// C++ enum variant: <span style='color: green;'>```SE_TabBarTabLeftButton = 49```</span>
  TabBarTabLeftButton = 49,
  /// C++ enum variant: <span style='color: green;'>```SE_TabBarTabRightButton = 50```</span>
  TabBarTabRightButton = 50,
  /// C++ enum variant: <span style='color: green;'>```SE_TabBarTabText = 51```</span>
  TabBarTabText = 51,
  /// C++ enum variant: <span style='color: green;'>```SE_ShapedFrameContents = 52```</span>
  ShapedFrameContents = 52,
  /// C++ enum variant: <span style='color: green;'>```SE_ToolBarHandle = 53```</span>
  ToolBarHandle = 53,
  /// C++ enum variant: <span style='color: green;'>```SE_TabBarScrollLeftButton = 54```</span>
  TabBarScrollLeftButton = 54,
  /// C++ enum variant: <span style='color: green;'>```SE_TabBarScrollRightButton = 55```</span>
  TabBarScrollRightButton = 55,
  /// C++ enum variant: <span style='color: green;'>```SE_TabBarTearIndicatorRight = 56```</span>
  TabBarTearIndicatorRight = 56,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::style::Style {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyle_G_static_cast_QObject_ptr(self as *mut ::style::Style) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyle_G_static_cast_QObject_ptr(self as *const ::style::Style as *mut ::style::Style)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style::Style> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style::Style {
    let ffi_result = ::ffi::qt_widgets_c_QStyle_G_static_cast_QStyle_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style::Style {
    let ffi_result = ::ffi::qt_widgets_c_QStyle_G_static_cast_QStyle_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style::Style {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyle_G_static_cast_QObject_ptr(self as *const ::style::Style as *mut ::style::Style)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style::Style {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyle_G_static_cast_QObject_ptr(self as *mut ::style::Style) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Style::draw_complex_control](../struct.Style.html#method.draw_complex_control) method.
  pub trait StyleDrawComplexControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ();
  }
  impl<'largs> StyleDrawComplexControlArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let cc = self.0;
      let opt = self.1;
      let p = self.2;
      ::ffi::qt_widgets_c_QStyle_drawComplexControl_cc_opt_p(original_self as *const ::style::Style, cc, opt, p)
    }
  }
  impl<'largs> StyleDrawComplexControlArgs<'largs>
    for (::style::ComplexControl,
                                                            *const ::style_option_complex::StyleOptionComplex,
                                                            *mut ::qt_gui::painter::Painter,
                                                            *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let cc = self.0;
      let opt = self.1;
      let p = self.2;
      let widget = self.3;
      ::ffi::qt_widgets_c_QStyle_drawComplexControl_cc_opt_p_widget(original_self as *const ::style::Style,
                                                                    cc,
                                                                    opt,
                                                                    p,
                                                                    widget)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::draw_control](../struct.Style.html#method.draw_control) method.
  pub trait StyleDrawControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ();
  }
  impl<'largs> StyleDrawControlArgs<'largs>
    for (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let element = self.0;
      let opt = self.1;
      let p = self.2;
      ::ffi::qt_widgets_c_QStyle_drawControl_element_opt_p(original_self as *const ::style::Style, element, opt, p)
    }
  }
  impl<'largs> StyleDrawControlArgs<'largs>
    for (::style::ControlElement,
                                                     *const ::style_option::StyleOption,
                                                     *mut ::qt_gui::painter::Painter,
                                                     *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let element = self.0;
      let opt = self.1;
      let p = self.2;
      let w = self.3;
      ::ffi::qt_widgets_c_QStyle_drawControl_element_opt_p_w(original_self as *const ::style::Style, element, opt, p, w)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::draw_item_text](../struct.Style.html#method.draw_item_text) method.
  pub trait StyleDrawItemTextArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ();
  }
  impl<'largs> StyleDrawItemTextArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                      &'largs ::qt_core::rect::Rect,
                                                      ::libc::c_int,
                                                      &'largs ::qt_gui::palette::Palette,
                                                      bool,
                                                      &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let painter = self.0;
      let rect = self.1;
      let flags = self.2;
      let pal = self.3;
      let enabled = self.4;
      let text = self.5;
      ::ffi::qt_widgets_c_QStyle_drawItemText_painter_rect_flags_pal_enabled_text(original_self as *const ::style::Style, painter, rect as *const ::qt_core::rect::Rect, flags, pal as *const ::qt_gui::palette::Palette, enabled, text as *const ::qt_core::string::String)
    }
  }
  impl<'largs> StyleDrawItemTextArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                      &'largs ::qt_core::rect::Rect,
                                                      ::libc::c_int,
                                                      &'largs ::qt_gui::palette::Palette,
                                                      bool,
                                                      &'largs ::qt_core::string::String,
                                                      &'largs ::qt_gui::palette::ColorRole) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let painter = self.0;
      let rect = self.1;
      let flags = self.2;
      let pal = self.3;
      let enabled = self.4;
      let text = self.5;
      let text_role = self.6;
      ::ffi::qt_widgets_c_QStyle_drawItemText_painter_rect_flags_pal_enabled_text_textRole(original_self as *const ::style::Style, painter, rect as *const ::qt_core::rect::Rect, flags, pal as *const ::qt_gui::palette::Palette, enabled, text as *const ::qt_core::string::String, text_role as *const ::qt_gui::palette::ColorRole)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::draw_primitive](../struct.Style.html#method.draw_primitive) method.
  pub trait StyleDrawPrimitiveArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ();
  }
  impl<'largs> StyleDrawPrimitiveArgs<'largs>
    for (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let pe = self.0;
      let opt = self.1;
      let p = self.2;
      ::ffi::qt_widgets_c_QStyle_drawPrimitive_pe_opt_p(original_self as *const ::style::Style, pe, opt, p)
    }
  }
  impl<'largs> StyleDrawPrimitiveArgs<'largs>
    for (::style::PrimitiveElement,
                                                       *const ::style_option::StyleOption,
                                                       *mut ::qt_gui::painter::Painter,
                                                       *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> () {
      let pe = self.0;
      let opt = self.1;
      let p = self.2;
      let w = self.3;
      ::ffi::qt_widgets_c_QStyle_drawPrimitive_pe_opt_p_w(original_self as *const ::style::Style, pe, opt, p, w)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::hit_test_complex_control](../struct.Style.html#method.hit_test_complex_control) method.
  pub trait StyleHitTestComplexControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::style::SubControl;
  }
  impl<'largs> StyleHitTestComplexControlArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::style::SubControl {
      let cc = self.0;
      let opt = self.1;
      let pt = self.2;
      ::ffi::qt_widgets_c_QStyle_hitTestComplexControl_cc_opt_pt(original_self as *const ::style::Style,
                                                                 cc,
                                                                 opt,
                                                                 pt as *const ::qt_core::point::Point)
    }
  }
  impl<'largs> StyleHitTestComplexControlArgs<'largs>
    for (::style::ComplexControl,
                                                               *const ::style_option_complex::StyleOptionComplex,
                                                               &'largs ::qt_core::point::Point,
                                                               *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::style::SubControl {
      let cc = self.0;
      let opt = self.1;
      let pt = self.2;
      let widget = self.3;
      ::ffi::qt_widgets_c_QStyle_hitTestComplexControl_cc_opt_pt_widget(original_self as *const ::style::Style,
                                                                        cc,
                                                                        opt,
                                                                        pt as *const ::qt_core::point::Point,
                                                                        widget)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::layout_spacing_unsafe](../struct.Style.html#method.layout_spacing_unsafe) method.
  pub trait StyleLayoutSpacingUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int;
  }
  impl<'largs> StyleLayoutSpacingUnsafeArgs<'largs>
    for (&'largs ::size_policy::ControlType,
                                                             &'largs ::size_policy::ControlType,
                                                             &'largs ::qt_core::qt::Orientation,
                                                             *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int {
      let control1 = self.0;
      let control2 = self.1;
      let orientation = self.2;
      let option = self.3;
      ::ffi::qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation_option(original_self as *const ::style::Style, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation, option)
    }
  }
  impl<'largs> StyleLayoutSpacingUnsafeArgs<'largs>
    for (&'largs ::size_policy::ControlType,
                                                             &'largs ::size_policy::ControlType,
                                                             &'largs ::qt_core::qt::Orientation,
                                                             *const ::style_option::StyleOption,
                                                             *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int {
      let control1 = self.0;
      let control2 = self.1;
      let orientation = self.2;
      let option = self.3;
      let widget = self.4;
      ::ffi::qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation_option_widget(original_self as *const ::style::Style, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::pixel_metric_unsafe](../struct.Style.html#method.pixel_metric_unsafe) method.
  pub trait StylePixelMetricUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int;
  }
  impl<'largs> StylePixelMetricUnsafeArgs<'largs> for (::style::PixelMetric, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int {
      let metric = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QStyle_pixelMetric_metric_option(original_self as *const ::style::Style, metric, option)
    }
  }
  impl<'largs> StylePixelMetricUnsafeArgs<'largs>
    for (::style::PixelMetric, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int {
      let metric = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QStyle_pixelMetric_metric_option_widget(original_self as *const ::style::Style,
                                                                  metric,
                                                                  option,
                                                                  widget)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::polish_unsafe](../struct.Style.html#method.polish_unsafe) method.
  pub trait StylePolishUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::style::Style) -> ();
  }
  impl<'largs> StylePolishUnsafeArgs<'largs> for *mut ::application::Application {
    unsafe fn exec(self, original_self: &'largs mut ::style::Style) -> () {
      let application = self;
      ::ffi::qt_widgets_c_QStyle_polish_application(original_self as *mut ::style::Style, application)
    }
  }
  impl<'largs> StylePolishUnsafeArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::style::Style) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QStyle_polish_widget(original_self as *mut ::style::Style, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::size_from_contents](../struct.Style.html#method.size_from_contents) method.
  pub trait StyleSizeFromContentsArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::size::Size;
  }
  impl<'largs> StyleSizeFromContentsArgs<'largs>
    for (::style::ContentsType, *const ::style_option::StyleOption, &'largs ::qt_core::size::Size) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::size::Size {
      let ct = self.0;
      let opt = self.1;
      let contents_size = self.2;
      {
        let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_sizeFromContents_to_output_ct_opt_contentsSize(original_self as *const ::style::Style, ct, opt, contents_size as *const ::qt_core::size::Size, &mut object);
        object
      }
    }
  }
  impl<'largs> StyleSizeFromContentsArgs<'largs>
    for (::style::ContentsType,
                                                          *const ::style_option::StyleOption,
                                                          &'largs ::qt_core::size::Size,
                                                          *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::size::Size {
      let ct = self.0;
      let opt = self.1;
      let contents_size = self.2;
      let w = self.3;
      {
        let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_sizeFromContents_to_output_ct_opt_contentsSize_w(original_self as *const ::style::Style, ct, opt, contents_size as *const ::qt_core::size::Size, w, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Style::slider_position_from_value](../struct.Style.html#method.slider_position_from_value) method.
  pub trait StyleSliderPositionFromValueArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl StyleSliderPositionFromValueArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::libc::c_int {
      let min = self.0;
      let max = self.1;
      let val = self.2;
      let space = self.3;
      unsafe { ::ffi::qt_widgets_c_QStyle_sliderPositionFromValue_min_max_val_space(min, max, val, space) }
    }
  }
  impl StyleSliderPositionFromValueArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, bool) {
    fn exec(self) -> ::libc::c_int {
      let min = self.0;
      let max = self.1;
      let val = self.2;
      let space = self.3;
      let upside_down = self.4;
      unsafe {
        ::ffi::qt_widgets_c_QStyle_sliderPositionFromValue_min_max_val_space_upsideDown(min,
                                                                                        max,
                                                                                        val,
                                                                                        space,
                                                                                        upside_down)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Style::slider_value_from_position](../struct.Style.html#method.slider_value_from_position) method.
  pub trait StyleSliderValueFromPositionArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl StyleSliderValueFromPositionArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::libc::c_int {
      let min = self.0;
      let max = self.1;
      let pos = self.2;
      let space = self.3;
      unsafe { ::ffi::qt_widgets_c_QStyle_sliderValueFromPosition_min_max_pos_space(min, max, pos, space) }
    }
  }
  impl StyleSliderValueFromPositionArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, bool) {
    fn exec(self) -> ::libc::c_int {
      let min = self.0;
      let max = self.1;
      let pos = self.2;
      let space = self.3;
      let upside_down = self.4;
      unsafe {
        ::ffi::qt_widgets_c_QStyle_sliderValueFromPosition_min_max_pos_space_upsideDown(min,
                                                                                        max,
                                                                                        pos,
                                                                                        space,
                                                                                        upside_down)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Style::standard_icon_unsafe](../struct.Style.html#method.standard_icon_unsafe) method.
  pub trait StyleStandardIconUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_gui::icon::Icon;
  }
  impl<'largs> StyleStandardIconUnsafeArgs<'largs> for (::style::StandardPixmap, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_gui::icon::Icon {
      let standard_icon = self.0;
      let option = self.1;
      {
        let mut object: ::qt_gui::icon::Icon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_standardIcon_to_output_standardIcon_option(original_self as *const ::style::Style,
                                                                              standard_icon,
                                                                              option,
                                                                              &mut object);
        object
      }
    }
  }
  impl<'largs> StyleStandardIconUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_gui::icon::Icon {
      let standard_icon = self.0;
      let option = self.1;
      let widget = self.2;
      {
        let mut object: ::qt_gui::icon::Icon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_standardIcon_to_output_standardIcon_option_widget(original_self as *const ::style::Style, standard_icon, option, widget, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Style::standard_pixmap_unsafe](../struct.Style.html#method.standard_pixmap_unsafe) method.
  pub trait StyleStandardPixmapUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>;
  }
  impl<'largs> StyleStandardPixmapUnsafeArgs<'largs> for (::style::StandardPixmap, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
      let standard_pixmap = self.0;
      let opt = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap_opt(original_self as *const ::style::Style,
                                                                            standard_pixmap,
                                                                            opt);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'largs> StyleStandardPixmapUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
      let standard_pixmap = self.0;
      let opt = self.1;
      let widget = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap_opt_widget(original_self as *const ::style::Style, standard_pixmap, opt, widget);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::style_hint_unsafe](../struct.Style.html#method.style_hint_unsafe) method.
  pub trait StyleStyleHintUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int;
  }
  impl<'largs> StyleStyleHintUnsafeArgs<'largs> for (::style::StyleHint, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int {
      let stylehint = self.0;
      let opt = self.1;
      ::ffi::qt_widgets_c_QStyle_styleHint_stylehint_opt(original_self as *const ::style::Style, stylehint, opt)
    }
  }
  impl<'largs> StyleStyleHintUnsafeArgs<'largs>
    for (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int {
      let stylehint = self.0;
      let opt = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QStyle_styleHint_stylehint_opt_widget(original_self as *const ::style::Style,
                                                                stylehint,
                                                                opt,
                                                                widget)
    }
  }
  impl<'largs> StyleStyleHintUnsafeArgs<'largs>
    for (::style::StyleHint,
                                                         *const ::style_option::StyleOption,
                                                         *const ::widget::Widget,
                                                         *mut ::style_hint_return::StyleHintReturn) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::libc::c_int {
      let stylehint = self.0;
      let opt = self.1;
      let widget = self.2;
      let return_data = self.3;
      ::ffi::qt_widgets_c_QStyle_styleHint_stylehint_opt_widget_returnData(original_self as *const ::style::Style,
                                                                           stylehint,
                                                                           opt,
                                                                           widget,
                                                                           return_data)
    }
  }
  /// This trait represents a set of arguments accepted by [Style::sub_control_rect](../struct.Style.html#method.sub_control_rect) method.
  pub trait StyleSubControlRectArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::rect::Rect;
  }
  impl<'largs> StyleSubControlRectArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, ::style::SubControl) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::rect::Rect {
      let cc = self.0;
      let opt = self.1;
      let sc = self.2;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_subControlRect_to_output_cc_opt_sc(original_self as *const ::style::Style,
                                                                      cc,
                                                                      opt,
                                                                      sc,
                                                                      &mut object);
        object
      }
    }
  }
  impl<'largs> StyleSubControlRectArgs<'largs>
    for (::style::ComplexControl,
                                                        *const ::style_option_complex::StyleOptionComplex,
                                                        ::style::SubControl,
                                                        *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::rect::Rect {
      let cc = self.0;
      let opt = self.1;
      let sc = self.2;
      let widget = self.3;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_subControlRect_to_output_cc_opt_sc_widget(original_self as *const ::style::Style,
                                                                             cc,
                                                                             opt,
                                                                             sc,
                                                                             widget,
                                                                             &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Style::sub_element_rect](../struct.Style.html#method.sub_element_rect) method.
  pub trait StyleSubElementRectArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::rect::Rect;
  }
  impl<'largs> StyleSubElementRectArgs<'largs> for (::style::SubElement, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::rect::Rect {
      let sub_element = self.0;
      let option = self.1;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_subElementRect_to_output_subElement_option(original_self as *const ::style::Style,
                                                                              sub_element,
                                                                              option,
                                                                              &mut object);
        object
      }
    }
  }
  impl<'largs> StyleSubElementRectArgs<'largs>
    for (::style::SubElement, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::style::Style) -> ::qt_core::rect::Rect {
      let sub_element = self.0;
      let option = self.1;
      let widget = self.2;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStyle_subElementRect_to_output_subElement_option_widget(original_self as *const ::style::Style, sub_element, option, widget, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Style::unpolish](../struct.Style.html#method.unpolish) method.
  pub trait StyleUnpolishArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::style::Style) -> ();
  }
  impl<'largs> StyleUnpolishArgs<'largs> for *mut ::application::Application {
    unsafe fn exec(self, original_self: &'largs mut ::style::Style) -> () {
      let application = self;
      ::ffi::qt_widgets_c_QStyle_unpolish_application(original_self as *mut ::style::Style, application)
    }
  }
  impl<'largs> StyleUnpolishArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::style::Style) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QStyle_unpolish_widget(original_self as *mut ::style::Style, widget)
    }
  }
}
