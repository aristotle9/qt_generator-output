#ifndef QT_GUI_C_QACTIONEVENT_H
#define QT_GUI_C_QACTIONEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QActionEvent_G_operator_eq_e_key(QKeyEvent* e, const QKeySequence::StandardKey* key);
QT_GUI_C_EXPORT bool qt_gui_c_QActionEvent_G_operator_eq_key_e(const QKeySequence::StandardKey* key, QKeyEvent* e);
QT_GUI_C_EXPORT bool qt_gui_c_QActionEvent_G_operator_eq_lhs_rhs(const QPointingDeviceUniqueId* lhs, const QPointingDeviceUniqueId* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QActionEvent_G_operator_neq(const QPointingDeviceUniqueId* lhs, const QPointingDeviceUniqueId* rhs);
QT_GUI_C_EXPORT void qt_gui_c_QActionEvent_G_operator_shl_to_output_QDebug_QEvent(const QDebug* arg1, const QEvent* arg2, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QActionEvent_G_operator_shl_to_output_QDebug_QTouchEvent_TouchPoint(const QDebug* arg1, const QTouchEvent::TouchPoint* arg2, QDebug* output);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QActionEvent_G_qHash_key(const QPointingDeviceUniqueId* key);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QActionEvent_G_qHash_key_seed(const QPointingDeviceUniqueId* key, unsigned int seed);
QT_GUI_C_EXPORT QActionEvent* qt_gui_c_QActionEvent_G_static_cast_QActionEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QActionEvent_G_static_cast_QEvent_ptr(QActionEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QActionEvent_delete(QActionEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QACTIONEVENT_H
