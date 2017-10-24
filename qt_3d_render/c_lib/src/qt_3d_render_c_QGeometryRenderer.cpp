#include "qt_3d_render_c_QGeometryRenderer.h"

QObject* qt_3d_render_c_QGeometryRenderer_G_static_cast_QObject_ptr(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_delete(Qt3DRender::QGeometryRenderer* this_ptr) {
  delete this_ptr;
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_firstInstance(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->firstInstance();
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_firstVertex(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->firstVertex();
}

Qt3DRender::QGeometry* qt_3d_render_c_Qt3DRender_QGeometryRenderer_geometry(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->geometry();
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_geometryFactory_to_output(const Qt3DRender::QGeometryRenderer* this_ptr, QSharedPointer< Qt3DRender::QGeometryFactory >* output) {
  new(output) QSharedPointer< Qt3DRender::QGeometryFactory >(this_ptr->geometryFactory());
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_indexOffset(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->indexOffset();
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_instanceCount(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->instanceCount();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QGeometryRenderer_metaObject(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QGeometryRenderer* qt_3d_render_c_Qt3DRender_QGeometryRenderer_new_no_args() {
  return new Qt3DRender::QGeometryRenderer();
}

Qt3DRender::QGeometryRenderer* qt_3d_render_c_Qt3DRender_QGeometryRenderer_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QGeometryRenderer(parent);
}

bool qt_3d_render_c_Qt3DRender_QGeometryRenderer_primitiveRestartEnabled(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->primitiveRestartEnabled();
}

Qt3DRender::QGeometryRenderer::PrimitiveType qt_3d_render_c_Qt3DRender_QGeometryRenderer_primitiveType(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->primitiveType();
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_qt_metacall(Qt3DRender::QGeometryRenderer* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QGeometryRenderer_qt_metacast(Qt3DRender::QGeometryRenderer* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_restartIndexValue(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->restartIndexValue();
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setFirstInstance(Qt3DRender::QGeometryRenderer* this_ptr, int firstInstance) {
  this_ptr->setFirstInstance(firstInstance);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setFirstVertex(Qt3DRender::QGeometryRenderer* this_ptr, int firstVertex) {
  this_ptr->setFirstVertex(firstVertex);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setGeometry(Qt3DRender::QGeometryRenderer* this_ptr, Qt3DRender::QGeometry* geometry) {
  this_ptr->setGeometry(geometry);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setGeometryFactory(Qt3DRender::QGeometryRenderer* this_ptr, const QSharedPointer< Qt3DRender::QGeometryFactory >* factory) {
  this_ptr->setGeometryFactory(*factory);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setIndexOffset(Qt3DRender::QGeometryRenderer* this_ptr, int indexOffset) {
  this_ptr->setIndexOffset(indexOffset);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setInstanceCount(Qt3DRender::QGeometryRenderer* this_ptr, int instanceCount) {
  this_ptr->setInstanceCount(instanceCount);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setPrimitiveRestartEnabled(Qt3DRender::QGeometryRenderer* this_ptr, bool enabled) {
  this_ptr->setPrimitiveRestartEnabled(enabled);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setPrimitiveType(Qt3DRender::QGeometryRenderer* this_ptr, Qt3DRender::QGeometryRenderer::PrimitiveType primitiveType) {
  this_ptr->setPrimitiveType(primitiveType);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setRestartIndexValue(Qt3DRender::QGeometryRenderer* this_ptr, int index) {
  this_ptr->setRestartIndexValue(index);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setVertexCount(Qt3DRender::QGeometryRenderer* this_ptr, int vertexCount) {
  this_ptr->setVertexCount(vertexCount);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setVerticesPerPatch(Qt3DRender::QGeometryRenderer* this_ptr, int verticesPerPatch) {
  this_ptr->setVerticesPerPatch(verticesPerPatch);
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QGeometryRenderer::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QGeometryRenderer_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QGeometryRenderer::tr(s, c, n));
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_vertexCount(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->vertexCount();
}

int qt_3d_render_c_Qt3DRender_QGeometryRenderer_verticesPerPatch(const Qt3DRender::QGeometryRenderer* this_ptr) {
  return this_ptr->verticesPerPatch();
}

