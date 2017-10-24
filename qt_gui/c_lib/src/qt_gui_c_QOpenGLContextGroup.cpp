#include "qt_gui_c_QOpenGLContextGroup.h"

QObject* qt_gui_c_QOpenGLContextGroup_G_static_cast_QObject_ptr(QOpenGLContextGroup* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLContextGroup* qt_gui_c_QOpenGLContextGroup_G_static_cast_QOpenGLContextGroup_ptr(QObject* ptr) {
  return static_cast<QOpenGLContextGroup*>(ptr);
}

QOpenGLContextGroup* qt_gui_c_QOpenGLContextGroup_currentContextGroup() {
  return QOpenGLContextGroup::currentContextGroup();
}

void qt_gui_c_QOpenGLContextGroup_delete(QOpenGLContextGroup* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QOpenGLContextGroup_metaObject(const QOpenGLContextGroup* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QOpenGLContextGroup_qt_metacall(QOpenGLContextGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLContextGroup_qt_metacast(QOpenGLContextGroup* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QOpenGLContextGroup_shares_to_output(const QOpenGLContextGroup* this_ptr, QList< QOpenGLContext* >* output) {
  new(output) QList< QOpenGLContext* >(this_ptr->shares());
}

void qt_gui_c_QOpenGLContextGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLContextGroup::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLContextGroup_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLContextGroup::tr(s, c, n));
}

