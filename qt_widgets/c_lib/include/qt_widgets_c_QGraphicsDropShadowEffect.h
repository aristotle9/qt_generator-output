#ifndef QT_WIDGETS_C_QGRAPHICSDROPSHADOWEFFECT_H
#define QT_WIDGETS_C_QGRAPHICSDROPSHADOWEFFECT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_dynamic_cast_QGraphicsDropShadowEffect_ptr(QGraphicsEffect* ptr);
QT_WIDGETS_C_EXPORT QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QGraphicsEffect(QGraphicsEffect* ptr);
QT_WIDGETS_C_EXPORT QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsEffect* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsEffect_ptr(QGraphicsDropShadowEffect* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QObject_ptr(QGraphicsDropShadowEffect* ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsDropShadowEffect_blurRadius(const QGraphicsDropShadowEffect* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_boundingRectFor_to_output(const QGraphicsDropShadowEffect* this_ptr, const QRectF* rect, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_color_to_output(const QGraphicsDropShadowEffect* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_delete(QGraphicsDropShadowEffect* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsDropShadowEffect_metaObject(const QGraphicsDropShadowEffect* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsDropShadowEffect* qt_widgets_c_QGraphicsDropShadowEffect_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_offset_to_output(const QGraphicsDropShadowEffect* this_ptr, QPointF* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsDropShadowEffect_qt_metacall(QGraphicsDropShadowEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsDropShadowEffect_qt_metacast(QGraphicsDropShadowEffect* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_setBlurRadius(QGraphicsDropShadowEffect* this_ptr, double blurRadius);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_setColor(QGraphicsDropShadowEffect* this_ptr, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_setOffset_d(QGraphicsDropShadowEffect* this_ptr, double d);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_setOffset_dx_dy(QGraphicsDropShadowEffect* this_ptr, double dx, double dy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_setOffset_ofs(QGraphicsDropShadowEffect* this_ptr, const QPointF* ofs);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_setXOffset(QGraphicsDropShadowEffect* this_ptr, double dx);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_setYOffset(QGraphicsDropShadowEffect* this_ptr, double dy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsDropShadowEffect_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsDropShadowEffect_xOffset(const QGraphicsDropShadowEffect* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsDropShadowEffect_yOffset(const QGraphicsDropShadowEffect* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSDROPSHADOWEFFECT_H