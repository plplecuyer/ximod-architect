# XIMOD Architect - translation metadata
# @language = jpn
# @font = Noto_Sans_JP/static/NotoSansJP-Regular.ttf
# @langname = 日本語
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = バージョン { $version }

# Status messages
status-ready = 準備完了
msg-save-success = FOMOD の保存に成功しました
msg-save-error = FOMOD の保存中にエラーが発生しました
msg-export-success = 配布用アーカイブが作成されました ({ $count } ファイル): { $path }
msg-export-error = 配布用アーカイブの作成中にエラーが発生しました: { $error }
msg-load-success = FOMOD の読み込みに成功しました
msg-load-error = FOMODの読み込みに失敗しました
msg-merge-success = FOMODのマージに成功しました
msg-merge-error = FOMODのマージに失敗しました
msg-no-root-selected = まずルートディレクトリを選択してください
msg-no-fomod-folder = 「fomod」フォルダが見つかりません。作成しますか？
msg-file-outside-root = ファイルがルートディレクトリ外にあります

# Menu - File
menu-file = ファイル
menu-new = 新規作成
menu-open = フォルダを開く...
menu-open-file = ファイルを開く...
menu-save = 保存
menu-recent = 最近使用したファイル
menu-exit = 終了
menu-merge = FOMODをマージ...
menu-export = 配布用アーカイブをエクスポート...

# Menu - Options
menu-options = オプション
menu-settings = 設定
menu-pre-save-script = 保存前スクリプト...
menu-post-save-script = 保存後スクリプト...
menu-translation = 翻訳...

# Menu - Help
menu-help = ヘルプ
menu-about = バージョン情報

# Tabs
tab-info = Mod情報
tab-steps = インストール手順
tab-required = 必須のインストール
tab-conditional = 条件付きインストール

# Info Tab
label-workspace = ワークスペース
label-root-dir = ルートディレクトリ:
label-mod-name = MOD名:
label-author = 作成者:
label-version = バージョン:
label-game-name = ゲーム名:
label-category = カテゴリ:
label-url = ウェブサイトURL:
label-header-image = ヘッダー画像:
label-description = 説明:
placeholder-select-dir = (ディレクトリを選択)
placeholder-select-game = (ゲームを選択)

# Steps Tab
label-step-name = ステップ名:
label-group-name = グループ名:
label-group-type = グループタイプ:
label-plugin-name = プラグイン名:
label-plugin-desc = 説明:
label-plugin-type = デフォルトタイプ:
label-plugin-image = 画像:
label-visibility = 表示条件
label-operator = 演算子:

# Buttons
btn-browse = 参照...
btn-clear = クリア
btn-add = 追加
btn-remove = 削除
btn-add-step = 新しいステップ
btn-delete-step = ステップの削除
btn-add-group = グループの追加
btn-remove-group = グループの削除
btn-add-plugin = プラグインの追加
btn-remove-plugin = プラグインの削除
btn-add-file = ファイルの追加
btn-add-folder = フォルダの追加
btn-remove-file = 削除
btn-add-flag = フラグの追加
btn-remove-flag = フラグを削除
btn-add-condition = 条件を追加
btn-remove-condition = 条件を削除
btn-add-dependency = 依存関係を追加
btn-remove-dependency = 依存関係を削除
btn-add-pattern = 新しいパターン
btn-remove-pattern = パターンを削除
btn-save = 保存
btn-cancel = キャンセル
btn-ok = OK
btn-yes = はい
btn-no = いいえ

# Condition/Dependency Labels
label-flag-name = フラグ名:
label-flag-value = 値:
label-condition-type = タイプ:
label-condition-name = 名前:
label-condition-value = 値:
label-dep-type = 依存関係タイプ:
label-dep-name = 名前／ファイル:
label-dep-value = 値／状態:

# Files
label-source = ソース
label-destination = 宛先
label-priority = 優先度
label-file-type = ファイルの種類
label-files = ファイル
label-dependencies = 依存関係

