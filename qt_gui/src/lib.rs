//! Bindings for [QtGui](http://doc.qt.io/qt-5/qtgui-module.html) library.
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
//! as the application object, and if you use `qt_gui` but not `qt_widgets`, you should use
//! `qt_gui::gui_application::GuiApplication`.



pub extern crate libc;
pub extern crate cpp_utils;

pub extern crate qt_core;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QAbstractTextDocumentLayout` C++ header
pub mod abstract_text_document_layout;
/// Entities from `QAbstractUndoItem` C++ header
pub mod abstract_undo_item;
/// Entities from `QAccessible` C++ header
pub mod accessible;
/// Entities from `QAccessibleActionInterface` C++ header
pub mod accessible_action_interface;
/// Entities from `QAccessibleEditableTextInterface` C++ header
pub mod accessible_editable_text_interface;
/// Entities from `QAccessibleEvent` C++ header
pub mod accessible_event;
/// Entities from `QAccessibleInterface` C++ header
pub mod accessible_interface;
/// Entities from `QAccessibleObject` C++ header
pub mod accessible_object;
/// Entities from `QAccessiblePlugin` C++ header
pub mod accessible_plugin;
/// Entities from `QAccessibleStateChangeEvent` C++ header
pub mod accessible_state_change_event;
/// Entities from `QAccessibleTableCellInterface` C++ header
pub mod accessible_table_cell_interface;
/// Entities from `QAccessibleTableInterface` C++ header
pub mod accessible_table_interface;
/// Entities from `QAccessibleTableModelChangeEvent` C++ header
pub mod accessible_table_model_change_event;
/// Entities from `QAccessibleTextCursorEvent` C++ header
pub mod accessible_text_cursor_event;
/// Entities from `QAccessibleTextInsertEvent` C++ header
pub mod accessible_text_insert_event;
/// Entities from `QAccessibleTextInterface` C++ header
pub mod accessible_text_interface;
/// Entities from `QAccessibleTextRemoveEvent` C++ header
pub mod accessible_text_remove_event;
/// Entities from `QAccessibleTextSelectionEvent` C++ header
pub mod accessible_text_selection_event;
/// Entities from `QAccessibleTextUpdateEvent` C++ header
pub mod accessible_text_update_event;
/// Entities from `QAccessibleValueChangeEvent` C++ header
pub mod accessible_value_change_event;
/// Entities from `QAccessibleValueInterface` C++ header
pub mod accessible_value_interface;
/// Entities from `QActionEvent` C++ header
pub mod action_event;
/// Entities from `QApplicationStateChangeEvent` C++ header
pub mod application_state_change_event;
/// Entities from `QBackingStore` C++ header
pub mod backing_store;
/// Entities from `QBitmap` C++ header
pub mod bitmap;
/// Entities from `QBrush` C++ header
pub mod brush;
/// Entities from `QClipboard` C++ header
pub mod clipboard;
/// Entities from `QCloseEvent` C++ header
pub mod close_event;
/// Entities from `QColor` C++ header
pub mod color;
/// Entities from `QConicalGradient` C++ header
pub mod conical_gradient;
/// Entities from `QContextMenuEvent` C++ header
pub mod context_menu_event;
/// Entities from `QCursor` C++ header
pub mod cursor;
/// Entities from `QDesktopServices` C++ header
pub mod desktop_services;
/// Entities from `QDoubleValidator` C++ header
pub mod double_validator;
/// Entities from `QDrag` C++ header
pub mod drag;
/// Entities from `QDragEnterEvent` C++ header
pub mod drag_enter_event;
/// Entities from `QDragLeaveEvent` C++ header
pub mod drag_leave_event;
/// Entities from `QDragMoveEvent` C++ header
pub mod drag_move_event;
/// Entities from `QDropEvent` C++ header
pub mod drop_event;
/// Entities from `QEnterEvent` C++ header
pub mod enter_event;
/// Entities from `QExposeEvent` C++ header
pub mod expose_event;
/// Entities from `QFileOpenEvent` C++ header
pub mod file_open_event;
/// Entities from `QFocusEvent` C++ header
pub mod focus_event;
/// Entities from `QFont` C++ header
pub mod font;
/// Entities from `QFontDatabase` C++ header
pub mod font_database;
/// Entities from `QFontInfo` C++ header
pub mod font_info;
/// Entities from `QFontMetrics` C++ header
pub mod font_metrics;
/// Entities from `QFontMetricsF` C++ header
pub mod font_metrics_f;
/// Entities from `QGenericPlugin` C++ header
pub mod generic_plugin;
/// Entities from `QGenericPluginFactory` C++ header
pub mod generic_plugin_factory;
/// Entities from `QGlyphRun` C++ header
pub mod glyph_run;
/// Entities from `QGradient` C++ header
pub mod gradient;
/// Entities from `QGuiApplication` C++ header
pub mod gui_application;
/// Entities from `QHelpEvent` C++ header
pub mod help_event;
/// Entities from `QHideEvent` C++ header
pub mod hide_event;
/// Entities from `QHoverEvent` C++ header
pub mod hover_event;
/// Entities from `QIcon` C++ header
pub mod icon;
/// Entities from `QIconDragEvent` C++ header
pub mod icon_drag_event;
/// Entities from `QIconEngine` C++ header
pub mod icon_engine;
/// Entities from `QIconEnginePlugin` C++ header
pub mod icon_engine_plugin;
/// Entities from `QImage` C++ header
pub mod image;
/// Entities from `QImageIOHandler` C++ header
pub mod image_io_handler;
/// Entities from `QImageIOPlugin` C++ header
pub mod image_io_plugin;
/// Entities from `QImageReader` C++ header
pub mod image_reader;
/// Entities from `QImageWriter` C++ header
pub mod image_writer;
/// Entities from `QInputEvent` C++ header
pub mod input_event;
/// Entities from `QInputMethod` C++ header
pub mod input_method;
/// Entities from `QInputMethodEvent` C++ header
pub mod input_method_event;
/// Entities from `QInputMethodQueryEvent` C++ header
pub mod input_method_query_event;
/// Entities from `QIntValidator` C++ header
pub mod int_validator;
/// Entities from `QKeyEvent` C++ header
pub mod key_event;
/// Entities from `QKeySequence` C++ header
pub mod key_sequence;
/// Entities from `QLinearGradient` C++ header
pub mod linear_gradient;
/// Entities from `QList` C++ header
pub mod list;
/// Entities from `QMatrix` C++ header
pub mod matrix;
/// Entities from `QMatrix4x4` C++ header
pub mod matrix_4x4;
/// Entities from `QMouseEvent` C++ header
pub mod mouse_event;
/// Entities from `QMoveEvent` C++ header
pub mod move_event;
/// Entities from `QMovie` C++ header
pub mod movie;
/// Entities from `QNativeGestureEvent` C++ header
pub mod native_gesture_event;
/// Entities from `QOffscreenSurface` C++ header
pub mod offscreen_surface;
/// Entities from `QOpenGLBuffer` C++ header
pub mod opengl_buffer;
/// Entities from `QOpenGLContext` C++ header
pub mod opengl_context;
/// Entities from `QOpenGLContextGroup` C++ header
pub mod opengl_context_group;
/// Entities from `QOpenGLDebugLogger` C++ header
pub mod opengl_debug_logger;
/// Entities from `QOpenGLDebugMessage` C++ header
pub mod opengl_debug_message;
/// Entities from `QOpenGLExtraFunctions` C++ header
pub mod opengl_extra_functions;
/// Entities from `QOpenGLFramebufferObject` C++ header
pub mod opengl_framebuffer_object;
/// Entities from `QOpenGLFramebufferObjectFormat` C++ header
pub mod opengl_framebuffer_object_format;
/// Entities from `QOpenGLFunctions` C++ header
pub mod opengl_functions;
/// Entities from `QOpenGLPaintDevice` C++ header
pub mod opengl_paint_device;
/// Entities from `QOpenGLPixelTransferOptions` C++ header
pub mod opengl_pixel_transfer_options;
/// Entities from `QOpenGLShader` C++ header
pub mod opengl_shader;
/// Entities from `QOpenGLShaderProgram` C++ header
pub mod opengl_shader_program;
/// Entities from `QOpenGLTexture` C++ header
pub mod opengl_texture;
/// Entities from `QOpenGLTextureBlitter` C++ header
pub mod opengl_texture_blitter;
/// Entities from `QOpenGLTimeMonitor` C++ header
pub mod opengl_time_monitor;
/// Entities from `QOpenGLTimerQuery` C++ header
pub mod opengl_timer_query;
/// Entities from `QOpenGLVersionFunctions` C++ header
pub mod opengl_version_functions;
/// Entities from `QOpenGLVersionProfile` C++ header
pub mod opengl_version_profile;
/// Entities from `QOpenGLVertexArrayObject` C++ header
pub mod opengl_vertex_array_object;
/// Entities from `QOpenGLWindow` C++ header
pub mod opengl_window;
/// Entities from `QPageLayout` C++ header
pub mod page_layout;
/// Entities from `QPageSize` C++ header
pub mod page_size;
/// Entities from `QPagedPaintDevice` C++ header
pub mod paged_paint_device;
/// Entities from `QPaintDevice` C++ header
pub mod paint_device;
/// Entities from `QPaintDeviceWindow` C++ header
pub mod paint_device_window;
/// Entities from `QPaintEngine` C++ header
pub mod paint_engine;
/// Entities from `QPaintEngineState` C++ header
pub mod paint_engine_state;
/// Entities from `QPaintEvent` C++ header
pub mod paint_event;
/// Entities from `QPainter` C++ header
pub mod painter;
/// Entities from `QPainterPath` C++ header
pub mod painter_path;
/// Entities from `QPainterPathStroker` C++ header
pub mod painter_path_stroker;
/// Entities from `QPair` C++ header
pub mod pair;
/// Entities from `QPalette` C++ header
pub mod palette;
/// Entities from `QPdfWriter` C++ header
pub mod pdf_writer;
/// Entities from `QPen` C++ header
pub mod pen;
/// Entities from `QPicture` C++ header
pub mod picture;
/// Entities from `QPictureFormatPlugin` C++ header
pub mod picture_format_plugin;
/// Entities from `QPictureIO` C++ header
pub mod picture_io;
/// Entities from `QPixelFormat` C++ header
pub mod pixel_format;
/// Entities from `QPixmap` C++ header
pub mod pixmap;
/// Entities from `QPixmapCache` C++ header
pub mod pixmap_cache;
/// Entities from `QPlatformSurfaceEvent` C++ header
pub mod platform_surface_event;
/// Entities from `QPointingDeviceUniqueId` C++ header
pub mod pointing_device_unique_id;
/// Entities from `QPolygon` C++ header
pub mod polygon;
/// Entities from `QPolygonF` C++ header
pub mod polygon_f;
/// Entities from `QQuaternion` C++ header
pub mod quaternion;
/// Entities from `QRadialGradient` C++ header
pub mod radial_gradient;
/// Entities from `QRasterWindow` C++ header
pub mod raster_window;
/// Entities from `QRawFont` C++ header
pub mod raw_font;
/// Entities from `QRegExpValidator` C++ header
pub mod reg_exp_validator;
/// Entities from `QRegion` C++ header
pub mod region;
/// Entities from `QRegularExpressionValidator` C++ header
pub mod regular_expression_validator;
/// Entities from `QResizeEvent` C++ header
pub mod resize_event;
/// Entities from `QRgb` C++ header
pub mod rgb;
/// Entities from `QRgba64` C++ header
pub mod rgba64;
/// Entities from `QScreen` C++ header
pub mod screen;
/// Entities from `QScreenOrientationChangeEvent` C++ header
pub mod screen_orientation_change_event;
/// Entities from `QScrollEvent` C++ header
pub mod scroll_event;
/// Entities from `QScrollPrepareEvent` C++ header
pub mod scroll_prepare_event;
/// Entities from `QSessionManager` C++ header
pub mod session_manager;
/// Entities from `QSet` C++ header
pub mod set;
/// Entities from `QShortcutEvent` C++ header
pub mod shortcut_event;
/// Entities from `QShowEvent` C++ header
pub mod show_event;
/// Binding Qt signals to Rust closures or extern functions.
///
/// Types in this module allow to connect Qt signals with certain argument types to a Rust closure.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots` module.
pub mod slots;
/// Entities from `QStandardItem` C++ header
pub mod standard_item;
/// Entities from `QStandardItemModel` C++ header
pub mod standard_item_model;
/// Entities from `QStaticText` C++ header
pub mod static_text;
/// Entities from `QStatusTipEvent` C++ header
pub mod status_tip_event;
/// Entities from `QStyleHints` C++ header
pub mod style_hints;
/// Entities from `QSurface` C++ header
pub mod surface;
/// Entities from `QSurfaceFormat` C++ header
pub mod surface_format;
/// Entities from `QSyntaxHighlighter` C++ header
pub mod syntax_highlighter;
/// Entities from `QTabletEvent` C++ header
pub mod tablet_event;
/// Entities from `QTextBlock` C++ header
pub mod text_block;
/// Entities from `QTextBlockFormat` C++ header
pub mod text_block_format;
/// Entities from `QTextBlockGroup` C++ header
pub mod text_block_group;
/// Entities from `QTextBlockUserData` C++ header
pub mod text_block_user_data;
/// Entities from `QTextCharFormat` C++ header
pub mod text_char_format;
/// Entities from `QTextCursor` C++ header
pub mod text_cursor;
/// Entities from `QTextDocument` C++ header
pub mod text_document;
/// Entities from `QTextDocumentFragment` C++ header
pub mod text_document_fragment;
/// Entities from `QTextDocumentWriter` C++ header
pub mod text_document_writer;
/// Entities from `QTextFormat` C++ header
pub mod text_format;
/// Entities from `QTextFragment` C++ header
pub mod text_fragment;
/// Entities from `QTextFrame` C++ header
pub mod text_frame;
/// Entities from `QTextFrameFormat` C++ header
pub mod text_frame_format;
/// Entities from `QTextImageFormat` C++ header
pub mod text_image_format;
/// Entities from `QTextInlineObject` C++ header
pub mod text_inline_object;
/// Entities from `QTextItem` C++ header
pub mod text_item;
/// Entities from `QTextLayout` C++ header
pub mod text_layout;
/// Entities from `QTextLength` C++ header
pub mod text_length;
/// Entities from `QTextLine` C++ header
pub mod text_line;
/// Entities from `QTextList` C++ header
pub mod text_list;
/// Entities from `QTextListFormat` C++ header
pub mod text_list_format;
/// Entities from `QTextObject` C++ header
pub mod text_object;
/// Entities from `QTextObjectInterface` C++ header
pub mod text_object_interface;
/// Entities from `QTextOption` C++ header
pub mod text_option;
/// Entities from `QTextTable` C++ header
pub mod text_table;
/// Entities from `QTextTableCell` C++ header
pub mod text_table_cell;
/// Entities from `QTextTableCellFormat` C++ header
pub mod text_table_cell_format;
/// Entities from `QTextTableFormat` C++ header
pub mod text_table_format;
/// Entities from `QToolBarChangeEvent` C++ header
pub mod tool_bar_change_event;
/// Entities from `QTouchDevice` C++ header
pub mod touch_device;
/// Entities from `QTouchEvent` C++ header
pub mod touch_event;
/// Entities from `QTransform` C++ header
pub mod transform;
/// Entities from `QValidator` C++ header
pub mod validator;
/// Entities from `QVector` C++ header
pub mod vector;
/// Entities from `QVector2D` C++ header
pub mod vector_2d;
/// Entities from `QVector3D` C++ header
pub mod vector_3d;
/// Entities from `QVector4D` C++ header
pub mod vector_4d;
/// Entities from `QWhatsThisClickedEvent` C++ header
pub mod whats_this_clicked_event;
/// Entities from `QWheelEvent` C++ header
pub mod wheel_event;
/// Entities from `QWindow` C++ header
pub mod window;
/// Entities from `QWindowStateChangeEvent` C++ header
pub mod window_state_change_event;
