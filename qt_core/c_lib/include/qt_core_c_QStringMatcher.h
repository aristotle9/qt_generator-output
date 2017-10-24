#ifndef QT_CORE_C_QSTRINGMATCHER_H
#define QT_CORE_C_QSTRINGMATCHER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_constructor_no_args(QStringMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_constructor_other(const QStringMatcher* other, QStringMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_constructor_pattern(const QString* pattern, QStringMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_constructor_pattern_cs(const QString* pattern, const Qt::CaseSensitivity* cs, QStringMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_constructor_uc_len(const QChar* uc, int len, QStringMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_constructor_uc_len_cs(const QChar* uc, int len, const Qt::CaseSensitivity* cs, QStringMatcher* output);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_destructor(QStringMatcher* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QStringMatcher_indexIn_str(const QStringMatcher* this_ptr, const QString* str);
QT_CORE_C_EXPORT int qt_core_c_QStringMatcher_indexIn_str_from(const QStringMatcher* this_ptr, const QString* str, int from);
QT_CORE_C_EXPORT int qt_core_c_QStringMatcher_indexIn_str_length(const QStringMatcher* this_ptr, const QChar* str, int length);
QT_CORE_C_EXPORT int qt_core_c_QStringMatcher_indexIn_str_length_from(const QStringMatcher* this_ptr, const QChar* str, int length, int from);
QT_CORE_C_EXPORT QStringMatcher* qt_core_c_QStringMatcher_operator_assign(QStringMatcher* this_ptr, const QStringMatcher* other);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_pattern_to_output(const QStringMatcher* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_setCaseSensitivity(QStringMatcher* this_ptr, const Qt::CaseSensitivity* cs);
QT_CORE_C_EXPORT void qt_core_c_QStringMatcher_setPattern(QStringMatcher* this_ptr, const QString* pattern);

} // extern "C"

#endif // QT_CORE_C_QSTRINGMATCHER_H
