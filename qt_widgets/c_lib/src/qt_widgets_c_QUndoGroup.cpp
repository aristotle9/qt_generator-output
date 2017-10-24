#include "qt_widgets_c_QUndoGroup.h"

QObject* qt_widgets_c_QUndoGroup_G_static_cast_QObject_ptr(QUndoGroup* ptr) {
  return static_cast<QObject*>(ptr);
}

QUndoGroup* qt_widgets_c_QUndoGroup_G_static_cast_QUndoGroup_ptr(QObject* ptr) {
  return static_cast<QUndoGroup*>(ptr);
}

QUndoStack* qt_widgets_c_QUndoGroup_activeStack(const QUndoGroup* this_ptr) {
  return this_ptr->activeStack();
}

void qt_widgets_c_QUndoGroup_addStack(QUndoGroup* this_ptr, QUndoStack* stack) {
  this_ptr->addStack(stack);
}

bool qt_widgets_c_QUndoGroup_canRedo(const QUndoGroup* this_ptr) {
  return this_ptr->canRedo();
}

bool qt_widgets_c_QUndoGroup_canUndo(const QUndoGroup* this_ptr) {
  return this_ptr->canUndo();
}

QAction* qt_widgets_c_QUndoGroup_createRedoAction_parent(const QUndoGroup* this_ptr, QObject* parent) {
  return this_ptr->createRedoAction(parent);
}

QAction* qt_widgets_c_QUndoGroup_createRedoAction_parent_prefix(const QUndoGroup* this_ptr, QObject* parent, const QString* prefix) {
  return this_ptr->createRedoAction(parent, *prefix);
}

QAction* qt_widgets_c_QUndoGroup_createUndoAction_parent(const QUndoGroup* this_ptr, QObject* parent) {
  return this_ptr->createUndoAction(parent);
}

QAction* qt_widgets_c_QUndoGroup_createUndoAction_parent_prefix(const QUndoGroup* this_ptr, QObject* parent, const QString* prefix) {
  return this_ptr->createUndoAction(parent, *prefix);
}

void qt_widgets_c_QUndoGroup_delete(QUndoGroup* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QUndoGroup_isClean(const QUndoGroup* this_ptr) {
  return this_ptr->isClean();
}

const QMetaObject* qt_widgets_c_QUndoGroup_metaObject(const QUndoGroup* this_ptr) {
  return this_ptr->metaObject();
}

QUndoGroup* qt_widgets_c_QUndoGroup_new_no_args() {
  return new QUndoGroup();
}

QUndoGroup* qt_widgets_c_QUndoGroup_new_parent(QObject* parent) {
  return new QUndoGroup(parent);
}

int qt_widgets_c_QUndoGroup_qt_metacall(QUndoGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QUndoGroup_qt_metacast(QUndoGroup* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QUndoGroup_redo(QUndoGroup* this_ptr) {
  this_ptr->redo();
}

void qt_widgets_c_QUndoGroup_redoText_to_output(const QUndoGroup* this_ptr, QString* output) {
  new(output) QString(this_ptr->redoText());
}

void qt_widgets_c_QUndoGroup_removeStack(QUndoGroup* this_ptr, QUndoStack* stack) {
  this_ptr->removeStack(stack);
}

void qt_widgets_c_QUndoGroup_setActiveStack(QUndoGroup* this_ptr, QUndoStack* stack) {
  this_ptr->setActiveStack(stack);
}

void qt_widgets_c_QUndoGroup_stacks_to_output(const QUndoGroup* this_ptr, QList< QUndoStack* >* output) {
  new(output) QList< QUndoStack* >(this_ptr->stacks());
}

void qt_widgets_c_QUndoGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUndoGroup::trUtf8(s, c, n));
}

void qt_widgets_c_QUndoGroup_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUndoGroup::tr(s, c, n));
}

void qt_widgets_c_QUndoGroup_undo(QUndoGroup* this_ptr) {
  this_ptr->undo();
}

void qt_widgets_c_QUndoGroup_undoText_to_output(const QUndoGroup* this_ptr, QString* output) {
  new(output) QString(this_ptr->undoText());
}

