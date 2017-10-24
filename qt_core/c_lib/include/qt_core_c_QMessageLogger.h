#ifndef QT_CORE_C_QMESSAGELOGGER_H
#define QT_CORE_C_QMESSAGELOGGER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_constructor_file_line_function(const char* file, int line, const char* function, QMessageLogger* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_constructor_file_line_function_category(const char* file, int line, const char* function, const char* category, QMessageLogger* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_constructor_no_args(QMessageLogger* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_critical_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_critical_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_debug_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_debug_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_destructor(QMessageLogger* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_info_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_info_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_warning_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageLogger_warning_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output);

} // extern "C"

#endif // QT_CORE_C_QMESSAGELOGGER_H
