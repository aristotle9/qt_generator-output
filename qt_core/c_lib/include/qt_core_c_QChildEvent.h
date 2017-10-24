#ifndef QT_CORE_C_QCHILDEVENT_H
#define QT_CORE_C_QCHILDEVENT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QChildEvent* qt_core_c_QChildEvent_G_dynamic_cast_QChildEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QChildEvent* qt_core_c_QChildEvent_G_static_cast_QChildEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QEvent* qt_core_c_QChildEvent_G_static_cast_QEvent_ptr(QChildEvent* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChildEvent_added(const QChildEvent* this_ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QChildEvent_child(const QChildEvent* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QChildEvent_delete(QChildEvent* this_ptr);
QT_CORE_C_EXPORT QChildEvent* qt_core_c_QChildEvent_new(QEvent::Type type, QObject* child);
QT_CORE_C_EXPORT bool qt_core_c_QChildEvent_polished(const QChildEvent* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChildEvent_removed(const QChildEvent* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QCHILDEVENT_H
