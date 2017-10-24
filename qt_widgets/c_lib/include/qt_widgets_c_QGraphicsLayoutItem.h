#ifndef QT_WIDGETS_C_QGRAPHICSLAYOUTITEM_H
#define QT_WIDGETS_C_QGRAPHICSLAYOUTITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_contentsRect_to_output(const QGraphicsLayoutItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_delete(QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_effectiveSizeHint_to_output_which(const QGraphicsLayoutItem* this_ptr, const Qt::SizeHint* which, QSizeF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_effectiveSizeHint_to_output_which_constraint(const QGraphicsLayoutItem* this_ptr, const Qt::SizeHint* which, const QSizeF* constraint, QSizeF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_geometry_to_output(const QGraphicsLayoutItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_getContentsMargins(const QGraphicsLayoutItem* this_ptr, double* left, double* top, double* right, double* bottom);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsLayoutItem_graphicsItem(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsLayoutItem_isLayout(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsLayoutItem_maximumHeight(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_maximumSize_to_output(const QGraphicsLayoutItem* this_ptr, QSizeF* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsLayoutItem_maximumWidth(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsLayoutItem_minimumHeight(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_minimumSize_to_output(const QGraphicsLayoutItem* this_ptr, QSizeF* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsLayoutItem_minimumWidth(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsLayoutItem_ownedByLayout(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsLayoutItem* qt_widgets_c_QGraphicsLayoutItem_parentLayoutItem(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsLayoutItem_preferredHeight(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_preferredSize_to_output(const QGraphicsLayoutItem* this_ptr, QSizeF* output);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsLayoutItem_preferredWidth(const QGraphicsLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setGeometry(QGraphicsLayoutItem* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMaximumHeight(QGraphicsLayoutItem* this_ptr, double height);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMaximumSize_size(QGraphicsLayoutItem* this_ptr, const QSizeF* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMaximumSize_w_h(QGraphicsLayoutItem* this_ptr, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMaximumWidth(QGraphicsLayoutItem* this_ptr, double width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMinimumHeight(QGraphicsLayoutItem* this_ptr, double height);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMinimumSize_size(QGraphicsLayoutItem* this_ptr, const QSizeF* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMinimumSize_w_h(QGraphicsLayoutItem* this_ptr, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setMinimumWidth(QGraphicsLayoutItem* this_ptr, double width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setParentLayoutItem(QGraphicsLayoutItem* this_ptr, QGraphicsLayoutItem* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setPreferredHeight(QGraphicsLayoutItem* this_ptr, double height);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setPreferredSize_size(QGraphicsLayoutItem* this_ptr, const QSizeF* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setPreferredSize_w_h(QGraphicsLayoutItem* this_ptr, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setPreferredWidth(QGraphicsLayoutItem* this_ptr, double width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_hPolicy_vPolicy(QGraphicsLayoutItem* this_ptr, const QSizePolicy::Policy* hPolicy, const QSizePolicy::Policy* vPolicy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_hPolicy_vPolicy_controlType(QGraphicsLayoutItem* this_ptr, const QSizePolicy::Policy* hPolicy, const QSizePolicy::Policy* vPolicy, const QSizePolicy::ControlType* controlType);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_policy(QGraphicsLayoutItem* this_ptr, const QSizePolicy* policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_sizePolicy_to_output(const QGraphicsLayoutItem* this_ptr, QSizePolicy* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsLayoutItem_updateGeometry(QGraphicsLayoutItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSLAYOUTITEM_H
