#include "qt_gui_c_QSyntaxHighlighter.h"

QObject* qt_gui_c_QSyntaxHighlighter_G_static_cast_QObject_ptr(QSyntaxHighlighter* ptr) {
  return static_cast<QObject*>(ptr);
}

QSyntaxHighlighter* qt_gui_c_QSyntaxHighlighter_G_static_cast_QSyntaxHighlighter_ptr(QObject* ptr) {
  return static_cast<QSyntaxHighlighter*>(ptr);
}

void qt_gui_c_QSyntaxHighlighter_delete(QSyntaxHighlighter* this_ptr) {
  delete this_ptr;
}

QTextDocument* qt_gui_c_QSyntaxHighlighter_document(const QSyntaxHighlighter* this_ptr) {
  return this_ptr->document();
}

const QMetaObject* qt_gui_c_QSyntaxHighlighter_metaObject(const QSyntaxHighlighter* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QSyntaxHighlighter_qt_metacall(QSyntaxHighlighter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QSyntaxHighlighter_qt_metacast(QSyntaxHighlighter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QSyntaxHighlighter_rehighlight(QSyntaxHighlighter* this_ptr) {
  this_ptr->rehighlight();
}

void qt_gui_c_QSyntaxHighlighter_rehighlightBlock(QSyntaxHighlighter* this_ptr, const QTextBlock* block) {
  this_ptr->rehighlightBlock(*block);
}

void qt_gui_c_QSyntaxHighlighter_setDocument(QSyntaxHighlighter* this_ptr, QTextDocument* doc) {
  this_ptr->setDocument(doc);
}

void qt_gui_c_QSyntaxHighlighter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSyntaxHighlighter::trUtf8(s, c, n));
}

void qt_gui_c_QSyntaxHighlighter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSyntaxHighlighter::tr(s, c, n));
}

