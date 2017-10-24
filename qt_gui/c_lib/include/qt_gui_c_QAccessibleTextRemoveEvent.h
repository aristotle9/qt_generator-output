#ifndef QT_GUI_C_QACCESSIBLETEXTREMOVEEVENT_H
#define QT_GUI_C_QACCESSIBLETEXTREMOVEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr);
QT_GUI_C_EXPORT QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr);
QT_GUI_C_EXPORT QAccessibleEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleTextRemoveEvent* ptr);
QT_GUI_C_EXPORT QAccessibleTextCursorEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(QAccessibleTextRemoveEvent* ptr);
QT_GUI_C_EXPORT QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(QAccessibleEvent* ptr);
QT_GUI_C_EXPORT QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(QAccessibleTextCursorEvent* ptr);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleTextRemoveEvent_changePosition(const QAccessibleTextRemoveEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextRemoveEvent_delete(QAccessibleTextRemoveEvent* this_ptr);
QT_GUI_C_EXPORT QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_new_iface_position_text(QAccessibleInterface* iface, int position, const QString* text);
QT_GUI_C_EXPORT QAccessibleTextRemoveEvent* qt_gui_c_QAccessibleTextRemoveEvent_new_obj_position_text(QObject* obj, int position, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleTextRemoveEvent_textRemoved_to_output(const QAccessibleTextRemoveEvent* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLETEXTREMOVEEVENT_H
