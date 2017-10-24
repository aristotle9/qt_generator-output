#include "qt_core_c_QTextBoundaryFinder.h"

unsigned int qt_core_c_QTextBoundaryFinder_boundaryReasons(const QTextBoundaryFinder* this_ptr) {
  return uint(this_ptr->boundaryReasons());
}

void qt_core_c_QTextBoundaryFinder_constructor_no_args(QTextBoundaryFinder* output) {
  new(output) QTextBoundaryFinder();
}

void qt_core_c_QTextBoundaryFinder_constructor_other(const QTextBoundaryFinder* other, QTextBoundaryFinder* output) {
  new(output) QTextBoundaryFinder(*other);
}

void qt_core_c_QTextBoundaryFinder_constructor_type_chars_length(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, QTextBoundaryFinder* output) {
  new(output) QTextBoundaryFinder(type, chars, length);
}

void qt_core_c_QTextBoundaryFinder_constructor_type_chars_length_buffer(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, unsigned char* buffer, QTextBoundaryFinder* output) {
  new(output) QTextBoundaryFinder(type, chars, length, buffer);
}

void qt_core_c_QTextBoundaryFinder_constructor_type_chars_length_buffer_bufferSize(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, unsigned char* buffer, int bufferSize, QTextBoundaryFinder* output) {
  new(output) QTextBoundaryFinder(type, chars, length, buffer, bufferSize);
}

void qt_core_c_QTextBoundaryFinder_constructor_type_string(QTextBoundaryFinder::BoundaryType type, const QString* string, QTextBoundaryFinder* output) {
  new(output) QTextBoundaryFinder(type, *string);
}

void qt_core_c_QTextBoundaryFinder_destructor(QTextBoundaryFinder* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QTextBoundaryFinder_isAtBoundary(const QTextBoundaryFinder* this_ptr) {
  return this_ptr->isAtBoundary();
}

bool qt_core_c_QTextBoundaryFinder_isValid(const QTextBoundaryFinder* this_ptr) {
  return this_ptr->isValid();
}

QTextBoundaryFinder* qt_core_c_QTextBoundaryFinder_operator_assign(QTextBoundaryFinder* this_ptr, const QTextBoundaryFinder* other) {
  return &this_ptr->operator=(*other);
}

int qt_core_c_QTextBoundaryFinder_position(const QTextBoundaryFinder* this_ptr) {
  return this_ptr->position();
}

void qt_core_c_QTextBoundaryFinder_setPosition(QTextBoundaryFinder* this_ptr, int position) {
  this_ptr->setPosition(position);
}

void qt_core_c_QTextBoundaryFinder_string_to_output(const QTextBoundaryFinder* this_ptr, QString* output) {
  new(output) QString(this_ptr->string());
}

void qt_core_c_QTextBoundaryFinder_toEnd(QTextBoundaryFinder* this_ptr) {
  this_ptr->toEnd();
}

int qt_core_c_QTextBoundaryFinder_toNextBoundary(QTextBoundaryFinder* this_ptr) {
  return this_ptr->toNextBoundary();
}

int qt_core_c_QTextBoundaryFinder_toPreviousBoundary(QTextBoundaryFinder* this_ptr) {
  return this_ptr->toPreviousBoundary();
}

void qt_core_c_QTextBoundaryFinder_toStart(QTextBoundaryFinder* this_ptr) {
  this_ptr->toStart();
}

QTextBoundaryFinder::BoundaryType qt_core_c_QTextBoundaryFinder_type(const QTextBoundaryFinder* this_ptr) {
  return this_ptr->type();
}

