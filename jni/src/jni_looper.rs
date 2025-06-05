// use ::std::os::unix::prelude::{BorrowedFd, FromRawFd, OwnedFd, RawFd};

use ::std::os::fd::{FromRawFd, RawFd, BorrowedFd, OwnedFd, AsFd, AsRawFd};

use ::android_activity::AndroidApp;
use ::jni::objects::{JObject, JValue, JValueGen};
use ::log::{debug, info};
use ::ndk::looper::FdEvent;

#[no_mangle]
pub fn setup_looper(app: &AndroidApp) -> Result<[OwnedFd;2], ()> {
    info!("android_main setup up looper");

    // let pipe_fs : [RawFd;2] = [Default::default(), Default::default()];
    let pipe_fs = {
        let mut _fs : [RawFd;2] = [Default::default(), Default::default()];
        unsafe {libc::pipe(_fs.as_mut_ptr()); }
        _fs.map(|f| unsafe { OwnedFd::from_raw_fd(f) } )
    };

    info!("before unsafe looper");
    let looper = unsafe {
        let non_null_looper = ::std::ptr::NonNull::new(app.main_looper_as_ptr()).unwrap();
        ndk::looper::ForeignLooper::from_ptr(non_null_looper)
    };
    info!("before add_fd");
    looper.add_fd_with_callback(
        pipe_fs[0].as_fd(),
        ::ndk::looper::FdEvent::INPUT,
        |fd: BorrowedFd<'_>, _event: ::ndk::looper::FdEvent | {
            callback(fd, _event, app)
        })
        .expect("looper");
    debug!("after callback");
    Ok(pipe_fs)
}

#[no_mangle]
pub fn callback(fd: BorrowedFd<'_>, _event: FdEvent, app: &AndroidApp ) -> bool {
    let mut cmd_i: u8 = 0;
    let read_result = unsafe {
        libc::read(fd.as_raw_fd(), &mut cmd_i as *mut u8 as *mut ::std::os::raw::c_void, 1)
    };
    // schedule_alarm();
    // send_notification(app);
    display_toast();
    return read_result > 0;
}
#[no_mangle]
fn display_toast()
{
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.expect("cannot get vm from ctx");
    let mut env = vm.attach_current_thread().expect("cannot attach to current thread");

    let toast_clazz = env.find_class("android/widget/Toast").expect("cannot load context class");
    let message = env.new_string("EHLO World").unwrap();
    let ctx_as_jobect = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
    let ctx_as_param = JValueGen::Object(&ctx_as_jobect);
    let args = &[ctx_as_param, JValue::Object(&message), JValue::Int(0)];
    let toastobj = env.call_static_method(toast_clazz, "makeText", "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;", args);
    if let Err(e) = toastobj.as_ref() {
        debug!("err: {}", e);
        let _ = env.exception_describe().unwrap();
        let _ = env.fatal_error("exception calling Toast::makeText");
    }
    let mut toastobj = toastobj.unwrap().l().unwrap();

    let _ = env.call_method(&mut toastobj, "show", "()V", &[]);
}
