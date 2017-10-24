#ifndef QT_CORE_C_QXMLSTREAMATTRIBUTE_H
#define QT_CORE_C_QXMLSTREAMATTRIBUTE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_G_swap(QXmlStreamStringRef* value1, QXmlStreamStringRef* value2);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_constructor_arg1(const QXmlStreamAttribute* arg1, QXmlStreamAttribute* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_constructor_namespaceUri_name_value(const QString* namespaceUri, const QString* name, const QString* value, QXmlStreamAttribute* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_constructor_no_args(QXmlStreamAttribute* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_constructor_qualifiedName_value(const QString* qualifiedName, const QString* value, QXmlStreamAttribute* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_destructor(QXmlStreamAttribute* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamAttribute_isDefault(const QXmlStreamAttribute* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_name_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_namespaceUri_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT QXmlStreamAttribute* qt_core_c_QXmlStreamAttribute_operator_assign(QXmlStreamAttribute* this_ptr, const QXmlStreamAttribute* arg1);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamAttribute_operator_eq(const QXmlStreamAttribute* this_ptr, const QXmlStreamAttribute* other);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamAttribute_operator_neq(const QXmlStreamAttribute* this_ptr, const QXmlStreamAttribute* other);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_prefix_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_qualifiedName_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamAttribute_value_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output);

} // extern "C"

#endif // QT_CORE_C_QXMLSTREAMATTRIBUTE_H
