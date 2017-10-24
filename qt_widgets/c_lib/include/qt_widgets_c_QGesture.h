#ifndef QT_WIDGETS_C_QGESTURE_H
#define QT_WIDGETS_C_QGESTURE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_G_operator_shl_to_output_QDebug_QGesture(const QDebug* arg1, const QGesture* arg2, QDebug* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_G_operator_shl_to_output_QDebug_QGestureEvent(const QDebug* arg1, const QGestureEvent* arg2, QDebug* output);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QGesture_G_static_cast_QGesture_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGesture_G_static_cast_QObject_ptr(QGesture* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_delete(QGesture* this_ptr);
QT_WIDGETS_C_EXPORT QGesture::GestureCancelPolicy qt_widgets_c_QGesture_gestureCancelPolicy(const QGesture* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGesture_hasHotSpot(const QGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_hotSpot_to_output(const QGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGesture_metaObject(const QGesture* this_ptr);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QGesture_new_no_args();
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QGesture_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGesture_qt_metacall(QGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGesture_qt_metacast(QGesture* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_setGestureCancelPolicy(QGesture* this_ptr, QGesture::GestureCancelPolicy policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_setHotSpot(QGesture* this_ptr, const QPointF* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGesture_unsetHotSpot(QGesture* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGESTURE_H
