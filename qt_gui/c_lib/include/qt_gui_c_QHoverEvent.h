#ifndef QT_GUI_C_QHOVEREVENT_H
#define QT_GUI_C_QHOVEREVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QHoverEvent* qt_gui_c_QHoverEvent_G_dynamic_cast_QHoverEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QHoverEvent_G_static_cast_QEvent_ptr(QHoverEvent* ptr);
QT_GUI_C_EXPORT QHoverEvent* qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QHoverEvent* qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QHoverEvent_G_static_cast_QInputEvent_ptr(QHoverEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QHoverEvent_delete(QHoverEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QHoverEvent_oldPosF(const QHoverEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QHoverEvent_oldPos_to_output(const QHoverEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QHoverEvent_posF(const QHoverEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QHoverEvent_pos_to_output(const QHoverEvent* this_ptr, QPoint* output);

} // extern "C"

#endif // QT_GUI_C_QHOVEREVENT_H
