#ifndef QT_GUI_C_QDRAGMOVEEVENT_H
#define QT_GUI_C_QDRAGMOVEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDragMoveEvent* qt_gui_c_QDragMoveEvent_G_dynamic_cast_QDragMoveEvent_ptr(QDropEvent* ptr);
QT_GUI_C_EXPORT QDragMoveEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QDropEvent(QDropEvent* ptr);
QT_GUI_C_EXPORT QDragMoveEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QDropEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QDropEvent_ptr(QDragMoveEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QEvent_ptr(QDragMoveEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDragMoveEvent_accept_no_args(QDragMoveEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDragMoveEvent_accept_r(QDragMoveEvent* this_ptr, const QRect* r);
QT_GUI_C_EXPORT void qt_gui_c_QDragMoveEvent_answerRect_to_output(const QDragMoveEvent* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QDragMoveEvent_delete(QDragMoveEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDragMoveEvent_ignore_no_args(QDragMoveEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDragMoveEvent_ignore_r(QDragMoveEvent* this_ptr, const QRect* r);

} // extern "C"

#endif // QT_GUI_C_QDRAGMOVEEVENT_H
