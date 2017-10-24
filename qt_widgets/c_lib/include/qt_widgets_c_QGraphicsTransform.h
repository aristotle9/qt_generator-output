#ifndef QT_WIDGETS_C_QGRAPHICSTRANSFORM_H
#define QT_WIDGETS_C_QGRAPHICSTRANSFORM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsTransform* qt_widgets_c_QGraphicsTransform_G_static_cast_QGraphicsTransform_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsTransform_G_static_cast_QObject_ptr(QGraphicsTransform* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTransform_applyTo(const QGraphicsTransform* this_ptr, QMatrix4x4* matrix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTransform_delete(QGraphicsTransform* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsTransform_metaObject(const QGraphicsTransform* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsTransform_qt_metacall(QGraphicsTransform* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsTransform_qt_metacast(QGraphicsTransform* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTransform_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTransform_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSTRANSFORM_H
