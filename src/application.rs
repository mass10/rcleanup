/// アプリケーション構造体
pub struct Application;

/// 一行の入力を得ます。
pub fn input_line() -> String {
	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		println!("[ERROR] {}", ret.err().unwrap());
		return String::new();
	}
	return line.trim().to_string();
}

/// プロンプトを出してユーザー確認を得ます。
#[allow(unused)]
fn confirm() -> bool {
	let answer = input_line().to_uppercase();
	return match answer.as_str() {
		"Y" => true,
		"YES" => true,
		_ => false,
	};
}

fn flush() {
	use std::io::Write;
	std::io::stdout().flush().unwrap();
}

fn prompt(path: &str) -> bool {
	println!("[QUESTION] ディレクトリを削除しますか？ {}", path);
	print!("(y/N)> ");
	flush();
	return confirm();
}

/// 指定されたディレクトリーを削除します。
///
/// ### Arguments
/// `path` パス
fn remove_dir_all(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	if !std::path::Path::new(path).exists() {
		return Ok(());
	}
	if !prompt(path) {
		return Ok(());
	}
	std::fs::remove_dir_all(path)?;
	return Ok(());
}

/// ディレクトリー配下を走査します。
fn find_file(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// 存在確認
	let source_path = std::path::Path::new(path);
	if !source_path.exists() {
		println!("[WARN] invalid path {}", source_path.to_str().unwrap());
		// 消えることは想定内、処理は継続
		return Ok(());
	}

	if source_path.is_dir() {
		// Cargo.toml の有無
		let exists_cargo_toml = source_path.join("Cargo.toml").is_file();
		// package.json の有無
		let exists_package_json = source_path.join("package.json").is_file();

		// 直下のエントリーを列挙
		let mut directories: Vec<String> = vec![];
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			// 直下のファイル、またはディレクトリー
			let entry = e?;
			let metadata = entry.metadata()?;
			if metadata.is_dir() {
				// ディレクトリー
				let filename = entry.file_name();

				// Cargo.toml が存在しているときは target ディレクトリーを削除します。
				if exists_cargo_toml && filename == "target" {
					let path = entry.path();
					remove_dir_all(path.to_str().unwrap())?;
					continue;
				}

				// package.json が存在しているときは node_modules ディレクトリーを削除します。
				if exists_package_json && filename == "node_modules" {
					let path = entry.path();
					remove_dir_all(path.to_str().unwrap())?;
					continue;
				}

				// その他のディレクトリーはさらに掘り下げます。
				let pathbuf = entry.path();
				let path = pathbuf.to_str().unwrap();
				directories.push(path.to_string());
			}
		}

		for path in &directories {
			find_file(path.as_str())?;
		}

		return Ok(());
	}

	return Ok(());
}

/// node_modules を探して削除します。
///
/// ### Arguments
/// `path` 起点のパス
fn execute_cleanup(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
			let result = execute_cleanup(arg.as_str());
			if result.is_err() {
				println!("[ERROR] {}", result.err().unwrap());
				return Ok(());
			}
		}

		return Ok(());
	}
}
