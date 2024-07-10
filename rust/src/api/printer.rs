use escpos_rust::{Printer, PrinterProfile};

///连接到打印机
pub fn connect_to_printer(vendor_id: u16, product_id: u16) -> Result<Printer, String> {
    let profile = PrinterProfile::usb_builder(vendor_id, product_id).build();
    match Printer::new(profile) {
        Ok(printer_device) => match printer_device {
            Some(device) => Ok(device),
            None => Err("get printer failed".to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

///打印简单的文本
pub fn printer_simple_text(printer: &Printer, text: String) -> Result<(), String> {
    let is_ok = printer.println(text);
    match is_ok {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

///打印原始数据
pub fn printer_raw_data(printer: &Printer, raw_data: Vec<u8>) -> Result<(), String> {
    let r = printer.raw(raw_data);
    match r {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
