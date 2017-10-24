#ifndef QT_WIDGETS_C_QSPACERITEM_H
#define QT_WIDGETS_C_QSPACERITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QSpacerItem* qt_widgets_c_QSpacerItem_G_dynamic_cast_QSpacerItem_ptr(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QSpacerItem_G_static_cast_QLayoutItem_ptr(QSpacerItem* ptr);
QT_WIDGETS_C_EXPORT QSpacerItem* qt_widgets_c_QSpacerItem_G_static_cast_QSpacerItem_ptr(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_changeSize_w_h(QSpacerItem* this_ptr, int w, int h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_changeSize_w_h_hData(QSpacerItem* this_ptr, int w, int h, const QSizePolicy::Policy* hData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_changeSize_w_h_hData_vData(QSpacerItem* this_ptr, int w, int h, const QSizePolicy::Policy* hData, const QSizePolicy::Policy* vData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_delete(QSpacerItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_geometry_to_output(const QSpacerItem* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSpacerItem_isEmpty(const QSpacerItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_maximumSize_to_output(const QSpacerItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_minimumSize_to_output(const QSpacerItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QSpacerItem* qt_widgets_c_QSpacerItem_new_w_h(int w, int h);
QT_WIDGETS_C_EXPORT QSpacerItem* qt_widgets_c_QSpacerItem_new_w_h_hData(int w, int h, const QSizePolicy::Policy* hData);
QT_WIDGETS_C_EXPORT QSpacerItem* qt_widgets_c_QSpacerItem_new_w_h_hData_vData(int w, int h, const QSizePolicy::Policy* hData, const QSizePolicy::Policy* vData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_setGeometry(QSpacerItem* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_sizeHint_to_output(const QSpacerItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSpacerItem_sizePolicy_to_output(const QSpacerItem* this_ptr, QSizePolicy* output);
QT_WIDGETS_C_EXPORT QSpacerItem* qt_widgets_c_QSpacerItem_spacerItem(QSpacerItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSPACERITEM_H
