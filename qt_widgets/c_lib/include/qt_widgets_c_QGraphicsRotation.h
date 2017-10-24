#ifndef QT_WIDGETS_C_QGRAPHICSROTATION_H
#define QT_WIDGETS_C_QGRAPHICSROTATION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsRotation* qt_widgets_c_QGraphicsRotation_G_dynamic_cast_QGraphicsRotation_ptr(QGraphicsTransform* ptr);
QT_WIDGETS_C_EXPORT QGraphicsRotation* qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QGraphicsTransform(QGraphicsTransform* ptr);
QT_WIDGETS_C_EXPORT QGraphicsRotation* qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsTransform* qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsTransform_ptr(QGraphicsRotation* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsRotation_G_static_cast_QObject_ptr(QGraphicsRotation* ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsRotation_angle(const QGraphicsRotation* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_applyTo(const QGraphicsRotation* this_ptr, QMatrix4x4* matrix);
QT_WIDGETS_C_EXPORT QVector3D* qt_widgets_c_QGraphicsRotation_axis_as_ptr(const QGraphicsRotation* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_delete(QGraphicsRotation* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsRotation_metaObject(const QGraphicsRotation* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsRotation* qt_widgets_c_QGraphicsRotation_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsRotation* qt_widgets_c_QGraphicsRotation_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT QVector3D* qt_widgets_c_QGraphicsRotation_origin_as_ptr(const QGraphicsRotation* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsRotation_qt_metacall(QGraphicsRotation* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsRotation_qt_metacast(QGraphicsRotation* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_setAngle(QGraphicsRotation* this_ptr, double arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_setAxis_QVector3D(QGraphicsRotation* this_ptr, const QVector3D* axis);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_setAxis_Qt_Axis(QGraphicsRotation* this_ptr, const Qt::Axis* axis);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_setOrigin(QGraphicsRotation* this_ptr, const QVector3D* point);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsRotation_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSROTATION_H
