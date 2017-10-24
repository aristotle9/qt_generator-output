#include "qt_core_c_QXmlStreamEntityResolver.h"

void qt_core_c_QXmlStreamEntityResolver_delete(QXmlStreamEntityResolver* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QXmlStreamEntityResolver_resolveEntity_to_output(QXmlStreamEntityResolver* this_ptr, const QString* publicId, const QString* systemId, QString* output) {
  new(output) QString(this_ptr->resolveEntity(*publicId, *systemId));
}

void qt_core_c_QXmlStreamEntityResolver_resolveUndeclaredEntity_to_output(QXmlStreamEntityResolver* this_ptr, const QString* name, QString* output) {
  new(output) QString(this_ptr->resolveUndeclaredEntity(*name));
}

