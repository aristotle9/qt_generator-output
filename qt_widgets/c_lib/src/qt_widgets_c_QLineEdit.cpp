#include "qt_widgets_c_QLineEdit.h"

QLineEdit* qt_widgets_c_QLineEdit_G_dynamic_cast_QLineEdit_ptr(QWidget* ptr) {
  return dynamic_cast<QLineEdit*>(ptr);
}

QLineEdit* qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QObject(QObject* ptr) {
  return static_cast<QLineEdit*>(ptr);
}

QLineEdit* qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QLineEdit*>(ptr);
}

QLineEdit* qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QWidget(QWidget* ptr) {
  return static_cast<QLineEdit*>(ptr);
}

QObject* qt_widgets_c_QLineEdit_G_static_cast_QObject_ptr(QLineEdit* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QLineEdit_G_static_cast_QPaintDevice_ptr(QLineEdit* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QLineEdit_G_static_cast_QWidget_ptr(QLineEdit* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QLineEdit_addAction_action_position(QLineEdit* this_ptr, QAction* action, QLineEdit::ActionPosition position) {
  this_ptr->addAction(action, position);
}

QAction* qt_widgets_c_QLineEdit_addAction_icon_position(QLineEdit* this_ptr, const QIcon* icon, QLineEdit::ActionPosition position) {
  return this_ptr->addAction(*icon, position);
}

void qt_widgets_c_QLineEdit_backspace(QLineEdit* this_ptr) {
  this_ptr->backspace();
}

void qt_widgets_c_QLineEdit_clear(QLineEdit* this_ptr) {
  this_ptr->clear();
}

QCompleter* qt_widgets_c_QLineEdit_completer(const QLineEdit* this_ptr) {
  return this_ptr->completer();
}

void qt_widgets_c_QLineEdit_copy(const QLineEdit* this_ptr) {
  this_ptr->copy();
}

QMenu* qt_widgets_c_QLineEdit_createStandardContextMenu(QLineEdit* this_ptr) {
  return this_ptr->createStandardContextMenu();
}

void qt_widgets_c_QLineEdit_cursorBackward_mark(QLineEdit* this_ptr, bool mark) {
  this_ptr->cursorBackward(mark);
}

void qt_widgets_c_QLineEdit_cursorBackward_mark_steps(QLineEdit* this_ptr, bool mark, int steps) {
  this_ptr->cursorBackward(mark, steps);
}

void qt_widgets_c_QLineEdit_cursorForward_mark(QLineEdit* this_ptr, bool mark) {
  this_ptr->cursorForward(mark);
}

void qt_widgets_c_QLineEdit_cursorForward_mark_steps(QLineEdit* this_ptr, bool mark, int steps) {
  this_ptr->cursorForward(mark, steps);
}

int qt_widgets_c_QLineEdit_cursorPosition(const QLineEdit* this_ptr) {
  return this_ptr->cursorPosition();
}

int qt_widgets_c_QLineEdit_cursorPositionAt(QLineEdit* this_ptr, const QPoint* pos) {
  return this_ptr->cursorPositionAt(*pos);
}

void qt_widgets_c_QLineEdit_cursorWordBackward(QLineEdit* this_ptr, bool mark) {
  this_ptr->cursorWordBackward(mark);
}

void qt_widgets_c_QLineEdit_cursorWordForward(QLineEdit* this_ptr, bool mark) {
  this_ptr->cursorWordForward(mark);
}

void qt_widgets_c_QLineEdit_cut(QLineEdit* this_ptr) {
  this_ptr->cut();
}

void qt_widgets_c_QLineEdit_del(QLineEdit* this_ptr) {
  this_ptr->del();
}

void qt_widgets_c_QLineEdit_delete(QLineEdit* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QLineEdit_deselect(QLineEdit* this_ptr) {
  this_ptr->deselect();
}

void qt_widgets_c_QLineEdit_displayText_to_output(const QLineEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->displayText());
}

bool qt_widgets_c_QLineEdit_dragEnabled(const QLineEdit* this_ptr) {
  return this_ptr->dragEnabled();
}

QLineEdit::EchoMode qt_widgets_c_QLineEdit_echoMode(const QLineEdit* this_ptr) {
  return this_ptr->echoMode();
}

void qt_widgets_c_QLineEdit_end(QLineEdit* this_ptr, bool mark) {
  this_ptr->end(mark);
}

bool qt_widgets_c_QLineEdit_event(QLineEdit* this_ptr, QEvent* arg1) {
  return this_ptr->event(arg1);
}

void qt_widgets_c_QLineEdit_getTextMargins(const QLineEdit* this_ptr, int* left, int* top, int* right, int* bottom) {
  this_ptr->getTextMargins(left, top, right, bottom);
}

bool qt_widgets_c_QLineEdit_hasAcceptableInput(const QLineEdit* this_ptr) {
  return this_ptr->hasAcceptableInput();
}

bool qt_widgets_c_QLineEdit_hasFrame(const QLineEdit* this_ptr) {
  return this_ptr->hasFrame();
}

bool qt_widgets_c_QLineEdit_hasSelectedText(const QLineEdit* this_ptr) {
  return this_ptr->hasSelectedText();
}

void qt_widgets_c_QLineEdit_home(QLineEdit* this_ptr, bool mark) {
  this_ptr->home(mark);
}

void qt_widgets_c_QLineEdit_inputMask_to_output(const QLineEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->inputMask());
}

void qt_widgets_c_QLineEdit_inputMethodQuery_to_output_arg1(const QLineEdit* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*arg1));
}

