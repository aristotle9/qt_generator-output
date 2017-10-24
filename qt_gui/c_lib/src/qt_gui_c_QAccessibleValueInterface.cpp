#include "qt_gui_c_QAccessibleValueInterface.h"

void qt_gui_c_QAccessibleValueInterface_currentValue_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->currentValue());
}

void qt_gui_c_QAccessibleValueInterface_delete(QAccessibleValueInterface* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QAccessibleValueInterface_maximumValue_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->maximumValue());
}

void qt_gui_c_QAccessibleValueInterface_minimumStepSize_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->minimumStepSize());
}

void qt_gui_c_QAccessibleValueInterface_minimumValue_to_output(const QAccessibleValueInterface* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->minimumValue());
}

void qt_gui_c_QAccessibleValueInterface_setCurrentValue(QAccessibleValueInterface* this_ptr, const QVariant* value) {
  this_ptr->setCurrentValue(*value);
}

