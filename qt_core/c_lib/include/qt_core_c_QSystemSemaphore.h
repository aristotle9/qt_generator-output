#ifndef QT_CORE_C_QSYSTEMSEMAPHORE_H
#define QT_CORE_C_QSYSTEMSEMAPHORE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QSystemSemaphore_acquire(QSystemSemaphore* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_constructor_key(const QString* key, QSystemSemaphore* output);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_constructor_key_initialValue(const QString* key, int initialValue, QSystemSemaphore* output);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_constructor_key_initialValue_mode(const QString* key, int initialValue, QSystemSemaphore::AccessMode mode, QSystemSemaphore* output);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_destructor(QSystemSemaphore* this_ptr);
QT_CORE_C_EXPORT QSystemSemaphore::SystemSemaphoreError qt_core_c_QSystemSemaphore_error(const QSystemSemaphore* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_errorString_to_output(const QSystemSemaphore* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_key_to_output(const QSystemSemaphore* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QSystemSemaphore_release_n(QSystemSemaphore* this_ptr, int n);
QT_CORE_C_EXPORT bool qt_core_c_QSystemSemaphore_release_no_args(QSystemSemaphore* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_setKey_key(QSystemSemaphore* this_ptr, const QString* key);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_setKey_key_initialValue(QSystemSemaphore* this_ptr, const QString* key, int initialValue);
QT_CORE_C_EXPORT void qt_core_c_QSystemSemaphore_setKey_key_initialValue_mode(QSystemSemaphore* this_ptr, const QString* key, int initialValue, QSystemSemaphore::AccessMode mode);

} // extern "C"

#endif // QT_CORE_C_QSYSTEMSEMAPHORE_H
