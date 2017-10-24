#ifndef QT_CORE_C_QTIMEREVENT_H
#define QT_CORE_C_QTIMEREVENT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QTimerEvent* qt_core_c_QTimerEvent_G_dynamic_cast_QTimerEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QEvent* qt_core_c_QTimerEvent_G_static_cast_QEvent_ptr(QTimerEvent* ptr);
QT_CORE_C_EXPORT QTimerEvent* qt_core_c_QTimerEvent_G_static_cast_QTimerEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimerEvent_delete(QTimerEvent* this_ptr);
QT_CORE_C_EXPORT QTimerEvent* qt_core_c_QTimerEvent_new(int timerId);
QT_CORE_C_EXPORT int qt_core_c_QTimerEvent_timerId(const QTimerEvent* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QTIMEREVENT_H
