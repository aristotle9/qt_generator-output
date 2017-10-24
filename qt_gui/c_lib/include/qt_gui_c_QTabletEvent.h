#ifndef QT_GUI_C_QTABLETEVENT_H
#define QT_GUI_C_QTABLETEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTabletEvent* qt_gui_c_QTabletEvent_G_dynamic_cast_QTabletEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QTabletEvent_G_static_cast_QEvent_ptr(QTabletEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QTabletEvent_G_static_cast_QInputEvent_ptr(QTabletEvent* ptr);
QT_GUI_C_EXPORT QTabletEvent* qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QTabletEvent* qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTabletEvent_delete(QTabletEvent* this_ptr);
QT_GUI_C_EXPORT QTabletEvent::TabletDevice qt_gui_c_QTabletEvent_device(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QTabletEvent_globalPosF(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTabletEvent_globalPos_to_output(const QTabletEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT int qt_gui_c_QTabletEvent_globalX(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTabletEvent_globalY(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTabletEvent_hiResGlobalX(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTabletEvent_hiResGlobalY(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT QTabletEvent::PointerType qt_gui_c_QTabletEvent_pointerType(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QTabletEvent_posF(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTabletEvent_pos_to_output(const QTabletEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT double qt_gui_c_QTabletEvent_pressure(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTabletEvent_rotation(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTabletEvent_tangentialPressure(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT qint64 qt_gui_c_QTabletEvent_uniqueId(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTabletEvent_x(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTabletEvent_xTilt(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTabletEvent_y(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTabletEvent_yTilt(const QTabletEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTabletEvent_z(const QTabletEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTABLETEVENT_H
