//! Bindings for [QtWidgets](http://doc.qt.io/qt-5/qtwidgets-module.html) library.
//!
//! This crate was generated using [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust).
//!
//! This is work in progress, so the API will significantly change in the future.
//! Some methods are missing, and some are inconvenient to use.
//! Some methods are unsafe even though they are not marked as unsafe.
//! Users must carefully track ownership of the objects, as usual Rust guarantees
//! do not take effect. This will hopefully improve in the future.
//! Please report any issues to the
//! [issue tracker](https://github.com/rust-qt/cpp_to_rust/issues).
//!
//! This crate was generated for **Qt 5.9.1**.
//! If Qt compatibility guarantees take effect, it should be compatible
//! with future minor releases and with past and future patch releases,
//! but API added in future releases won't be available. The crate is not compatible
//! with past minor Qt releases. If you need to use a Qt version incompatible with this crate,
//! use [qt_generator](https://github.com/rust-qt/cpp_to_rust/tree/master/qt_generator/qt_generator)
//! to generate crates for your Qt version.
//!
//! Refer to `qt_core` crate documentation for general information about Qt crates.
//! Note that if you use `qt_widgets`, you should use `qt_widgets::application::Application`
//! as the application object.

pub extern crate libc;
pub extern crate cpp_utils;

pub extern crate qt_core;

