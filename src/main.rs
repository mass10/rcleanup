mod application;

fn usage() {
	println!("USAGE:");
	println!("    rcleanup {{location}}");
	println!();
}

/// アプリケーションのエントリーポイント
fn main() {
	// コマンドライン引数
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		usage();
		return;
	}

	// アプリケーションのインスタンスを初期化します。
	let result = application::Application::new();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_millis(2500));
		return;
	}
	let application = result.unwrap();

	// アプリケーションを実行します。
	let result = application.run(&args);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_millis(2500));
		return;
	}

	std::thread::sleep(std::time::Duration::from_millis(2500));
}
