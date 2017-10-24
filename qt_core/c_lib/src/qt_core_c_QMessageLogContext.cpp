#include "qt_core_c_QMessageLogContext.h"

void qt_core_c_QMessageLogContext_G_qFormatLogMessage_to_output(QtMsgType type, const QMessageLogContext* context, const QString* buf, QString* output) {
  new(output) QString(qFormatLogMessage(type, *context, *buf));
}

void qt_core_c_QMessageLogContext_G_qSetMessagePattern(const QString* messagePattern) {
  qSetMessagePattern(*messagePattern);
}

const char* qt_core_c_QMessageLogContext_category(const QMessageLogContext* this_ptr) {
  return this_ptr->category;
}

void qt_core_c_QMessageLogContext_constructor_fileName_lineNumber_functionName_categoryName(const char* fileName, int lineNumber, const char* functionName, const char* categoryName, QMessageLogContext* output) {
  new(output) QMessageLogContext(fileName, lineNumber, functionName, categoryName);
}

void qt_core_c_QMessageLogContext_constructor_no_args(QMessageLogContext* output) {
  new(output) QMessageLogContext();
}

void qt_core_c_QMessageLogContext_destructor(QMessageLogContext* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

const char* qt_core_c_QMessageLogContext_file(const QMessageLogContext* this_ptr) {
  return this_ptr->file;
}

const char* qt_core_c_QMessageLogContext_function(const QMessageLogContext* this_ptr) {
  return this_ptr->function;
}

int qt_core_c_QMessageLogContext_line(const QMessageLogContext* this_ptr) {
  return this_ptr->line;
}

void qt_core_c_QMessageLogContext_set_category(QMessageLogContext* this_ptr, const char* value) {
  this_ptr->category = value;
}

void qt_core_c_QMessageLogContext_set_file(QMessageLogContext* this_ptr, const char* value) {
  this_ptr->file = value;
}

void qt_core_c_QMessageLogContext_set_function(QMessageLogContext* this_ptr, const char* value) {
  this_ptr->function = value;
}

void qt_core_c_QMessageLogContext_set_line(QMessageLogContext* this_ptr, int value) {
  this_ptr->line = value;
}

void qt_core_c_QMessageLogContext_set_version(QMessageLogContext* this_ptr, int value) {
  this_ptr->version = value;
}

int qt_core_c_QMessageLogContext_version(const QMessageLogContext* this_ptr) {
  return this_ptr->version;
}

