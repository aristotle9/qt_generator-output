#include "qt_core_c_QSharedData.h"

void qt_core_c_QSharedData_constructor_arg1(const QSharedData* arg1, QSharedData* output) {
  new(output) QSharedData(*arg1);
}

void qt_core_c_QSharedData_constructor_no_args(QSharedData* output) {
  new(output) QSharedData();
}

void qt_core_c_QSharedData_destructor(QSharedData* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

