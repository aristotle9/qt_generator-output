#include "qt_3d_render_c_QSceneLoader.h"

QObject* qt_3d_render_c_QSceneLoader_G_static_cast_QObject_ptr(Qt3DRender::QSceneLoader* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QSceneLoader* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QSceneLoader* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QSceneLoader* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QSceneLoader*>(ptr);
}

Qt3DRender::QSceneLoader* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QSceneLoader*>(ptr);
}

Qt3DRender::QSceneLoader* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QSceneLoader*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_Qt3DRender_QSceneLoader_component(const Qt3DRender::QSceneLoader* this_ptr, const QString* entityName, Qt3DRender::QSceneLoader::ComponentType componentType) {
  return this_ptr->component(*entityName, componentType);
}

void qt_3d_render_c_Qt3DRender_QSceneLoader_delete(Qt3DRender::QSceneLoader* this_ptr) {
  delete this_ptr;
}

Qt3DCore::QEntity* qt_3d_render_c_Qt3DRender_QSceneLoader_entity(const Qt3DRender::QSceneLoader* this_ptr, const QString* entityName) {
  return this_ptr->entity(*entityName);
}

void qt_3d_render_c_Qt3DRender_QSceneLoader_entityNames_to_output(const Qt3DRender::QSceneLoader* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->entityNames());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QSceneLoader_metaObject(const Qt3DRender::QSceneLoader* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QSceneLoader* qt_3d_render_c_Qt3DRender_QSceneLoader_new_no_args() {
  return new Qt3DRender::QSceneLoader();
}

Qt3DRender::QSceneLoader* qt_3d_render_c_Qt3DRender_QSceneLoader_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QSceneLoader(parent);
}

int qt_3d_render_c_Qt3DRender_QSceneLoader_qt_metacall(Qt3DRender::QSceneLoader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QSceneLoader_qt_metacast(Qt3DRender::QSceneLoader* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QSceneLoader_setSource(Qt3DRender::QSceneLoader* this_ptr, const QUrl* arg) {
  this_ptr->setSource(*arg);
}

void qt_3d_render_c_Qt3DRender_QSceneLoader_setStatus(Qt3DRender::QSceneLoader* this_ptr, Qt3DRender::QSceneLoader::Status status) {
  this_ptr->setStatus(status);
}

void qt_3d_render_c_Qt3DRender_QSceneLoader_source_to_output(const Qt3DRender::QSceneLoader* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->source());
}

Qt3DRender::QSceneLoader::Status qt_3d_render_c_Qt3DRender_QSceneLoader_status(const Qt3DRender::QSceneLoader* this_ptr) {
  return this_ptr->status();
}

void qt_3d_render_c_Qt3DRender_QSceneLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QSceneLoader::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QSceneLoader_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QSceneLoader::tr(s, c, n));
}

