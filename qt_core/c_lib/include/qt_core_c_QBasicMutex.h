#ifndef QT_CORE_C_QBASICMUTEX_H
#define QT_CORE_C_QBASICMUTEX_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QBasicMutex_delete(QBasicMutex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBasicMutex_isRecursive(QBasicMutex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBasicMutex_isRecursive_const(const QBasicMutex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBasicMutex_lock(QBasicMutex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBasicMutex_tryLock(QBasicMutex* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBasicMutex_try_lock(QBasicMutex* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBasicMutex_unlock(QBasicMutex* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QBASICMUTEX_H
