#ifndef QT_WIDGETS_C_QGRAPHICSITEMANIMATION_H
#define QT_WIDGETS_C_QGRAPHICSITEMANIMATION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsItemAnimation* qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QGraphicsItemAnimation_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QObject_ptr(QGraphicsItemAnimation* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_clear(QGraphicsItemAnimation* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_delete(QGraphicsItemAnimation* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsItemAnimation_horizontalScaleAt(const QGraphicsItemAnimation* this_ptr, double step);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsItemAnimation_horizontalShearAt(const QGraphicsItemAnimation* this_ptr, double step);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsItemAnimation_item(const QGraphicsItemAnimation* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_matrixAt_to_output(const QGraphicsItemAnimation* this_ptr, double step, QMatrix* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsItemAnimation_metaObject(const QGraphicsItemAnimation* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsItemAnimation* qt_widgets_c_QGraphicsItemAnimation_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsItemAnimation* qt_widgets_c_QGraphicsItemAnimation_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_posAt_to_output(const QGraphicsItemAnimation* this_ptr, double step, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_posList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsItemAnimation_qt_metacall(QGraphicsItemAnimation* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsItemAnimation_qt_metacast(QGraphicsItemAnimation* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_reset(QGraphicsItemAnimation* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsItemAnimation_rotationAt(const QGraphicsItemAnimation* this_ptr, double step);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_rotationList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, double > >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_scaleList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setItem(QGraphicsItemAnimation* this_ptr, QGraphicsItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setPosAt(QGraphicsItemAnimation* this_ptr, double step, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setRotationAt(QGraphicsItemAnimation* this_ptr, double step, double angle);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setScaleAt(QGraphicsItemAnimation* this_ptr, double step, double sx, double sy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setShearAt(QGraphicsItemAnimation* this_ptr, double step, double sh, double sv);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setStep(QGraphicsItemAnimation* this_ptr, double x);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setTimeLine(QGraphicsItemAnimation* this_ptr, QTimeLine* timeLine);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_setTranslationAt(QGraphicsItemAnimation* this_ptr, double step, double dx, double dy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_shearList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output);
QT_WIDGETS_C_EXPORT QTimeLine* qt_widgets_c_QGraphicsItemAnimation_timeLine(const QGraphicsItemAnimation* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsItemAnimation_translationList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsItemAnimation_verticalScaleAt(const QGraphicsItemAnimation* this_ptr, double step);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsItemAnimation_verticalShearAt(const QGraphicsItemAnimation* this_ptr, double step);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsItemAnimation_xTranslationAt(const QGraphicsItemAnimation* this_ptr, double step);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsItemAnimation_yTranslationAt(const QGraphicsItemAnimation* this_ptr, double step);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSITEMANIMATION_H
