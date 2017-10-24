#ifndef QT_CORE_C_QREADWRITELOCK_H
#define QT_CORE_C_QREADWRITELOCK_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QReadWriteLock_delete(QReadWriteLock* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QReadWriteLock_lockForRead(QReadWriteLock* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QReadWriteLock_lockForWrite(QReadWriteLock* this_ptr);
QT_CORE_C_EXPORT QReadWriteLock* qt_core_c_QReadWriteLock_new_no_args();
QT_CORE_C_EXPORT QReadWriteLock* qt_core_c_QReadWriteLock_new_recursionMode(QReadWriteLock::RecursionMode recursionMode);
QT_CORE_C_EXPORT bool qt_core_c_QReadWriteLock_tryLockForRead_no_args(QReadWriteLock* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QReadWriteLock_tryLockForRead_timeout(QReadWriteLock* this_ptr, int timeout);
QT_CORE_C_EXPORT bool qt_core_c_QReadWriteLock_tryLockForWrite_no_args(QReadWriteLock* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QReadWriteLock_tryLockForWrite_timeout(QReadWriteLock* this_ptr, int timeout);
QT_CORE_C_EXPORT void qt_core_c_QReadWriteLock_unlock(QReadWriteLock* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QREADWRITELOCK_H