pub extern crate qt_gui;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QAbstractButton` C++ header
pub mod abstract_button;
/// Entities from `QAbstractGraphicsShapeItem` C++ header
pub mod abstract_graphics_shape_item;
/// Entities from `QAbstractItemDelegate` C++ header
pub mod abstract_item_delegate;
/// Entities from `QAbstractItemView` C++ header
pub mod abstract_item_view;
/// Entities from `QAbstractScrollArea` C++ header
pub mod abstract_scroll_area;
/// Entities from `QAbstractSlider` C++ header
pub mod abstract_slider;
/// Entities from `QAbstractSpinBox` C++ header
pub mod abstract_spin_box;
/// Entities from `QAccessibleWidget` C++ header
pub mod accessible_widget;
/// Entities from `QAction` C++ header
pub mod action;
/// Entities from `QActionGroup` C++ header
pub mod action_group;
/// Entities from `QApplication` C++ header
pub mod application;
/// Entities from `QBoxLayout` C++ header
pub mod box_layout;
/// Entities from `QButtonGroup` C++ header
pub mod button_group;
/// Entities from `QCalendarWidget` C++ header
pub mod calendar_widget;
/// Entities from `QCheckBox` C++ header
pub mod check_box;
/// Entities from `QColorDialog` C++ header
pub mod color_dialog;
/// Entities from `QColormap` C++ header
pub mod colormap;
/// Entities from `QColumnView` C++ header
pub mod column_view;
/// Entities from `QComboBox` C++ header
pub mod combo_box;
/// Entities from `QCommandLinkButton` C++ header
pub mod command_link_button;
/// Entities from `QCommonStyle` C++ header
pub mod common_style;
/// Entities from `QCompleter` C++ header
pub mod completer;
/// Entities from `QDataWidgetMapper` C++ header
pub mod data_widget_mapper;
/// Entities from `QDateEdit` C++ header
pub mod date_edit;
/// Entities from `QDateTimeEdit` C++ header
pub mod date_time_edit;
/// Entities from `QDesktopWidget` C++ header
pub mod desktop_widget;
/// Entities from `QDial` C++ header
pub mod dial;
/// Entities from `QDialog` C++ header
pub mod dialog;
/// Entities from `QDialogButtonBox` C++ header
pub mod dialog_button_box;
/// Entities from `QDirModel` C++ header
pub mod dir_model;
/// Entities from `QDockWidget` C++ header
pub mod dock_widget;
/// Entities from `QDoubleSpinBox` C++ header
pub mod double_spin_box;
/// Entities from `QErrorMessage` C++ header
pub mod error_message;
/// Entities from `QFileDialog` C++ header
pub mod file_dialog;
/// Entities from `QFileIconProvider` C++ header
pub mod file_icon_provider;
/// Entities from `QFileSystemModel` C++ header
pub mod file_system_model;
/// Entities from `QFocusFrame` C++ header
pub mod focus_frame;
/// Entities from `QFontComboBox` C++ header
pub mod font_combo_box;
/// Entities from `QFontDialog` C++ header
pub mod font_dialog;
/// Entities from `QFormLayout` C++ header
pub mod form_layout;
/// Entities from `QFrame` C++ header
pub mod frame;
/// Entities from `QGesture` C++ header
pub mod gesture;
/// Entities from `QGestureEvent` C++ header
pub mod gesture_event;
/// Entities from `QGestureRecognizer` C++ header
pub mod gesture_recognizer;
/// Entities from `QGraphicsAnchor` C++ header
pub mod graphics_anchor;
/// Entities from `QGraphicsAnchorLayout` C++ header
pub mod graphics_anchor_layout;
/// Entities from `QGraphicsBlurEffect` C++ header
pub mod graphics_blur_effect;
/// Entities from `QGraphicsColorizeEffect` C++ header
pub mod graphics_colorize_effect;
/// Entities from `QGraphicsDropShadowEffect` C++ header
pub mod graphics_drop_shadow_effect;
/// Entities from `QGraphicsEffect` C++ header
pub mod graphics_effect;
/// Entities from `QGraphicsEllipseItem` C++ header
pub mod graphics_ellipse_item;
/// Entities from `QGraphicsGridLayout` C++ header
pub mod graphics_grid_layout;
/// Entities from `QGraphicsItem` C++ header
pub mod graphics_item;
/// Entities from `QGraphicsItemAnimation` C++ header
pub mod graphics_item_animation;
/// Entities from `QGraphicsItemGroup` C++ header
pub mod graphics_item_group;
/// Entities from `QGraphicsLayout` C++ header
pub mod graphics_layout;
/// Entities from `QGraphicsLayoutItem` C++ header
pub mod graphics_layout_item;
/// Entities from `QGraphicsLineItem` C++ header
pub mod graphics_line_item;
/// Entities from `QGraphicsLinearLayout` C++ header
pub mod graphics_linear_layout;
/// Entities from `QGraphicsObject` C++ header
pub mod graphics_object;
/// Entities from `QGraphicsOpacityEffect` C++ header
pub mod graphics_opacity_effect;
/// Entities from `QGraphicsPathItem` C++ header
pub mod graphics_path_item;
/// Entities from `QGraphicsPixmapItem` C++ header
pub mod graphics_pixmap_item;
/// Entities from `QGraphicsPolygonItem` C++ header
pub mod graphics_polygon_item;
/// Entities from `QGraphicsProxyWidget` C++ header
pub mod graphics_proxy_widget;
/// Entities from `QGraphicsRectItem` C++ header
pub mod graphics_rect_item;
/// Entities from `QGraphicsRotation` C++ header
pub mod graphics_rotation;
/// Entities from `QGraphicsScale` C++ header
pub mod graphics_scale;
/// Entities from `QGraphicsScene` C++ header
pub mod graphics_scene;
/// Entities from `QGraphicsSceneContextMenuEvent` C++ header
pub mod graphics_scene_context_menu_event;
/// Entities from `QGraphicsSceneDragDropEvent` C++ header
pub mod graphics_scene_drag_drop_event;
/// Entities from `QGraphicsSceneEvent` C++ header
pub mod graphics_scene_event;
/// Entities from `QGraphicsSceneHelpEvent` C++ header
pub mod graphics_scene_help_event;
/// Entities from `QGraphicsSceneHoverEvent` C++ header
pub mod graphics_scene_hover_event;
/// Entities from `QGraphicsSceneMouseEvent` C++ header
pub mod graphics_scene_mouse_event;
/// Entities from `QGraphicsSceneMoveEvent` C++ header
pub mod graphics_scene_move_event;
/// Entities from `QGraphicsSceneResizeEvent` C++ header
pub mod graphics_scene_resize_event;
/// Entities from `QGraphicsSceneWheelEvent` C++ header
pub mod graphics_scene_wheel_event;
/// Entities from `QGraphicsSimpleTextItem` C++ header
pub mod graphics_simple_text_item;
/// Entities from `QGraphicsTextItem` C++ header
pub mod graphics_text_item;
/// Entities from `QGraphicsTransform` C++ header
pub mod graphics_transform;
/// Entities from `QGraphicsView` C++ header
pub mod graphics_view;
/// Entities from `QGraphicsWidget` C++ header
pub mod graphics_widget;
/// Entities from `QGridLayout` C++ header
pub mod grid_layout;
/// Entities from `QGroupBox` C++ header
pub mod group_box;
/// Entities from `QHBoxLayout` C++ header
pub mod h_box_layout;
/// Entities from `QHash` C++ header
pub mod hash;
/// Entities from `QHeaderView` C++ header
pub mod header_view;
/// Entities from `QInputDialog` C++ header
pub mod input_dialog;
/// Entities from `QItemDelegate` C++ header
pub mod item_delegate;
/// Entities from `QItemEditorCreatorBase` C++ header
pub mod item_editor_creator_base;
/// Entities from `QItemEditorFactory` C++ header
pub mod item_editor_factory;
/// Entities from `QKeyEventTransition` C++ header
pub mod key_event_transition;
/// Entities from `QKeySequenceEdit` C++ header
pub mod key_sequence_edit;
/// Entities from `QLCDNumber` C++ header
pub mod l_c_d_number;
/// Entities from `QLabel` C++ header
pub mod label;
/// Entities from `QLayout` C++ header
pub mod layout;
/// Entities from `QLayoutItem` C++ header
pub mod layout_item;
/// Entities from `QLineEdit` C++ header
pub mod line_edit;
/// Entities from `QList` C++ header
pub mod list;
/// Entities from `QListView` C++ header
pub mod list_view;
/// Entities from `QListWidget` C++ header
pub mod list_widget;
/// Entities from `QListWidgetItem` C++ header
pub mod list_widget_item;
/// Entities from `QMainWindow` C++ header
pub mod main_window;
/// Entities from `QMap` C++ header
pub mod map;
/// Entities from `QMdiArea` C++ header
pub mod mdi_area;
/// Entities from `QMdiSubWindow` C++ header
pub mod mdi_sub_window;
/// Entities from `QMenu` C++ header
pub mod menu;
/// Entities from `QMenuBar` C++ header
pub mod menu_bar;
/// Entities from `QMessageBox` C++ header
pub mod message_box;
/// Entities from `QMouseEventTransition` C++ header
pub mod mouse_event_transition;
/// Entities from `QOpenGLWidget` C++ header
pub mod opengl_widget;
/// Entities from `QPair` C++ header
pub mod pair;
/// Entities from `QPanGesture` C++ header
pub mod pan_gesture;
/// Entities from `QPinchGesture` C++ header
pub mod pinch_gesture;
/// Entities from `QPlainTextDocumentLayout` C++ header
pub mod plain_text_document_layout;
/// Entities from `QPlainTextEdit` C++ header
pub mod plain_text_edit;
/// Entities from `QProgressBar` C++ header
pub mod progress_bar;
/// Entities from `QProgressDialog` C++ header
pub mod progress_dialog;
/// Entities from `QProxyStyle` C++ header
pub mod proxy_style;
/// Entities from `QPushButton` C++ header
pub mod push_button;
/// Entities from `QRadioButton` C++ header
pub mod radio_button;
/// Entities from `QRubberBand` C++ header
pub mod rubber_band;
/// Entities from `QScrollArea` C++ header
pub mod scroll_area;
/// Entities from `QScrollBar` C++ header
pub mod scroll_bar;
/// Entities from `QScroller` C++ header
pub mod scroller;
/// Entities from `QScrollerProperties` C++ header
pub mod scroller_properties;
/// Entities from `QShortcut` C++ header
pub mod shortcut;
/// Entities from `QSizeGrip` C++ header
pub mod size_grip;
/// Entities from `QSizePolicy` C++ header
pub mod size_policy;
/// Entities from `QSlider` C++ header
pub mod slider;
/// Binding Qt signals to Rust closures or extern functions.
///
/// Types in this module allow to connect Qt signals with certain argument types to a Rust closure.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots` module.
pub mod slots;
/// Entities from `QSpacerItem` C++ header
pub mod spacer_item;
/// Entities from `QSpinBox` C++ header
pub mod spin_box;
/// Entities from `QSplashScreen` C++ header
pub mod splash_screen;
/// Entities from `QSplitter` C++ header
pub mod splitter;
/// Entities from `QSplitterHandle` C++ header
pub mod splitter_handle;
/// Entities from `QStackedLayout` C++ header
pub mod stacked_layout;
/// Entities from `QStackedWidget` C++ header
pub mod stacked_widget;
/// Entities from `QStatusBar` C++ header
pub mod status_bar;
/// Entities from `QStyle` C++ header
pub mod style;
/// Entities from `QStyleFactory` C++ header
pub mod style_factory;
/// Entities from `QStyleHintReturn` C++ header
pub mod style_hint_return;
/// Entities from `QStyleHintReturnMask` C++ header
pub mod style_hint_return_mask;
/// Entities from `QStyleHintReturnVariant` C++ header
pub mod style_hint_return_variant;
/// Entities from `QStyleOption` C++ header
pub mod style_option;
/// Entities from `QStyleOptionButton` C++ header
pub mod style_option_button;
/// Entities from `QStyleOptionComboBox` C++ header
pub mod style_option_combo_box;
/// Entities from `QStyleOptionComplex` C++ header
pub mod style_option_complex;
/// Entities from `QStyleOptionDockWidget` C++ header
pub mod style_option_dock_widget;
/// Entities from `QStyleOptionFocusRect` C++ header
pub mod style_option_focus_rect;
/// Entities from `QStyleOptionFrame` C++ header
pub mod style_option_frame;
/// Entities from `QStyleOptionGraphicsItem` C++ header
pub mod style_option_graphics_item;
/// Entities from `QStyleOptionGroupBox` C++ header
pub mod style_option_group_box;
/// Entities from `QStyleOptionHeader` C++ header
pub mod style_option_header;
/// Entities from `QStyleOptionMenuItem` C++ header
pub mod style_option_menu_item;
/// Entities from `QStyleOptionProgressBar` C++ header
pub mod style_option_progress_bar;
/// Entities from `QStyleOptionRubberBand` C++ header
pub mod style_option_rubber_band;
/// Entities from `QStyleOptionSizeGrip` C++ header
pub mod style_option_size_grip;
/// Entities from `QStyleOptionSlider` C++ header
pub mod style_option_slider;
/// Entities from `QStyleOptionSpinBox` C++ header
pub mod style_option_spin_box;
/// Entities from `QStyleOptionTab` C++ header
pub mod style_option_tab;
/// Entities from `QStyleOptionTabBarBase` C++ header
pub mod style_option_tab_bar_base;
/// Entities from `QStyleOptionTabWidgetFrame` C++ header
pub mod style_option_tab_widget_frame;
/// Entities from `QStyleOptionTitleBar` C++ header
pub mod style_option_title_bar;
/// Entities from `QStyleOptionToolBar` C++ header
pub mod style_option_tool_bar;
/// Entities from `QStyleOptionToolBox` C++ header
pub mod style_option_tool_box;
/// Entities from `QStyleOptionToolButton` C++ header
pub mod style_option_tool_button;
/// Entities from `QStyleOptionViewItem` C++ header
pub mod style_option_view_item;
/// Entities from `QStylePainter` C++ header
pub mod style_painter;
/// Entities from `QStylePlugin` C++ header
pub mod style_plugin;
/// Entities from `QStyledItemDelegate` C++ header
pub mod styled_item_delegate;
/// Entities from `QSwipeGesture` C++ header
pub mod swipe_gesture;
/// Entities from `QSystemTrayIcon` C++ header
pub mod system_tray_icon;
/// Entities from `QTabBar` C++ header
pub mod tab_bar;
/// Entities from `QTabWidget` C++ header
pub mod tab_widget;
/// Entities from `QTableView` C++ header
pub mod table_view;
/// Entities from `QTableWidget` C++ header
pub mod table_widget;
/// Entities from `QTableWidgetItem` C++ header
pub mod table_widget_item;
/// Entities from `QTableWidgetSelectionRange` C++ header
pub mod table_widget_selection_range;
/// Entities from `QTapAndHoldGesture` C++ header
pub mod tap_and_hold_gesture;
/// Entities from `QTapGesture` C++ header
pub mod tap_gesture;
/// Entities from `QTextBrowser` C++ header
pub mod text_browser;
/// Entities from `QTextEdit` C++ header
pub mod text_edit;
/// Entities from `QTileRules` C++ header
pub mod tile_rules;
/// Entities from `QTimeEdit` C++ header
pub mod time_edit;
/// Entities from `QToolBar` C++ header
pub mod tool_bar;
/// Entities from `QToolBox` C++ header
pub mod tool_box;
/// Entities from `QToolButton` C++ header
pub mod tool_button;
/// Entities from `QToolTip` C++ header
pub mod tool_tip;
/// Entities from `QTreeView` C++ header
pub mod tree_view;
/// Entities from `QTreeWidget` C++ header
pub mod tree_widget;
/// Entities from `QTreeWidgetItem` C++ header
pub mod tree_widget_item;
/// Entities from `QTreeWidgetItemIterator` C++ header
pub mod tree_widget_item_iterator;
/// Entities from `QUndoCommand` C++ header
pub mod undo_command;
/// Entities from `QUndoGroup` C++ header
pub mod undo_group;
/// Entities from `QUndoStack` C++ header
pub mod undo_stack;
/// Entities from `QUndoView` C++ header
pub mod undo_view;
/// Entities from `QVBoxLayout` C++ header
pub mod v_box_layout;
/// Entities from `QVector` C++ header
pub mod vector;
/// Entities from `QWhatsThis` C++ header
pub mod whats_this;
/// Entities from `QWidget` C++ header
pub mod widget;
/// Entities from `QWidgetAction` C++ header
pub mod widget_action;
/// Entities from `QWidgetItem` C++ header
pub mod widget_item;
/// Entities from `QWizard` C++ header
pub mod wizard;
/// Entities from `QWizardPage` C++ header
pub mod wizard_page;
