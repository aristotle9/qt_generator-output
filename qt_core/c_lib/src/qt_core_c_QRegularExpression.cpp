#include "qt_core_c_QRegularExpression.h"

QDataStream* qt_core_c_QRegularExpression_G_operator_shl(QDataStream* out, const QRegularExpression* re) {
  return &operator<<(*out, *re);
}

void qt_core_c_QRegularExpression_G_operator_shl_to_output_debug_match(const QDebug* debug, const QRegularExpressionMatch* match, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *match));
}

void qt_core_c_QRegularExpression_G_operator_shl_to_output_debug_re(const QDebug* debug, const QRegularExpression* re, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *re));
}

QDataStream* qt_core_c_QRegularExpression_G_operator_shr(QDataStream* in, QRegularExpression* re) {
  return &operator>>(*in, *re);
}

unsigned int qt_core_c_QRegularExpression_G_qHash_key(const QRegularExpression* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QRegularExpression_G_qHash_key_seed(const QRegularExpression* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_core_c_QRegularExpression_G_swap_QRegularExpressionMatchIterator_QRegularExpressionMatchIterator(QRegularExpressionMatchIterator* value1, QRegularExpressionMatchIterator* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QRegularExpression_G_swap_QRegularExpressionMatch_QRegularExpressionMatch(QRegularExpressionMatch* value1, QRegularExpressionMatch* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QRegularExpression_G_swap_QRegularExpression_QRegularExpression(QRegularExpression* value1, QRegularExpression* value2) {
  swap(*value1, *value2);
}

int qt_core_c_QRegularExpression_captureCount(const QRegularExpression* this_ptr) {
  return this_ptr->captureCount();
}

void qt_core_c_QRegularExpression_constructor_no_args(QRegularExpression* output) {
  new(output) QRegularExpression();
}

void qt_core_c_QRegularExpression_constructor_pattern(const QString* pattern, QRegularExpression* output) {
  new(output) QRegularExpression(*pattern);
}

void qt_core_c_QRegularExpression_constructor_pattern_options(const QString* pattern, unsigned int options, QRegularExpression* output) {
  new(output) QRegularExpression(*pattern, QFlags< QRegularExpression::PatternOption >(options));
}

void qt_core_c_QRegularExpression_constructor_re(const QRegularExpression* re, QRegularExpression* output) {
  new(output) QRegularExpression(*re);
}

void qt_core_c_QRegularExpression_destructor(QRegularExpression* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QRegularExpression_errorString_to_output(const QRegularExpression* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QRegularExpression_escape_to_output(const QString* str, QString* output) {
  new(output) QString(QRegularExpression::escape(*str));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subject(const QRegularExpression* this_ptr, const QString* subject, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subject));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef(const QRegularExpression* this_ptr, const QStringRef* subjectRef, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subjectRef));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subjectRef, offset));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset_matchType(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subjectRef, offset, matchType));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subjectRef, offset, matchType, QFlags< QRegularExpression::MatchOption >(matchOptions)));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subject, offset));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset_matchType(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subject, offset, matchType));
}

void qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(this_ptr->globalMatch(*subject, offset, matchType, QFlags< QRegularExpression::MatchOption >(matchOptions)));
}

bool qt_core_c_QRegularExpression_isValid(const QRegularExpression* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QRegularExpression_match_to_output_subject(const QRegularExpression* this_ptr, const QString* subject, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subject));
}

void qt_core_c_QRegularExpression_match_to_output_subjectRef(const QRegularExpression* this_ptr, const QStringRef* subjectRef, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subjectRef));
}

void qt_core_c_QRegularExpression_match_to_output_subjectRef_offset(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subjectRef, offset));
}

void qt_core_c_QRegularExpression_match_to_output_subjectRef_offset_matchType(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subjectRef, offset, matchType));
}

void qt_core_c_QRegularExpression_match_to_output_subjectRef_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QStringRef* subjectRef, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subjectRef, offset, matchType, QFlags< QRegularExpression::MatchOption >(matchOptions)));
}

void qt_core_c_QRegularExpression_match_to_output_subject_offset(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subject, offset));
}

void qt_core_c_QRegularExpression_match_to_output_subject_offset_matchType(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subject, offset, matchType));
}

void qt_core_c_QRegularExpression_match_to_output_subject_offset_matchType_matchOptions(const QRegularExpression* this_ptr, const QString* subject, int offset, QRegularExpression::MatchType matchType, unsigned int matchOptions, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->match(*subject, offset, matchType, QFlags< QRegularExpression::MatchOption >(matchOptions)));
}

void qt_core_c_QRegularExpression_namedCaptureGroups_to_output(const QRegularExpression* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->namedCaptureGroups());
}

QRegularExpression* qt_core_c_QRegularExpression_operator_assign(QRegularExpression* this_ptr, const QRegularExpression* re) {
  return &this_ptr->operator=(*re);
}

bool qt_core_c_QRegularExpression_operator_eq(const QRegularExpression* this_ptr, const QRegularExpression* re) {
  return this_ptr->operator==(*re);
}

bool qt_core_c_QRegularExpression_operator_neq(const QRegularExpression* this_ptr, const QRegularExpression* re) {
  return this_ptr->operator!=(*re);
}

void qt_core_c_QRegularExpression_optimize(const QRegularExpression* this_ptr) {
  this_ptr->optimize();
}

int qt_core_c_QRegularExpression_patternErrorOffset(const QRegularExpression* this_ptr) {
  return this_ptr->patternErrorOffset();
}

unsigned int qt_core_c_QRegularExpression_patternOptions(const QRegularExpression* this_ptr) {
  return uint(this_ptr->patternOptions());
}

void qt_core_c_QRegularExpression_pattern_to_output(const QRegularExpression* this_ptr, QString* output) {
  new(output) QString(this_ptr->pattern());
}

void qt_core_c_QRegularExpression_setPattern(QRegularExpression* this_ptr, const QString* pattern) {
  this_ptr->setPattern(*pattern);
}

void qt_core_c_QRegularExpression_setPatternOptions(QRegularExpression* this_ptr, unsigned int options) {
  this_ptr->setPatternOptions(QFlags< QRegularExpression::PatternOption >(options));
}

void qt_core_c_QRegularExpression_swap(QRegularExpression* this_ptr, QRegularExpression* other) {
  this_ptr->swap(*other);
}

