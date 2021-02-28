mod application;

/// アプリケーションのエントリーポイント
fn main() {
	// アプリケーションのインスタンスを初期化します。
	let app = application::Application::new();
	if app.is_err() {
		println!("[ERROR] {}", app.err().unwrap());
		return;
	}
	let app = app.unwrap();

	// アプリケーションを実行します。
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
