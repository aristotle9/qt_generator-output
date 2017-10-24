#include "qt_widgets_c_QStyleFactory.h"

QStyle* qt_widgets_c_QStyleFactory_create(const QString* arg1) {
  return QStyleFactory::create(*arg1);
}

void qt_widgets_c_QStyleFactory_delete(QStyleFactory* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QStyleFactory_keys_to_output(QStringList* output) {
  new(output) QStringList(QStyleFactory::keys());
}

