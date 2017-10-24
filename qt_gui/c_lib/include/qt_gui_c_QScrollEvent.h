#ifndef QT_GUI_C_QSCROLLEVENT_H
#define QT_GUI_C_QSCROLLEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QScrollEvent_G_static_cast_QEvent_ptr(QScrollEvent* ptr);
QT_GUI_C_EXPORT QScrollEvent* qt_gui_c_QScrollEvent_G_static_cast_QScrollEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScrollEvent_contentPos_to_output(const QScrollEvent* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QScrollEvent_delete(QScrollEvent* this_ptr);
QT_GUI_C_EXPORT QScrollEvent* qt_gui_c_QScrollEvent_new(const QPointF* contentPos, const QPointF* overshoot, QScrollEvent::ScrollState scrollState);
QT_GUI_C_EXPORT void qt_gui_c_QScrollEvent_overshootDistance_to_output(const QScrollEvent* this_ptr, QPointF* output);
QT_GUI_C_EXPORT QScrollEvent::ScrollState qt_gui_c_QScrollEvent_scrollState(const QScrollEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QSCROLLEVENT_H