# Settings Dialog
settings-title = 設定
settings-tab-general = 一般
settings-tab-recent-files = 最近のファイル
settings-language = 言語:
settings-theme = テーマ:
settings-font-size = フォントサイズ:
settings-replace-newlines = 説明文内の改行を処理する
settings-max-recent = 最近使用したファイルの最大数:
settings-window-width = ウィンドウの幅:
settings-window-height = ウィンドウの高さ:
settings-no-recent-files = 最近使用したファイルはありません。

# Status messages for settings
status-settings-saved = 設定が正常に保存されました

# About Dialog
about-title = XIMOD Architect について
about-description = Bethesda 社のゲームMOD用クロスプラットフォーム FOMOD インストーラ作成ツールです。
about-license = MIT ライセンスの下で提供されています
about-copyright = © 2025-2026 XIMOD Team
about-credit = Wenderer氏によるオリジナルツールのRust移植版：

# Script Dialog
script-title = スクリプトの編集
script-info = スクリプトは保存の前または後に実行されます。以下のマクロを使用できます:
script-macros = 利用可能なマクロ：
macro-modname = $MODNAME$ - MOD名
macro-modauthor = $MODAUTHOR$ - 作成者名
macro-modversion = $MODVERSION$ - MODバージョン
macro-modroot = $MODROOT$ - ルートディレクトリのパス
macro-date = $DATE$ - 現在の日付 (YYYY-MM-DD)
macro-time = $TIME$ - 現在の時刻 (HH:MM:SS)
macro-random = $RANDOM$ - 乱数

# Plugin Dependencies
label-default-type = デフォルトのタイプ:
label-pattern-type = パターンのタイプ:
label-pattern-operator = パターンの演算子:

# Conditional Files
label-pattern = パターン

# Validation Messages
validation-no-name = モジュール名が必要です
validation-no-steps = 少なくとも1つのステップまたは必須ファイルが必要です
validation-empty-step = ステップ { $num } に名前がありません
validation-empty-group = ステップ { $step }、グループ { $group } に名前がありません
validation-no-plugins = ステップ { $step }、グループ "{ $name }" にプラグインがありません

# File States
state-active = アクティブ
state-inactive = 非アクティブ
state-missing = 欠落

# Confirmation
confirm-title = 確認
confirm-delete = この項目を削除してもよろしいですか？
confirm-discard = 未保存の変更があります。変更を破棄して続行しますか？
confirm-unsaved = 未保存の変更があります。閉じる前に保存しますか？
confirm-save-issues = このプロジェクトには以下の問題があります：
confirm-save-anyway = それでも保存しますか？

# Errors
error-invalid-xml = 無効な XML ファイル
error-parse-failed = FOMOD の解析に失敗しました
error-write-failed = ファイルの書き込みに失敗しました
error-create-dir = ディレクトリの作成に失敗しました

# Default names (generated when creating new items)
default-step-name = ステップ { $num }
default-group-name = グループ { $num }
default-plugin-name = プラグイン { $num }
pattern-label = パターン { $num }

# Selection prompts
msg-select-group-first = まずグループを選択してください。
msg-select-plugin-edit = 編集するプラグインを選択してください。
label-empty = (空)
image-no-image = 画像なし

# File dialog filters
filter-images = 画像
filter-xml = XML

# Dependency types
dep-type-flag = フラグ
dep-type-file = ファイル

# Status bar
status-modified = 変更されました

# Status messages (errors)
msg-settings-save-error = 設定の保存に失敗しました
msg-script-save-error = スクリプトの保存に失敗しました

# Translation editor
trans-title = 翻訳エディタ
trans-source-lang = 表示言語:
trans-target-lang = 翻訳先言語:
trans-col-key = キー
trans-col-source = ラベル
trans-col-target = 翻訳
trans-saved = 翻訳が保存されました
trans-save-error = 翻訳の保存に失敗しました

# XML editor
xml-editor-title = XMLエディタ
xml-editor-edit = 編集
xml-editor-apply = 適用
xml-editor-revert = 取り消し
xml-editor-readonly = 読み取り専用
xml-editor-editing = 編集中 — グラフィカルタブはロックされています
xml-editor-error = エラー:
xml-editor-applied = XMLの変更が適用されました
xml-editor-wellformed = 構文が正しいXMLです
xml-editor-error-at = 行 { $line }、列 { $col }: { $msg }

