#ifndef QT_CORE_C_QRUNNABLE_H
#define QT_CORE_C_QRUNNABLE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QRunnable_autoDelete(const QRunnable* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRunnable_delete(QRunnable* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRunnable_run(QRunnable* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QRunnable_setAutoDelete(QRunnable* this_ptr, bool _autoDelete);

} // extern "C"

#endif // QT_CORE_C_QRUNNABLE_H
