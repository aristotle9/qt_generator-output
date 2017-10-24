#include "qt_core_c_QFactoryInterface.h"

void qt_core_c_QFactoryInterface_delete(QFactoryInterface* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QFactoryInterface_keys_to_output(const QFactoryInterface* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->keys());
}

