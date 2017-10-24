#include "qt_widgets_c_QHeaderView.h"

QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_dynamic_cast_QHeaderView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QHeaderView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QHeaderView_G_static_cast_QAbstractItemView_ptr(QHeaderView* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QHeaderView_G_static_cast_QAbstractScrollArea_ptr(QHeaderView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QHeaderView_G_static_cast_QFrame_ptr(QHeaderView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QObject(QObject* ptr) {
  return static_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QHeaderView*>(ptr);
}

QHeaderView* qt_widgets_c_QHeaderView_G_static_cast_QHeaderView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QHeaderView*>(ptr);
}

QObject* qt_widgets_c_QHeaderView_G_static_cast_QObject_ptr(QHeaderView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QHeaderView_G_static_cast_QPaintDevice_ptr(QHeaderView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QHeaderView_G_static_cast_QWidget_ptr(QHeaderView* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QHeaderView_cascadingSectionResizes(const QHeaderView* this_ptr) {
  return this_ptr->cascadingSectionResizes();
}

int qt_widgets_c_QHeaderView_count(const QHeaderView* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QHeaderView_defaultSectionSize(const QHeaderView* this_ptr) {
  return this_ptr->defaultSectionSize();
}

void qt_widgets_c_QHeaderView_delete(QHeaderView* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QHeaderView_doItemsLayout(QHeaderView* this_ptr) {
  this_ptr->doItemsLayout();
}

void qt_widgets_c_QHeaderView_headerDataChanged(QHeaderView* this_ptr, const Qt::Orientation* orientation, int logicalFirst, int logicalLast) {
  this_ptr->headerDataChanged(*orientation, logicalFirst, logicalLast);
}

int qt_widgets_c_QHeaderView_hiddenSectionCount(const QHeaderView* this_ptr) {
  return this_ptr->hiddenSectionCount();
}

void qt_widgets_c_QHeaderView_hideSection(QHeaderView* this_ptr, int logicalIndex) {
  this_ptr->hideSection(logicalIndex);
}

bool qt_widgets_c_QHeaderView_highlightSections(const QHeaderView* this_ptr) {
  return this_ptr->highlightSections();
}

bool qt_widgets_c_QHeaderView_isSectionHidden(const QHeaderView* this_ptr, int logicalIndex) {
  return this_ptr->isSectionHidden(logicalIndex);
}

bool qt_widgets_c_QHeaderView_isSortIndicatorShown(const QHeaderView* this_ptr) {
  return this_ptr->isSortIndicatorShown();
}

int qt_widgets_c_QHeaderView_length(const QHeaderView* this_ptr) {
  return this_ptr->length();
}

int qt_widgets_c_QHeaderView_logicalIndex(const QHeaderView* this_ptr, int visualIndex) {
  return this_ptr->logicalIndex(visualIndex);
}

int qt_widgets_c_QHeaderView_logicalIndexAt_pos(const QHeaderView* this_ptr, const QPoint* pos) {
  return this_ptr->logicalIndexAt(*pos);
}

int qt_widgets_c_QHeaderView_logicalIndexAt_position(const QHeaderView* this_ptr, int position) {
  return this_ptr->logicalIndexAt(position);
}

int qt_widgets_c_QHeaderView_logicalIndexAt_x_y(const QHeaderView* this_ptr, int x, int y) {
  return this_ptr->logicalIndexAt(x, y);
}

int qt_widgets_c_QHeaderView_maximumSectionSize(const QHeaderView* this_ptr) {
  return this_ptr->maximumSectionSize();
}

const QMetaObject* qt_widgets_c_QHeaderView_metaObject(const QHeaderView* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QHeaderView_minimumSectionSize(const QHeaderView* this_ptr) {
  return this_ptr->minimumSectionSize();
}

void qt_widgets_c_QHeaderView_moveSection(QHeaderView* this_ptr, int from, int to) {
  this_ptr->moveSection(from, to);
}

QHeaderView* qt_widgets_c_QHeaderView_new_orientation(const Qt::Orientation* orientation) {
  return new QHeaderView(*orientation);
}

QHeaderView* qt_widgets_c_QHeaderView_new_orientation_parent(const Qt::Orientation* orientation, QWidget* parent) {
  return new QHeaderView(*orientation, parent);
}

int qt_widgets_c_QHeaderView_offset(const QHeaderView* this_ptr) {
  return this_ptr->offset();
}

int qt_widgets_c_QHeaderView_qt_metacall(QHeaderView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QHeaderView_qt_metacast(QHeaderView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QHeaderView_reset(QHeaderView* this_ptr) {
  this_ptr->reset();
}

void qt_widgets_c_QHeaderView_resetDefaultSectionSize(QHeaderView* this_ptr) {
  this_ptr->resetDefaultSectionSize();
}

int qt_widgets_c_QHeaderView_resizeContentsPrecision(const QHeaderView* this_ptr) {
  return this_ptr->resizeContentsPrecision();
}

void qt_widgets_c_QHeaderView_resizeSection(QHeaderView* this_ptr, int logicalIndex, int size) {
  this_ptr->resizeSection(logicalIndex, size);
}

void qt_widgets_c_QHeaderView_resizeSections(QHeaderView* this_ptr, const QHeaderView::ResizeMode* mode) {
  this_ptr->resizeSections(*mode);
}

bool qt_widgets_c_QHeaderView_restoreState(QHeaderView* this_ptr, const QByteArray* state) {
  return this_ptr->restoreState(*state);
}

void qt_widgets_c_QHeaderView_saveState_to_output(const QHeaderView* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->saveState());
}

int qt_widgets_c_QHeaderView_sectionPosition(const QHeaderView* this_ptr, int logicalIndex) {
  return this_ptr->sectionPosition(logicalIndex);
}

QHeaderView::ResizeMode qt_widgets_c_QHeaderView_sectionResizeMode(const QHeaderView* this_ptr, int logicalIndex) {
  return this_ptr->sectionResizeMode(logicalIndex);
}

int qt_widgets_c_QHeaderView_sectionSize(const QHeaderView* this_ptr, int logicalIndex) {
  return this_ptr->sectionSize(logicalIndex);
}

int qt_widgets_c_QHeaderView_sectionSizeHint(const QHeaderView* this_ptr, int logicalIndex) {
  return this_ptr->sectionSizeHint(logicalIndex);
}

int qt_widgets_c_QHeaderView_sectionViewportPosition(const QHeaderView* this_ptr, int logicalIndex) {
  return this_ptr->sectionViewportPosition(logicalIndex);
}

bool qt_widgets_c_QHeaderView_sectionsClickable(const QHeaderView* this_ptr) {
  return this_ptr->sectionsClickable();
}

bool qt_widgets_c_QHeaderView_sectionsHidden(const QHeaderView* this_ptr) {
  return this_ptr->sectionsHidden();
}

bool qt_widgets_c_QHeaderView_sectionsMovable(const QHeaderView* this_ptr) {
  return this_ptr->sectionsMovable();
}

bool qt_widgets_c_QHeaderView_sectionsMoved(const QHeaderView* this_ptr) {
  return this_ptr->sectionsMoved();
}

void qt_widgets_c_QHeaderView_setCascadingSectionResizes(QHeaderView* this_ptr, bool enable) {
  this_ptr->setCascadingSectionResizes(enable);
}

void qt_widgets_c_QHeaderView_setDefaultSectionSize(QHeaderView* this_ptr, int size) {
  this_ptr->setDefaultSectionSize(size);
}

void qt_widgets_c_QHeaderView_setHighlightSections(QHeaderView* this_ptr, bool highlight) {
  this_ptr->setHighlightSections(highlight);
}

void qt_widgets_c_QHeaderView_setMaximumSectionSize(QHeaderView* this_ptr, int size) {
  this_ptr->setMaximumSectionSize(size);
}

void qt_widgets_c_QHeaderView_setMinimumSectionSize(QHeaderView* this_ptr, int size) {
  this_ptr->setMinimumSectionSize(size);
}

void qt_widgets_c_QHeaderView_setModel(QHeaderView* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_widgets_c_QHeaderView_setOffset(QHeaderView* this_ptr, int offset) {
  this_ptr->setOffset(offset);
}

void qt_widgets_c_QHeaderView_setOffsetToLastSection(QHeaderView* this_ptr) {
  this_ptr->setOffsetToLastSection();
}

void qt_widgets_c_QHeaderView_setOffsetToSectionPosition(QHeaderView* this_ptr, int visualIndex) {
  this_ptr->setOffsetToSectionPosition(visualIndex);
}

void qt_widgets_c_QHeaderView_setResizeContentsPrecision(QHeaderView* this_ptr, int precision) {
  this_ptr->setResizeContentsPrecision(precision);
}

void qt_widgets_c_QHeaderView_setSectionHidden(QHeaderView* this_ptr, int logicalIndex, bool hide) {
  this_ptr->setSectionHidden(logicalIndex, hide);
}

void qt_widgets_c_QHeaderView_setSectionResizeMode_logicalIndex_mode(QHeaderView* this_ptr, int logicalIndex, QHeaderView::ResizeMode mode) {
  this_ptr->setSectionResizeMode(logicalIndex, mode);
}

void qt_widgets_c_QHeaderView_setSectionResizeMode_mode(QHeaderView* this_ptr, QHeaderView::ResizeMode mode) {
  this_ptr->setSectionResizeMode(mode);
}

void qt_widgets_c_QHeaderView_setSectionsClickable(QHeaderView* this_ptr, bool clickable) {
  this_ptr->setSectionsClickable(clickable);
}

void qt_widgets_c_QHeaderView_setSectionsMovable(QHeaderView* this_ptr, bool movable) {
  this_ptr->setSectionsMovable(movable);
}

void qt_widgets_c_QHeaderView_setSortIndicator(QHeaderView* this_ptr, int logicalIndex, const Qt::SortOrder* order) {
  this_ptr->setSortIndicator(logicalIndex, *order);
}

void qt_widgets_c_QHeaderView_setSortIndicatorShown(QHeaderView* this_ptr, bool show) {
  this_ptr->setSortIndicatorShown(show);
}

void qt_widgets_c_QHeaderView_setStretchLastSection(QHeaderView* this_ptr, bool stretch) {
  this_ptr->setStretchLastSection(stretch);
}

void qt_widgets_c_QHeaderView_setVisible(QHeaderView* this_ptr, bool v) {
  this_ptr->setVisible(v);
}

void qt_widgets_c_QHeaderView_showSection(QHeaderView* this_ptr, int logicalIndex) {
  this_ptr->showSection(logicalIndex);
}

void qt_widgets_c_QHeaderView_sizeHint_to_output(const QHeaderView* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

int qt_widgets_c_QHeaderView_sortIndicatorSection(const QHeaderView* this_ptr) {
  return this_ptr->sortIndicatorSection();
}

bool qt_widgets_c_QHeaderView_stretchLastSection(const QHeaderView* this_ptr) {
  return this_ptr->stretchLastSection();
}

int qt_widgets_c_QHeaderView_stretchSectionCount(const QHeaderView* this_ptr) {
  return this_ptr->stretchSectionCount();
}

void qt_widgets_c_QHeaderView_swapSections(QHeaderView* this_ptr, int first, int second) {
  this_ptr->swapSections(first, second);
}

void qt_widgets_c_QHeaderView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QHeaderView::trUtf8(s, c, n));
}

void qt_widgets_c_QHeaderView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QHeaderView::tr(s, c, n));
}

int qt_widgets_c_QHeaderView_visualIndex(const QHeaderView* this_ptr, int logicalIndex) {
  return this_ptr->visualIndex(logicalIndex);
}

int qt_widgets_c_QHeaderView_visualIndexAt(const QHeaderView* this_ptr, int position) {
  return this_ptr->visualIndexAt(position);
}

