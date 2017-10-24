#ifndef QT_CORE_C_QEVENT_H
#define QT_CORE_C_QEVENT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QEvent_accept(QEvent* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEvent_delete(QEvent* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEvent_ignore(QEvent* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QEvent_isAccepted(const QEvent* this_ptr);
QT_CORE_C_EXPORT QEvent* qt_core_c_QEvent_new_other(const QEvent* other);
QT_CORE_C_EXPORT QEvent* qt_core_c_QEvent_new_type(QEvent::Type type);
QT_CORE_C_EXPORT QEvent* qt_core_c_QEvent_operator_assign(QEvent* this_ptr, const QEvent* other);
QT_CORE_C_EXPORT int qt_core_c_QEvent_registerEventType_hint(int hint);
QT_CORE_C_EXPORT int qt_core_c_QEvent_registerEventType_no_args();
QT_CORE_C_EXPORT void qt_core_c_QEvent_setAccepted(QEvent* this_ptr, bool accepted);
QT_CORE_C_EXPORT bool qt_core_c_QEvent_spontaneous(const QEvent* this_ptr);
QT_CORE_C_EXPORT QEvent::Type qt_core_c_QEvent_type(const QEvent* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QEVENT_H
