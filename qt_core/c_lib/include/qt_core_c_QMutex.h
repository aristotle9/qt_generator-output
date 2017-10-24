#ifndef QT_CORE_C_QMUTEX_H
#define QT_CORE_C_QMUTEX_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QBasicMutex* qt_core_c_QMutex_G_static_cast_QBasicMutex_ptr(QMutex* ptr);
QT_CORE_C_EXPORT QMutex* qt_core_c_QMutex_G_static_cast_QMutex_ptr(QBasicMutex* ptr);
QT_CORE_C_EXPORT void qt_core_c_QMutex_delete(QMutex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMutex_isRecursive(const QMutex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMutex_lock(QMutex* this_ptr);
QT_CORE_C_EXPORT QMutex* qt_core_c_QMutex_new_mode(QMutex::RecursionMode mode);
QT_CORE_C_EXPORT QMutex* qt_core_c_QMutex_new_no_args();
QT_CORE_C_EXPORT bool qt_core_c_QMutex_tryLock_no_args(QMutex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMutex_tryLock_timeout(QMutex* this_ptr, int timeout);
QT_CORE_C_EXPORT bool qt_core_c_QMutex_try_lock(QMutex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMutex_unlock(QMutex* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QMUTEX_H
