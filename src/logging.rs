use crate::ffi_to_c_string;
use std::fmt::Write;
use std::os::raw::c_char;
use std::ptr::null;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{FmtSubscriber, Layer};

type LogCallback = unsafe extern "C" fn(QtMsgType, *const c_char, *const c_char, i32);

pub struct CustomLayer {
    callback: LogCallback,
}

struct PrintlnVisitor<'a> {
    string: &'a mut String,
}

/// cbindgen:ignore
#[repr(i8)]
#[allow(dead_code)]
pub enum QtMsgType {
    Debug = 0,
    Warning = 1,
    Critical = 2,
    Fatal = 3,
    Info = 4,
}

impl tracing::field::Visit for PrintlnVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            write!(self.string, "{:#?} ", value).unwrap();
        } else {
            write!(self.string, "{} = {:?} ", field.name(), value).unwrap();
        }
    }
}

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut buffer: String = String::new();
        let mut visitor = PrintlnVisitor {
            string: &mut buffer,
        };
        event.record(&mut visitor);

        let msg_type = match *event.metadata().level() {
            Level::ERROR => QtMsgType::Critical,
            Level::WARN => QtMsgType::Warning,
            Level::INFO => QtMsgType::Info,
            Level::DEBUG => QtMsgType::Debug,
            Level::TRACE => QtMsgType::Debug,
        };

        unsafe {
            let file = if let Some(file) = event.metadata().file() {
                ffi_to_c_string(&file.to_string())
            } else {
                null()
            };

            let line = if let Some(line) = event.metadata().line() {
                line as i32
            } else {
                -1
            };

            (self.callback)(msg_type, ffi_to_c_string(&buffer), file, line);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_tracing_callback(callback: LogCallback) {
    tracing_subscriber::registry()
        .with(CustomLayer { callback })
        .init();
}

#[no_mangle]
pub extern "C" fn physis_initialize_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
