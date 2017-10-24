#ifndef QT_GUI_C_QFOCUSEVENT_H
#define QT_GUI_C_QFOCUSEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QFocusEvent_G_static_cast_QEvent_ptr(QFocusEvent* ptr);
QT_GUI_C_EXPORT QFocusEvent* qt_gui_c_QFocusEvent_G_static_cast_QFocusEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFocusEvent_delete(QFocusEvent* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFocusEvent_gotFocus(const QFocusEvent* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFocusEvent_lostFocus(const QFocusEvent* this_ptr);
QT_GUI_C_EXPORT QFocusEvent* qt_gui_c_QFocusEvent_new_type(QEvent::Type type);
QT_GUI_C_EXPORT QFocusEvent* qt_gui_c_QFocusEvent_new_type_reason(QEvent::Type type, const Qt::FocusReason* reason);

} // extern "C"

#endif // QT_GUI_C_QFOCUSEVENT_H
