/// アプリケーション構造体
pub struct Application;

/// 指定されたディレクトリーを削除します。
///
/// ### Arguments
/// `path` パス
fn remove_dir_all(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	if !std::path::Path::new(path).exists() {
		return Ok(());
	}
	std::fs::remove_dir_all(path)?;
	return Ok(());
}

/// ディレクトリー配下を走査します。
fn find_file(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(path);
	if !source_path.exists() {
		println!("[ERROR] invalid path {}", source_path.to_str().unwrap());
		return Ok(());
	}
	if source_path.is_dir() {
		let name = source_path.file_name().unwrap().to_str().unwrap();
		if name == "node_modules" {
			println!("delete ... {}", source_path.to_str().unwrap());
			remove_dir_all(source_path.as_os_str().to_str().unwrap())?;
			return Ok(());
		}
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			let path = entry.path();
			find_file(&path.as_os_str().to_str().unwrap())?;
		}
		return Ok(());
	}
	return Ok(());
}

/// node_modules を探して削除します。
///
/// ### Arguments
/// `path` 起点のパス
fn rcleanup(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	find_file(path)?;
	return Ok(());
}

impl Application {
	/// 新しいインスタンスを返します。
	pub fn new() -> std::result::Result<Application, std::boxed::Box<dyn std::error::Error>> {
		return Ok(Application {});
	}

	/// アプリケーションを実行します。
	///
	/// ### Arguments
	/// `args` 走査の起点となるパスを指定します。複数指定することもできます。
	pub fn run(&self, args: &Vec<String>) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// ディレクトリーのクリーンアップ
		for arg in args {
			let result = rcleanup(arg.as_str());
			if result.is_err() {
				println!("[ERROR] {}", result.err().unwrap());
				return Ok(());
			}
		}

		return Ok(());
	}
}
