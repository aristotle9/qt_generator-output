#ifndef QT_CORE_C_QABSTRACTNATIVEEVENTFILTER_H
#define QT_CORE_C_QABSTRACTNATIVEEVENTFILTER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QAbstractNativeEventFilter_delete(QAbstractNativeEventFilter* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractNativeEventFilter_nativeEventFilter(QAbstractNativeEventFilter* this_ptr, const QByteArray* eventType, void* message, long* result);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTNATIVEEVENTFILTER_H
