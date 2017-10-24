#ifndef QT_CORE_C_QXMLSTREAMENTITYRESOLVER_H
#define QT_CORE_C_QXMLSTREAMENTITYRESOLVER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QXmlStreamEntityResolver_delete(QXmlStreamEntityResolver* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamEntityResolver_resolveEntity_to_output(QXmlStreamEntityResolver* this_ptr, const QString* publicId, const QString* systemId, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamEntityResolver_resolveUndeclaredEntity_to_output(QXmlStreamEntityResolver* this_ptr, const QString* name, QString* output);

} // extern "C"

#endif // QT_CORE_C_QXMLSTREAMENTITYRESOLVER_H
