#ifndef QT_WIDGETS_C_QFORMLAYOUT_H
#define QT_WIDGETS_C_QFORMLAYOUT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QFormLayout* qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QFormLayout* qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QFormLayout* qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QFormLayout* qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QFormLayout* qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QFormLayout_G_static_cast_QLayoutItem_ptr(QFormLayout* ptr);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QFormLayout_G_static_cast_QLayout_ptr(QFormLayout* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QFormLayout_G_static_cast_QObject_ptr(QFormLayout* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_TakeRowResult_destructor(QFormLayout::TakeRowResult* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QFormLayout_TakeRowResult_fieldItem(const QFormLayout::TakeRowResult* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QFormLayout_TakeRowResult_labelItem(const QFormLayout::TakeRowResult* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_TakeRowResult_set_fieldItem(QFormLayout::TakeRowResult* this_ptr, QLayoutItem* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_TakeRowResult_set_labelItem(QFormLayout::TakeRowResult* this_ptr, QLayoutItem* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_addItem(QFormLayout* this_ptr, QLayoutItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_addRow_QLayout(QFormLayout* this_ptr, QLayout* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_addRow_QString_QLayout(QFormLayout* this_ptr, const QString* labelText, QLayout* field);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_addRow_QString_QWidget(QFormLayout* this_ptr, const QString* labelText, QWidget* field);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_addRow_QWidget(QFormLayout* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_addRow_QWidget_QLayout(QFormLayout* this_ptr, QWidget* label, QLayout* field);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_addRow_QWidget_QWidget(QFormLayout* this_ptr, QWidget* label, QWidget* field);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFormLayout_count(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_delete(QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT QFormLayout::FieldGrowthPolicy qt_widgets_c_QFormLayout_fieldGrowthPolicy(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_getItemPosition(const QFormLayout* this_ptr, int index, int* rowPtr, QFormLayout::ItemRole* rolePtr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_getLayoutPosition(const QFormLayout* this_ptr, QLayout* layout, int* rowPtr, QFormLayout::ItemRole* rolePtr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_getWidgetPosition(const QFormLayout* this_ptr, QWidget* widget, int* rowPtr, QFormLayout::ItemRole* rolePtr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFormLayout_hasHeightForWidth(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFormLayout_heightForWidth(const QFormLayout* this_ptr, int width);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFormLayout_horizontalSpacing(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_insertRow_int_QLayout(QFormLayout* this_ptr, int row, QLayout* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_insertRow_int_QString_QLayout(QFormLayout* this_ptr, int row, const QString* labelText, QLayout* field);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_insertRow_int_QString_QWidget(QFormLayout* this_ptr, int row, const QString* labelText, QWidget* field);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_insertRow_int_QWidget(QFormLayout* this_ptr, int row, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_insertRow_int_QWidget_QLayout(QFormLayout* this_ptr, int row, QWidget* label, QLayout* field);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_insertRow_int_QWidget_QWidget(QFormLayout* this_ptr, int row, QWidget* label, QWidget* field);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_invalidate(QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QFormLayout_itemAt_index(const QFormLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QFormLayout_itemAt_row_role(const QFormLayout* this_ptr, int row, QFormLayout::ItemRole role);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QFormLayout_labelForField_QLayout(const QFormLayout* this_ptr, QLayout* field);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QFormLayout_labelForField_QWidget(const QFormLayout* this_ptr, QWidget* field);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QFormLayout_metaObject(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_minimumSize_to_output(const QFormLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QFormLayout* qt_widgets_c_QFormLayout_new_no_args();
QT_WIDGETS_C_EXPORT QFormLayout* qt_widgets_c_QFormLayout_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFormLayout_qt_metacall(QFormLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QFormLayout_qt_metacast(QFormLayout* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_removeRow_layout(QFormLayout* this_ptr, QLayout* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_removeRow_row(QFormLayout* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_removeRow_widget(QFormLayout* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFormLayout_rowCount(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT QFormLayout::RowWrapPolicy qt_widgets_c_QFormLayout_rowWrapPolicy(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setFieldGrowthPolicy(QFormLayout* this_ptr, QFormLayout::FieldGrowthPolicy policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setGeometry(QFormLayout* this_ptr, const QRect* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setHorizontalSpacing(QFormLayout* this_ptr, int spacing);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setItem(QFormLayout* this_ptr, int row, QFormLayout::ItemRole role, QLayoutItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setLayout(QFormLayout* this_ptr, int row, QFormLayout::ItemRole role, QLayout* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setRowWrapPolicy(QFormLayout* this_ptr, QFormLayout::RowWrapPolicy policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setSpacing(QFormLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setVerticalSpacing(QFormLayout* this_ptr, int spacing);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_setWidget(QFormLayout* this_ptr, int row, QFormLayout::ItemRole role, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_sizeHint_to_output(const QFormLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFormLayout_spacing(const QFormLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QFormLayout_takeAt(QFormLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_takeRow_to_output_layout(QFormLayout* this_ptr, QLayout* layout, QFormLayout::TakeRowResult* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_takeRow_to_output_row(QFormLayout* this_ptr, int row, QFormLayout::TakeRowResult* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_takeRow_to_output_widget(QFormLayout* this_ptr, QWidget* widget, QFormLayout::TakeRowResult* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFormLayout_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFormLayout_verticalSpacing(const QFormLayout* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QFORMLAYOUT_H
