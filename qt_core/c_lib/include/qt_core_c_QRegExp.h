#ifndef QT_CORE_C_QREGEXP_H
#define QT_CORE_C_QREGEXP_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDataStream* qt_core_c_QRegExp_G_operator_shl(QDataStream* out, const QRegExp* regExp);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_G_operator_shl_to_output(const QDebug* arg1, const QRegExp* arg2, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QRegExp_G_operator_shr(QDataStream* in, QRegExp* regExp);
QT_CORE_C_EXPORT unsigned int qt_core_c_QRegExp_G_qHash_key(const QRegExp* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QRegExp_G_qHash_key_seed(const QRegExp* key, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_cap_to_output_const_no_args(const QRegExp* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_cap_to_output_const_nth(const QRegExp* this_ptr, int nth, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_cap_to_output_no_args(QRegExp* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_cap_to_output_nth(QRegExp* this_ptr, int nth, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_captureCount(const QRegExp* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_capturedTexts_to_output(QRegExp* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_capturedTexts_to_output_const(const QRegExp* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_constructor_no_args(QRegExp* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_constructor_pattern(const QString* pattern, QRegExp* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_constructor_pattern_cs(const QString* pattern, const Qt::CaseSensitivity* cs, QRegExp* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_constructor_pattern_cs_syntax(const QString* pattern, const Qt::CaseSensitivity* cs, QRegExp::PatternSyntax syntax, QRegExp* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_constructor_rx(const QRegExp* rx, QRegExp* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_destructor(QRegExp* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_errorString_to_output(QRegExp* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_errorString_to_output_const(const QRegExp* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_escape_to_output(const QString* str, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QRegExp_exactMatch(const QRegExp* this_ptr, const QString* str);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_indexIn_str(const QRegExp* this_ptr, const QString* str);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_indexIn_str_offset(const QRegExp* this_ptr, const QString* str, int offset);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_indexIn_str_offset_caretMode(const QRegExp* this_ptr, const QString* str, int offset, QRegExp::CaretMode caretMode);
QT_CORE_C_EXPORT bool qt_core_c_QRegExp_isEmpty(const QRegExp* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QRegExp_isMinimal(const QRegExp* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QRegExp_isValid(const QRegExp* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_lastIndexIn_str(const QRegExp* this_ptr, const QString* str);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_lastIndexIn_str_offset(const QRegExp* this_ptr, const QString* str, int offset);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_lastIndexIn_str_offset_caretMode(const QRegExp* this_ptr, const QString* str, int offset, QRegExp::CaretMode caretMode);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_matchedLength(const QRegExp* this_ptr);
QT_CORE_C_EXPORT QRegExp* qt_core_c_QRegExp_operator_assign(QRegExp* this_ptr, const QRegExp* rx);
QT_CORE_C_EXPORT bool qt_core_c_QRegExp_operator_eq(const QRegExp* this_ptr, const QRegExp* rx);
QT_CORE_C_EXPORT bool qt_core_c_QRegExp_operator_neq(const QRegExp* this_ptr, const QRegExp* rx);
QT_CORE_C_EXPORT QRegExp::PatternSyntax qt_core_c_QRegExp_patternSyntax(const QRegExp* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_pattern_to_output(const QRegExp* this_ptr, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_pos_const_no_args(const QRegExp* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_pos_const_nth(const QRegExp* this_ptr, int nth);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_pos_no_args(QRegExp* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QRegExp_pos_nth(QRegExp* this_ptr, int nth);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_setCaseSensitivity(QRegExp* this_ptr, const Qt::CaseSensitivity* cs);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_setMinimal(QRegExp* this_ptr, bool minimal);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_setPattern(QRegExp* this_ptr, const QString* pattern);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_setPatternSyntax(QRegExp* this_ptr, QRegExp::PatternSyntax syntax);
QT_CORE_C_EXPORT void qt_core_c_QRegExp_swap(QRegExp* this_ptr, QRegExp* other);

} // extern "C"

#endif // QT_CORE_C_QREGEXP_H
