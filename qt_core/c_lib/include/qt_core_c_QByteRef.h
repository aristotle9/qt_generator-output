#ifndef QT_CORE_C_QBYTEREF_H
#define QT_CORE_C_QBYTEREF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT char qt_core_c_QByteRef_convert_to_char(const QByteRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QByteRef_destructor(QByteRef* this_ptr);
QT_CORE_C_EXPORT QByteRef* qt_core_c_QByteRef_operator_assign_QByteRef(QByteRef* this_ptr, const QByteRef* c);
QT_CORE_C_EXPORT QByteRef* qt_core_c_QByteRef_operator_assign_char(QByteRef* this_ptr, char c);
QT_CORE_C_EXPORT bool qt_core_c_QByteRef_operator_eq(const QByteRef* this_ptr, char c);
QT_CORE_C_EXPORT bool qt_core_c_QByteRef_operator_ge(const QByteRef* this_ptr, char c);
QT_CORE_C_EXPORT bool qt_core_c_QByteRef_operator_gt(const QByteRef* this_ptr, char c);
QT_CORE_C_EXPORT bool qt_core_c_QByteRef_operator_le(const QByteRef* this_ptr, char c);
QT_CORE_C_EXPORT bool qt_core_c_QByteRef_operator_lt(const QByteRef* this_ptr, char c);
QT_CORE_C_EXPORT bool qt_core_c_QByteRef_operator_neq(const QByteRef* this_ptr, char c);

} // extern "C"

#endif // QT_CORE_C_QBYTEREF_H
