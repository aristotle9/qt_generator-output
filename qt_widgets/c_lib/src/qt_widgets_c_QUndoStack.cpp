#include "qt_widgets_c_QUndoStack.h"

QObject* qt_widgets_c_QUndoStack_G_static_cast_QObject_ptr(QUndoStack* ptr) {
  return static_cast<QObject*>(ptr);
}

QUndoStack* qt_widgets_c_QUndoStack_G_static_cast_QUndoStack_ptr(QObject* ptr) {
  return static_cast<QUndoStack*>(ptr);
}

void qt_widgets_c_QUndoStack_beginMacro(QUndoStack* this_ptr, const QString* text) {
  this_ptr->beginMacro(*text);
}

bool qt_widgets_c_QUndoStack_canRedo(const QUndoStack* this_ptr) {
  return this_ptr->canRedo();
}

bool qt_widgets_c_QUndoStack_canUndo(const QUndoStack* this_ptr) {
  return this_ptr->canUndo();
}

int qt_widgets_c_QUndoStack_cleanIndex(const QUndoStack* this_ptr) {
  return this_ptr->cleanIndex();
}

void qt_widgets_c_QUndoStack_clear(QUndoStack* this_ptr) {
  this_ptr->clear();
}

const QUndoCommand* qt_widgets_c_QUndoStack_command(const QUndoStack* this_ptr, int index) {
  return this_ptr->command(index);
}

int qt_widgets_c_QUndoStack_count(const QUndoStack* this_ptr) {
  return this_ptr->count();
}

QAction* qt_widgets_c_QUndoStack_createRedoAction_parent(const QUndoStack* this_ptr, QObject* parent) {
  return this_ptr->createRedoAction(parent);
}

QAction* qt_widgets_c_QUndoStack_createRedoAction_parent_prefix(const QUndoStack* this_ptr, QObject* parent, const QString* prefix) {
  return this_ptr->createRedoAction(parent, *prefix);
}

QAction* qt_widgets_c_QUndoStack_createUndoAction_parent(const QUndoStack* this_ptr, QObject* parent) {
  return this_ptr->createUndoAction(parent);
}

QAction* qt_widgets_c_QUndoStack_createUndoAction_parent_prefix(const QUndoStack* this_ptr, QObject* parent, const QString* prefix) {
  return this_ptr->createUndoAction(parent, *prefix);
}

void qt_widgets_c_QUndoStack_delete(QUndoStack* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QUndoStack_endMacro(QUndoStack* this_ptr) {
  this_ptr->endMacro();
}

int qt_widgets_c_QUndoStack_index(const QUndoStack* this_ptr) {
  return this_ptr->index();
}

bool qt_widgets_c_QUndoStack_isActive(const QUndoStack* this_ptr) {
  return this_ptr->isActive();
}

bool qt_widgets_c_QUndoStack_isClean(const QUndoStack* this_ptr) {
  return this_ptr->isClean();
}

const QMetaObject* qt_widgets_c_QUndoStack_metaObject(const QUndoStack* this_ptr) {
  return this_ptr->metaObject();
}

QUndoStack* qt_widgets_c_QUndoStack_new_no_args() {
  return new QUndoStack();
}

QUndoStack* qt_widgets_c_QUndoStack_new_parent(QObject* parent) {
  return new QUndoStack(parent);
}

void qt_widgets_c_QUndoStack_push(QUndoStack* this_ptr, QUndoCommand* cmd) {
  this_ptr->push(cmd);
}

int qt_widgets_c_QUndoStack_qt_metacall(QUndoStack* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QUndoStack_qt_metacast(QUndoStack* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QUndoStack_redo(QUndoStack* this_ptr) {
  this_ptr->redo();
}

void qt_widgets_c_QUndoStack_redoText_to_output(const QUndoStack* this_ptr, QString* output) {
  new(output) QString(this_ptr->redoText());
}

void qt_widgets_c_QUndoStack_resetClean(QUndoStack* this_ptr) {
  this_ptr->resetClean();
}

void qt_widgets_c_QUndoStack_setActive_active(QUndoStack* this_ptr, bool active) {
  this_ptr->setActive(active);
}

void qt_widgets_c_QUndoStack_setActive_no_args(QUndoStack* this_ptr) {
  this_ptr->setActive();
}

void qt_widgets_c_QUndoStack_setClean(QUndoStack* this_ptr) {
  this_ptr->setClean();
}

void qt_widgets_c_QUndoStack_setIndex(QUndoStack* this_ptr, int idx) {
  this_ptr->setIndex(idx);
}

void qt_widgets_c_QUndoStack_setUndoLimit(QUndoStack* this_ptr, int limit) {
  this_ptr->setUndoLimit(limit);
}

void qt_widgets_c_QUndoStack_text_to_output(const QUndoStack* this_ptr, int idx, QString* output) {
  new(output) QString(this_ptr->text(idx));
}

void qt_widgets_c_QUndoStack_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUndoStack::trUtf8(s, c, n));
}

void qt_widgets_c_QUndoStack_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUndoStack::tr(s, c, n));
}

void qt_widgets_c_QUndoStack_undo(QUndoStack* this_ptr) {
  this_ptr->undo();
}

int qt_widgets_c_QUndoStack_undoLimit(const QUndoStack* this_ptr) {
  return this_ptr->undoLimit();
}

void qt_widgets_c_QUndoStack_undoText_to_output(const QUndoStack* this_ptr, QString* output) {
  new(output) QString(this_ptr->undoText());
}

