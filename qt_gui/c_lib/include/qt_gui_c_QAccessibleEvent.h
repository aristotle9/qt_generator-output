#ifndef QT_GUI_C_QACCESSIBLEEVENT_H
#define QT_GUI_C_QACCESSIBLEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleEvent_accessibleInterface(const QAccessibleEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleEvent_child(const QAccessibleEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleEvent_delete(QAccessibleEvent* this_ptr);
QT_GUI_C_EXPORT QAccessibleEvent* qt_gui_c_QAccessibleEvent_new_iface_typ(QAccessibleInterface* iface, const QAccessible::Event* typ);
QT_GUI_C_EXPORT QAccessibleEvent* qt_gui_c_QAccessibleEvent_new_obj_typ(QObject* obj, const QAccessible::Event* typ);
QT_GUI_C_EXPORT QObject* qt_gui_c_QAccessibleEvent_object(const QAccessibleEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleEvent_setChild(QAccessibleEvent* this_ptr, int chld);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLEEVENT_H
