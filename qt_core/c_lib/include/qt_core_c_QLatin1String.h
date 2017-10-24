#ifndef QT_CORE_C_QLATIN1STRING_H
#define QT_CORE_C_QLATIN1STRING_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QLatin1String_at_to_output(const QLatin1String* this_ptr, int i, QLatin1Char* output);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_constructor_QByteArray(const QByteArray* s, QLatin1String* output);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_constructor_char(const char* s, QLatin1String* output);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_constructor_char_int(const char* s, int sz, QLatin1String* output);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_constructor_no_args(QLatin1String* output);
QT_CORE_C_EXPORT const char* qt_core_c_QLatin1String_data(const QLatin1String* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_destructor(QLatin1String* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QLatin1String_latin1(const QLatin1String* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_left_to_output(const QLatin1String* this_ptr, int n, QLatin1String* output);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_mid_to_output_pos(const QLatin1String* this_ptr, int pos, QLatin1String* output);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_mid_to_output_pos_n(const QLatin1String* this_ptr, int pos, int n, QLatin1String* output);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_eq_QByteArray(const QLatin1String* this_ptr, const QByteArray* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_eq_QString(const QLatin1String* this_ptr, const QString* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_eq_char(const QLatin1String* this_ptr, const char* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_ge_QByteArray(const QLatin1String* this_ptr, const QByteArray* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_ge_QString(const QLatin1String* this_ptr, const QString* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_ge_char(const QLatin1String* this_ptr, const char* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_gt_QByteArray(const QLatin1String* this_ptr, const QByteArray* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_gt_QString(const QLatin1String* this_ptr, const QString* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_gt_char(const QLatin1String* this_ptr, const char* s);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_operator_index_to_output(const QLatin1String* this_ptr, int i, QLatin1Char* output);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_le_QByteArray(const QLatin1String* this_ptr, const QByteArray* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_le_QString(const QLatin1String* this_ptr, const QString* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_le_char(const QLatin1String* this_ptr, const char* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_lt_QByteArray(const QLatin1String* this_ptr, const QByteArray* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_lt_QString(const QLatin1String* this_ptr, const QString* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_lt_char(const QLatin1String* this_ptr, const char* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_neq_QByteArray(const QLatin1String* this_ptr, const QByteArray* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_neq_QString(const QLatin1String* this_ptr, const QString* s);
QT_CORE_C_EXPORT bool qt_core_c_QLatin1String_operator_neq_char(const QLatin1String* this_ptr, const char* s);
QT_CORE_C_EXPORT void qt_core_c_QLatin1String_right_to_output(const QLatin1String* this_ptr, int n, QLatin1String* output);
QT_CORE_C_EXPORT int qt_core_c_QLatin1String_size(const QLatin1String* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QLATIN1STRING_H
