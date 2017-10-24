#ifndef QT_WIDGETS_C_QBOXLAYOUT_H
#define QT_WIDGETS_C_QBOXLAYOUT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QBoxLayout* qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QBoxLayout* qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QBoxLayout* qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QBoxLayout* qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QBoxLayout* qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QBoxLayout_G_static_cast_QLayoutItem_ptr(QBoxLayout* ptr);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QBoxLayout_G_static_cast_QLayout_ptr(QBoxLayout* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QBoxLayout_G_static_cast_QObject_ptr(QBoxLayout* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addItem(QBoxLayout* this_ptr, QLayoutItem* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addLayout_layout(QBoxLayout* this_ptr, QLayout* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addLayout_layout_stretch(QBoxLayout* this_ptr, QLayout* layout, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addSpacerItem(QBoxLayout* this_ptr, QSpacerItem* spacerItem);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addSpacing(QBoxLayout* this_ptr, int size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addStretch_no_args(QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addStretch_stretch(QBoxLayout* this_ptr, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_addStrut(QBoxLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QBoxLayout_count(const QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_delete(QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT QBoxLayout::Direction qt_widgets_c_QBoxLayout_direction(const QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QBoxLayout_hasHeightForWidth(const QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QBoxLayout_heightForWidth(const QBoxLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_insertItem(QBoxLayout* this_ptr, int index, QLayoutItem* arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_insertLayout_index_layout(QBoxLayout* this_ptr, int index, QLayout* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_insertLayout_index_layout_stretch(QBoxLayout* this_ptr, int index, QLayout* layout, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_insertSpacerItem(QBoxLayout* this_ptr, int index, QSpacerItem* spacerItem);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_insertSpacing(QBoxLayout* this_ptr, int index, int size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_insertStretch_index(QBoxLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_insertStretch_index_stretch(QBoxLayout* this_ptr, int index, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_invalidate(QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QBoxLayout_itemAt(const QBoxLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_maximumSize_to_output(const QBoxLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QBoxLayout_metaObject(const QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QBoxLayout_minimumHeightForWidth(const QBoxLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_minimumSize_to_output(const QBoxLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QBoxLayout* qt_widgets_c_QBoxLayout_new_arg1(QBoxLayout::Direction arg1);
QT_WIDGETS_C_EXPORT QBoxLayout* qt_widgets_c_QBoxLayout_new_arg1_parent(QBoxLayout::Direction arg1, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QBoxLayout_qt_metacall(QBoxLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QBoxLayout_qt_metacast(QBoxLayout* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_setDirection(QBoxLayout* this_ptr, QBoxLayout::Direction arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_setGeometry(QBoxLayout* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_setSpacing(QBoxLayout* this_ptr, int spacing);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_setStretch(QBoxLayout* this_ptr, int index, int stretch);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QBoxLayout_setStretchFactor_l_stretch(QBoxLayout* this_ptr, QLayout* l, int stretch);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QBoxLayout_setStretchFactor_w_stretch(QBoxLayout* this_ptr, QWidget* w, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_sizeHint_to_output(const QBoxLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QBoxLayout_spacing(const QBoxLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QBoxLayout_stretch(const QBoxLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QBoxLayout_takeAt(QBoxLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QBoxLayout_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QBOXLAYOUT_H
