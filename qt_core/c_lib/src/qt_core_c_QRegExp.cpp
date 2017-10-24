#include "qt_core_c_QRegExp.h"

QDataStream* qt_core_c_QRegExp_G_operator_shl(QDataStream* out, const QRegExp* regExp) {
  return &operator<<(*out, *regExp);
}

void qt_core_c_QRegExp_G_operator_shl_to_output(const QDebug* arg1, const QRegExp* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QRegExp_G_operator_shr(QDataStream* in, QRegExp* regExp) {
  return &operator>>(*in, *regExp);
}

unsigned int qt_core_c_QRegExp_G_qHash_key(const QRegExp* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QRegExp_G_qHash_key_seed(const QRegExp* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_core_c_QRegExp_cap_to_output_const_no_args(const QRegExp* this_ptr, QString* output) {
  new(output) QString(this_ptr->cap());
}

void qt_core_c_QRegExp_cap_to_output_const_nth(const QRegExp* this_ptr, int nth, QString* output) {
  new(output) QString(this_ptr->cap(nth));
}

void qt_core_c_QRegExp_cap_to_output_no_args(QRegExp* this_ptr, QString* output) {
  new(output) QString(this_ptr->cap());
}

void qt_core_c_QRegExp_cap_to_output_nth(QRegExp* this_ptr, int nth, QString* output) {
  new(output) QString(this_ptr->cap(nth));
}

int qt_core_c_QRegExp_captureCount(const QRegExp* this_ptr) {
  return this_ptr->captureCount();
}

void qt_core_c_QRegExp_capturedTexts_to_output(QRegExp* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->capturedTexts());
}

void qt_core_c_QRegExp_capturedTexts_to_output_const(const QRegExp* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->capturedTexts());
}

void qt_core_c_QRegExp_constructor_no_args(QRegExp* output) {
  new(output) QRegExp();
}

void qt_core_c_QRegExp_constructor_pattern(const QString* pattern, QRegExp* output) {
  new(output) QRegExp(*pattern);
}

void qt_core_c_QRegExp_constructor_pattern_cs(const QString* pattern, const Qt::CaseSensitivity* cs, QRegExp* output) {
  new(output) QRegExp(*pattern, *cs);
}

void qt_core_c_QRegExp_constructor_pattern_cs_syntax(const QString* pattern, const Qt::CaseSensitivity* cs, QRegExp::PatternSyntax syntax, QRegExp* output) {
  new(output) QRegExp(*pattern, *cs, syntax);
}

void qt_core_c_QRegExp_constructor_rx(const QRegExp* rx, QRegExp* output) {
  new(output) QRegExp(*rx);
}

void qt_core_c_QRegExp_destructor(QRegExp* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QRegExp_errorString_to_output(QRegExp* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QRegExp_errorString_to_output_const(const QRegExp* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QRegExp_escape_to_output(const QString* str, QString* output) {
  new(output) QString(QRegExp::escape(*str));
}

bool qt_core_c_QRegExp_exactMatch(const QRegExp* this_ptr, const QString* str) {
  return this_ptr->exactMatch(*str);
}

int qt_core_c_QRegExp_indexIn_str(const QRegExp* this_ptr, const QString* str) {
  return this_ptr->indexIn(*str);
}

int qt_core_c_QRegExp_indexIn_str_offset(const QRegExp* this_ptr, const QString* str, int offset) {
  return this_ptr->indexIn(*str, offset);
}

int qt_core_c_QRegExp_indexIn_str_offset_caretMode(const QRegExp* this_ptr, const QString* str, int offset, QRegExp::CaretMode caretMode) {
  return this_ptr->indexIn(*str, offset, caretMode);
}

bool qt_core_c_QRegExp_isEmpty(const QRegExp* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QRegExp_isMinimal(const QRegExp* this_ptr) {
  return this_ptr->isMinimal();
}

bool qt_core_c_QRegExp_isValid(const QRegExp* this_ptr) {
  return this_ptr->isValid();
}

int qt_core_c_QRegExp_lastIndexIn_str(const QRegExp* this_ptr, const QString* str) {
  return this_ptr->lastIndexIn(*str);
}

int qt_core_c_QRegExp_lastIndexIn_str_offset(const QRegExp* this_ptr, const QString* str, int offset) {
  return this_ptr->lastIndexIn(*str, offset);
}

int qt_core_c_QRegExp_lastIndexIn_str_offset_caretMode(const QRegExp* this_ptr, const QString* str, int offset, QRegExp::CaretMode caretMode) {
  return this_ptr->lastIndexIn(*str, offset, caretMode);
}

int qt_core_c_QRegExp_matchedLength(const QRegExp* this_ptr) {
  return this_ptr->matchedLength();
}

QRegExp* qt_core_c_QRegExp_operator_assign(QRegExp* this_ptr, const QRegExp* rx) {
  return &this_ptr->operator=(*rx);
}

bool qt_core_c_QRegExp_operator_eq(const QRegExp* this_ptr, const QRegExp* rx) {
  return this_ptr->operator==(*rx);
}

bool qt_core_c_QRegExp_operator_neq(const QRegExp* this_ptr, const QRegExp* rx) {
  return this_ptr->operator!=(*rx);
}

QRegExp::PatternSyntax qt_core_c_QRegExp_patternSyntax(const QRegExp* this_ptr) {
  return this_ptr->patternSyntax();
}

void qt_core_c_QRegExp_pattern_to_output(const QRegExp* this_ptr, QString* output) {
  new(output) QString(this_ptr->pattern());
}

int qt_core_c_QRegExp_pos_const_no_args(const QRegExp* this_ptr) {
  return this_ptr->pos();
}

int qt_core_c_QRegExp_pos_const_nth(const QRegExp* this_ptr, int nth) {
  return this_ptr->pos(nth);
}

int qt_core_c_QRegExp_pos_no_args(QRegExp* this_ptr) {
  return this_ptr->pos();
}

int qt_core_c_QRegExp_pos_nth(QRegExp* this_ptr, int nth) {
  return this_ptr->pos(nth);
}

void qt_core_c_QRegExp_setCaseSensitivity(QRegExp* this_ptr, const Qt::CaseSensitivity* cs) {
  this_ptr->setCaseSensitivity(*cs);
}

void qt_core_c_QRegExp_setMinimal(QRegExp* this_ptr, bool minimal) {
  this_ptr->setMinimal(minimal);
}

void qt_core_c_QRegExp_setPattern(QRegExp* this_ptr, const QString* pattern) {
  this_ptr->setPattern(*pattern);
}

void qt_core_c_QRegExp_setPatternSyntax(QRegExp* this_ptr, QRegExp::PatternSyntax syntax) {
  this_ptr->setPatternSyntax(syntax);
}

void qt_core_c_QRegExp_swap(QRegExp* this_ptr, QRegExp* other) {
  this_ptr->swap(*other);
}

