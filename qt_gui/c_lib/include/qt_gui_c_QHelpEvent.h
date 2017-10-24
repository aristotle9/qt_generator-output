#ifndef QT_GUI_C_QHELPEVENT_H
#define QT_GUI_C_QHELPEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QHelpEvent_G_static_cast_QEvent_ptr(QHelpEvent* ptr);
QT_GUI_C_EXPORT QHelpEvent* qt_gui_c_QHelpEvent_G_static_cast_QHelpEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QHelpEvent_delete(QHelpEvent* this_ptr);
QT_GUI_C_EXPORT const QPoint* qt_gui_c_QHelpEvent_globalPos(const QHelpEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QHelpEvent_globalX(const QHelpEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QHelpEvent_globalY(const QHelpEvent* this_ptr);
QT_GUI_C_EXPORT QHelpEvent* qt_gui_c_QHelpEvent_new(QEvent::Type type, const QPoint* pos, const QPoint* globalPos);
QT_GUI_C_EXPORT const QPoint* qt_gui_c_QHelpEvent_pos(const QHelpEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QHelpEvent_x(const QHelpEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QHelpEvent_y(const QHelpEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QHELPEVENT_H
