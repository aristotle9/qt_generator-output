#ifndef QT_WIDGETS_C_QPANGESTURE_H
#define QT_WIDGETS_C_QPANGESTURE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QPanGesture* qt_widgets_c_QPanGesture_G_dynamic_cast_QPanGesture_ptr(QGesture* ptr);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QPanGesture_G_static_cast_QGesture_ptr(QPanGesture* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QPanGesture_G_static_cast_QObject_ptr(QPanGesture* ptr);
QT_WIDGETS_C_EXPORT QPanGesture* qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QGesture(QGesture* ptr);
QT_WIDGETS_C_EXPORT QPanGesture* qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QPanGesture_acceleration(const QPanGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_delete(QPanGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_delta_to_output(const QPanGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_lastOffset_to_output(const QPanGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QPanGesture_metaObject(const QPanGesture* this_ptr);
QT_WIDGETS_C_EXPORT QPanGesture* qt_widgets_c_QPanGesture_new_no_args();
QT_WIDGETS_C_EXPORT QPanGesture* qt_widgets_c_QPanGesture_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_offset_to_output(const QPanGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPanGesture_qt_metacall(QPanGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QPanGesture_qt_metacast(QPanGesture* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_setAcceleration(QPanGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_setLastOffset(QPanGesture* this_ptr, const QPointF* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_setOffset(QPanGesture* this_ptr, const QPointF* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPanGesture_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QPANGESTURE_H
