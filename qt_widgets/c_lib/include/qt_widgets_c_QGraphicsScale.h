#ifndef QT_WIDGETS_C_QGRAPHICSSCALE_H
#define QT_WIDGETS_C_QGRAPHICSSCALE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsScale* qt_widgets_c_QGraphicsScale_G_dynamic_cast_QGraphicsScale_ptr(QGraphicsTransform* ptr);
QT_WIDGETS_C_EXPORT QGraphicsScale* qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QGraphicsTransform(QGraphicsTransform* ptr);
QT_WIDGETS_C_EXPORT QGraphicsScale* qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsTransform* qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsTransform_ptr(QGraphicsScale* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsScale_G_static_cast_QObject_ptr(QGraphicsScale* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_applyTo(const QGraphicsScale* this_ptr, QMatrix4x4* matrix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_delete(QGraphicsScale* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsScale_metaObject(const QGraphicsScale* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsScale* qt_widgets_c_QGraphicsScale_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsScale* qt_widgets_c_QGraphicsScale_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT QVector3D* qt_widgets_c_QGraphicsScale_origin_as_ptr(const QGraphicsScale* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsScale_qt_metacall(QGraphicsScale* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsScale_qt_metacast(QGraphicsScale* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_setOrigin(QGraphicsScale* this_ptr, const QVector3D* point);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_setXScale(QGraphicsScale* this_ptr, double arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_setYScale(QGraphicsScale* this_ptr, double arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_setZScale(QGraphicsScale* this_ptr, double arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsScale_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsScale_xScale(const QGraphicsScale* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsScale_yScale(const QGraphicsScale* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsScale_zScale(const QGraphicsScale* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSSCALE_H
