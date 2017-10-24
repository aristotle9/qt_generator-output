#ifndef QT_GUI_C_QACCESSIBLE_H
#define QT_GUI_C_QACCESSIBLE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QAccessible_G_operator_eq(const QAccessible::State* first, const QAccessible::State* second);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_G_operator_shl_to_output_d_ev(const QDebug* d, const QAccessibleEvent* ev, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_G_operator_shl_to_output_d_iface(const QDebug* d, const QAccessibleInterface* iface, QDebug* output);
QT_GUI_C_EXPORT const char* qt_gui_c_QAccessible_G_qAccessibleEventString(const QAccessible::Event* event);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_G_qAccessibleLocalizedActionDescription_to_output(const QString* actionName, QString* output);
QT_GUI_C_EXPORT const char* qt_gui_c_QAccessible_G_qAccessibleRoleString(const QAccessible::Role* role);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_active(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_animated(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_busy(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_checkStateMixed(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_checkable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_checked(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_collapsed(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_constructor(QAccessible::State* output);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_defaultButton(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_destructor(QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_disabled(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_editable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_expandable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_expanded(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_extSelectable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_focusable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_focused(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_hasPopup(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_hotTracked(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_invalid(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_invisible(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_linked(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_marqueed(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_modal(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_movable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_multiLine(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_multiSelectable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_offscreen(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_passwordEdit(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_pressed(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_readOnly(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_searchEdit(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_selectable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_selectableText(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_selected(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_selfVoicing(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_active(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_animated(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_busy(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_checkStateMixed(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_checkable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_checked(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_collapsed(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_defaultButton(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_disabled(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_editable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_expandable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_expanded(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_extSelectable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_focusable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_focused(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_hasPopup(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_hotTracked(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_invalid(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_invisible(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_linked(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_marqueed(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_modal(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_movable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_multiLine(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_multiSelectable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_offscreen(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_passwordEdit(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_pressed(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_readOnly(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_searchEdit(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_selectable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_selectableText(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_selected(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_selfVoicing(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_sizeable(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_supportsAutoCompletion(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_State_set_traversed(QAccessible::State* this_ptr, quint64 value);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_sizeable(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_supportsAutoCompletion(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT quint64 qt_gui_c_QAccessible_State_traversed(const QAccessible::State* this_ptr);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessible_accessibleInterface(unsigned int uniqueId);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_cleanup();
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_delete(QAccessible* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_deleteAccessibleInterface(unsigned int uniqueId);
QT_GUI_C_EXPORT void (*qt_gui_c_QAccessible_installRootObjectHandler(void (*arg1)(QObject*)))(QObject*);
QT_GUI_C_EXPORT void (*qt_gui_c_QAccessible_installUpdateHandler(void (*arg1)(QAccessibleEvent*)))(QAccessibleEvent*);
QT_GUI_C_EXPORT bool qt_gui_c_QAccessible_isActive();
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_qAccessibleTextBoundaryHelper_to_output(const QTextCursor* cursor, QAccessible::TextBoundaryType boundaryType, QPair< int, int >* output);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessible_queryAccessibleInterface(QObject* arg1);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QAccessible_registerAccessibleInterface(QAccessibleInterface* iface);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_setActive(bool active);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_setRootObject(QObject* object);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QAccessible_uniqueId(QAccessibleInterface* iface);
QT_GUI_C_EXPORT void qt_gui_c_QAccessible_updateAccessibility(QAccessibleEvent* event);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLE_H
