#ifndef QT_GUI_C_QMOUSEEVENT_H
#define QT_GUI_C_QMOUSEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QMouseEvent* qt_gui_c_QMouseEvent_G_dynamic_cast_QMouseEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QMouseEvent_G_static_cast_QEvent_ptr(QMouseEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QMouseEvent_G_static_cast_QInputEvent_ptr(QMouseEvent* ptr);
QT_GUI_C_EXPORT QMouseEvent* qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QMouseEvent* qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMouseEvent_delete(QMouseEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMouseEvent_globalPos_to_output(const QMouseEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT int qt_gui_c_QMouseEvent_globalX(const QMouseEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QMouseEvent_globalY(const QMouseEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QMouseEvent_localPos(const QMouseEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMouseEvent_pos_to_output(const QMouseEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QMouseEvent_screenPos(const QMouseEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMouseEvent_setLocalPos(QMouseEvent* this_ptr, const QPointF* localPosition);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QMouseEvent_windowPos(const QMouseEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QMouseEvent_x(const QMouseEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QMouseEvent_y(const QMouseEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QMOUSEEVENT_H
