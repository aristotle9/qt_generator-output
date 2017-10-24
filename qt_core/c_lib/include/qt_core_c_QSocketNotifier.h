#ifndef QT_CORE_C_QSOCKETNOTIFIER_H
#define QT_CORE_C_QSOCKETNOTIFIER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSocketNotifier* qt_core_c_QSocketNotifier_G_dynamic_cast_QSocketNotifier_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSocketNotifier_G_static_cast_QObject_ptr(QSocketNotifier* ptr);
QT_CORE_C_EXPORT QSocketNotifier* qt_core_c_QSocketNotifier_G_static_cast_QSocketNotifier_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QSocketNotifier_delete(QSocketNotifier* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSocketNotifier_isEnabled(const QSocketNotifier* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSocketNotifier_metaObject(const QSocketNotifier* this_ptr);
QT_CORE_C_EXPORT QSocketNotifier* qt_core_c_QSocketNotifier_new_socket_arg2(qintptr socket, QSocketNotifier::Type arg2);
QT_CORE_C_EXPORT QSocketNotifier* qt_core_c_QSocketNotifier_new_socket_arg2_parent(qintptr socket, QSocketNotifier::Type arg2, QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QSocketNotifier_setEnabled(QSocketNotifier* this_ptr, bool arg1);
QT_CORE_C_EXPORT qintptr qt_core_c_QSocketNotifier_socket(const QSocketNotifier* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSocketNotifier_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSocketNotifier_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT QSocketNotifier::Type qt_core_c_QSocketNotifier_type(const QSocketNotifier* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QSOCKETNOTIFIER_H
