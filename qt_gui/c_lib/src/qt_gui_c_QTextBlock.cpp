#include "qt_gui_c_QTextBlock.h"

void qt_gui_c_QTextBlock_begin_to_output(const QTextBlock* this_ptr, QTextBlock::iterator* output) {
  new(output) QTextBlock::iterator(this_ptr->begin());
}

int qt_gui_c_QTextBlock_blockFormatIndex(const QTextBlock* this_ptr) {
  return this_ptr->blockFormatIndex();
}

void qt_gui_c_QTextBlock_blockFormat_to_output(const QTextBlock* this_ptr, QTextBlockFormat* output) {
  new(output) QTextBlockFormat(this_ptr->blockFormat());
}

int qt_gui_c_QTextBlock_blockNumber(const QTextBlock* this_ptr) {
  return this_ptr->blockNumber();
}

int qt_gui_c_QTextBlock_charFormatIndex(const QTextBlock* this_ptr) {
  return this_ptr->charFormatIndex();
}

void qt_gui_c_QTextBlock_charFormat_to_output(const QTextBlock* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->charFormat());
}

void qt_gui_c_QTextBlock_clearLayout(QTextBlock* this_ptr) {
  this_ptr->clearLayout();
}

void qt_gui_c_QTextBlock_constructor_no_args(QTextBlock* output) {
  new(output) QTextBlock();
}

void qt_gui_c_QTextBlock_constructor_o(const QTextBlock* o, QTextBlock* output) {
  new(output) QTextBlock(*o);
}

bool qt_gui_c_QTextBlock_contains(const QTextBlock* this_ptr, int position) {
  return this_ptr->contains(position);
}

void qt_gui_c_QTextBlock_destructor(QTextBlock* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

const QTextDocument* qt_gui_c_QTextBlock_document(const QTextBlock* this_ptr) {
  return this_ptr->document();
}

void qt_gui_c_QTextBlock_end_to_output(const QTextBlock* this_ptr, QTextBlock::iterator* output) {
  new(output) QTextBlock::iterator(this_ptr->end());
}

int qt_gui_c_QTextBlock_firstLineNumber(const QTextBlock* this_ptr) {
  return this_ptr->firstLineNumber();
}

int qt_gui_c_QTextBlock_fragmentIndex(const QTextBlock* this_ptr) {
  return this_ptr->fragmentIndex();
}

bool qt_gui_c_QTextBlock_isValid(const QTextBlock* this_ptr) {
  return this_ptr->isValid();
}

bool qt_gui_c_QTextBlock_isVisible(const QTextBlock* this_ptr) {
  return this_ptr->isVisible();
}

bool qt_gui_c_QTextBlock_iterator_atEnd(const QTextBlock::iterator* this_ptr) {
  return this_ptr->atEnd();
}

void qt_gui_c_QTextBlock_iterator_constructor_no_args(QTextBlock::iterator* output) {
  new(output) QTextBlock::iterator();
}

void qt_gui_c_QTextBlock_iterator_constructor_o(const QTextBlock::iterator* o, QTextBlock::iterator* output) {
  new(output) QTextBlock::iterator(*o);
}

void qt_gui_c_QTextBlock_iterator_destructor(QTextBlock::iterator* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextBlock_iterator_fragment_to_output(const QTextBlock::iterator* this_ptr, QTextFragment* output) {
  new(output) QTextFragment(this_ptr->fragment());
}

QTextBlock::iterator* qt_gui_c_QTextBlock_iterator_operator_dec(QTextBlock::iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_gui_c_QTextBlock_iterator_operator_dec_postfix_to_output(QTextBlock::iterator* this_ptr, int arg1, QTextBlock::iterator* output) {
  new(output) QTextBlock::iterator(this_ptr->operator--(arg1));
}

bool qt_gui_c_QTextBlock_iterator_operator_eq(const QTextBlock::iterator* this_ptr, const QTextBlock::iterator* o) {
  return this_ptr->operator==(*o);
}

QTextBlock::iterator* qt_gui_c_QTextBlock_iterator_operator_inc(QTextBlock::iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_gui_c_QTextBlock_iterator_operator_inc_postfix_to_output(QTextBlock::iterator* this_ptr, int arg1, QTextBlock::iterator* output) {
  new(output) QTextBlock::iterator(this_ptr->operator++(arg1));
}

bool qt_gui_c_QTextBlock_iterator_operator_neq(const QTextBlock::iterator* this_ptr, const QTextBlock::iterator* o) {
  return this_ptr->operator!=(*o);
}

QTextLayout* qt_gui_c_QTextBlock_layout(const QTextBlock* this_ptr) {
  return this_ptr->layout();
}

int qt_gui_c_QTextBlock_length(const QTextBlock* this_ptr) {
  return this_ptr->length();
}

int qt_gui_c_QTextBlock_lineCount(const QTextBlock* this_ptr) {
  return this_ptr->lineCount();
}

void qt_gui_c_QTextBlock_next_to_output(const QTextBlock* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->next());
}

QTextBlock* qt_gui_c_QTextBlock_operator_assign(QTextBlock* this_ptr, const QTextBlock* o) {
  return &this_ptr->operator=(*o);
}

bool qt_gui_c_QTextBlock_operator_eq(const QTextBlock* this_ptr, const QTextBlock* o) {
  return this_ptr->operator==(*o);
}

bool qt_gui_c_QTextBlock_operator_lt(const QTextBlock* this_ptr, const QTextBlock* o) {
  return this_ptr->operator<(*o);
}

bool qt_gui_c_QTextBlock_operator_neq(const QTextBlock* this_ptr, const QTextBlock* o) {
  return this_ptr->operator!=(*o);
}

int qt_gui_c_QTextBlock_position(const QTextBlock* this_ptr) {
  return this_ptr->position();
}

void qt_gui_c_QTextBlock_previous_to_output(const QTextBlock* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->previous());
}

int qt_gui_c_QTextBlock_revision(const QTextBlock* this_ptr) {
  return this_ptr->revision();
}

void qt_gui_c_QTextBlock_setLineCount(QTextBlock* this_ptr, int count) {
  this_ptr->setLineCount(count);
}

void qt_gui_c_QTextBlock_setRevision(QTextBlock* this_ptr, int rev) {
  this_ptr->setRevision(rev);
}

void qt_gui_c_QTextBlock_setUserData(QTextBlock* this_ptr, QTextBlockUserData* data) {
  this_ptr->setUserData(data);
}

void qt_gui_c_QTextBlock_setUserState(QTextBlock* this_ptr, int state) {
  this_ptr->setUserState(state);
}

void qt_gui_c_QTextBlock_setVisible(QTextBlock* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_gui_c_QTextBlock_textFormats_to_output(const QTextBlock* this_ptr, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(this_ptr->textFormats());
}

QTextList* qt_gui_c_QTextBlock_textList(const QTextBlock* this_ptr) {
  return this_ptr->textList();
}

void qt_gui_c_QTextBlock_text_to_output(const QTextBlock* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

QTextBlockUserData* qt_gui_c_QTextBlock_userData(const QTextBlock* this_ptr) {
  return this_ptr->userData();
}

int qt_gui_c_QTextBlock_userState(const QTextBlock* this_ptr) {
  return this_ptr->userState();
}

