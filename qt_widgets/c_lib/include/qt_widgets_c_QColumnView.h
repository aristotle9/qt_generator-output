#ifndef QT_WIDGETS_C_QCOLUMNVIEW_H
#define QT_WIDGETS_C_QCOLUMNVIEW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QColumnView_G_static_cast_QAbstractItemView_ptr(QColumnView* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QColumnView_G_static_cast_QAbstractScrollArea_ptr(QColumnView* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QColumnView_G_static_cast_QFrame_ptr(QColumnView* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QColumnView_G_static_cast_QObject_ptr(QColumnView* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QColumnView_G_static_cast_QPaintDevice_ptr(QColumnView* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QColumnView_G_static_cast_QWidget_ptr(QColumnView* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_columnWidths_to_output(const QColumnView* this_ptr, QList< int >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_delete(QColumnView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_indexAt_to_output(const QColumnView* this_ptr, const QPoint* point, QModelIndex* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QColumnView_metaObject(const QColumnView* this_ptr);
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_new_no_args();
QT_WIDGETS_C_EXPORT QColumnView* qt_widgets_c_QColumnView_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QColumnView_previewWidget(const QColumnView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QColumnView_qt_metacall(QColumnView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QColumnView_qt_metacast(QColumnView* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QColumnView_resizeGripsVisible(const QColumnView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_scrollTo_index(QColumnView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_scrollTo_index_hint(QColumnView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_selectAll(QColumnView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_setColumnWidths(QColumnView* this_ptr, const QList< int >* list);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_setModel(QColumnView* this_ptr, QAbstractItemModel* model);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_setPreviewWidget(QColumnView* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_setResizeGripsVisible(QColumnView* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_setRootIndex(QColumnView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_setSelectionModel(QColumnView* this_ptr, QItemSelectionModel* selectionModel);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_sizeHint_to_output(const QColumnView* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColumnView_visualRect_to_output(const QColumnView* this_ptr, const QModelIndex* index, QRect* output);

} // extern "C"

#endif // QT_WIDGETS_C_QCOLUMNVIEW_H
