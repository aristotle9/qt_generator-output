#ifndef QT_GUI_C_QMOVEEVENT_H
#define QT_GUI_C_QMOVEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QMoveEvent_G_static_cast_QEvent_ptr(QMoveEvent* ptr);
QT_GUI_C_EXPORT QMoveEvent* qt_gui_c_QMoveEvent_G_static_cast_QMoveEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QMoveEvent_delete(QMoveEvent* this_ptr);
QT_GUI_C_EXPORT QMoveEvent* qt_gui_c_QMoveEvent_new(const QPoint* pos, const QPoint* oldPos);
QT_GUI_C_EXPORT const QPoint* qt_gui_c_QMoveEvent_oldPos(const QMoveEvent* this_ptr);
QT_GUI_C_EXPORT const QPoint* qt_gui_c_QMoveEvent_pos(const QMoveEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QMOVEEVENT_H
