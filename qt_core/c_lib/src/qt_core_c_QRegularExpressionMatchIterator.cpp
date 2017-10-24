#include "qt_core_c_QRegularExpressionMatchIterator.h"

void qt_core_c_QRegularExpressionMatchIterator_constructor_iterator(const QRegularExpressionMatchIterator* iterator, QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator(*iterator);
}

void qt_core_c_QRegularExpressionMatchIterator_constructor_no_args(QRegularExpressionMatchIterator* output) {
  new(output) QRegularExpressionMatchIterator();
}

void qt_core_c_QRegularExpressionMatchIterator_destructor(QRegularExpressionMatchIterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QRegularExpressionMatchIterator_hasNext(const QRegularExpressionMatchIterator* this_ptr) {
  return this_ptr->hasNext();
}

bool qt_core_c_QRegularExpressionMatchIterator_isValid(const QRegularExpressionMatchIterator* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QRegularExpressionMatchIterator_next_to_output(QRegularExpressionMatchIterator* this_ptr, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->next());
}

QRegularExpressionMatchIterator* qt_core_c_QRegularExpressionMatchIterator_operator_assign(QRegularExpressionMatchIterator* this_ptr, const QRegularExpressionMatchIterator* iterator) {
  return &this_ptr->operator=(*iterator);
}

void qt_core_c_QRegularExpressionMatchIterator_peekNext_to_output(const QRegularExpressionMatchIterator* this_ptr, QRegularExpressionMatch* output) {
  new(output) QRegularExpressionMatch(this_ptr->peekNext());
}

void qt_core_c_QRegularExpressionMatchIterator_regularExpression_to_output(const QRegularExpressionMatchIterator* this_ptr, QRegularExpression* output) {
  new(output) QRegularExpression(this_ptr->regularExpression());
}

void qt_core_c_QRegularExpressionMatchIterator_swap(QRegularExpressionMatchIterator* this_ptr, QRegularExpressionMatchIterator* other) {
  this_ptr->swap(*other);
}

