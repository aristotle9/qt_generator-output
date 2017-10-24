#ifndef QT_WIDGETS_C_QGRAPHICSLAYOUT_H
#define QT_WIDGETS_C_QGRAPHICSLAYOUT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsLayout* qt_widgets_c_QGraphicsLayout_G_dynamic_cast_QGraphicsLayout_ptr(QGraphicsLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsLayoutItem* qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsLayout* ptr);
QT_WIDGETS_C_EXPORT QGraphicsLayout* qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayout_ptr(QGraphicsLayoutItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_activate(QGraphicsLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsLayout_count(const QGraphicsLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_delete(QGraphicsLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_getContentsMargins(const QGraphicsLayout* this_ptr, double* left, double* top, double* right, double* bottom);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsLayout_instantInvalidatePropagation();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_invalidate(QGraphicsLayout* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsLayout_isActivated(const QGraphicsLayout* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsLayoutItem* qt_widgets_c_QGraphicsLayout_itemAt(const QGraphicsLayout* this_ptr, int i);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_removeAt(QGraphicsLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_setContentsMargins(QGraphicsLayout* this_ptr, double left, double top, double right, double bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_setInstantInvalidatePropagation(bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_updateGeometry(QGraphicsLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayout_widgetEvent(QGraphicsLayout* this_ptr, QEvent* e);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSLAYOUT_H
