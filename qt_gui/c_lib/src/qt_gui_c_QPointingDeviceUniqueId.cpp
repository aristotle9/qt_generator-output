#include "qt_gui_c_QPointingDeviceUniqueId.h"

void qt_gui_c_QPointingDeviceUniqueId_constructor(QPointingDeviceUniqueId* output) {
  new(output) QPointingDeviceUniqueId();
}

void qt_gui_c_QPointingDeviceUniqueId_destructor(QPointingDeviceUniqueId* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QPointingDeviceUniqueId_fromNumericId_to_output(qint64 id, QPointingDeviceUniqueId* output) {
  new(output) QPointingDeviceUniqueId(QPointingDeviceUniqueId::fromNumericId(id));
}

bool qt_gui_c_QPointingDeviceUniqueId_isValid(const QPointingDeviceUniqueId* this_ptr) {
  return this_ptr->isValid();
}

qint64 qt_gui_c_QPointingDeviceUniqueId_numericId(const QPointingDeviceUniqueId* this_ptr) {
  return this_ptr->numericId();
}

