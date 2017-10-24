#ifndef QT_CORE_C_QMESSAGELOGCONTEXT_H
#define QT_CORE_C_QMESSAGELOGCONTEXT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_G_qFormatLogMessage_to_output(QtMsgType type, const QMessageLogContext* context, const QString* buf, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_G_qSetMessagePattern(const QString* messagePattern);
QT_CORE_C_EXPORT const char* qt_core_c_QMessageLogContext_category(const QMessageLogContext* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_constructor_fileName_lineNumber_functionName_categoryName(const char* fileName, int lineNumber, const char* functionName, const char* categoryName, QMessageLogContext* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_constructor_no_args(QMessageLogContext* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_destructor(QMessageLogContext* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMessageLogContext_file(const QMessageLogContext* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMessageLogContext_function(const QMessageLogContext* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMessageLogContext_line(const QMessageLogContext* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_set_category(QMessageLogContext* this_ptr, const char* value);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_set_file(QMessageLogContext* this_ptr, const char* value);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_set_function(QMessageLogContext* this_ptr, const char* value);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_set_line(QMessageLogContext* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogContext_set_version(QMessageLogContext* this_ptr, int value);
QT_CORE_C_EXPORT int qt_core_c_QMessageLogContext_version(const QMessageLogContext* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QMESSAGELOGCONTEXT_H
