#ifndef QT_CORE_C_QVERSIONNUMBER_H
#define QT_CORE_C_QVERSIONNUMBER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_G_operator_eq(const QVersionNumber* lhs, const QVersionNumber* rhs);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_G_operator_ge(const QVersionNumber* lhs, const QVersionNumber* rhs);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_G_operator_gt(const QVersionNumber* lhs, const QVersionNumber* rhs);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_G_operator_le(const QVersionNumber* lhs, const QVersionNumber* rhs);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_G_operator_lt(const QVersionNumber* lhs, const QVersionNumber* rhs);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_G_operator_neq(const QVersionNumber* lhs, const QVersionNumber* rhs);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QVersionNumber_G_operator_shl(QDataStream* out, const QVersionNumber* version);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_G_operator_shl_to_output(const QDebug* arg1, const QVersionNumber* version, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QVersionNumber_G_operator_shr(QDataStream* in, QVersionNumber* version);
QT_CORE_C_EXPORT unsigned int qt_core_c_QVersionNumber_G_qHash_key(const QVersionNumber* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QVersionNumber_G_qHash_key_seed(const QVersionNumber* key, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_commonPrefix_to_output(const QVersionNumber* v1, const QVersionNumber* v2, QVersionNumber* output);
QT_CORE_C_EXPORT int qt_core_c_QVersionNumber_compare(const QVersionNumber* v1, const QVersionNumber* v2);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_constructor_maj(int maj, QVersionNumber* output);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_constructor_maj_min(int maj, int min, QVersionNumber* output);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_constructor_maj_min_mic(int maj, int min, int mic, QVersionNumber* output);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_constructor_no_args(QVersionNumber* output);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_constructor_seg(const QVector< int >* seg, QVersionNumber* output);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_destructor(QVersionNumber* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_fromString_to_output_string(const QString* string, QVersionNumber* output);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_fromString_to_output_string_suffixIndex(const QString* string, int* suffixIndex, QVersionNumber* output);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_isNormalized(const QVersionNumber* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_isNull(const QVersionNumber* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QVersionNumber_isPrefixOf(const QVersionNumber* this_ptr, const QVersionNumber* other);
QT_CORE_C_EXPORT int qt_core_c_QVersionNumber_majorVersion(const QVersionNumber* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QVersionNumber_microVersion(const QVersionNumber* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QVersionNumber_minorVersion(const QVersionNumber* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_normalized_to_output(const QVersionNumber* this_ptr, QVersionNumber* output);
QT_CORE_C_EXPORT int qt_core_c_QVersionNumber_segmentAt(const QVersionNumber* this_ptr, int index);
QT_CORE_C_EXPORT int qt_core_c_QVersionNumber_segmentCount(const QVersionNumber* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_segments_to_output(const QVersionNumber* this_ptr, QVector< int >* output);
QT_CORE_C_EXPORT void qt_core_c_QVersionNumber_toString_to_output(const QVersionNumber* this_ptr, QString* output);

} // extern "C"

#endif // QT_CORE_C_QVERSIONNUMBER_H
