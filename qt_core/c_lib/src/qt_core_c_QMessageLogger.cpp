#include "qt_core_c_QMessageLogger.h"

void qt_core_c_QMessageLogger_constructor_file_line_function(const char* file, int line, const char* function, QMessageLogger* output) {
  new(output) QMessageLogger(file, line, function);
}

void qt_core_c_QMessageLogger_constructor_file_line_function_category(const char* file, int line, const char* function, const char* category, QMessageLogger* output) {
  new(output) QMessageLogger(file, line, function, category);
}

void qt_core_c_QMessageLogger_constructor_no_args(QMessageLogger* output) {
  new(output) QMessageLogger();
}

void qt_core_c_QMessageLogger_critical_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output) {
  new(output) QDebug(this_ptr->critical(*cat));
}

void qt_core_c_QMessageLogger_critical_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output) {
  new(output) QDebug(this_ptr->critical());
}

void qt_core_c_QMessageLogger_debug_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output) {
  new(output) QDebug(this_ptr->debug(*cat));
}

void qt_core_c_QMessageLogger_debug_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output) {
  new(output) QDebug(this_ptr->debug());
}

void qt_core_c_QMessageLogger_destructor(QMessageLogger* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QMessageLogger_info_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output) {
  new(output) QDebug(this_ptr->info(*cat));
}

void qt_core_c_QMessageLogger_info_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output) {
  new(output) QDebug(this_ptr->info());
}

void qt_core_c_QMessageLogger_warning_to_output_cat(const QMessageLogger* this_ptr, const QLoggingCategory* cat, QDebug* output) {
  new(output) QDebug(this_ptr->warning(*cat));
}

void qt_core_c_QMessageLogger_warning_to_output_no_args(const QMessageLogger* this_ptr, QDebug* output) {
  new(output) QDebug(this_ptr->warning());
}

