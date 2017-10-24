#ifndef QT_CORE_C_QANIMATIONDRIVER_H
#define QT_CORE_C_QANIMATIONDRIVER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAnimationDriver* qt_core_c_QAnimationDriver_G_dynamic_cast_QAnimationDriver_ptr(QObject* ptr);
QT_CORE_C_EXPORT QAnimationDriver* qt_core_c_QAnimationDriver_G_static_cast_QAnimationDriver_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAnimationDriver_G_static_cast_QObject_ptr(QAnimationDriver* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationDriver_advance(QAnimationDriver* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationDriver_delete(QAnimationDriver* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QAnimationDriver_elapsed(const QAnimationDriver* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationDriver_install(QAnimationDriver* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAnimationDriver_isRunning(const QAnimationDriver* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAnimationDriver_metaObject(const QAnimationDriver* this_ptr);
QT_CORE_C_EXPORT QAnimationDriver* qt_core_c_QAnimationDriver_new_no_args();
QT_CORE_C_EXPORT QAnimationDriver* qt_core_c_QAnimationDriver_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QAnimationDriver_setStartTime(QAnimationDriver* this_ptr, qint64 startTime);
QT_CORE_C_EXPORT qint64 qt_core_c_QAnimationDriver_startTime(const QAnimationDriver* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationDriver_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAnimationDriver_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAnimationDriver_uninstall(QAnimationDriver* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QANIMATIONDRIVER_H
