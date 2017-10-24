#ifndef QT_GUI_C_QWHEELEVENT_H
#define QT_GUI_C_QWHEELEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QWheelEvent* qt_gui_c_QWheelEvent_G_dynamic_cast_QWheelEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QWheelEvent_G_static_cast_QEvent_ptr(QWheelEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QWheelEvent_G_static_cast_QInputEvent_ptr(QWheelEvent* ptr);
QT_GUI_C_EXPORT QWheelEvent* qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QWheelEvent* qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QWheelEvent_angleDelta_to_output(const QWheelEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QWheelEvent_delete(QWheelEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QWheelEvent_delta(const QWheelEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QWheelEvent_globalPosF(const QWheelEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QWheelEvent_globalPos_to_output(const QWheelEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT int qt_gui_c_QWheelEvent_globalX(const QWheelEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QWheelEvent_globalY(const QWheelEvent* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QWheelEvent_inverted(const QWheelEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QWheelEvent_pixelDelta_to_output(const QWheelEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QWheelEvent_posF(const QWheelEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QWheelEvent_pos_to_output(const QWheelEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT int qt_gui_c_QWheelEvent_x(const QWheelEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QWheelEvent_y(const QWheelEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QWHEELEVENT_H
