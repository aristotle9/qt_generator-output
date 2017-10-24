#ifndef QT_WIDGETS_C_QSTYLEOPTIONGRAPHICSITEM_H
#define QT_WIDGETS_C_QSTYLEOPTIONGRAPHICSITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyleOptionGraphicsItem* qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOptionGraphicsItem_ptr(QStyleOption* ptr);
QT_WIDGETS_C_EXPORT QStyleOption* qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOption_ptr(QStyleOptionGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionGraphicsItem_delete(QStyleOptionGraphicsItem* this_ptr);
QT_WIDGETS_C_EXPORT const QRectF* qt_widgets_c_QStyleOptionGraphicsItem_exposedRect(const QStyleOptionGraphicsItem* this_ptr);
QT_WIDGETS_C_EXPORT QRectF* qt_widgets_c_QStyleOptionGraphicsItem_exposedRect_mut(QStyleOptionGraphicsItem* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QStyleOptionGraphicsItem_levelOfDetail(const QStyleOptionGraphicsItem* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QStyleOptionGraphicsItem_levelOfDetailFromTransform(const QTransform* worldTransform);
QT_WIDGETS_C_EXPORT const QMatrix* qt_widgets_c_QStyleOptionGraphicsItem_matrix(const QStyleOptionGraphicsItem* this_ptr);
QT_WIDGETS_C_EXPORT QMatrix* qt_widgets_c_QStyleOptionGraphicsItem_matrix_mut(QStyleOptionGraphicsItem* this_ptr);
QT_WIDGETS_C_EXPORT QStyleOptionGraphicsItem* qt_widgets_c_QStyleOptionGraphicsItem_new_no_args();
QT_WIDGETS_C_EXPORT QStyleOptionGraphicsItem* qt_widgets_c_QStyleOptionGraphicsItem_new_other(const QStyleOptionGraphicsItem* other);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionGraphicsItem_set_exposedRect(QStyleOptionGraphicsItem* this_ptr, const QRectF* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionGraphicsItem_set_levelOfDetail(QStyleOptionGraphicsItem* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleOptionGraphicsItem_set_matrix(QStyleOptionGraphicsItem* this_ptr, const QMatrix* value);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEOPTIONGRAPHICSITEM_H