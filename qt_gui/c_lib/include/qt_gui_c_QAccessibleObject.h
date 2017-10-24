#ifndef QT_GUI_C_QACCESSIBLEOBJECT_H
#define QT_GUI_C_QACCESSIBLEOBJECT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QAccessibleObject* qt_gui_c_QAccessibleObject_G_dynamic_cast_QAccessibleObject_ptr(QAccessibleInterface* ptr);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleInterface_ptr(QAccessibleObject* ptr);
QT_GUI_C_EXPORT QAccessibleObject* qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleObject_ptr(QAccessibleInterface* ptr);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleObject_childAt(const QAccessibleObject* this_ptr, int x, int y);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleObject_isValid(const QAccessibleObject* this_ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QAccessibleObject_object(const QAccessibleObject* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleObject_rect_to_output(const QAccessibleObject* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleObject_setText(QAccessibleObject* this_ptr, const QAccessible::Text* t, const QString* text);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLEOBJECT_H
