#ifndef QT_WIDGETS_C_QLISTVIEW_H
#define QT_WIDGETS_C_QLISTVIEW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QListView_G_static_cast_QAbstractItemView_ptr(QListView* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QListView_G_static_cast_QAbstractScrollArea_ptr(QListView* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QListView_G_static_cast_QFrame_ptr(QListView* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QListView_G_static_cast_QObject_ptr(QListView* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QListView_G_static_cast_QPaintDevice_ptr(QListView* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QListView_G_static_cast_QWidget_ptr(QListView* ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListView_batchSize(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_clearPropertyFlags(QListView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_delete(QListView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_doItemsLayout(QListView* this_ptr);
QT_WIDGETS_C_EXPORT QListView::Flow qt_widgets_c_QListView_flow(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_gridSize_to_output(const QListView* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_indexAt_to_output(const QListView* this_ptr, const QPoint* p, QModelIndex* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListView_isRowHidden(const QListView* this_ptr, int row);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListView_isSelectionRectVisible(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListView_isWrapping(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT QListView::LayoutMode qt_widgets_c_QListView_layoutMode(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QListView_metaObject(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListView_modelColumn(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT QListView::Movement qt_widgets_c_QListView_movement(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_new_no_args();
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QListView_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListView_qt_metacall(QListView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QListView_qt_metacast(QListView* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_reset(QListView* this_ptr);
QT_WIDGETS_C_EXPORT QListView::ResizeMode qt_widgets_c_QListView_resizeMode(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_scrollTo_index(QListView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_scrollTo_index_hint(QListView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setBatchSize(QListView* this_ptr, int batchSize);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setFlow(QListView* this_ptr, QListView::Flow flow);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setGridSize(QListView* this_ptr, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setLayoutMode(QListView* this_ptr, QListView::LayoutMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setModelColumn(QListView* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setMovement(QListView* this_ptr, QListView::Movement movement);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setResizeMode(QListView* this_ptr, QListView::ResizeMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setRootIndex(QListView* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setRowHidden(QListView* this_ptr, int row, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setSelectionRectVisible(QListView* this_ptr, bool show);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setSpacing(QListView* this_ptr, int space);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setUniformItemSizes(QListView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setViewMode(QListView* this_ptr, QListView::ViewMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setWordWrap(QListView* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_setWrapping(QListView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListView_spacing(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListView_uniformItemSizes(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT QListView::ViewMode qt_widgets_c_QListView_viewMode(const QListView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListView_visualRect_to_output(const QListView* this_ptr, const QModelIndex* index, QRect* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListView_wordWrap(const QListView* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QLISTVIEW_H
