#ifndef QT_CORE_C_QWRITELOCKER_H
#define QT_CORE_C_QWRITELOCKER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QWriteLocker_constructor(QReadWriteLock* readWriteLock, QWriteLocker* output);
QT_CORE_C_EXPORT void qt_core_c_QWriteLocker_destructor(QWriteLocker* this_ptr);
QT_CORE_C_EXPORT QReadWriteLock* qt_core_c_QWriteLocker_readWriteLock(const QWriteLocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QWriteLocker_relock(QWriteLocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QWriteLocker_unlock(QWriteLocker* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QWRITELOCKER_H
