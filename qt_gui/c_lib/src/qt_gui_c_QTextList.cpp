#include "qt_gui_c_QTextList.h"

QTextList* qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextBlockGroup(QTextBlockGroup* ptr) {
  return dynamic_cast<QTextList*>(ptr);
}

QTextList* qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextObject(QTextObject* ptr) {
  return dynamic_cast<QTextList*>(ptr);
}

QObject* qt_gui_c_QTextList_G_static_cast_QObject_ptr(QTextList* ptr) {
  return static_cast<QObject*>(ptr);
}

QTextBlockGroup* qt_gui_c_QTextList_G_static_cast_QTextBlockGroup_ptr(QTextList* ptr) {
  return static_cast<QTextBlockGroup*>(ptr);
}

QTextList* qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QObject(QObject* ptr) {
  return static_cast<QTextList*>(ptr);
}

QTextList* qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextBlockGroup(QTextBlockGroup* ptr) {
  return static_cast<QTextList*>(ptr);
}

QTextList* qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextObject(QTextObject* ptr) {
  return static_cast<QTextList*>(ptr);
}

QTextObject* qt_gui_c_QTextList_G_static_cast_QTextObject_ptr(QTextList* ptr) {
  return static_cast<QTextObject*>(ptr);
}

void qt_gui_c_QTextList_add(QTextList* this_ptr, const QTextBlock* block) {
  this_ptr->add(*block);
}

int qt_gui_c_QTextList_count(const QTextList* this_ptr) {
  return this_ptr->count();
}

void qt_gui_c_QTextList_delete(QTextList* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QTextList_format_to_output(const QTextList* this_ptr, QTextListFormat* output) {
  new(output) QTextListFormat(this_ptr->format());
}

bool qt_gui_c_QTextList_isEmpty(const QTextList* this_ptr) {
  return this_ptr->isEmpty();
}

int qt_gui_c_QTextList_itemNumber(const QTextList* this_ptr, const QTextBlock* arg1) {
  return this_ptr->itemNumber(*arg1);
}

void qt_gui_c_QTextList_itemText_to_output(const QTextList* this_ptr, const QTextBlock* arg1, QString* output) {
  new(output) QString(this_ptr->itemText(*arg1));
}

void qt_gui_c_QTextList_item_to_output(const QTextList* this_ptr, int i, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->item(i));
}

const QMetaObject* qt_gui_c_QTextList_metaObject(const QTextList* this_ptr) {
  return this_ptr->metaObject();
}

QTextList* qt_gui_c_QTextList_new(QTextDocument* doc) {
  return new QTextList(doc);
}

int qt_gui_c_QTextList_qt_metacall(QTextList* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QTextList_qt_metacast(QTextList* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QTextList_remove(QTextList* this_ptr, const QTextBlock* arg1) {
  this_ptr->remove(*arg1);
}

void qt_gui_c_QTextList_removeItem(QTextList* this_ptr, int i) {
  this_ptr->removeItem(i);
}

void qt_gui_c_QTextList_setFormat(QTextList* this_ptr, const QTextListFormat* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QTextList_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextList::trUtf8(s, c, n));
}

void qt_gui_c_QTextList_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextList::tr(s, c, n));
}

