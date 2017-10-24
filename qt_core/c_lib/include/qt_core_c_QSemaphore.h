#ifndef QT_CORE_C_QSEMAPHORE_H
#define QT_CORE_C_QSEMAPHORE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QSemaphore_acquire_n(QSemaphore* this_ptr, int n);
QT_CORE_C_EXPORT void qt_core_c_QSemaphore_acquire_no_args(QSemaphore* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QSemaphore_available(const QSemaphore* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSemaphore_constructor_n(int n, QSemaphore* output);
QT_CORE_C_EXPORT void qt_core_c_QSemaphore_constructor_no_args(QSemaphore* output);
QT_CORE_C_EXPORT void qt_core_c_QSemaphore_destructor(QSemaphore* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSemaphore_release_n(QSemaphore* this_ptr, int n);
QT_CORE_C_EXPORT void qt_core_c_QSemaphore_release_no_args(QSemaphore* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSemaphore_tryAcquire_n(QSemaphore* this_ptr, int n);
QT_CORE_C_EXPORT bool qt_core_c_QSemaphore_tryAcquire_n_timeout(QSemaphore* this_ptr, int n, int timeout);
QT_CORE_C_EXPORT bool qt_core_c_QSemaphore_tryAcquire_no_args(QSemaphore* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QSEMAPHORE_H
