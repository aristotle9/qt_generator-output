#ifndef QT_GUI_C_QACCESSIBLEACTIONINTERFACE_H
#define QT_GUI_C_QACCESSIBLEACTIONINTERFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_actionNames_to_output(const QAccessibleActionInterface* this_ptr, QStringList* output);
QT_GUI_C_EXPORT const QString* qt_gui_c_QAccessibleActionInterface_decreaseAction();
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_delete(QAccessibleActionInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_doAction(QAccessibleActionInterface* this_ptr, const QString* actionName);
QT_GUI_C_EXPORT const QString* qt_gui_c_QAccessibleActionInterface_increaseAction();
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_keyBindingsForAction_to_output(const QAccessibleActionInterface* this_ptr, const QString* actionName, QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_localizedActionDescription_to_output(const QAccessibleActionInterface* this_ptr, const QString* name, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_localizedActionName_to_output(const QAccessibleActionInterface* this_ptr, const QString* name, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_nextPageAction_to_output(QString* output);
QT_GUI_C_EXPORT const QString* qt_gui_c_QAccessibleActionInterface_pressAction();
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_previousPageAction_to_output(QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_scrollDownAction_to_output(QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_scrollLeftAction_to_output(QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_scrollRightAction_to_output(QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_scrollUpAction_to_output(QString* output);
QT_GUI_C_EXPORT const QString* qt_gui_c_QAccessibleActionInterface_setFocusAction();
QT_GUI_C_EXPORT const QString* qt_gui_c_QAccessibleActionInterface_showMenuAction();
QT_GUI_C_EXPORT const QString* qt_gui_c_QAccessibleActionInterface_toggleAction();
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleActionInterface_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLEACTIONINTERFACE_H
