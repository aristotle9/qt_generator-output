#include "qt_core_c_QMetaType.h"

bool qt_core_c_QMetaType_compare(const void* lhs, const void* rhs, int typeId, int* result) {
  return QMetaType::compare(lhs, rhs, typeId, result);
}

void* qt_core_c_QMetaType_construct_type_where_copy(int type, void* where, const void* copy) {
  return QMetaType::construct(type, where, copy);
}

void* qt_core_c_QMetaType_construct_where(const QMetaType* this_ptr, void* where) {
  return this_ptr->construct(where);
}

void* qt_core_c_QMetaType_construct_where_copy(const QMetaType* this_ptr, void* where, const void* copy) {
  return this_ptr->construct(where, copy);
}

void qt_core_c_QMetaType_constructor(const int type, QMetaType* output) {
  new(output) QMetaType(type);
}

bool qt_core_c_QMetaType_convert(const void* from, int fromTypeId, void* to, int toTypeId) {
  return QMetaType::convert(from, fromTypeId, to, toTypeId);
}

void* qt_core_c_QMetaType_create_copy(const QMetaType* this_ptr, const void* copy) {
  return this_ptr->create(copy);
}

void* qt_core_c_QMetaType_create_no_args(const QMetaType* this_ptr) {
  return this_ptr->create();
}

void* qt_core_c_QMetaType_create_type(int type) {
  return QMetaType::create(type);
}

void* qt_core_c_QMetaType_create_type_copy(int type, const void* copy) {
  return QMetaType::create(type, copy);
}

bool qt_core_c_QMetaType_debugStream(QDebug* dbg, const void* rhs, int typeId) {
  return QMetaType::debugStream(*dbg, rhs, typeId);
}

void qt_core_c_QMetaType_destroy_data(const QMetaType* this_ptr, void* data) {
  this_ptr->destroy(data);
}

void qt_core_c_QMetaType_destroy_type_data(int type, void* data) {
  QMetaType::destroy(type, data);
}

void qt_core_c_QMetaType_destruct_data(const QMetaType* this_ptr, void* data) {
  this_ptr->destruct(data);
}

void qt_core_c_QMetaType_destruct_type_where(int type, void* where) {
  QMetaType::destruct(type, where);
}

void qt_core_c_QMetaType_destructor(QMetaType* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QMetaType_equals(const void* lhs, const void* rhs, int typeId, int* result) {
  return QMetaType::equals(lhs, rhs, typeId, result);
}

unsigned int qt_core_c_QMetaType_flags(const QMetaType* this_ptr) {
  return uint(this_ptr->flags());
}

bool qt_core_c_QMetaType_hasRegisteredComparators(int typeId) {
  return QMetaType::hasRegisteredComparators(typeId);
}

bool qt_core_c_QMetaType_hasRegisteredConverterFunction(int fromTypeId, int toTypeId) {
  return QMetaType::hasRegisteredConverterFunction(fromTypeId, toTypeId);
}

bool qt_core_c_QMetaType_hasRegisteredDebugStreamOperator(int typeId) {
  return QMetaType::hasRegisteredDebugStreamOperator(typeId);
}

bool qt_core_c_QMetaType_isRegistered_no_args(const QMetaType* this_ptr) {
  return this_ptr->isRegistered();
}

bool qt_core_c_QMetaType_isRegistered_type(int type) {
  return QMetaType::isRegistered(type);
}

bool qt_core_c_QMetaType_isValid(const QMetaType* this_ptr) {
  return this_ptr->isValid();
}

bool qt_core_c_QMetaType_load(QDataStream* stream, int type, void* data) {
  return QMetaType::load(*stream, type, data);
}

const QMetaObject* qt_core_c_QMetaType_metaObject(const QMetaType* this_ptr) {
  return this_ptr->metaObject();
}

const QMetaObject* qt_core_c_QMetaType_metaObjectForType(int type) {
  return QMetaType::metaObjectForType(type);
}

int qt_core_c_QMetaType_registerNormalizedTypedef(const QByteArray* normalizedTypeName, int aliasId) {
  return QMetaType::registerNormalizedTypedef(*normalizedTypeName, aliasId);
}

int qt_core_c_QMetaType_registerType(const char* typeName, void (*deleter)(void*), void* (*creator)(const void*)) {
  return QMetaType::registerType(typeName, deleter, creator);
}

int qt_core_c_QMetaType_registerTypedef(const char* typeName, int aliasId) {
  return QMetaType::registerTypedef(typeName, aliasId);
}

bool qt_core_c_QMetaType_save(QDataStream* stream, int type, const void* data) {
  return QMetaType::save(*stream, type, data);
}

int qt_core_c_QMetaType_sizeOf_no_args(const QMetaType* this_ptr) {
  return this_ptr->sizeOf();
}

int qt_core_c_QMetaType_sizeOf_type(int type) {
  return QMetaType::sizeOf(type);
}

unsigned int qt_core_c_QMetaType_typeFlags(int type) {
  return uint(QMetaType::typeFlags(type));
}

const char* qt_core_c_QMetaType_typeName(int type) {
  return QMetaType::typeName(type);
}

int qt_core_c_QMetaType_type_QByteArray(const QByteArray* typeName) {
  return QMetaType::type(*typeName);
}

int qt_core_c_QMetaType_type_char(const char* typeName) {
  return QMetaType::type(typeName);
}

bool qt_core_c_QMetaType_unregisterType(int type) {
  return QMetaType::unregisterType(type);
}

