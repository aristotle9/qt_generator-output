#ifndef QT_WIDGETS_C_QGRAPHICSWIDGET_H
#define QT_WIDGETS_C_QGRAPHICSWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsObject(QGraphicsObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsItem_ptr(QGraphicsWidget* ptr);
QT_WIDGETS_C_EXPORT QGraphicsLayoutItem* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsWidget* ptr);
QT_WIDGETS_C_EXPORT QGraphicsObject* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsObject_ptr(QGraphicsWidget* ptr);
QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsObject(QGraphicsObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsWidget_G_static_cast_QObject_ptr(QGraphicsWidget* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_actions_to_output(const QGraphicsWidget* this_ptr, QList< QAction* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_addAction(QGraphicsWidget* this_ptr, QAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_addActions(QGraphicsWidget* this_ptr, const QList< QAction* >* actions);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_adjustSize(QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsWidget_autoFillBackground(const QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_boundingRect_to_output(const QGraphicsWidget* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsWidget_close(QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_delete(QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsWidget* qt_widgets_c_QGraphicsWidget_focusWidget(const QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_font_to_output(const QGraphicsWidget* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_getContentsMargins(const QGraphicsWidget* this_ptr, double* left, double* top, double* right, double* bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_getWindowFrameMargins(const QGraphicsWidget* this_ptr, double* left, double* top, double* right, double* bottom);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsWidget_grabShortcut_sequence(QGraphicsWidget* this_ptr, const QKeySequence* sequence);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsWidget_grabShortcut_sequence_context(QGraphicsWidget* this_ptr, const QKeySequence* sequence, const Qt::ShortcutContext* context);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_insertAction(QGraphicsWidget* this_ptr, QAction* before, QAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_insertActions(QGraphicsWidget* this_ptr, QAction* before, const QList< QAction* >* actions);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsWidget_isActiveWindow(const QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsLayout* qt_widgets_c_QGraphicsWidget_layout(const QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsWidget_metaObject(const QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_paintWindowFrame_painter_option(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_paintWindowFrame_painter_option_widget(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_paint_painter_option(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_paint_painter_option_widget(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_palette_to_output(const QGraphicsWidget* this_ptr, QPalette* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsWidget_qt_metacall(QGraphicsWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsWidget_qt_metacast(QGraphicsWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_rect_to_output(const QGraphicsWidget* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_releaseShortcut(QGraphicsWidget* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_removeAction(QGraphicsWidget* this_ptr, QAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_resize_size(QGraphicsWidget* this_ptr, const QSizeF* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_resize_w_h(QGraphicsWidget* this_ptr, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setAttribute_attribute(QGraphicsWidget* this_ptr, const Qt::WidgetAttribute* attribute);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setAttribute_attribute_on(QGraphicsWidget* this_ptr, const Qt::WidgetAttribute* attribute, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setAutoFillBackground(QGraphicsWidget* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setContentsMargins(QGraphicsWidget* this_ptr, double left, double top, double right, double bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setFocusPolicy(QGraphicsWidget* this_ptr, const Qt::FocusPolicy* policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setFont(QGraphicsWidget* this_ptr, const QFont* font);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setGeometry_rect(QGraphicsWidget* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setGeometry_x_y_w_h(QGraphicsWidget* this_ptr, double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setLayout(QGraphicsWidget* this_ptr, QGraphicsLayout* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setLayoutDirection(QGraphicsWidget* this_ptr, const Qt::LayoutDirection* direction);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setPalette(QGraphicsWidget* this_ptr, const QPalette* palette);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setShortcutAutoRepeat_id(QGraphicsWidget* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setShortcutAutoRepeat_id_enabled(QGraphicsWidget* this_ptr, int id, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setShortcutEnabled_id(QGraphicsWidget* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setShortcutEnabled_id_enabled(QGraphicsWidget* this_ptr, int id, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setStyle(QGraphicsWidget* this_ptr, QStyle* style);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setTabOrder(QGraphicsWidget* first, QGraphicsWidget* second);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setWindowFrameMargins(QGraphicsWidget* this_ptr, double left, double top, double right, double bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_setWindowTitle(QGraphicsWidget* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_shape_to_output(const QGraphicsWidget* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_size_to_output(const QGraphicsWidget* this_ptr, QSizeF* output);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QGraphicsWidget_style(const QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsWidget_testAttribute(const QGraphicsWidget* this_ptr, const Qt::WidgetAttribute* attribute);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsWidget_type(const QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_unsetLayoutDirection(QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_unsetWindowFrameMargins(QGraphicsWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_windowFrameGeometry_to_output(const QGraphicsWidget* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_windowFrameRect_to_output(const QGraphicsWidget* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsWidget_windowTitle_to_output(const QGraphicsWidget* this_ptr, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSWIDGET_H
