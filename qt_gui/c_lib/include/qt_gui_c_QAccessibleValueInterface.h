#ifndef QT_GUI_C_QACCESSIBLEVALUEINTERFACE_H
#define QT_GUI_C_QACCESSIBLEVALUEINTERFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QAccessibleValueInterface_currentValue_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleValueInterface_delete(QAccessibleValueInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleValueInterface_maximumValue_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleValueInterface_minimumStepSize_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleValueInterface_minimumValue_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessibleValueInterface_setCurrentValue(QAccessibleValueInterface* this_ptr, const QVariant* value);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLEVALUEINTERFACE_H