void qt_widgets_c_QLineEdit_inputMethodQuery_to_output_property_argument(const QLineEdit* this_ptr, const Qt::InputMethodQuery* property, const QVariant* argument, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*property, *argument));
}

void qt_widgets_c_QLineEdit_insert(QLineEdit* this_ptr, const QString* arg1) {
  this_ptr->insert(*arg1);
}

bool qt_widgets_c_QLineEdit_isClearButtonEnabled(const QLineEdit* this_ptr) {
  return this_ptr->isClearButtonEnabled();
}

bool qt_widgets_c_QLineEdit_isModified(const QLineEdit* this_ptr) {
  return this_ptr->isModified();
}

bool qt_widgets_c_QLineEdit_isReadOnly(const QLineEdit* this_ptr) {
  return this_ptr->isReadOnly();
}

bool qt_widgets_c_QLineEdit_isRedoAvailable(const QLineEdit* this_ptr) {
  return this_ptr->isRedoAvailable();
}

bool qt_widgets_c_QLineEdit_isUndoAvailable(const QLineEdit* this_ptr) {
  return this_ptr->isUndoAvailable();
}

int qt_widgets_c_QLineEdit_maxLength(const QLineEdit* this_ptr) {
  return this_ptr->maxLength();
}

const QMetaObject* qt_widgets_c_QLineEdit_metaObject(const QLineEdit* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QLineEdit_minimumSizeHint_to_output(const QLineEdit* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QLineEdit* qt_widgets_c_QLineEdit_new_arg1(const QString* arg1) {
  return new QLineEdit(*arg1);
}

QLineEdit* qt_widgets_c_QLineEdit_new_arg1_parent(const QString* arg1, QWidget* parent) {
  return new QLineEdit(*arg1, parent);
}

QLineEdit* qt_widgets_c_QLineEdit_new_no_args() {
  return new QLineEdit();
}

QLineEdit* qt_widgets_c_QLineEdit_new_parent(QWidget* parent) {
  return new QLineEdit(parent);
}

void qt_widgets_c_QLineEdit_paste(QLineEdit* this_ptr) {
  this_ptr->paste();
}

void qt_widgets_c_QLineEdit_placeholderText_to_output(const QLineEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->placeholderText());
}

int qt_widgets_c_QLineEdit_qt_metacall(QLineEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QLineEdit_qt_metacast(QLineEdit* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QLineEdit_redo(QLineEdit* this_ptr) {
  this_ptr->redo();
}

void qt_widgets_c_QLineEdit_selectAll(QLineEdit* this_ptr) {
  this_ptr->selectAll();
}

void qt_widgets_c_QLineEdit_selectedText_to_output(const QLineEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->selectedText());
}

int qt_widgets_c_QLineEdit_selectionStart(const QLineEdit* this_ptr) {
  return this_ptr->selectionStart();
}

void qt_widgets_c_QLineEdit_setClearButtonEnabled(QLineEdit* this_ptr, bool enable) {
  this_ptr->setClearButtonEnabled(enable);
}

void qt_widgets_c_QLineEdit_setCompleter(QLineEdit* this_ptr, QCompleter* completer) {
  this_ptr->setCompleter(completer);
}

void qt_widgets_c_QLineEdit_setCursorMoveStyle(QLineEdit* this_ptr, const Qt::CursorMoveStyle* style) {
  this_ptr->setCursorMoveStyle(*style);
}

void qt_widgets_c_QLineEdit_setCursorPosition(QLineEdit* this_ptr, int arg1) {
  this_ptr->setCursorPosition(arg1);
}

void qt_widgets_c_QLineEdit_setDragEnabled(QLineEdit* this_ptr, bool b) {
  this_ptr->setDragEnabled(b);
}

void qt_widgets_c_QLineEdit_setEchoMode(QLineEdit* this_ptr, QLineEdit::EchoMode arg1) {
  this_ptr->setEchoMode(arg1);
}

void qt_widgets_c_QLineEdit_setFrame(QLineEdit* this_ptr, bool arg1) {
  this_ptr->setFrame(arg1);
}

void qt_widgets_c_QLineEdit_setInputMask(QLineEdit* this_ptr, const QString* inputMask) {
  this_ptr->setInputMask(*inputMask);
}

void qt_widgets_c_QLineEdit_setMaxLength(QLineEdit* this_ptr, int arg1) {
  this_ptr->setMaxLength(arg1);
}

void qt_widgets_c_QLineEdit_setModified(QLineEdit* this_ptr, bool arg1) {
  this_ptr->setModified(arg1);
}

void qt_widgets_c_QLineEdit_setPlaceholderText(QLineEdit* this_ptr, const QString* arg1) {
  this_ptr->setPlaceholderText(*arg1);
}

void qt_widgets_c_QLineEdit_setReadOnly(QLineEdit* this_ptr, bool arg1) {
  this_ptr->setReadOnly(arg1);
}

void qt_widgets_c_QLineEdit_setSelection(QLineEdit* this_ptr, int arg1, int arg2) {
  this_ptr->setSelection(arg1, arg2);
}

void qt_widgets_c_QLineEdit_setText(QLineEdit* this_ptr, const QString* arg1) {
  this_ptr->setText(*arg1);
}

void qt_widgets_c_QLineEdit_setTextMargins_left_top_right_bottom(QLineEdit* this_ptr, int left, int top, int right, int bottom) {
  this_ptr->setTextMargins(left, top, right, bottom);
}

void qt_widgets_c_QLineEdit_setTextMargins_margins(QLineEdit* this_ptr, const QMargins* margins) {
  this_ptr->setTextMargins(*margins);
}

void qt_widgets_c_QLineEdit_setValidator(QLineEdit* this_ptr, const QValidator* arg1) {
  this_ptr->setValidator(arg1);
}

void qt_widgets_c_QLineEdit_sizeHint_to_output(const QLineEdit* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QLineEdit_textMargins_to_output(const QLineEdit* this_ptr, QMargins* output) {
  new(output) QMargins(this_ptr->textMargins());
}

void qt_widgets_c_QLineEdit_text_to_output(const QLineEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QLineEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLineEdit::trUtf8(s, c, n));
}

void qt_widgets_c_QLineEdit_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLineEdit::tr(s, c, n));
}

void qt_widgets_c_QLineEdit_undo(QLineEdit* this_ptr) {
  this_ptr->undo();
}

const QValidator* qt_widgets_c_QLineEdit_validator(const QLineEdit* this_ptr) {
  return this_ptr->validator();
}

