#include "qt_widgets_c_QUndoCommand.h"

void qt_widgets_c_QUndoCommand_actionText_to_output(const QUndoCommand* this_ptr, QString* output) {
  new(output) QString(this_ptr->actionText());
}

const QUndoCommand* qt_widgets_c_QUndoCommand_child(const QUndoCommand* this_ptr, int index) {
  return this_ptr->child(index);
}

int qt_widgets_c_QUndoCommand_childCount(const QUndoCommand* this_ptr) {
  return this_ptr->childCount();
}

void qt_widgets_c_QUndoCommand_delete(QUndoCommand* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QUndoCommand_id(const QUndoCommand* this_ptr) {
  return this_ptr->id();
}

bool qt_widgets_c_QUndoCommand_isObsolete(const QUndoCommand* this_ptr) {
  return this_ptr->isObsolete();
}

bool qt_widgets_c_QUndoCommand_mergeWith(QUndoCommand* this_ptr, const QUndoCommand* other) {
  return this_ptr->mergeWith(other);
}

QUndoCommand* qt_widgets_c_QUndoCommand_new_no_args() {
  return new QUndoCommand();
}

QUndoCommand* qt_widgets_c_QUndoCommand_new_parent(QUndoCommand* parent) {
  return new QUndoCommand(parent);
}

QUndoCommand* qt_widgets_c_QUndoCommand_new_text(const QString* text) {
  return new QUndoCommand(*text);
}

QUndoCommand* qt_widgets_c_QUndoCommand_new_text_parent(const QString* text, QUndoCommand* parent) {
  return new QUndoCommand(*text, parent);
}

void qt_widgets_c_QUndoCommand_redo(QUndoCommand* this_ptr) {
  this_ptr->redo();
}

void qt_widgets_c_QUndoCommand_setObsolete(QUndoCommand* this_ptr, bool obsolete) {
  this_ptr->setObsolete(obsolete);
}

void qt_widgets_c_QUndoCommand_setText(QUndoCommand* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QUndoCommand_text_to_output(const QUndoCommand* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QUndoCommand_undo(QUndoCommand* this_ptr) {
  this_ptr->undo();
}

