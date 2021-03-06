#ifndef QT_WIDGETS_C_QWIDGET_H
#define QT_WIDGETS_C_QWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_G_operator_shl_to_output(const QDebug* arg1, const QWidget* arg2, QDebug* output);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QWidget_G_static_cast_QObject_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QWidget_G_static_cast_QPaintDevice_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_acceptDrops(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_accessibleDescription_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_accessibleName_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_actions_to_output(const QWidget* this_ptr, QList< QAction* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_activateWindow(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_addAction(QWidget* this_ptr, QAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_addActions(QWidget* this_ptr, const QList< QAction* >* actions);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_adjustSize(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_autoFillBackground(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QBackingStore* qt_widgets_c_QWidget_backingStore(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_baseSize_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_childAt_p(const QWidget* this_ptr, const QPoint* p);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_childAt_x_y(const QWidget* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_childrenRect_to_output(const QWidget* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT QRegion* qt_widgets_c_QWidget_childrenRegion_as_ptr(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_clearFocus(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_clearMask(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_close(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_contentsMargins_to_output(const QWidget* this_ptr, QMargins* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_contentsRect_to_output(const QWidget* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_createWinId(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QCursor* qt_widgets_c_QWidget_cursor_as_ptr(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_delete(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_devType(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT unsigned long long qt_widgets_c_QWidget_effectiveWinId(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_ensurePolished(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_find(unsigned long long arg1);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_focusProxy(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_focusWidget(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QFont* qt_widgets_c_QWidget_font(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_fontInfo_to_output(const QWidget* this_ptr, QFontInfo* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_fontMetrics_to_output(const QWidget* this_ptr, QFontMetrics* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_frameGeometry_to_output(const QWidget* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_frameSize_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT const QRect* qt_widgets_c_QWidget_geometry(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_getContentsMargins(const QWidget* this_ptr, int* left, int* top, int* right, int* bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_grabKeyboard(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_grabMouse_arg1(QWidget* this_ptr, const QCursor* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_grabMouse_no_args(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_grabShortcut_key(QWidget* this_ptr, const QKeySequence* key);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_grabShortcut_key_context(QWidget* this_ptr, const QKeySequence* key, const Qt::ShortcutContext* context);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QWidget_grab_as_ptr_no_args(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QWidget_grab_as_ptr_rectangle(QWidget* this_ptr, const QRect* rectangle);
QT_WIDGETS_C_EXPORT QGraphicsEffect* qt_widgets_c_QWidget_graphicsEffect(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsProxyWidget* qt_widgets_c_QWidget_graphicsProxyWidget(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_hasFocus(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_hasHeightForWidth(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_hasMouseTracking(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_hasTabletTracking(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_height(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_heightForWidth(const QWidget* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_hide(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_inputMethodQuery_to_output(const QWidget* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_insertAction(QWidget* this_ptr, QAction* before, QAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_insertActions(QWidget* this_ptr, QAction* before, const QList< QAction* >* actions);
QT_WIDGETS_C_EXPORT unsigned long long qt_widgets_c_QWidget_internalWinId(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isActiveWindow(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isAncestorOf(const QWidget* this_ptr, const QWidget* child);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isEnabled(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isEnabledTo(const QWidget* this_ptr, const QWidget* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isEnabledToTLW(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isFullScreen(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isHidden(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isLeftToRight(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isMaximized(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isMinimized(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isModal(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isRightToLeft(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isTopLevel(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isVisible(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isVisibleTo(const QWidget* this_ptr, const QWidget* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isWindow(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_isWindowModified(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_keyboardGrabber();
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QWidget_layout(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_locale_to_output(const QWidget* this_ptr, QLocale* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_lower(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_mapFromGlobal_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_mapFromParent_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_mapFrom_to_output(const QWidget* this_ptr, const QWidget* arg1, const QPoint* arg2, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_mapToGlobal_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_mapToParent_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_mapTo_to_output(const QWidget* this_ptr, const QWidget* arg1, const QPoint* arg2, QPoint* output);
QT_WIDGETS_C_EXPORT QRegion* qt_widgets_c_QWidget_mask_as_ptr(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_maximumHeight(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_maximumSize_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_maximumWidth(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QWidget_metaObject(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_minimumHeight(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_minimumSizeHint_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_minimumSize_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_minimumWidth(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_mouseGrabber();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_move_arg1(QWidget* this_ptr, const QPoint* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_move_x_y(QWidget* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_nativeParentWidget(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_nextInFocusChain(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_normalGeometry_to_output(const QWidget* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT QPaintEngine* qt_widgets_c_QWidget_paintEngine(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QPalette* qt_widgets_c_QWidget_palette(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_parentWidget(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_pos_to_output(const QWidget* this_ptr, QPoint* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_previousInFocusChain(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_qt_metacall(QWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QWidget_qt_metacast(QWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_raise(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_rect_to_output(const QWidget* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_releaseKeyboard(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_releaseMouse(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_releaseShortcut(QWidget* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_removeAction(QWidget* this_ptr, QAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_painter(QWidget* this_ptr, QPainter* painter);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_painter_targetOffset(QWidget* this_ptr, QPainter* painter, const QPoint* targetOffset);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_painter_targetOffset_sourceRegion(QWidget* this_ptr, QPainter* painter, const QPoint* targetOffset, const QRegion* sourceRegion);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_painter_targetOffset_sourceRegion_renderFlags(QWidget* this_ptr, QPainter* painter, const QPoint* targetOffset, const QRegion* sourceRegion, unsigned int renderFlags);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_target(QWidget* this_ptr, QPaintDevice* target);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_target_targetOffset(QWidget* this_ptr, QPaintDevice* target, const QPoint* targetOffset);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_target_targetOffset_sourceRegion(QWidget* this_ptr, QPaintDevice* target, const QPoint* targetOffset, const QRegion* sourceRegion);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_render_target_targetOffset_sourceRegion_renderFlags(QWidget* this_ptr, QPaintDevice* target, const QPoint* targetOffset, const QRegion* sourceRegion, unsigned int renderFlags);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_repaint_QRect(QWidget* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_repaint_QRegion(QWidget* this_ptr, const QRegion* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_repaint_int_int_int_int(QWidget* this_ptr, int x, int y, int w, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_repaint_no_args(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_resize_arg1(QWidget* this_ptr, const QSize* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_resize_w_h(QWidget* this_ptr, int w, int h);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_restoreGeometry(QWidget* this_ptr, const QByteArray* geometry);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_saveGeometry_to_output(const QWidget* this_ptr, QByteArray* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_scroll_dx_dy(QWidget* this_ptr, int dx, int dy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_scroll_dx_dy_arg3(QWidget* this_ptr, int dx, int dy, const QRect* arg3);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setAcceptDrops(QWidget* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setAccessibleDescription(QWidget* this_ptr, const QString* description);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setAccessibleName(QWidget* this_ptr, const QString* name);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setAttribute_arg1(QWidget* this_ptr, const Qt::WidgetAttribute* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setAttribute_arg1_on(QWidget* this_ptr, const Qt::WidgetAttribute* arg1, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setAutoFillBackground(QWidget* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setBackgroundRole(QWidget* this_ptr, const QPalette::ColorRole* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setBaseSize_arg1(QWidget* this_ptr, const QSize* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setBaseSize_basew_baseh(QWidget* this_ptr, int basew, int baseh);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setContentsMargins_left_top_right_bottom(QWidget* this_ptr, int left, int top, int right, int bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setContentsMargins_margins(QWidget* this_ptr, const QMargins* margins);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setContextMenuPolicy(QWidget* this_ptr, const Qt::ContextMenuPolicy* policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setCursor(QWidget* this_ptr, const QCursor* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setDisabled(QWidget* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setEnabled(QWidget* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFixedHeight(QWidget* this_ptr, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFixedSize_arg1(QWidget* this_ptr, const QSize* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFixedSize_w_h(QWidget* this_ptr, int w, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFixedWidth(QWidget* this_ptr, int w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFocusPolicy(QWidget* this_ptr, const Qt::FocusPolicy* policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFocusProxy(QWidget* this_ptr, QWidget* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFocus_no_args(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFocus_reason(QWidget* this_ptr, const Qt::FocusReason* reason);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setFont(QWidget* this_ptr, const QFont* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setForegroundRole(QWidget* this_ptr, const QPalette::ColorRole* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setGeometry_arg1(QWidget* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setGeometry_x_y_w_h(QWidget* this_ptr, int x, int y, int w, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setGraphicsEffect(QWidget* this_ptr, QGraphicsEffect* effect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setHidden(QWidget* this_ptr, bool hidden);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setLayout(QWidget* this_ptr, QLayout* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setLayoutDirection(QWidget* this_ptr, const Qt::LayoutDirection* direction);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setLocale(QWidget* this_ptr, const QLocale* locale);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMask_QBitmap(QWidget* this_ptr, const QBitmap* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMask_QRegion(QWidget* this_ptr, const QRegion* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMaximumHeight(QWidget* this_ptr, int maxh);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMaximumSize_arg1(QWidget* this_ptr, const QSize* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMaximumSize_maxw_maxh(QWidget* this_ptr, int maxw, int maxh);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMaximumWidth(QWidget* this_ptr, int maxw);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMinimumHeight(QWidget* this_ptr, int minh);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMinimumSize_arg1(QWidget* this_ptr, const QSize* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMinimumSize_minw_minh(QWidget* this_ptr, int minw, int minh);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMinimumWidth(QWidget* this_ptr, int minw);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setMouseTracking(QWidget* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setPalette(QWidget* this_ptr, const QPalette* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setParent(QWidget* this_ptr, QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setShortcutAutoRepeat_id(QWidget* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setShortcutAutoRepeat_id_enable(QWidget* this_ptr, int id, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setShortcutEnabled_id(QWidget* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setShortcutEnabled_id_enable(QWidget* this_ptr, int id, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setSizeIncrement_arg1(QWidget* this_ptr, const QSize* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setSizeIncrement_w_h(QWidget* this_ptr, int w, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setSizePolicy_arg1(QWidget* this_ptr, const QSizePolicy* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setSizePolicy_horizontal_vertical(QWidget* this_ptr, const QSizePolicy::Policy* horizontal, const QSizePolicy::Policy* vertical);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setStatusTip(QWidget* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setStyle(QWidget* this_ptr, QStyle* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setStyleSheet(QWidget* this_ptr, const QString* styleSheet);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setTabOrder(QWidget* arg1, QWidget* arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setTabletTracking(QWidget* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setToolTip(QWidget* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setToolTipDuration(QWidget* this_ptr, int msec);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setUpdatesEnabled(QWidget* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setVisible(QWidget* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWhatsThis(QWidget* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowFilePath(QWidget* this_ptr, const QString* filePath);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowFlag_arg1(QWidget* this_ptr, const Qt::WindowType* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowFlag_arg1_on(QWidget* this_ptr, const Qt::WindowType* arg1, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowIcon(QWidget* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowIconText(QWidget* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowModality(QWidget* this_ptr, const Qt::WindowModality* windowModality);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowModified(QWidget* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowOpacity(QWidget* this_ptr, double level);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowRole(QWidget* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_setWindowTitle(QWidget* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_show(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_showFullScreen(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_showMaximized(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_showMinimized(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_showNormal(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_sizeHint_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_sizeIncrement_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_sizePolicy_to_output(const QWidget* this_ptr, QSizePolicy* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_size_to_output(const QWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_stackUnder(QWidget* this_ptr, QWidget* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_statusTip_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QWidget_style(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_styleSheet_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_testAttribute(const QWidget* this_ptr, const Qt::WidgetAttribute* arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_toolTipDuration(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_toolTip_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_topLevelWidget(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_underMouse(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_ungrabGesture(QWidget* this_ptr, const Qt::GestureType* type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_unsetCursor(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_unsetLayoutDirection(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_unsetLocale(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_updateGeometry(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_update_QRect(QWidget* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_update_QRegion(QWidget* this_ptr, const QRegion* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_update_int_int_int_int(QWidget* this_ptr, int x, int y, int w, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_update_no_args(QWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidget_updatesEnabled(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QRegion* qt_widgets_c_QWidget_visibleRegion_as_ptr(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_whatsThis_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_width(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT unsigned long long qt_widgets_c_QWidget_winId(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidget_window(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_windowFilePath_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT QWindow* qt_widgets_c_QWidget_windowHandle(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_windowIconText_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_windowIcon_to_output(const QWidget* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QWidget_windowOpacity(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_windowRole_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidget_windowTitle_to_output(const QWidget* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_x(const QWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidget_y(const QWidget* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QWIDGET_H
