#![no_std]
#![no_main]

#[link(name = "kernel32")]
unsafe extern "system" {
    // 標準出力のハンドルを取得する関数
    fn GetStdHandle(nStdHandle: i32) -> isize;
    // コンソールに文字列を書き出す関数
    fn WriteConsoleA(
        hConsoleOutput: isize,
        lpBuffer: *const u8,
        nNumberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *mut u8,
    ) -> i32;
    // プロセスを終了する関数
    fn ExitProcess(uExitCode: u32) -> !;
}

// 標準出力ハンドルの定数（-11）
const STD_OUTPUT_HANDLE: i32 = -11;
use core::panic::PanicInfo;
// panic 発生時のハンドラ
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// エントリーポイント（C ランタイムの初期化前に呼ばれる関数）
// リンカオプションで /ENTRY:mainCRTStartup を指定する必要があります。
#[unsafe(no_mangle)]
pub extern "system" fn mainCRTStartup() -> ! {
    unsafe {
        // 標準出力のハンドルを取得
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        // 出力する文字列（C文字列として null 終端）
        let message = b"Hello, World!\r\n";
        let mut written = 0;
        // コンソールに書き出し
        WriteConsoleA(
            handle,
            message.as_ptr(),
            message.len() as u32,
            &mut written,
            core::ptr::null_mut(),
        );
        // プロセス終了
        ExitProcess(0);
    }
}
