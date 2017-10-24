#include "qt_3d_core_c_QBackendNode.h"

Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNodeMapper_create(const Qt3DCore::QBackendNodeMapper* this_ptr, const QSharedPointer< Qt3DCore::QNodeCreatedChangeBase >* change) {
  return this_ptr->create(*change);
}

void qt_3d_core_c_Qt3DCore_QBackendNodeMapper_delete(Qt3DCore::QBackendNodeMapper* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QBackendNodeMapper_destroy(const Qt3DCore::QBackendNodeMapper* this_ptr, const Qt3DCore::QNodeId* id) {
  this_ptr->destroy(*id);
}

Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNodeMapper_get(const Qt3DCore::QBackendNodeMapper* this_ptr, const Qt3DCore::QNodeId* id) {
  return this_ptr->get(*id);
}

void qt_3d_core_c_Qt3DCore_QBackendNode_delete(Qt3DCore::QBackendNode* this_ptr) {
  delete this_ptr;
}

bool qt_3d_core_c_Qt3DCore_QBackendNode_isEnabled(const Qt3DCore::QBackendNode* this_ptr) {
  return this_ptr->isEnabled();
}

Qt3DCore::QBackendNode::Mode qt_3d_core_c_Qt3DCore_QBackendNode_mode(const Qt3DCore::QBackendNode* this_ptr) {
  return this_ptr->mode();
}

Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNode_new_mode(Qt3DCore::QBackendNode::Mode mode) {
  return new Qt3DCore::QBackendNode(mode);
}

Qt3DCore::QBackendNode* qt_3d_core_c_Qt3DCore_QBackendNode_new_no_args() {
  return new Qt3DCore::QBackendNode();
}

void qt_3d_core_c_Qt3DCore_QBackendNode_peerId_to_output(const Qt3DCore::QBackendNode* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->peerId());
}

void qt_3d_core_c_Qt3DCore_QBackendNode_setEnabled(Qt3DCore::QBackendNode* this_ptr, bool enabled) {
  this_ptr->setEnabled(enabled);
}

