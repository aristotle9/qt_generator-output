#ifndef QT_CORE_C_QREGULAREXPRESSION_H
#define QT_CORE_C_QREGULAREXPRESSION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDataStream* qt_core_c_QRegularExpression_G_operator_shl(QDataStream* out, const QRegularExpression* re);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_G_operator_shl_to_output_debug_match(const QDebug* debug, const QRegularExpressionMatch* match, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_G_operator_shl_to_output_debug_re(const QDebug* debug, const QRegularExpression* re, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QRegularExpression_G_operator_shr(QDataStream* in, QRegularExpression* re);
QT_CORE_C_EXPORT unsigned int qt_core_c_QRegularExpression_G_qHash_key(const QRegularExpression* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QRegularExpression_G_qHash_key_seed(const QRegularExpression* key, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_G_swap_QRegularExpressionMatchIterator_QRegularExpressionMatchIterator(QRegularExpressionMatchIterator* value1, QRegularExpressionMatchIterator* value2);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_G_swap_QRegularExpressionMatch_QRegularExpressionMatch(QRegularExpressionMatch* value1, QRegularExpressionMatch* value2);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_G_swap_QRegularExpression_QRegularExpression(QRegularExpression* value1, QRegularExpression* value2);
QT_CORE_C_EXPORT int qt_core_c_QRegularExpression_captureCount(const QRegularExpression* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_constructor_no_args(QRegularExpression* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_constructor_pattern(const QString* pattern, QRegularExpression* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_constructor_pattern_options(const QString* pattern, unsigned int options, QRegularExpression* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_constructor_re(const QRegularExpression* re, QRegularExpression* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_destructor(QRegularExpression* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_errorString_to_output(const QRegularExpression* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_escape_to_output(const QString* str, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subject(const QRegularExpression* this_ptr, const QString* subject, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef(const QRegularExpression* this_ptr, const QStringRef* subjectRef, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset_matchType(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset_matchType(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatchIterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QRegularExpression_isValid(const QRegularExpression* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subject(const QRegularExpression* this_ptr, const QString* subject, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subjectRef(const QRegularExpression* this_ptr, const QStringRef* subjectRef, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subjectRef_offset(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subjectRef_offset_matchType(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subjectRef_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subject_offset(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subject_offset_matchType(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_match_to_output_subject_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatch* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_namedCaptureGroups_to_output(const QRegularExpression* this_ptr, QStringList* output);
QT_CORE_C_EXPORT QRegularExpression* qt_core_c_QRegularExpression_operator_assign(QRegularExpression* this_ptr, const QRegularExpression* re);
QT_CORE_C_EXPORT bool qt_core_c_QRegularExpression_operator_eq(const QRegularExpression* this_ptr, const QRegularExpression* re);
QT_CORE_C_EXPORT bool qt_core_c_QRegularExpression_operator_neq(const QRegularExpression* this_ptr, const QRegularExpression* re);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_optimize(const QRegularExpression* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QRegularExpression_patternErrorOffset(const QRegularExpression* this_ptr);
QT_CORE_C_EXPORT unsigned int qt_core_c_QRegularExpression_patternOptions(const QRegularExpression* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_pattern_to_output(const QRegularExpression* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_setPattern(QRegularExpression* this_ptr, const QString* pattern);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_setPatternOptions(QRegularExpression* this_ptr, unsigned int options);
QT_CORE_C_EXPORT void qt_core_c_QRegularExpression_swap(QRegularExpression* this_ptr, QRegularExpression* other);

} // extern "C"

#endif // QT_CORE_C_QREGULAREXPRESSION_H
