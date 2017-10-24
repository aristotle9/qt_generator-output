#ifndef QT_WIDGETS_C_QTAPGESTURE_H
#define QT_WIDGETS_C_QTAPGESTURE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTapGesture* qt_widgets_c_QTapGesture_G_dynamic_cast_QTapGesture_ptr(QGesture* ptr);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QTapGesture_G_static_cast_QGesture_ptr(QTapGesture* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTapGesture_G_static_cast_QObject_ptr(QTapGesture* ptr);
QT_WIDGETS_C_EXPORT QTapGesture* qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QGesture(QGesture* ptr);
QT_WIDGETS_C_EXPORT QTapGesture* qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapGesture_delete(QTapGesture* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTapGesture_metaObject(const QTapGesture* this_ptr);
QT_WIDGETS_C_EXPORT QTapGesture* qt_widgets_c_QTapGesture_new_no_args();
QT_WIDGETS_C_EXPORT QTapGesture* qt_widgets_c_QTapGesture_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapGesture_position_to_output(const QTapGesture* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTapGesture_qt_metacall(QTapGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTapGesture_qt_metacast(QTapGesture* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapGesture_setPosition(QTapGesture* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTapGesture_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QTAPGESTURE_H