# Country / flag picker
settings-country-name = 国名:
settings-pick-country = クリックして国を選択してください
flags-title = 国を選択
flags-filter = フィルタ:
flags-none = 該当する国旗が見つかりません

# Translation editor: country & font
trans-endonym = 国名：
trans-font = フォント：
trans-no-font = (なし)
trans-browse = 参照…
trans-google-fonts = Google Fonts
trans-pick-country = クリックして国を選択してください
trans-font-outside = フォントはまず assets/fonts フォルダにインストールする必要があります。
trans-font-dir-missing = assets/fonts フォルダが見つかりませんでした。

# Translation submission
trans-lang-endonym = 言語の現地名:
trans-author = 作成者:
trans-submit = 送信…
trans-submit-hint = ZIP ファイルを作成し、入力済みのメールを開く
trans-data-updated = 参照データが更新されました (Languages.json / Countries.json)
trans-package-ready = アーカイブの準備完了:
trans-package-error = アーカイブを作成できませんでした:

# ISO 639-3 requirement
trans-lang-not-iso = 翻訳は、ISO 639-3 コードを持つ言語でのみ可能です。

# FOMOD installer preview
menu-preview = インストーラのプレビュー…
preview-title = FOMOD インストーラのプレビュー
preview-refresh = 更新
preview-assumptions = ファイルの想定
preview-details = 詳細
preview-back = 戻る
preview-next = 次へ
preview-install = インストール
preview-close = 閉じる
preview-restart = 再起動
preview-summary-title = インストールされるファイル
preview-empty = インストールされるファイルはありません。
preview-none-option = (なし)
preview-invalid = 続行するには、必須の選択を完了してください。
preview-no-steps = 表示されている手順はありません。インストール概要を参照してください。
preview-select-hint = オプションを選択すると、その説明が表示されます。
preview-col-source = ソース
preview-col-dest = 宛先
preview-col-priority = 優先度
preview-sel-exactlyone = オプションを1つだけ選択してください。
preview-sel-atmostone = オプションを最大1つまで選択してください。
preview-sel-any = 任意の数のオプションを選択してください。
preview-sel-all = すべてのオプションがインストールされます。
preview-sel-atleastone = 少なくとも 1 つのオプションを選択してください。

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = FOMOD を検証
validate-report-title = FOMOD 検証
validate-ok = 問題は見つかりませんでした。FOMOD はスキーマに準拠しています。
xml-editor-schema-ok = ModConfig 5.0 スキーマに準拠しています。
xml-editor-schema-issues = スキーマの問題:
schema-line-col = 行 { $line }、列 { $col }: { $msg }
schema-wrong-root = 予期しないルート "{ $found }" ("{ $expected }" が期待されていました)。
schema-unknown = 「{ $parent }」内に予期しない要素「{ $element }」があります。
schema-missing = 「{ $parent }」には「{ $child }」が含まれている必要があります。
schema-needs-one = 「{ $parent }」には少なくとも1つの「{ $child }」が含まれている必要があります。
schema-too-many = 「{ $parent }」内では「{ $child }」は1回のみ出現可能です。
schema-missing-attr = 「{ $element }」には属性「{ $attr }」が必要です。
schema-bad-enum = { $element }/@{ $attr } に対して無効な値 "{ $value }" が指定されています（期待される値: { $allowed }）。
schema-choose-one = "{ $parent }" には、{ $options } のうち正確に 1 つが含まれている必要があります。

# Reordering (steps / groups / plugins)
reorder-before = 前に移動
reorder-after = 後に移動

# Country / language database explorer (Properties)
menu-properties = プロパティ…
prop-title = 国・言語データベース
prop-tab-countries = 国
prop-tab-languages = 言語
prop-filter = フィルタ:
prop-official-langs = 公用語
prop-spoken-langs = 話されている言語
prop-endonym = 国の自称
prop-font = フォント
prop-spoken-in = 話されている地域
prop-select-country = 国を選択して詳細を表示してください。
prop-select-lang = 言語を選択して詳細を表示してください。

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = ゲームの Nexus Mods ページを開く
