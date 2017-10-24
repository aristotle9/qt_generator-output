#ifndef QT_CORE_C_QMUTEXLOCKER_H
#define QT_CORE_C_QMUTEXLOCKER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMutexLocker_constructor(QBasicMutex* m, QMutexLocker* output);
QT_CORE_C_EXPORT void qt_core_c_QMutexLocker_destructor(QMutexLocker* this_ptr);
QT_CORE_C_EXPORT QMutex* qt_core_c_QMutexLocker_mutex(const QMutexLocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMutexLocker_relock(QMutexLocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMutexLocker_unlock(QMutexLocker* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QMUTEXLOCKER_H
