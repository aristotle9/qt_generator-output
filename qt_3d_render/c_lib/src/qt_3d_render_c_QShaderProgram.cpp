#include "qt_3d_render_c_QShaderProgram.h"

QObject* qt_3d_render_c_QShaderProgram_G_static_cast_QObject_ptr(Qt3DRender::QShaderProgram* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QShaderProgram* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QShaderProgram* qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QShaderProgram*>(ptr);
}

Qt3DRender::QShaderProgram* qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QShaderProgram*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_computeShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->computeShaderCode());
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_delete(Qt3DRender::QShaderProgram* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_fragmentShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->fragmentShaderCode());
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_geometryShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->geometryShaderCode());
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_loadSource_to_output(const QUrl* sourceUrl, QByteArray* output) {
  new(output) QByteArray(Qt3DRender::QShaderProgram::loadSource(*sourceUrl));
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_log_to_output(const Qt3DRender::QShaderProgram* this_ptr, QString* output) {
  new(output) QString(this_ptr->log());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QShaderProgram_metaObject(const Qt3DRender::QShaderProgram* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QShaderProgram* qt_3d_render_c_Qt3DRender_QShaderProgram_new_no_args() {
  return new Qt3DRender::QShaderProgram();
}

Qt3DRender::QShaderProgram* qt_3d_render_c_Qt3DRender_QShaderProgram_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QShaderProgram(parent);
}

int qt_3d_render_c_Qt3DRender_QShaderProgram_qt_metacall(Qt3DRender::QShaderProgram* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QShaderProgram_qt_metacast(Qt3DRender::QShaderProgram* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_setComputeShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* computeShaderCode) {
  this_ptr->setComputeShaderCode(*computeShaderCode);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_setFragmentShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* fragmentShaderCode) {
  this_ptr->setFragmentShaderCode(*fragmentShaderCode);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_setGeometryShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* geometryShaderCode) {
  this_ptr->setGeometryShaderCode(*geometryShaderCode);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_setShaderCode(Qt3DRender::QShaderProgram* this_ptr, Qt3DRender::QShaderProgram::ShaderType type, const QByteArray* shaderCode) {
  this_ptr->setShaderCode(type, *shaderCode);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_setTessellationControlShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* tessellationControlShaderCode) {
  this_ptr->setTessellationControlShaderCode(*tessellationControlShaderCode);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_setTessellationEvaluationShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* tessellationEvaluationShaderCode) {
  this_ptr->setTessellationEvaluationShaderCode(*tessellationEvaluationShaderCode);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_setVertexShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* vertexShaderCode) {
  this_ptr->setVertexShaderCode(*vertexShaderCode);
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_shaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, Qt3DRender::QShaderProgram::ShaderType type, QByteArray* output) {
  new(output) QByteArray(this_ptr->shaderCode(type));
}

Qt3DRender::QShaderProgram::Status qt_3d_render_c_Qt3DRender_QShaderProgram_status(const Qt3DRender::QShaderProgram* this_ptr) {
  return this_ptr->status();
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_tessellationControlShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->tessellationControlShaderCode());
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_tessellationEvaluationShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->tessellationEvaluationShaderCode());
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QShaderProgram::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QShaderProgram::tr(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QShaderProgram_vertexShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->vertexShaderCode());
}

