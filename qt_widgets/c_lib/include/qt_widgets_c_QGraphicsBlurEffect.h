#ifndef QT_WIDGETS_C_QGRAPHICSBLUREFFECT_H
#define QT_WIDGETS_C_QGRAPHICSBLUREFFECT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_G_dynamic_cast_QGraphicsBlurEffect_ptr(QGraphicsEffect* ptr);
QT_WIDGETS_C_EXPORT QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QGraphicsEffect(QGraphicsEffect* ptr);
QT_WIDGETS_C_EXPORT QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsEffect* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsEffect_ptr(QGraphicsBlurEffect* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QObject_ptr(QGraphicsBlurEffect* ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QGraphicsBlurEffect_blurHints(const QGraphicsBlurEffect* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsBlurEffect_blurRadius(const QGraphicsBlurEffect* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsBlurEffect_boundingRectFor_to_output(const QGraphicsBlurEffect* this_ptr, const QRectF* rect, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsBlurEffect_delete(QGraphicsBlurEffect* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsBlurEffect_metaObject(const QGraphicsBlurEffect* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsBlurEffect* qt_widgets_c_QGraphicsBlurEffect_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsBlurEffect_qt_metacall(QGraphicsBlurEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsBlurEffect_qt_metacast(QGraphicsBlurEffect* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsBlurEffect_setBlurHints(QGraphicsBlurEffect* this_ptr, unsigned int hints);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsBlurEffect_setBlurRadius(QGraphicsBlurEffect* this_ptr, double blurRadius);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsBlurEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsBlurEffect_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSBLUREFFECT_H
