#include "qt_gui_c_QTextFrame.h"

QTextFrame* qt_gui_c_QTextFrame_G_dynamic_cast_QTextFrame_ptr(QTextObject* ptr) {
  return dynamic_cast<QTextFrame*>(ptr);
}

QObject* qt_gui_c_QTextFrame_G_static_cast_QObject_ptr(QTextFrame* ptr) {
  return static_cast<QObject*>(ptr);
}

QTextFrame* qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QObject(QObject* ptr) {
  return static_cast<QTextFrame*>(ptr);
}

QTextFrame* qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QTextObject(QTextObject* ptr) {
  return static_cast<QTextFrame*>(ptr);
}

QTextObject* qt_gui_c_QTextFrame_G_static_cast_QTextObject_ptr(QTextFrame* ptr) {
  return static_cast<QTextObject*>(ptr);
}

void qt_gui_c_QTextFrame_begin_to_output(const QTextFrame* this_ptr, QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator(this_ptr->begin());
}

void qt_gui_c_QTextFrame_childFrames_to_output(const QTextFrame* this_ptr, QList< QTextFrame* >* output) {
  new(output) QList< QTextFrame* >(this_ptr->childFrames());
}

void qt_gui_c_QTextFrame_delete(QTextFrame* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QTextFrame_end_to_output(const QTextFrame* this_ptr, QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator(this_ptr->end());
}

QTextCursor* qt_gui_c_QTextFrame_firstCursorPosition_as_ptr(const QTextFrame* this_ptr) {
  return new QTextCursor(this_ptr->firstCursorPosition());
}

int qt_gui_c_QTextFrame_firstPosition(const QTextFrame* this_ptr) {
  return this_ptr->firstPosition();
}

void qt_gui_c_QTextFrame_frameFormat_to_output(const QTextFrame* this_ptr, QTextFrameFormat* output) {
  new(output) QTextFrameFormat(this_ptr->frameFormat());
}

bool qt_gui_c_QTextFrame_iterator_atEnd(const QTextFrame::iterator* this_ptr) {
  return this_ptr->atEnd();
}

void qt_gui_c_QTextFrame_iterator_constructor_no_args(QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator();
}

void qt_gui_c_QTextFrame_iterator_constructor_o(const QTextFrame::iterator* o, QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator(*o);
}

void qt_gui_c_QTextFrame_iterator_currentBlock_to_output(const QTextFrame::iterator* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->currentBlock());
}

QTextFrame* qt_gui_c_QTextFrame_iterator_currentFrame(const QTextFrame::iterator* this_ptr) {
  return this_ptr->currentFrame();
}

void qt_gui_c_QTextFrame_iterator_destructor(QTextFrame::iterator* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

QTextFrame::iterator* qt_gui_c_QTextFrame_iterator_operator_assign(QTextFrame::iterator* this_ptr, const QTextFrame::iterator* o) {
  return &this_ptr->operator=(*o);
}

QTextFrame::iterator* qt_gui_c_QTextFrame_iterator_operator_dec(QTextFrame::iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_gui_c_QTextFrame_iterator_operator_dec_postfix_to_output(QTextFrame::iterator* this_ptr, int arg1, QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator(this_ptr->operator--(arg1));
}

bool qt_gui_c_QTextFrame_iterator_operator_eq(const QTextFrame::iterator* this_ptr, const QTextFrame::iterator* o) {
  return this_ptr->operator==(*o);
}

QTextFrame::iterator* qt_gui_c_QTextFrame_iterator_operator_inc(QTextFrame::iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_gui_c_QTextFrame_iterator_operator_inc_postfix_to_output(QTextFrame::iterator* this_ptr, int arg1, QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator(this_ptr->operator++(arg1));
}

bool qt_gui_c_QTextFrame_iterator_operator_neq(const QTextFrame::iterator* this_ptr, const QTextFrame::iterator* o) {
  return this_ptr->operator!=(*o);
}

QTextFrame* qt_gui_c_QTextFrame_iterator_parentFrame(const QTextFrame::iterator* this_ptr) {
  return this_ptr->parentFrame();
}

QTextCursor* qt_gui_c_QTextFrame_lastCursorPosition_as_ptr(const QTextFrame* this_ptr) {
  return new QTextCursor(this_ptr->lastCursorPosition());
}

int qt_gui_c_QTextFrame_lastPosition(const QTextFrame* this_ptr) {
  return this_ptr->lastPosition();
}

const QMetaObject* qt_gui_c_QTextFrame_metaObject(const QTextFrame* this_ptr) {
  return this_ptr->metaObject();
}

QTextFrame* qt_gui_c_QTextFrame_new(QTextDocument* doc) {
  return new QTextFrame(doc);
}

QTextFrame* qt_gui_c_QTextFrame_parentFrame(const QTextFrame* this_ptr) {
  return this_ptr->parentFrame();
}

int qt_gui_c_QTextFrame_qt_metacall(QTextFrame* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QTextFrame_qt_metacast(QTextFrame* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QTextFrame_setFrameFormat(QTextFrame* this_ptr, const QTextFrameFormat* format) {
  this_ptr->setFrameFormat(*format);
}

void qt_gui_c_QTextFrame_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextFrame::trUtf8(s, c, n));
}

void qt_gui_c_QTextFrame_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextFrame::tr(s, c, n));
}

