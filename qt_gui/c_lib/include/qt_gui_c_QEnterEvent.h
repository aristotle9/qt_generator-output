#ifndef QT_GUI_C_QENTEREVENT_H
#define QT_GUI_C_QENTEREVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEnterEvent* qt_gui_c_QEnterEvent_G_static_cast_QEnterEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QEnterEvent_G_static_cast_QEvent_ptr(QEnterEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QEnterEvent_delete(QEnterEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QEnterEvent_globalPos_to_output(const QEnterEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT int qt_gui_c_QEnterEvent_globalX(const QEnterEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QEnterEvent_globalY(const QEnterEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QEnterEvent_localPos(const QEnterEvent* this_ptr);
QT_GUI_C_EXPORT QEnterEvent* qt_gui_c_QEnterEvent_new(const QPointF* localPos, const QPointF* windowPos, const QPointF* screenPos);
QT_GUI_C_EXPORT void qt_gui_c_QEnterEvent_pos_to_output(const QEnterEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QEnterEvent_screenPos(const QEnterEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QEnterEvent_windowPos(const QEnterEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QEnterEvent_x(const QEnterEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QEnterEvent_y(const QEnterEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QENTEREVENT_H
