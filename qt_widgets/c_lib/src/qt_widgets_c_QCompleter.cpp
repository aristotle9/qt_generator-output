#include "qt_widgets_c_QCompleter.h"

QCompleter* qt_widgets_c_QCompleter_G_static_cast_QCompleter_ptr(QObject* ptr) {
  return static_cast<QCompleter*>(ptr);
}

QObject* qt_widgets_c_QCompleter_G_static_cast_QObject_ptr(QCompleter* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QCompleter_complete_no_args(QCompleter* this_ptr) {
  this_ptr->complete();
}

void qt_widgets_c_QCompleter_complete_rect(QCompleter* this_ptr, const QRect* rect) {
  this_ptr->complete(*rect);
}

int qt_widgets_c_QCompleter_completionColumn(const QCompleter* this_ptr) {
  return this_ptr->completionColumn();
}

int qt_widgets_c_QCompleter_completionCount(const QCompleter* this_ptr) {
  return this_ptr->completionCount();
}

QCompleter::CompletionMode qt_widgets_c_QCompleter_completionMode(const QCompleter* this_ptr) {
  return this_ptr->completionMode();
}

QAbstractItemModel* qt_widgets_c_QCompleter_completionModel(const QCompleter* this_ptr) {
  return this_ptr->completionModel();
}

void qt_widgets_c_QCompleter_completionPrefix_to_output(const QCompleter* this_ptr, QString* output) {
  new(output) QString(this_ptr->completionPrefix());
}

int qt_widgets_c_QCompleter_completionRole(const QCompleter* this_ptr) {
  return this_ptr->completionRole();
}

void qt_widgets_c_QCompleter_currentCompletion_to_output(const QCompleter* this_ptr, QString* output) {
  new(output) QString(this_ptr->currentCompletion());
}

void qt_widgets_c_QCompleter_currentIndex_to_output(const QCompleter* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->currentIndex());
}

int qt_widgets_c_QCompleter_currentRow(const QCompleter* this_ptr) {
  return this_ptr->currentRow();
}

void qt_widgets_c_QCompleter_delete(QCompleter* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QCompleter_maxVisibleItems(const QCompleter* this_ptr) {
  return this_ptr->maxVisibleItems();
}

const QMetaObject* qt_widgets_c_QCompleter_metaObject(const QCompleter* this_ptr) {
  return this_ptr->metaObject();
}

QAbstractItemModel* qt_widgets_c_QCompleter_model(const QCompleter* this_ptr) {
  return this_ptr->model();
}

QCompleter::ModelSorting qt_widgets_c_QCompleter_modelSorting(const QCompleter* this_ptr) {
  return this_ptr->modelSorting();
}

QCompleter* qt_widgets_c_QCompleter_new_completions(const QStringList* completions) {
  return new QCompleter(*completions);
}

QCompleter* qt_widgets_c_QCompleter_new_completions_parent(const QStringList* completions, QObject* parent) {
  return new QCompleter(*completions, parent);
}

QCompleter* qt_widgets_c_QCompleter_new_model(QAbstractItemModel* model) {
  return new QCompleter(model);
}

QCompleter* qt_widgets_c_QCompleter_new_model_parent(QAbstractItemModel* model, QObject* parent) {
  return new QCompleter(model, parent);
}

QCompleter* qt_widgets_c_QCompleter_new_no_args() {
  return new QCompleter();
}

QCompleter* qt_widgets_c_QCompleter_new_parent(QObject* parent) {
  return new QCompleter(parent);
}

void qt_widgets_c_QCompleter_pathFromIndex_to_output(const QCompleter* this_ptr, const QModelIndex* index, QString* output) {
  new(output) QString(this_ptr->pathFromIndex(*index));
}

QAbstractItemView* qt_widgets_c_QCompleter_popup(const QCompleter* this_ptr) {
  return this_ptr->popup();
}

int qt_widgets_c_QCompleter_qt_metacall(QCompleter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QCompleter_qt_metacast(QCompleter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QCompleter_setCaseSensitivity(QCompleter* this_ptr, const Qt::CaseSensitivity* caseSensitivity) {
  this_ptr->setCaseSensitivity(*caseSensitivity);
}

void qt_widgets_c_QCompleter_setCompletionColumn(QCompleter* this_ptr, int column) {
  this_ptr->setCompletionColumn(column);
}

void qt_widgets_c_QCompleter_setCompletionMode(QCompleter* this_ptr, QCompleter::CompletionMode mode) {
  this_ptr->setCompletionMode(mode);
}

void qt_widgets_c_QCompleter_setCompletionPrefix(QCompleter* this_ptr, const QString* prefix) {
  this_ptr->setCompletionPrefix(*prefix);
}

void qt_widgets_c_QCompleter_setCompletionRole(QCompleter* this_ptr, int role) {
  this_ptr->setCompletionRole(role);
}

bool qt_widgets_c_QCompleter_setCurrentRow(QCompleter* this_ptr, int row) {
  return this_ptr->setCurrentRow(row);
}

void qt_widgets_c_QCompleter_setMaxVisibleItems(QCompleter* this_ptr, int maxItems) {
  this_ptr->setMaxVisibleItems(maxItems);
}

void qt_widgets_c_QCompleter_setModel(QCompleter* this_ptr, QAbstractItemModel* c) {
  this_ptr->setModel(c);
}

void qt_widgets_c_QCompleter_setModelSorting(QCompleter* this_ptr, QCompleter::ModelSorting sorting) {
  this_ptr->setModelSorting(sorting);
}

void qt_widgets_c_QCompleter_setPopup(QCompleter* this_ptr, QAbstractItemView* popup) {
  this_ptr->setPopup(popup);
}

void qt_widgets_c_QCompleter_setWidget(QCompleter* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

void qt_widgets_c_QCompleter_setWrapAround(QCompleter* this_ptr, bool wrap) {
  this_ptr->setWrapAround(wrap);
}

void qt_widgets_c_QCompleter_splitPath_to_output(const QCompleter* this_ptr, const QString* path, QStringList* output) {
  new(output) QStringList(this_ptr->splitPath(*path));
}

void qt_widgets_c_QCompleter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCompleter::trUtf8(s, c, n));
}

void qt_widgets_c_QCompleter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCompleter::tr(s, c, n));
}

QWidget* qt_widgets_c_QCompleter_widget(const QCompleter* this_ptr) {
  return this_ptr->widget();
}

bool qt_widgets_c_QCompleter_wrapAround(const QCompleter* this_ptr) {
  return this_ptr->wrapAround();
}

