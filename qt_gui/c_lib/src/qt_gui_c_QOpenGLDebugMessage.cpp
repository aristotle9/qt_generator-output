#include "qt_gui_c_QOpenGLDebugMessage.h"

void qt_gui_c_QOpenGLDebugMessage_constructor_debugMessage(const QOpenGLDebugMessage* debugMessage, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(*debugMessage);
}

void qt_gui_c_QOpenGLDebugMessage_constructor_no_args(QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage();
}

void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text(const QString* text, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createApplicationMessage(*text));
}

void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id(const QString* text, GLuint id, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createApplicationMessage(*text, id));
}

void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id_severity(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createApplicationMessage(*text, id, severity));
}

void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id_severity_type(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage::Type type, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createApplicationMessage(*text, id, severity, type));
}

void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text(const QString* text, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createThirdPartyMessage(*text));
}

void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id(const QString* text, GLuint id, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createThirdPartyMessage(*text, id));
}

void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id_severity(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createThirdPartyMessage(*text, id, severity));
}

void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id_severity_type(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage::Type type, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(QOpenGLDebugMessage::createThirdPartyMessage(*text, id, severity, type));
}

void qt_gui_c_QOpenGLDebugMessage_destructor(QOpenGLDebugMessage* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

GLuint qt_gui_c_QOpenGLDebugMessage_id(const QOpenGLDebugMessage* this_ptr) {
  return this_ptr->id();
}

void qt_gui_c_QOpenGLDebugMessage_message_to_output(const QOpenGLDebugMessage* this_ptr, QString* output) {
  new(output) QString(this_ptr->message());
}

QOpenGLDebugMessage* qt_gui_c_QOpenGLDebugMessage_operator_assign(QOpenGLDebugMessage* this_ptr, const QOpenGLDebugMessage* debugMessage) {
  return &this_ptr->operator=(*debugMessage);
}

bool qt_gui_c_QOpenGLDebugMessage_operator_eq(const QOpenGLDebugMessage* this_ptr, const QOpenGLDebugMessage* debugMessage) {
  return this_ptr->operator==(*debugMessage);
}

bool qt_gui_c_QOpenGLDebugMessage_operator_neq(const QOpenGLDebugMessage* this_ptr, const QOpenGLDebugMessage* debugMessage) {
  return this_ptr->operator!=(*debugMessage);
}

QOpenGLDebugMessage::Severity qt_gui_c_QOpenGLDebugMessage_severity(const QOpenGLDebugMessage* this_ptr) {
  return this_ptr->severity();
}

QOpenGLDebugMessage::Source qt_gui_c_QOpenGLDebugMessage_source(const QOpenGLDebugMessage* this_ptr) {
  return this_ptr->source();
}

void qt_gui_c_QOpenGLDebugMessage_swap(QOpenGLDebugMessage* this_ptr, QOpenGLDebugMessage* other) {
  this_ptr->swap(*other);
}

QOpenGLDebugMessage::Type qt_gui_c_QOpenGLDebugMessage_type(const QOpenGLDebugMessage* this_ptr) {
  return this_ptr->type();
}

