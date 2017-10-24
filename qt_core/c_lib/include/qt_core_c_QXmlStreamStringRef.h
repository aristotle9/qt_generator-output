#ifndef QT_CORE_C_QXMLSTREAMSTRINGREF_H
#define QT_CORE_C_QXMLSTREAMSTRINGREF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_clear(QXmlStreamStringRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_constructor_QString(const QString* aString, QXmlStreamStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_constructor_QStringRef(const QStringRef* aString, QXmlStreamStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_constructor_QXmlStreamStringRef(const QXmlStreamStringRef* other, QXmlStreamStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_constructor_no_args(QXmlStreamStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_convert_to_QStringRef_to_output(const QXmlStreamStringRef* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_destructor(QXmlStreamStringRef* this_ptr);
QT_CORE_C_EXPORT QXmlStreamStringRef* qt_core_c_QXmlStreamStringRef_operator_assign(QXmlStreamStringRef* this_ptr, const QXmlStreamStringRef* other);
QT_CORE_C_EXPORT int qt_core_c_QXmlStreamStringRef_position(const QXmlStreamStringRef* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QXmlStreamStringRef_size(const QXmlStreamStringRef* this_ptr);
QT_CORE_C_EXPORT const QString* qt_core_c_QXmlStreamStringRef_string(const QXmlStreamStringRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamStringRef_swap(QXmlStreamStringRef* this_ptr, QXmlStreamStringRef* other);

} // extern "C"

#endif // QT_CORE_C_QXMLSTREAMSTRINGREF_H
