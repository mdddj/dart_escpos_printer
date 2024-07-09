
## api
```dart
///连接到打印机
Future<Printer> connectToPrinter(
        {required int vendorId, required int productId}) =>
    RustLib.instance.api.crateApiPrinterConnectToPrinter(
        vendorId: vendorId, productId: productId);

///打印简单的文本
Future<bool> printerSimpleText(
        {required Printer printer, required String text}) =>
    RustLib.instance.api
        .crateApiPrinterPrinterSimpleText(printer: printer, text: text);

///打印原始数据
Future<bool> printerRawData(
        {required Printer printer, required List<int> rawData}) =>
    RustLib.instance.api
        .crateApiPrinterPrinterRawData(printer: printer, rawData: rawData);

```