#ifndef QT_CORE_C_QLOGGINGCATEGORY_H
#define QT_CORE_C_QLOGGINGCATEGORY_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT const char* qt_core_c_QLoggingCategory_categoryName(const QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLoggingCategory_constructor_category(const char* category, QLoggingCategory* output);
QT_CORE_C_EXPORT void qt_core_c_QLoggingCategory_constructor_category_severityLevel(const char* category, QtMsgType severityLevel, QLoggingCategory* output);
QT_CORE_C_EXPORT QLoggingCategory* qt_core_c_QLoggingCategory_defaultCategory();
QT_CORE_C_EXPORT void qt_core_c_QLoggingCategory_destructor(QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT void (*qt_core_c_QLoggingCategory_installFilter(void (*arg1)(QLoggingCategory*)))(QLoggingCategory*);
QT_CORE_C_EXPORT bool qt_core_c_QLoggingCategory_isCriticalEnabled(const QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLoggingCategory_isDebugEnabled(const QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLoggingCategory_isEnabled(const QLoggingCategory* this_ptr, QtMsgType type);
QT_CORE_C_EXPORT bool qt_core_c_QLoggingCategory_isInfoEnabled(const QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLoggingCategory_isWarningEnabled(const QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT QLoggingCategory* qt_core_c_QLoggingCategory_operator_call(QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT const QLoggingCategory* qt_core_c_QLoggingCategory_operator_call_const(const QLoggingCategory* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLoggingCategory_setEnabled(QLoggingCategory* this_ptr, QtMsgType type, bool enable);
QT_CORE_C_EXPORT void qt_core_c_QLoggingCategory_setFilterRules(const QString* rules);

} // extern "C"

#endif // QT_CORE_C_QLOGGINGCATEGORY_H
