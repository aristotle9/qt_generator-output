#ifndef QT_GUI_C_QOPENGLDEBUGLOGGER_H
#define QT_GUI_C_QOPENGLDEBUGLOGGER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_message(const QDebug* debug, const QOpenGLDebugMessage* message, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_severity(const QDebug* debug, const QOpenGLDebugMessage::Severity* severity, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_source(const QDebug* debug, const QOpenGLDebugMessage::Source* source, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_type(const QDebug* debug, const QOpenGLDebugMessage::Type* type, QDebug* output);
QT_GUI_C_EXPORT QObject* qt_gui_c_QOpenGLDebugLogger_G_static_cast_QObject_ptr(QOpenGLDebugLogger* ptr);
QT_GUI_C_EXPORT QOpenGLDebugLogger* qt_gui_c_QOpenGLDebugLogger_G_static_cast_QOpenGLDebugLogger_ptr(QObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_G_swap(QOpenGLDebugMessage* value1, QOpenGLDebugMessage* value2);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_delete(QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLDebugLogger_initialize(QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLDebugLogger_isLogging(const QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_logMessage(QOpenGLDebugLogger* this_ptr, const QOpenGLDebugMessage* debugMessage);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_loggedMessages_to_output(const QOpenGLDebugLogger* this_ptr, QList< QOpenGLDebugMessage >* output);
QT_GUI_C_EXPORT QOpenGLDebugLogger::LoggingMode qt_gui_c_QOpenGLDebugLogger_loggingMode(const QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT qint64 qt_gui_c_QOpenGLDebugLogger_maximumMessageLength(const QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOpenGLDebugLogger_metaObject(const QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT QOpenGLDebugLogger* qt_gui_c_QOpenGLDebugLogger_new_no_args();
QT_GUI_C_EXPORT QOpenGLDebugLogger* qt_gui_c_QOpenGLDebugLogger_new_parent(QObject* parent);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_popGroup(QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_pushGroup_name(QOpenGLDebugLogger* this_ptr, const QString* name);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_pushGroup_name_id(QOpenGLDebugLogger* this_ptr, const QString* name, GLuint id);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_pushGroup_name_id_source(QOpenGLDebugLogger* this_ptr, const QString* name, GLuint id, const QOpenGLDebugMessage::Source* source);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLDebugLogger_qt_metacall(QOpenGLDebugLogger* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLDebugLogger_qt_metacast(QOpenGLDebugLogger* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_startLogging_loggingMode(QOpenGLDebugLogger* this_ptr, QOpenGLDebugLogger::LoggingMode loggingMode);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_startLogging_no_args(QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_stopLogging(QOpenGLDebugLogger* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLDebugLogger_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QOPENGLDEBUGLOGGER_H
