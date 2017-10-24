#ifndef QT_WIDGETS_C_QLAYOUTITEM_H
#define QT_WIDGETS_C_QLAYOUTITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayoutItem_delete(QLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayoutItem_geometry_to_output(const QLayoutItem* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLayoutItem_hasHeightForWidth(const QLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayoutItem_heightForWidth(const QLayoutItem* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayoutItem_invalidate(QLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLayoutItem_isEmpty(const QLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QLayoutItem_layout(QLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayoutItem_maximumSize_to_output(const QLayoutItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayoutItem_minimumHeightForWidth(const QLayoutItem* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayoutItem_minimumSize_to_output(const QLayoutItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayoutItem_setGeometry(QLayoutItem* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayoutItem_sizeHint_to_output(const QLayoutItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QSpacerItem* qt_widgets_c_QLayoutItem_spacerItem(QLayoutItem* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QLayoutItem_widget(QLayoutItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QLAYOUTITEM_H
