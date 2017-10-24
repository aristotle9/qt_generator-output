#ifndef QT_CORE_C_QXMLSTREAMREADER_H
#define QT_CORE_C_QXMLSTREAMREADER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_addData_QByteArray(QXmlStreamReader* this_ptr, const QByteArray* data);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_addData_QString(QXmlStreamReader* this_ptr, const QString* data);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_addData_char(QXmlStreamReader* this_ptr, const char* data);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_addExtraNamespaceDeclaration(QXmlStreamReader* this_ptr, const QXmlStreamNamespaceDeclaration* extraNamespaceDeclaraction);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_addExtraNamespaceDeclarations(QXmlStreamReader* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* extraNamespaceDeclaractions);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_atEnd(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_attributes_to_output(const QXmlStreamReader* this_ptr, QXmlStreamAttributes* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QXmlStreamReader_characterOffset(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_clear(QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QXmlStreamReader_columnNumber(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_constructor_QByteArray(const QByteArray* data, QXmlStreamReader* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_constructor_QIODevice(QIODevice* device, QXmlStreamReader* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_constructor_QString(const QString* data, QXmlStreamReader* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_constructor_char(const char* data, QXmlStreamReader* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_constructor_no_args(QXmlStreamReader* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_destructor(QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QXmlStreamReader_device(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_documentEncoding_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_documentVersion_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_dtdName_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_dtdPublicId_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_dtdSystemId_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_entityDeclarations_to_output(const QXmlStreamReader* this_ptr, QVector< QXmlStreamEntityDeclaration >* output);
QT_CORE_C_EXPORT QXmlStreamEntityResolver* qt_core_c_QXmlStreamReader_entityResolver(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT QXmlStreamReader::Error qt_core_c_QXmlStreamReader_error(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_errorString_to_output(const QXmlStreamReader* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_hasError(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isCDATA(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isCharacters(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isComment(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isDTD(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isEndDocument(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isEndElement(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isEntityReference(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isProcessingInstruction(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isStandaloneDocument(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isStartDocument(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isStartElement(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_isWhitespace(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QXmlStreamReader_lineNumber(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_name_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_namespaceDeclarations_to_output(const QXmlStreamReader* this_ptr, QVector< QXmlStreamNamespaceDeclaration >* output);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_namespaceProcessing(const QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_namespaceUri_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_notationDeclarations_to_output(const QXmlStreamReader* this_ptr, QVector< QXmlStreamNotationDeclaration >* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_prefix_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_processingInstructionData_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_processingInstructionTarget_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_qualifiedName_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_raiseError_message(QXmlStreamReader* this_ptr, const QString* message);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_raiseError_no_args(QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_readElementText_to_output_behaviour(QXmlStreamReader* this_ptr, QXmlStreamReader::ReadElementTextBehaviour behaviour, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_readElementText_to_output_no_args(QXmlStreamReader* this_ptr, QString* output);
QT_CORE_C_EXPORT QXmlStreamReader::TokenType qt_core_c_QXmlStreamReader_readNext(QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamReader_readNextStartElement(QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_setDevice(QXmlStreamReader* this_ptr, QIODevice* device);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_setEntityResolver(QXmlStreamReader* this_ptr, QXmlStreamEntityResolver* resolver);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_setNamespaceProcessing(QXmlStreamReader* this_ptr, bool arg1);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_skipCurrentElement(QXmlStreamReader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_text_to_output(const QXmlStreamReader* this_ptr, QStringRef* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamReader_tokenString_to_output(const QXmlStreamReader* this_ptr, QString* output);
QT_CORE_C_EXPORT QXmlStreamReader::TokenType qt_core_c_QXmlStreamReader_tokenType(const QXmlStreamReader* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QXMLSTREAMREADER_H
