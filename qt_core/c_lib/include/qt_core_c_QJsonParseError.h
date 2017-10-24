#ifndef QT_CORE_C_QJSONPARSEERROR_H
#define QT_CORE_C_QJSONPARSEERROR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QJsonParseError_destructor(QJsonParseError* this_ptr);
QT_CORE_C_EXPORT QJsonParseError::ParseError qt_core_c_QJsonParseError_error(const QJsonParseError* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonParseError_errorString_to_output(const QJsonParseError* this_ptr, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QJsonParseError_offset(const QJsonParseError* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonParseError_set_error(QJsonParseError* this_ptr, QJsonParseError::ParseError value);
QT_CORE_C_EXPORT void qt_core_c_QJsonParseError_set_offset(QJsonParseError* this_ptr, int value);

} // extern "C"

#endif // QT_CORE_C_QJSONPARSEERROR_H
