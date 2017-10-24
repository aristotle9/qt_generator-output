#ifndef QT_CORE_C_QOBJECTCLEANUPHANDLER_H
#define QT_CORE_C_QOBJECTCLEANUPHANDLER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QObjectCleanupHandler* qt_core_c_QObjectCleanupHandler_G_dynamic_cast_QObjectCleanupHandler_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObjectCleanupHandler* qt_core_c_QObjectCleanupHandler_G_static_cast_QObjectCleanupHandler_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QObjectCleanupHandler_G_static_cast_QObject_ptr(QObjectCleanupHandler* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QObjectCleanupHandler_add(QObjectCleanupHandler* this_ptr, QObject* object);
QT_CORE_C_EXPORT void qt_core_c_QObjectCleanupHandler_clear(QObjectCleanupHandler* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObjectCleanupHandler_delete(QObjectCleanupHandler* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QObjectCleanupHandler_isEmpty(const QObjectCleanupHandler* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QObjectCleanupHandler_metaObject(const QObjectCleanupHandler* this_ptr);
QT_CORE_C_EXPORT QObjectCleanupHandler* qt_core_c_QObjectCleanupHandler_new();
QT_CORE_C_EXPORT void qt_core_c_QObjectCleanupHandler_remove(QObjectCleanupHandler* this_ptr, QObject* object);
QT_CORE_C_EXPORT void qt_core_c_QObjectCleanupHandler_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QObjectCleanupHandler_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QOBJECTCLEANUPHANDLER_H
