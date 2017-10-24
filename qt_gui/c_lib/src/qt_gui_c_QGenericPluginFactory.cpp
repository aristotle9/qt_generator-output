#include "qt_gui_c_QGenericPluginFactory.h"

QObject* qt_gui_c_QGenericPluginFactory_create(const QString* arg1, const QString* arg2) {
  return QGenericPluginFactory::create(*arg1, *arg2);
}

void qt_gui_c_QGenericPluginFactory_delete(QGenericPluginFactory* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QGenericPluginFactory_keys_to_output(QStringList* output) {
  new(output) QStringList(QGenericPluginFactory::keys());
}

