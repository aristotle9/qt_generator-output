#ifndef QT_CORE_C_QWAITCONDITION_H
#define QT_CORE_C_QWAITCONDITION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QWaitCondition_constructor(QWaitCondition* output);
QT_CORE_C_EXPORT void qt_core_c_QWaitCondition_destructor(QWaitCondition* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QWaitCondition_notify_all(QWaitCondition* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QWaitCondition_notify_one(QWaitCondition* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QWaitCondition_wait_lockedMutex(QWaitCondition* this_ptr, QMutex* lockedMutex);
QT_CORE_C_EXPORT bool qt_core_c_QWaitCondition_wait_lockedMutex_time(QWaitCondition* this_ptr, QMutex* lockedMutex, unsigned long time);
QT_CORE_C_EXPORT bool qt_core_c_QWaitCondition_wait_lockedReadWriteLock(QWaitCondition* this_ptr, QReadWriteLock* lockedReadWriteLock);
QT_CORE_C_EXPORT bool qt_core_c_QWaitCondition_wait_lockedReadWriteLock_time(QWaitCondition* this_ptr, QReadWriteLock* lockedReadWriteLock, unsigned long time);
QT_CORE_C_EXPORT void qt_core_c_QWaitCondition_wakeAll(QWaitCondition* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QWaitCondition_wakeOne(QWaitCondition* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QWAITCONDITION_H
