#ifndef QT_GUI_C_QCONTEXTMENUEVENT_H
#define QT_GUI_C_QCONTEXTMENUEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QContextMenuEvent* qt_gui_c_QContextMenuEvent_G_dynamic_cast_QContextMenuEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QContextMenuEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QContextMenuEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QEvent_ptr(QContextMenuEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QInputEvent_ptr(QContextMenuEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QContextMenuEvent_delete(QContextMenuEvent* this_ptr);
QT_GUI_C_EXPORT const QPoint* qt_gui_c_QContextMenuEvent_globalPos(const QContextMenuEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QContextMenuEvent_globalX(const QContextMenuEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QContextMenuEvent_globalY(const QContextMenuEvent* this_ptr);
QT_GUI_C_EXPORT QContextMenuEvent* qt_gui_c_QContextMenuEvent_new_reason_pos(QContextMenuEvent::Reason reason, const QPoint* pos);
QT_GUI_C_EXPORT QContextMenuEvent* qt_gui_c_QContextMenuEvent_new_reason_pos_globalPos(QContextMenuEvent::Reason reason, const QPoint* pos, const QPoint* globalPos);
QT_GUI_C_EXPORT const QPoint* qt_gui_c_QContextMenuEvent_pos(const QContextMenuEvent* this_ptr);
QT_GUI_C_EXPORT QContextMenuEvent::Reason qt_gui_c_QContextMenuEvent_reason(const QContextMenuEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QContextMenuEvent_x(const QContextMenuEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QContextMenuEvent_y(const QContextMenuEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QCONTEXTMENUEVENT_H
