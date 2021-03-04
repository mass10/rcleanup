/// アプリケーション定義のエラー
#[derive(Debug, Clone)]
pub struct ApplicationError {
	/// エラーメッセージ
	pub description: String,
}

/// ApplicationError に [std::fmt::Display] としての振る舞いを実装します。
impl std::fmt::Display for ApplicationError {
	/// 文字列表現を返す既定の動作です。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		return write!(f, "{}", self.description);
	}
}

/// ApplicationError に [std::error::Error] としての振る舞いを実装します。
impl std::error::Error for ApplicationError {
	/// 文字列表現を返す既定の動作です。
	fn description(&self) -> &str {
		return &self.description;
	}
}
