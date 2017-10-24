#ifndef QT_CORE_C_QUUID_H
#define QT_CORE_C_QUUID_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QUuid_G_operator_ge(const QUuid* lhs, const QUuid* rhs);
QT_CORE_C_EXPORT bool qt_core_c_QUuid_G_operator_le(const QUuid* lhs, const QUuid* rhs);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QUuid_G_operator_shl(QDataStream* arg1, const QUuid* arg2);
QT_CORE_C_EXPORT void qt_core_c_QUuid_G_operator_shl_to_output(const QDebug* arg1, const QUuid* arg2, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QUuid_G_operator_shr(QDataStream* arg1, QUuid* arg2);
QT_CORE_C_EXPORT unsigned int qt_core_c_QUuid_G_qHash_uuid(const QUuid* uuid);
QT_CORE_C_EXPORT unsigned int qt_core_c_QUuid_G_qHash_uuid_seed(const QUuid* uuid, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QUuid_constructor_QByteArray(const QByteArray* arg1, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_constructor_QString(const QString* arg1, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_constructor_char(const char* arg1, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_constructor_no_args(QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_constructor_unsigned_int_unsigned_short_unsigned_short_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char(unsigned int l, unsigned short w1, unsigned short w2, unsigned char b1, unsigned char b2, unsigned char b3, unsigned char b4, unsigned char b5, unsigned char b6, unsigned char b7, unsigned char b8, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_createUuidV3_to_output_QUuid_QByteArray(const QUuid* ns, const QByteArray* baseData, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_createUuidV3_to_output_QUuid_QString(const QUuid* ns, const QString* baseData, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_createUuidV5_to_output_QUuid_QByteArray(const QUuid* ns, const QByteArray* baseData, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_createUuidV5_to_output_QUuid_QString(const QUuid* ns, const QString* baseData, QUuid* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_createUuid_to_output(QUuid* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QUuid_data1(const QUuid* this_ptr);
QT_CORE_C_EXPORT unsigned short qt_core_c_QUuid_data2(const QUuid* this_ptr);
QT_CORE_C_EXPORT unsigned short qt_core_c_QUuid_data3(const QUuid* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QUuid_destructor(QUuid* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QUuid_fromRfc4122_to_output(const QByteArray* arg1, QUuid* output);
QT_CORE_C_EXPORT bool qt_core_c_QUuid_isNull(const QUuid* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QUuid_operator_eq(const QUuid* this_ptr, const QUuid* orig);
QT_CORE_C_EXPORT bool qt_core_c_QUuid_operator_gt(const QUuid* this_ptr, const QUuid* other);
QT_CORE_C_EXPORT bool qt_core_c_QUuid_operator_lt(const QUuid* this_ptr, const QUuid* other);
QT_CORE_C_EXPORT bool qt_core_c_QUuid_operator_neq(const QUuid* this_ptr, const QUuid* orig);
QT_CORE_C_EXPORT void qt_core_c_QUuid_set_data1(QUuid* this_ptr, unsigned int value);
QT_CORE_C_EXPORT void qt_core_c_QUuid_set_data2(QUuid* this_ptr, unsigned short value);
QT_CORE_C_EXPORT void qt_core_c_QUuid_set_data3(QUuid* this_ptr, unsigned short value);
QT_CORE_C_EXPORT void qt_core_c_QUuid_toByteArray_to_output(const QUuid* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_toRfc4122_to_output(const QUuid* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUuid_toString_to_output(const QUuid* this_ptr, QString* output);

} // extern "C"

#endif // QT_CORE_C_QUUID_H
