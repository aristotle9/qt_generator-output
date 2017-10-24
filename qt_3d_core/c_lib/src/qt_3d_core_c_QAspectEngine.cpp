#include "qt_3d_core_c_QAspectEngine.h"

QObject* qt_3d_core_c_QAspectEngine_G_static_cast_QObject_ptr(Qt3DCore::QAspectEngine* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QAspectEngine* qt_3d_core_c_QAspectEngine_G_static_cast_Qt3DCore_QAspectEngine_ptr(QObject* ptr) {
  return static_cast<Qt3DCore::QAspectEngine*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_aspects_to_output(const Qt3DCore::QAspectEngine* this_ptr, QVector< Qt3DCore::QAbstractAspect* >* output) {
  new(output) QVector< Qt3DCore::QAbstractAspect* >(this_ptr->aspects());
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_delete(Qt3DCore::QAspectEngine* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_executeCommand_to_output(Qt3DCore::QAspectEngine* this_ptr, const QString* command, QVariant* output) {
  new(output) QVariant(this_ptr->executeCommand(*command));
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QAspectEngine_metaObject(const Qt3DCore::QAspectEngine* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QAspectEngine* qt_3d_core_c_Qt3DCore_QAspectEngine_new_no_args() {
  return new Qt3DCore::QAspectEngine();
}

Qt3DCore::QAspectEngine* qt_3d_core_c_Qt3DCore_QAspectEngine_new_parent(QObject* parent) {
  return new Qt3DCore::QAspectEngine(parent);
}

int qt_3d_core_c_Qt3DCore_QAspectEngine_qt_metacall(Qt3DCore::QAspectEngine* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_core_c_Qt3DCore_QAspectEngine_qt_metacast(Qt3DCore::QAspectEngine* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_registerAspect_aspect(Qt3DCore::QAspectEngine* this_ptr, Qt3DCore::QAbstractAspect* aspect) {
  this_ptr->registerAspect(aspect);
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_registerAspect_name(Qt3DCore::QAspectEngine* this_ptr, const QString* name) {
  this_ptr->registerAspect(*name);
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_rootEntity_to_output(const Qt3DCore::QAspectEngine* this_ptr, QSharedPointer< Qt3DCore::QEntity >* output) {
  new(output) QSharedPointer< Qt3DCore::QEntity >(this_ptr->rootEntity());
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_setRootEntity(Qt3DCore::QAspectEngine* this_ptr, const QSharedPointer< Qt3DCore::QEntity >* root) {
  this_ptr->setRootEntity(*root);
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QAspectEngine::trUtf8(s, c, n));
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QAspectEngine::tr(s, c, n));
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_unregisterAspect_aspect(Qt3DCore::QAspectEngine* this_ptr, Qt3DCore::QAbstractAspect* aspect) {
  this_ptr->unregisterAspect(aspect);
}

void qt_3d_core_c_Qt3DCore_QAspectEngine_unregisterAspect_name(Qt3DCore::QAspectEngine* this_ptr, const QString* name) {
  this_ptr->unregisterAspect(*name);
}

