#ifndef QT_WIDGETS_C_QHEADERVIEW_H
#define QT_WIDGETS_C_QHEADERVIEW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QHeaderView_G_static_cast_QAbstractItemView_ptr(QHeaderView* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QHeaderView_G_static_cast_QAbstractScrollArea_ptr(QHeaderView* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QHeaderView_G_static_cast_QFrame_ptr(QHeaderView* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QHeaderView_G_static_cast_QObject_ptr(QHeaderView* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QHeaderView_G_static_cast_QPaintDevice_ptr(QHeaderView* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QHeaderView_G_static_cast_QWidget_ptr(QHeaderView* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_cascadingSectionResizes(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_count(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_defaultSectionSize(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_delete(QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_doItemsLayout(QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_headerDataChanged(QHeaderView* this_ptr, const Qt::Orientation* orientation, int logicalFirst, int logicalLast);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_hiddenSectionCount(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_hideSection(QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_highlightSections(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_isSectionHidden(const QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_isSortIndicatorShown(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_length(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_logicalIndex(const QHeaderView* this_ptr, int visualIndex);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_logicalIndexAt_pos(const QHeaderView* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_logicalIndexAt_position(const QHeaderView* this_ptr, int position);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_logicalIndexAt_x_y(const QHeaderView* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_maximumSectionSize(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QHeaderView_metaObject(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_minimumSectionSize(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_moveSection(QHeaderView* this_ptr, int from, int to);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_new_orientation(const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT QHeaderView* qt_widgets_c_QHeaderView_new_orientation_parent(const Qt::Orientation* orientation, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_offset(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_qt_metacall(QHeaderView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QHeaderView_qt_metacast(QHeaderView* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_reset(QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_resetDefaultSectionSize(QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_resizeContentsPrecision(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_resizeSection(QHeaderView* this_ptr, int logicalIndex, int size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_resizeSections(QHeaderView* this_ptr, const QHeaderView::ResizeMode* mode);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_restoreState(QHeaderView* this_ptr, const QByteArray* state);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_saveState_to_output(const QHeaderView* this_ptr, QByteArray* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_sectionPosition(const QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT QHeaderView::ResizeMode qt_widgets_c_QHeaderView_sectionResizeMode(const QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_sectionSize(const QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_sectionSizeHint(const QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_sectionViewportPosition(const QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_sectionsClickable(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_sectionsHidden(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_sectionsMovable(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_sectionsMoved(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setCascadingSectionResizes(QHeaderView* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setDefaultSectionSize(QHeaderView* this_ptr, int size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setHighlightSections(QHeaderView* this_ptr, bool highlight);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setMaximumSectionSize(QHeaderView* this_ptr, int size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setMinimumSectionSize(QHeaderView* this_ptr, int size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setModel(QHeaderView* this_ptr, QAbstractItemModel* model);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setOffset(QHeaderView* this_ptr, int offset);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setOffsetToLastSection(QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setOffsetToSectionPosition(QHeaderView* this_ptr, int visualIndex);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setResizeContentsPrecision(QHeaderView* this_ptr, int precision);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setSectionHidden(QHeaderView* this_ptr, int logicalIndex, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setSectionResizeMode_logicalIndex_mode(QHeaderView* this_ptr, int logicalIndex, QHeaderView::ResizeMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setSectionResizeMode_mode(QHeaderView* this_ptr, QHeaderView::ResizeMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setSectionsClickable(QHeaderView* this_ptr, bool clickable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setSectionsMovable(QHeaderView* this_ptr, bool movable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setSortIndicator(QHeaderView* this_ptr, int logicalIndex, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setSortIndicatorShown(QHeaderView* this_ptr, bool show);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setStretchLastSection(QHeaderView* this_ptr, bool stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_setVisible(QHeaderView* this_ptr, bool v);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_showSection(QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_sizeHint_to_output(const QHeaderView* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_sortIndicatorSection(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QHeaderView_stretchLastSection(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_stretchSectionCount(const QHeaderView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_swapSections(QHeaderView* this_ptr, int first, int second);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QHeaderView_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_visualIndex(const QHeaderView* this_ptr, int logicalIndex);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QHeaderView_visualIndexAt(const QHeaderView* this_ptr, int position);

} // extern "C"

#endif // QT_WIDGETS_C_QHEADERVIEW_H
