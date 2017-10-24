#include "qt_core_c_QXmlStreamReader.h"

void qt_core_c_QXmlStreamReader_addData_QByteArray(QXmlStreamReader* this_ptr, const QByteArray* data) {
  this_ptr->addData(*data);
}

void qt_core_c_QXmlStreamReader_addData_QString(QXmlStreamReader* this_ptr, const QString* data) {
  this_ptr->addData(*data);
}

void qt_core_c_QXmlStreamReader_addData_char(QXmlStreamReader* this_ptr, const char* data) {
  this_ptr->addData(data);
}

void qt_core_c_QXmlStreamReader_addExtraNamespaceDeclaration(QXmlStreamReader* this_ptr, const QXmlStreamNamespaceDeclaration* extraNamespaceDeclaraction) {
  this_ptr->addExtraNamespaceDeclaration(*extraNamespaceDeclaraction);
}

void qt_core_c_QXmlStreamReader_addExtraNamespaceDeclarations(QXmlStreamReader* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* extraNamespaceDeclaractions) {
  this_ptr->addExtraNamespaceDeclarations(*extraNamespaceDeclaractions);
}

bool qt_core_c_QXmlStreamReader_atEnd(const QXmlStreamReader* this_ptr) {
  return this_ptr->atEnd();
}

void qt_core_c_QXmlStreamReader_attributes_to_output(const QXmlStreamReader* this_ptr, QXmlStreamAttributes* output) {
  new(output) QXmlStreamAttributes(this_ptr->attributes());
}

qint64 qt_core_c_QXmlStreamReader_characterOffset(const QXmlStreamReader* this_ptr) {
  return this_ptr->characterOffset();
}

void qt_core_c_QXmlStreamReader_clear(QXmlStreamReader* this_ptr) {
  this_ptr->clear();
}

qint64 qt_core_c_QXmlStreamReader_columnNumber(const QXmlStreamReader* this_ptr) {
  return this_ptr->columnNumber();
}

void qt_core_c_QXmlStreamReader_constructor_QByteArray(const QByteArray* data, QXmlStreamReader* output) {
  new(output) QXmlStreamReader(*data);
}

void qt_core_c_QXmlStreamReader_constructor_QIODevice(QIODevice* device, QXmlStreamReader* output) {
  new(output) QXmlStreamReader(device);
}

void qt_core_c_QXmlStreamReader_constructor_QString(const QString* data, QXmlStreamReader* output) {
  new(output) QXmlStreamReader(*data);
}

void qt_core_c_QXmlStreamReader_constructor_char(const char* data, QXmlStreamReader* output) {
  new(output) QXmlStreamReader(data);
}

void qt_core_c_QXmlStreamReader_constructor_no_args(QXmlStreamReader* output) {
  new(output) QXmlStreamReader();
}

void qt_core_c_QXmlStreamReader_destructor(QXmlStreamReader* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QIODevice* qt_core_c_QXmlStreamReader_device(const QXmlStreamReader* this_ptr) {
  return this_ptr->device();
}

void qt_core_c_QXmlStreamReader_documentEncoding_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->documentEncoding());
}

void qt_core_c_QXmlStreamReader_documentVersion_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->documentVersion());
}

void qt_core_c_QXmlStreamReader_dtdName_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->dtdName());
}

void qt_core_c_QXmlStreamReader_dtdPublicId_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->dtdPublicId());
}

void qt_core_c_QXmlStreamReader_dtdSystemId_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->dtdSystemId());
}

void qt_core_c_QXmlStreamReader_entityDeclarations_to_output(const QXmlStreamReader* this_ptr, QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >(this_ptr->entityDeclarations());
}

QXmlStreamEntityResolver* qt_core_c_QXmlStreamReader_entityResolver(const QXmlStreamReader* this_ptr) {
  return this_ptr->entityResolver();
}

QXmlStreamReader::Error qt_core_c_QXmlStreamReader_error(const QXmlStreamReader* this_ptr) {
  return this_ptr->error();
}

