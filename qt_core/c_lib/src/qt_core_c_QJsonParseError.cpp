#include "qt_core_c_QJsonParseError.h"

void qt_core_c_QJsonParseError_destructor(QJsonParseError* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QJsonParseError::ParseError qt_core_c_QJsonParseError_error(const QJsonParseError* this_ptr) {
  return this_ptr->error;
}

void qt_core_c_QJsonParseError_errorString_to_output(const QJsonParseError* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

int qt_core_c_QJsonParseError_offset(const QJsonParseError* this_ptr) {
  return this_ptr->offset;
}

void qt_core_c_QJsonParseError_set_error(QJsonParseError* this_ptr, QJsonParseError::ParseError value) {
  this_ptr->error = value;
}

void qt_core_c_QJsonParseError_set_offset(QJsonParseError* this_ptr, int value) {
  this_ptr->offset = value;
}

