#ifndef QT_CORE_C_QREADLOCKER_H
#define QT_CORE_C_QREADLOCKER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QReadLocker_constructor(QReadWriteLock* readWriteLock, QReadLocker* output);
QT_CORE_C_EXPORT void qt_core_c_QReadLocker_destructor(QReadLocker* this_ptr);
QT_CORE_C_EXPORT QReadWriteLock* qt_core_c_QReadLocker_readWriteLock(const QReadLocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QReadLocker_relock(QReadLocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QReadLocker_unlock(QReadLocker* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QREADLOCKER_H
