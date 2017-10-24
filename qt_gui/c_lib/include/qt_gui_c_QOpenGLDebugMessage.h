#ifndef QT_GUI_C_QOPENGLDEBUGMESSAGE_H
#define QT_GUI_C_QOPENGLDEBUGMESSAGE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_constructor_debugMessage(const QOpenGLDebugMessage* debugMessage, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_constructor_no_args(QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text(const QString* text, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id(const QString* text, GLuint id, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id_severity(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id_severity_type(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage::Type type, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text(const QString* text, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id(const QString* text, GLuint id, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id_severity(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id_severity_type(const QString* text, GLuint id, QOpenGLDebugMessage::Severity severity, QOpenGLDebugMessage::Type type, QOpenGLDebugMessage* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_destructor(QOpenGLDebugMessage* this_ptr);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLDebugMessage_id(const QOpenGLDebugMessage* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_message_to_output(const QOpenGLDebugMessage* this_ptr, QString* output);
QT_GUI_C_EXPORT QOpenGLDebugMessage* qt_gui_c_QOpenGLDebugMessage_operator_assign(QOpenGLDebugMessage* this_ptr, const QOpenGLDebugMessage* debugMessage);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLDebugMessage_operator_eq(const QOpenGLDebugMessage* this_ptr, const QOpenGLDebugMessage* debugMessage);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLDebugMessage_operator_neq(const QOpenGLDebugMessage* this_ptr, const QOpenGLDebugMessage* debugMessage);
QT_GUI_C_EXPORT QOpenGLDebugMessage::Severity qt_gui_c_QOpenGLDebugMessage_severity(const QOpenGLDebugMessage* this_ptr);
QT_GUI_C_EXPORT QOpenGLDebugMessage::Source qt_gui_c_QOpenGLDebugMessage_source(const QOpenGLDebugMessage* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugMessage_swap(QOpenGLDebugMessage* this_ptr, QOpenGLDebugMessage* other);
QT_GUI_C_EXPORT QOpenGLDebugMessage::Type qt_gui_c_QOpenGLDebugMessage_type(const QOpenGLDebugMessage* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QOPENGLDEBUGMESSAGE_H