void qt_core_c_QXmlStreamReader_errorString_to_output(const QXmlStreamReader* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

bool qt_core_c_QXmlStreamReader_hasError(const QXmlStreamReader* this_ptr) {
  return this_ptr->hasError();
}

bool qt_core_c_QXmlStreamReader_isCDATA(const QXmlStreamReader* this_ptr) {
  return this_ptr->isCDATA();
}

bool qt_core_c_QXmlStreamReader_isCharacters(const QXmlStreamReader* this_ptr) {
  return this_ptr->isCharacters();
}

bool qt_core_c_QXmlStreamReader_isComment(const QXmlStreamReader* this_ptr) {
  return this_ptr->isComment();
}

bool qt_core_c_QXmlStreamReader_isDTD(const QXmlStreamReader* this_ptr) {
  return this_ptr->isDTD();
}

bool qt_core_c_QXmlStreamReader_isEndDocument(const QXmlStreamReader* this_ptr) {
  return this_ptr->isEndDocument();
}

bool qt_core_c_QXmlStreamReader_isEndElement(const QXmlStreamReader* this_ptr) {
  return this_ptr->isEndElement();
}

bool qt_core_c_QXmlStreamReader_isEntityReference(const QXmlStreamReader* this_ptr) {
  return this_ptr->isEntityReference();
}

bool qt_core_c_QXmlStreamReader_isProcessingInstruction(const QXmlStreamReader* this_ptr) {
  return this_ptr->isProcessingInstruction();
}

bool qt_core_c_QXmlStreamReader_isStandaloneDocument(const QXmlStreamReader* this_ptr) {
  return this_ptr->isStandaloneDocument();
}

bool qt_core_c_QXmlStreamReader_isStartDocument(const QXmlStreamReader* this_ptr) {
  return this_ptr->isStartDocument();
}

bool qt_core_c_QXmlStreamReader_isStartElement(const QXmlStreamReader* this_ptr) {
  return this_ptr->isStartElement();
}

bool qt_core_c_QXmlStreamReader_isWhitespace(const QXmlStreamReader* this_ptr) {
  return this_ptr->isWhitespace();
}

qint64 qt_core_c_QXmlStreamReader_lineNumber(const QXmlStreamReader* this_ptr) {
  return this_ptr->lineNumber();
}

void qt_core_c_QXmlStreamReader_name_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->name());
}

void qt_core_c_QXmlStreamReader_namespaceDeclarations_to_output(const QXmlStreamReader* this_ptr, QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >(this_ptr->namespaceDeclarations());
}

bool qt_core_c_QXmlStreamReader_namespaceProcessing(const QXmlStreamReader* this_ptr) {
  return this_ptr->namespaceProcessing();
}

void qt_core_c_QXmlStreamReader_namespaceUri_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->namespaceUri());
}

void qt_core_c_QXmlStreamReader_notationDeclarations_to_output(const QXmlStreamReader* this_ptr, QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >(this_ptr->notationDeclarations());
}

void qt_core_c_QXmlStreamReader_prefix_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->prefix());
}

void qt_core_c_QXmlStreamReader_processingInstructionData_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->processingInstructionData());
}

void qt_core_c_QXmlStreamReader_processingInstructionTarget_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->processingInstructionTarget());
}

void qt_core_c_QXmlStreamReader_qualifiedName_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->qualifiedName());
}

void qt_core_c_QXmlStreamReader_raiseError_message(QXmlStreamReader* this_ptr, const QString* message) {
  this_ptr->raiseError(*message);
}

void qt_core_c_QXmlStreamReader_raiseError_no_args(QXmlStreamReader* this_ptr) {
  this_ptr->raiseError();
}

void qt_core_c_QXmlStreamReader_readElementText_to_output_behaviour(QXmlStreamReader* this_ptr, QXmlStreamReader::ReadElementTextBehaviour behaviour, QString* output) {
  new(output) QString(this_ptr->readElementText(behaviour));
}

void qt_core_c_QXmlStreamReader_readElementText_to_output_no_args(QXmlStreamReader* this_ptr, QString* output) {
  new(output) QString(this_ptr->readElementText());
}

QXmlStreamReader::TokenType qt_core_c_QXmlStreamReader_readNext(QXmlStreamReader* this_ptr) {
  return this_ptr->readNext();
}

bool qt_core_c_QXmlStreamReader_readNextStartElement(QXmlStreamReader* this_ptr) {
  return this_ptr->readNextStartElement();
}

void qt_core_c_QXmlStreamReader_setDevice(QXmlStreamReader* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_core_c_QXmlStreamReader_setEntityResolver(QXmlStreamReader* this_ptr, QXmlStreamEntityResolver* resolver) {
  this_ptr->setEntityResolver(resolver);
}

void qt_core_c_QXmlStreamReader_setNamespaceProcessing(QXmlStreamReader* this_ptr, bool arg1) {
  this_ptr->setNamespaceProcessing(arg1);
}

void qt_core_c_QXmlStreamReader_skipCurrentElement(QXmlStreamReader* this_ptr) {
  this_ptr->skipCurrentElement();
}

void qt_core_c_QXmlStreamReader_text_to_output(const QXmlStreamReader* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->text());
}

void qt_core_c_QXmlStreamReader_tokenString_to_output(const QXmlStreamReader* this_ptr, QString* output) {
  new(output) QString(this_ptr->tokenString());
}

QXmlStreamReader::TokenType qt_core_c_QXmlStreamReader_tokenType(const QXmlStreamReader* this_ptr) {
  return this_ptr->tokenType();
}

