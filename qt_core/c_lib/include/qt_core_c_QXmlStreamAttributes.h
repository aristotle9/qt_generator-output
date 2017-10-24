#ifndef QT_CORE_C_QXMLSTREAMATTRIBUTES_H
#define QT_CORE_C_QXMLSTREAMATTRIBUTES_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QVector< QXmlStreamAttribute >* qt_core_c_QXmlStreamAttributes_G_static_cast_QVector_QXmlStreamAttribute_ptr(QXmlStreamAttributes* ptr);
QT_CORE_C_EXPORT QXmlStreamAttributes* qt_core_c_QXmlStreamAttributes_G_static_cast_QXmlStreamAttributes_ptr(QVector< QXmlStreamAttribute >* ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_append_namespaceUri_name_value(QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QString* name, const QString* value);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_append_qualifiedName_value(QXmlStreamAttributes* this_ptr, const QString* qualifiedName, const QString* value);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_constructor(QXmlStreamAttributes* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_destructor(QXmlStreamAttributes* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamAttributes_hasAttribute_QLatin1String(const QXmlStreamAttributes* this_ptr, const QLatin1String* qualifiedName);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamAttributes_hasAttribute_QString(const QXmlStreamAttributes* this_ptr, const QString* qualifiedName);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamAttributes_hasAttribute_QString_QString(const QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_value_to_output_QLatin1String(const QXmlStreamAttributes* this_ptr, const QLatin1String* qualifiedName, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_value_to_output_QLatin1String_QLatin1String(const QXmlStreamAttributes* this_ptr, const QLatin1String* namespaceUri, const QLatin1String* name, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_value_to_output_QString(const QXmlStreamAttributes* this_ptr, const QString* qualifiedName, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_value_to_output_QString_QLatin1String(const QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QLatin1String* name, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttributes_value_to_output_QString_QString(const QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QString* name, QStringRef* output);

} // extern "C"

#endif // QT_CORE_C_QXMLSTREAMATTRIBUTES_H
