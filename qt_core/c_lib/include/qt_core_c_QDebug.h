#ifndef QT_CORE_C_QDEBUG_H
#define QT_CORE_C_QDEBUG_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QDebug_G_swap(QDebug* value1, QDebug* value2);
QT_CORE_C_EXPORT bool qt_core_c_QDebug_autoInsertSpaces(const QDebug* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDebug_constructor_device(QIODevice* device, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QDebug_constructor_o(const QDebug* o, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QDebug_constructor_string(QString* string, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QDebug_constructor_t(QtMsgType t, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QDebug_destructor(QDebug* this_ptr);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_maybeQuote_c(QDebug* this_ptr, char c);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_maybeQuote_no_args(QDebug* this_ptr);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_maybeSpace(QDebug* this_ptr);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_noquote(QDebug* this_ptr);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_nospace(QDebug* this_ptr);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_assign(QDebug* this_ptr, const QDebug* other);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_QChar(QDebug* this_ptr, const QChar* t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_QLatin1String(QDebug* this_ptr, const QLatin1String* t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_QTextStreamManipulator(QDebug* this_ptr, const QTextStreamManipulator* m);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_bool(QDebug* this_ptr, bool t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_char(QDebug* this_ptr, char t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_char16_t(QDebug* this_ptr, char16_t t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_char32_t(QDebug* this_ptr, char32_t t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_const_QByteArray_ref(QDebug* this_ptr, const QByteArray* t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_const_QStringRef_ref(QDebug* this_ptr, const QStringRef* t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_const_QString_ref(QDebug* this_ptr, const QString* t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_const_char_ptr(QDebug* this_ptr, const char* t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_const_void_ptr(QDebug* this_ptr, const void* t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_double(QDebug* this_ptr, double t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_float(QDebug* this_ptr, float t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_int(QDebug* this_ptr, int t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_long(QDebug* this_ptr, long t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_qint64(QDebug* this_ptr, qint64 t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_quint64(QDebug* this_ptr, quint64 t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_short(QDebug* this_ptr, short t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_unsigned_int(QDebug* this_ptr, unsigned int t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_unsigned_long(QDebug* this_ptr, unsigned long t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_operator_shl_unsigned_short(QDebug* this_ptr, unsigned short t);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_quote(QDebug* this_ptr);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_resetFormat(QDebug* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDebug_setAutoInsertSpaces(QDebug* this_ptr, bool b);
QT_CORE_C_EXPORT void qt_core_c_QDebug_setVerbosity(QDebug* this_ptr, int verbosityLevel);
QT_CORE_C_EXPORT QDebug* qt_core_c_QDebug_space(QDebug* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDebug_swap(QDebug* this_ptr, QDebug* other);
QT_CORE_C_EXPORT int qt_core_c_QDebug_verbosity(const QDebug* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QDEBUG_H
