#include "qt_core_c_QRegularExpressionMatch.h"

int qt_core_c_QRegularExpressionMatch_capturedEnd_name(const QRegularExpressionMatch* this_ptr, const QString* name) {
  return this_ptr->capturedEnd(*name);
}

int qt_core_c_QRegularExpressionMatch_capturedEnd_no_args(const QRegularExpressionMatch* this_ptr) {
  return this_ptr->capturedEnd();
}

int qt_core_c_QRegularExpressionMatch_capturedEnd_nth(const QRegularExpressionMatch* this_ptr, int nth) {
  return this_ptr->capturedEnd(nth);
}

int qt_core_c_QRegularExpressionMatch_capturedLength_name(const QRegularExpressionMatch* this_ptr, const QString* name) {
  return this_ptr->capturedLength(*name);
}

int qt_core_c_QRegularExpressionMatch_capturedLength_no_args(const QRegularExpressionMatch* this_ptr) {
  return this_ptr->capturedLength();
}

int qt_core_c_QRegularExpressionMatch_capturedLength_nth(const QRegularExpressionMatch* this_ptr, int nth) {
  return this_ptr->capturedLength(nth);
}

void qt_core_c_QRegularExpressionMatch_capturedRef_to_output_name(const QRegularExpressionMatch* this_ptr, const QString* name, QStringRef* output) {
  new(output) QStringRef(this_ptr->capturedRef(*name));
}

void qt_core_c_QRegularExpressionMatch_capturedRef_to_output_no_args(const QRegularExpressionMatch* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->capturedRef());
}

void qt_core_c_QRegularExpressionMatch_capturedRef_to_output_nth(const QRegularExpressionMatch* this_ptr, int nth, QStringRef* output) {
  new(output) QStringRef(this_ptr->capturedRef(nth));
}

int qt_core_c_QRegularExpressionMatch_capturedStart_name(const QRegularExpressionMatch* this_ptr, const QString* name) {
  return this_ptr->capturedStart(*name);
}

int qt_core_c_QRegularExpressionMatch_capturedStart_no_args(const QRegularExpressionMatch* this_ptr) {
  return this_ptr->capturedStart();
}

int qt_core_c_QRegularExpressionMatch_capturedStart_nth(const QRegularExpressionMatch* this_ptr, int nth) {
  return this_ptr->capturedStart(nth);
}

void qt_core_c_QRegularExpressionMatch_capturedTexts_to_output(const QRegularExpressionMatch* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->capturedTexts());
}

void qt_core_c_QRegularExpressionMatch_captured_to_output_name(const QRegularExpressionMatch* this_ptr, const QString* name, QString* output) {
  new(output) QString(this_ptr->captured(*name));
}

void qt_core_c_QRegularExpressionMatch_captured_to_output_no_args(const QRegularExpressionMatch* this_ptr, QString* output) {
  new(output) QString(this_ptr->captured());
}

void qt_core_c_QRegularExpressionMatch_captured_to_output_nth(const QRegularExpressionMatch* this_ptr, int nth, QString* output) {
  new(output) QString(this_ptr->captured(nth));
}

void qt_core_c_QRegularExpressionMatch_constructor_match(const QRegularExpressionMatch* match, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(*match);
}

void qt_core_c_QRegularExpressionMatch_constructor_no_args(QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch();
}

void qt_core_c_QRegularExpressionMatch_destructor(QRegularExpressionMatch* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QRegularExpressionMatch_hasMatch(const QRegularExpressionMatch* this_ptr) {
  return this_ptr->hasMatch();
}

bool qt_core_c_QRegularExpressionMatch_hasPartialMatch(const QRegularExpressionMatch* this_ptr) {
  return this_ptr->hasPartialMatch();
}

bool qt_core_c_QRegularExpressionMatch_isValid(const QRegularExpressionMatch* this_ptr) {
  return this_ptr->isValid();
}

int qt_core_c_QRegularExpressionMatch_lastCapturedIndex(const QRegularExpressionMatch* this_ptr) {
  return this_ptr->lastCapturedIndex();
}

QRegularExpressionMatch* qt_core_c_QRegularExpressionMatch_operator_assign(QRegularExpressionMatch* this_ptr, const QRegularExpressionMatch* match) {
  return &this_ptr->operator=(*match);
}

void qt_core_c_QRegularExpressionMatch_regularExpression_to_output(const QRegularExpressionMatch* this_ptr, QRegularExpression* output) {
  new(output) QRegularExpression(this_ptr->regularExpression());
}

void qt_core_c_QRegularExpressionMatch_swap(QRegularExpressionMatch* this_ptr, QRegularExpressionMatch* other) {
  this_ptr->swap(*other);
}

