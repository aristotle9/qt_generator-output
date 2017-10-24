#ifndef QT_WIDGETS_C_QGRAPHICSOBJECT_H
#define QT_WIDGETS_C_QGRAPHICSOBJECT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsObject* qt_widgets_c_QGraphicsObject_G_dynamic_cast_QGraphicsObject_ptr(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsItem_ptr(QGraphicsObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsObject* qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsObject* qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsObject_G_static_cast_QObject_ptr(QGraphicsObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsObject_delete(QGraphicsObject* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsObject_metaObject(const QGraphicsObject* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsObject_qt_metacall(QGraphicsObject* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsObject_qt_metacast(QGraphicsObject* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsObject_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsObject_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsObject_ungrabGesture(QGraphicsObject* this_ptr, const Qt::GestureType* type);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSOBJECT_H
