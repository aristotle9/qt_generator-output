#include "qt_core_c_QAbstractNativeEventFilter.h"

void qt_core_c_QAbstractNativeEventFilter_delete(QAbstractNativeEventFilter* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QAbstractNativeEventFilter_nativeEventFilter(QAbstractNativeEventFilter* this_ptr, const QByteArray* eventType, void* message, long* result) {
  return this_ptr->nativeEventFilter(*eventType, message, result);
}

