#ifndef QT_WIDGETS_C_QPINCHGESTURE_H
#define QT_WIDGETS_C_QPINCHGESTURE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QPinchGesture* qt_widgets_c_QPinchGesture_G_dynamic_cast_QPinchGesture_ptr(QGesture* ptr);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QPinchGesture_G_static_cast_QGesture_ptr(QPinchGesture* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QPinchGesture_G_static_cast_QObject_ptr(QPinchGesture* ptr);
QT_WIDGETS_C_EXPORT QPinchGesture* qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QGesture(QGesture* ptr);
QT_WIDGETS_C_EXPORT QPinchGesture* qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_centerPoint_to_output(const QPinchGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QPinchGesture_changeFlags(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_delete(QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_lastCenterPoint_to_output(const QPinchGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QPinchGesture_lastRotationAngle(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QPinchGesture_lastScaleFactor(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QPinchGesture_metaObject(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT QPinchGesture* qt_widgets_c_QPinchGesture_new_no_args();
QT_WIDGETS_C_EXPORT QPinchGesture* qt_widgets_c_QPinchGesture_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPinchGesture_qt_metacall(QPinchGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QPinchGesture_qt_metacast(QPinchGesture* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QPinchGesture_rotationAngle(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QPinchGesture_scaleFactor(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setCenterPoint(QPinchGesture* this_ptr, const QPointF* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setChangeFlags(QPinchGesture* this_ptr, unsigned int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setLastCenterPoint(QPinchGesture* this_ptr, const QPointF* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setLastRotationAngle(QPinchGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setLastScaleFactor(QPinchGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setRotationAngle(QPinchGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setScaleFactor(QPinchGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setStartCenterPoint(QPinchGesture* this_ptr, const QPointF* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setTotalChangeFlags(QPinchGesture* this_ptr, unsigned int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setTotalRotationAngle(QPinchGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_setTotalScaleFactor(QPinchGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_startCenterPoint_to_output(const QPinchGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QPinchGesture_totalChangeFlags(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QPinchGesture_totalRotationAngle(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QPinchGesture_totalScaleFactor(const QPinchGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPinchGesture_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QPINCHGESTURE_H
