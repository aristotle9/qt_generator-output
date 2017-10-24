#include "qt_gui_c_QOpenGLDebugLogger.h"

void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_message(const QDebug* debug, const QOpenGLDebugMessage* message, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *message));
}

void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_severity(const QDebug* debug, const QOpenGLDebugMessage::Severity* severity, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *severity));
}

void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_source(const QDebug* debug, const QOpenGLDebugMessage::Source* source, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *source));
}

void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_type(const QDebug* debug, const QOpenGLDebugMessage::Type* type, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *type));
}

QObject* qt_gui_c_QOpenGLDebugLogger_G_static_cast_QObject_ptr(QOpenGLDebugLogger* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLDebugLogger* qt_gui_c_QOpenGLDebugLogger_G_static_cast_QOpenGLDebugLogger_ptr(QObject* ptr) {
  return static_cast<QOpenGLDebugLogger*>(ptr);
}

void qt_gui_c_QOpenGLDebugLogger_G_swap(QOpenGLDebugMessage* value1, QOpenGLDebugMessage* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QOpenGLDebugLogger_delete(QOpenGLDebugLogger* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QOpenGLDebugLogger_initialize(QOpenGLDebugLogger* this_ptr) {
  return this_ptr->initialize();
}

bool qt_gui_c_QOpenGLDebugLogger_isLogging(const QOpenGLDebugLogger* this_ptr) {
  return this_ptr->isLogging();
}

void qt_gui_c_QOpenGLDebugLogger_logMessage(QOpenGLDebugLogger* this_ptr, const QOpenGLDebugMessage* debugMessage) {
  this_ptr->logMessage(*debugMessage);
}

void qt_gui_c_QOpenGLDebugLogger_loggedMessages_to_output(const QOpenGLDebugLogger* this_ptr, QList< QOpenGLDebugMessage >* output) {
  new(output) QList< QOpenGLDebugMessage >(this_ptr->loggedMessages());
}

QOpenGLDebugLogger::LoggingMode qt_gui_c_QOpenGLDebugLogger_loggingMode(const QOpenGLDebugLogger* this_ptr) {
  return this_ptr->loggingMode();
}

qint64 qt_gui_c_QOpenGLDebugLogger_maximumMessageLength(const QOpenGLDebugLogger* this_ptr) {
  return this_ptr->maximumMessageLength();
}

const QMetaObject* qt_gui_c_QOpenGLDebugLogger_metaObject(const QOpenGLDebugLogger* this_ptr) {
  return this_ptr->metaObject();
}

QOpenGLDebugLogger* qt_gui_c_QOpenGLDebugLogger_new_no_args() {
  return new QOpenGLDebugLogger();
}

QOpenGLDebugLogger* qt_gui_c_QOpenGLDebugLogger_new_parent(QObject* parent) {
  return new QOpenGLDebugLogger(parent);
}

void qt_gui_c_QOpenGLDebugLogger_popGroup(QOpenGLDebugLogger* this_ptr) {
  this_ptr->popGroup();
}

void qt_gui_c_QOpenGLDebugLogger_pushGroup_name(QOpenGLDebugLogger* this_ptr, const QString* name) {
  this_ptr->pushGroup(*name);
}

void qt_gui_c_QOpenGLDebugLogger_pushGroup_name_id(QOpenGLDebugLogger* this_ptr, const QString* name, GLuint id) {
  this_ptr->pushGroup(*name, id);
}

void qt_gui_c_QOpenGLDebugLogger_pushGroup_name_id_source(QOpenGLDebugLogger* this_ptr, const QString* name, GLuint id, const QOpenGLDebugMessage::Source* source) {
  this_ptr->pushGroup(*name, id, *source);
}

int qt_gui_c_QOpenGLDebugLogger_qt_metacall(QOpenGLDebugLogger* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLDebugLogger_qt_metacast(QOpenGLDebugLogger* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QOpenGLDebugLogger_startLogging_loggingMode(QOpenGLDebugLogger* this_ptr, QOpenGLDebugLogger::LoggingMode loggingMode) {
  this_ptr->startLogging(loggingMode);
}

void qt_gui_c_QOpenGLDebugLogger_startLogging_no_args(QOpenGLDebugLogger* this_ptr) {
  this_ptr->startLogging();
}

void qt_gui_c_QOpenGLDebugLogger_stopLogging(QOpenGLDebugLogger* this_ptr) {
  this_ptr->stopLogging();
}

void qt_gui_c_QOpenGLDebugLogger_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLDebugLogger::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLDebugLogger_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLDebugLogger::tr(s, c, n));
}

