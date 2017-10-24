#ifndef QT_CORE_C_QEVENTLOOPLOCKER_H
#define QT_CORE_C_QEVENTLOOPLOCKER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QEventLoopLocker_constructor_loop(QEventLoop* loop, QEventLoopLocker* output);
QT_CORE_C_EXPORT void qt_core_c_QEventLoopLocker_constructor_no_args(QEventLoopLocker* output);
QT_CORE_C_EXPORT void qt_core_c_QEventLoopLocker_constructor_thread(QThread* thread, QEventLoopLocker* output);
QT_CORE_C_EXPORT void qt_core_c_QEventLoopLocker_destructor(QEventLoopLocker* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QEVENTLOOPLOCKER_H
