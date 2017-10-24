#ifndef QT_GUI_C_QACCESSIBLEINTERFACE_H
#define QT_GUI_C_QACCESSIBLEINTERFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QAccessibleActionInterface* qt_gui_c_QAccessibleInterface_actionInterface(QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleInterface_backgroundColor_to_output(const QAccessibleInterface* this_ptr, QColor* output);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleInterface_child(const QAccessibleInterface* this_ptr, int index);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleInterface_childAt(const QAccessibleInterface* this_ptr, int x, int y);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleInterface_childCount(const QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT QAccessibleEditableTextInterface* qt_gui_c_QAccessibleInterface_editableTextInterface(QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleInterface_focusChild(const QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleInterface_foregroundColor_to_output(const QAccessibleInterface* this_ptr, QColor* output);
QT_GUI_C_EXPORT int qt_gui_c_QAccessibleInterface_indexOfChild(const QAccessibleInterface* this_ptr, const QAccessibleInterface* arg1);
QT_GUI_C_EXPORT void* qt_gui_c_QAccessibleInterface_interface_cast(QAccessibleInterface* this_ptr, const QAccessible::InterfaceType* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessibleInterface_isValid(const QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QAccessibleInterface_object(const QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessibleInterface_parent(const QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleInterface_rect_to_output(const QAccessibleInterface* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleInterface_setText(QAccessibleInterface* this_ptr, const QAccessible::Text* t, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleInterface_state_to_output(const QAccessibleInterface* this_ptr, QAccessible::State* output);
QT_GUI_C_EXPORT QAccessibleTableCellInterface* qt_gui_c_QAccessibleInterface_tableCellInterface(QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT QAccessibleTableInterface* qt_gui_c_QAccessibleInterface_tableInterface(QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT QAccessibleTextInterface* qt_gui_c_QAccessibleInterface_textInterface(QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleInterface_text_to_output(const QAccessibleInterface* this_ptr, const QAccessible::Text* t, QString* output);
QT_GUI_C_EXPORT QAccessibleValueInterface* qt_gui_c_QAccessibleInterface_valueInterface(QAccessibleInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleInterface_virtual_hook(QAccessibleInterface* this_ptr, int id, void* data);
QT_GUI_C_EXPORT QWindow* qt_gui_c_QAccessibleInterface_window(const QAccessibleInterface* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLEINTERFACE_H
