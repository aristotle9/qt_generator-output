#ifndef QT_CORE_C_QBYTEARRAYMATCHER_H
#define QT_CORE_C_QBYTEARRAYMATCHER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QByteArrayMatcher_constructor_no_args(QByteArrayMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QByteArrayMatcher_constructor_other(const QByteArrayMatcher* other, QByteArrayMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QByteArrayMatcher_constructor_pattern(const QByteArray* pattern, QByteArrayMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QByteArrayMatcher_constructor_pattern_length(const char* pattern, int length, QByteArrayMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QByteArrayMatcher_destructor(QByteArrayMatcher* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QByteArrayMatcher_indexIn_ba(const QByteArrayMatcher* this_ptr, const QByteArray* ba);
QT_CORE_C_EXPORT int qt_core_c_QByteArrayMatcher_indexIn_ba_from(const QByteArrayMatcher* this_ptr, const QByteArray* ba, int from);
QT_CORE_C_EXPORT int qt_core_c_QByteArrayMatcher_indexIn_str_len(const QByteArrayMatcher* this_ptr, const char* str, int len);
QT_CORE_C_EXPORT int qt_core_c_QByteArrayMatcher_indexIn_str_len_from(const QByteArrayMatcher* this_ptr, const char* str, int len, int from);
QT_CORE_C_EXPORT QByteArrayMatcher* qt_core_c_QByteArrayMatcher_operator_assign(QByteArrayMatcher* this_ptr, const QByteArrayMatcher* other);
QT_CORE_C_EXPORT void qt_core_c_QByteArrayMatcher_pattern_to_output(const QByteArrayMatcher* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QByteArrayMatcher_setPattern(QByteArrayMatcher* this_ptr, const QByteArray* pattern);

} // extern "C"

#endif // QT_CORE_C_QBYTEARRAYMATCHER_H
