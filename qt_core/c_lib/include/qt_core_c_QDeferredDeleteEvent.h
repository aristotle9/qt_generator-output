#ifndef QT_CORE_C_QDEFERREDDELETEEVENT_H
#define QT_CORE_C_QDEFERREDDELETEEVENT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDeferredDeleteEvent* qt_core_c_QDeferredDeleteEvent_G_dynamic_cast_QDeferredDeleteEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QDeferredDeleteEvent* qt_core_c_QDeferredDeleteEvent_G_static_cast_QDeferredDeleteEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QEvent* qt_core_c_QDeferredDeleteEvent_G_static_cast_QEvent_ptr(QDeferredDeleteEvent* ptr);
QT_CORE_C_EXPORT void qt_core_c_QDeferredDeleteEvent_delete(QDeferredDeleteEvent* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QDeferredDeleteEvent_loopLevel(const QDeferredDeleteEvent* this_ptr);
QT_CORE_C_EXPORT QDeferredDeleteEvent* qt_core_c_QDeferredDeleteEvent_new();

} // extern "C"

#endif // QT_CORE_C_QDEFERREDDELETEEVENT_H
