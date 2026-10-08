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
menu-open = フォルダを開く…
menu-open-file = ファイルを開く…
menu-save = 保存
menu-recent = 最近使用したファイル
menu-exit = 終了
menu-merge = FOMODをマージ…
menu-export = 配布用アーカイブをエクスポート…
# Menu - Options
menu-options = オプション
menu-settings = 設定…
menu-pre-save-script = 保存前スクリプト…
menu-post-save-script = 保存後スクリプト…
menu-translation = インターフェースを翻訳…
# Menu - Help
menu-help = ヘルプ
menu-check-updates = 更新を確認…
menu-about = バージョン情報

# Update check
update-checking = 更新を確認しています…
update-up-to-date = XIMOD Architect は最新です。
update-check-failed = 更新を確認できませんでした。後でもう一度お試しください。
update-available-status = バージョン { $version } が利用可能です。
update-banner-text = XIMOD Architect { $version } が利用可能です。
update-download = ダウンロード:
update-skip = このバージョンをスキップ
update-later = 後で

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
label-plugin-name = オプション名:
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
btn-add-plugin = オプションの追加
btn-remove-plugin = オプションの削除
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = グループ全体の宛先
label-page-dest = インストール先（ページ全体）
btn-apply-group-dest = このグループのすべてのオプションに適用
btn-apply-page-dest = このページのすべてのオプションに適用
group-dest-hint = このグループの各オプションのすべてのファイルに単一のインストール先を設定します。
page-dest-hint = このページの各オプションのすべてのファイルに単一のインストール先を設定します（すべてのグループ）。
bulk-dest-nofiles = 更新するファイルがまだありません — まずオプションにファイルを追加してください。
status-dest-applied = { $num } 個のファイルに宛先を適用しました。
preview-hidden-steps = 現在の選択により { $num } 個のステップが非表示です。
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
settings-check-updates = 起動時に更新を確認
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
label-plugin-dependencies = オプション依存関係
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
validation-no-plugins = ステップ { $step }、グループ "{ $name }" にオプションがありません

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
default-plugin-name = オプション { $num }
pattern-label = パターン { $num }

# Selection prompts
msg-select-group-first = まずグループを選択してください。
msg-select-plugin-edit = 編集するオプションを選択してください。
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

# Referenced-file verification (V2)
verify-no-root = ファイル検証をスキップ：ルートフォルダーが未設定です
loc-header = ヘッダー画像
loc-required = 必須ファイル
loc-conditional = 条件セット { $num }
loc-plugin = ステップ { $step }、グループ { $group }、オプション「{ $plugin }」
verify-missing-file = ファイルがありません: { $path }（{ $loc }）
verify-missing-folder = フォルダーがありません: { $path }（{ $loc }）
verify-missing-image = 画像がありません: { $path }（{ $loc }）
verify-absolute = 絶対パス（移植不可）: { $path }（{ $loc }）
verify-outside = パスがルートフォルダーの外を指しています: { $path }（{ $loc }）
verify-orphan = 孤立ファイル（どのオプションからも参照されていません）: { $path }
conflict-certain = 配置先の競合: 「{ $path }」は { $count } 個のオプション（{ $locs }）が書き込みます — 互いに上書きします。
conflict-potential = 配置先の競合の可能性: 「{ $path }」は { $count } 件の参照（{ $locs }）の対象です — 上書きは選択／条件に依存します。

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = FOMODを閉じる
menu-close-all-fomods = すべてのFOMODを閉じる
tab-untitled = （無題）
msg-drop-not-fomod = ドロップされた項目はFOMODではありません（「fomod」フォルダーが見つかりません）
exit-title = 未保存の変更
exit-unsaved = 保存されていないFOMODがあります。保存しますか？
tab-close-hint = このFOMODを閉じる
menu-new-from-folder = フォルダーから新規…
menu-templates = テンプレート…
templates-title = 再利用可能なテンプレート
templates-empty = 保存されたテンプレートはまだありません。上で選択したステップを保存して作成してください。
templates-insert = 挿入
templates-save-step = 選択したステップを保存
templates-name-hint = テンプレート名（任意）
msg-wizard-success = フォルダーから雛形を作成しました：{ $num } 個のオプション。
msg-wizard-error = エラー：{ $error }
msg-template-saved = テンプレートを保存しました：{ $name }
msg-template-inserted = テンプレートをプロジェクトに挿入しました。
msg-template-no-step = テンプレートとして保存するには、まずステップを選択してください。
msg-template-no-dir = テンプレートフォルダーが見つかりませんでした。
msg-drop-assigned = オプションに { $added } 個のソースを追加しました（ルート外の { $rejected } 個は無視）。
menu-compare = 比較…
compare-title = FOMOD の比較
compare-none = 差分はありません。
btn-optimize-image = 画像を最適化
msg-image-optimized = ヘッダー画像を最適化しました。
msg-image-ok = ヘッダー画像はすでに制限内です。
msg-no-header-image = 最適化するヘッダー画像がありません。
verify-image-large = 画像が大きすぎます（{ $width }×{ $height }）：{ $path }
verify-image-format = 未対応の画像形式（.{ $ext }）：{ $path }
verify-image-unreadable = 読み取れない画像：{ $path }
menu-condition-editor = 条件エディター…
condeditor-title = 条件エディター
condeditor-set-by = 設定元:
condeditor-used-by = 使用先:
condeditor-filedeps = ファイル依存関係
condeditor-empty = このプロジェクトにはフラグや依存関係がありません。
condeditor-orphan-set = 設定されているが未使用
condeditor-orphan-used = 使用されているが未設定
msg-img-optimized = 画像を最適化しました。
msg-img-ok = 画像はすでに制限内です。
msg-img-none = 最適化する画像がありません。
msg-crash-recovery = 前回のセッションは予期せず終了しました。プロジェクトのバックアップを { $path } に保存しました
export-progress-title = 配布アーカイブを作成しています…
export-progress-files = { $done } / { $total } ファイル
msg-export-cancelled = エクスポートをキャンセルしました。不完全なアーカイブは削除されました。
verify-running = ディスク上のファイルを確認しています…
verify-stale = 注意：ファイルの確認中にプロジェクトが変更されました。検証を再実行してください。
prop-col-name = 名前
menu-save-as = 名前を付けて保存…
menu-project = プロジェクト
menu-tools = ツール
menu-manual = ユーザーマニュアル
msg-manual-missing = ユーザーマニュアル（PDF）がアプリケーションの横に見つかりませんでした。
toolbar-new = 新規
toolbar-open = 開く
toolbar-save = 保存
toolbar-validate = 検証
toolbar-preview = プレビュー
toolbar-export = エクスポート
dialog-choose-root = MODのルートフォルダーを選択
exit-unsaved-docs = 未保存: { $names }
status-summary = { $steps } ステップ · { $options } オプション
section-groups = グループ
section-options = オプション
section-flags = 条件フラグ
section-files = インストールするファイル
hint-group-type = このグループでユーザーがオプションを選択する方法。
hint-default-type = 依存パターンがどれも一致しない場合のオプションの提示方法：必須、任意、推奨、使用不可…
hint-operator = すべての条件を満たす（AND）か、いずれか1つ（OR）。
hint-flags = フラグはこのオプションが選択されたときに設定する名前付きの値です。他のステップやオプションはそれを判定して表示・非表示・必須にできます。
hint-plugin-dependencies = フラグやゲーム内のファイルに応じてオプションの種類を変えるパターン。例：別のMODが導入済みなら「必須」。
hint-files = このオプション選択時にゲームのDataフォルダーへコピーされるファイルとフォルダー。宛先はData基準。同じファイルを書く場合は優先度の高い方が勝ちます。
hint-visibility = このステップを表示するために満たすべき条件。常に表示するには空のままにします。
seltype-exactly-one = 1つだけ（必須）
seltype-at-most-one = 最大1つ
seltype-any = 任意の数
seltype-all = すべて（選択なし）
seltype-at-least-one = 少なくとも1つ
plugtype-required = 必須
plugtype-optional = 任意
plugtype-recommended = 推奨
plugtype-not-usable = 使用不可
plugtype-could-be-usable = 使用できる可能性あり
plugtype-required-hint = 常にインストールされ、ユーザーはチェックを外せません。
plugtype-optional-hint = 未チェックで提示され、ユーザーが決めます。
plugtype-recommended-hint = チェック済みで提示され、ユーザーは外せます。
plugtype-not-usable-hint = グレー表示され選択できません。
plugtype-could-be-usable-hint = 選択できますが、動作しない可能性があるとインストーラーが警告します。
op-and = すべての条件（AND）
op-or = いずれかの条件（OR）
theme-dark = ダーク
theme-light = ライト
theme-system = システムに従う
condeditor-setter-loc = ステップ { "{step}" } / グループ { "{group}" } / 「{ "{name}" }」
condeditor-pattern-of = 「{ "{name}" }」のパターン → { "{type}" }
condeditor-visibility-of = ステップ { "{step}" } の表示条件
condeditor-cond-set = 条件セット { "{num}" }
condeditor-needs = { "{ctx}" }（要求 = { "{value}" }）
condeditor-file-dep = { "{ctx}" }：ファイル「{ "{name}" }」（{ "{state}" }）
menu-translate-fomod = FOMODを翻訳…
ftr-title = FOMODを翻訳
ftr-open-folder = MODフォルダを開く…
ftr-from-active = アクティブなプロジェクトから
ftr-from-active-hint = メインウィンドウで開いているプロジェクトのFOMODを翻訳します（先に保存しておく必要があります）。
ftr-no-fomod = FOMODが読み込まれていません。
ftr-encoding = 元のファイルのエンコーディングです。翻訳後のファイルも同じエンコーディングで書き込まれます。
ftr-source-lang = 翻訳元
ftr-target-lang = 翻訳先
ftr-lang-locked = （FOMODの読み込み後は言語を変更できません）
ftr-translator = 翻訳者:
ftr-save = 翻訳を保存
ftr-export = 翻訳済みファイルをエクスポート
ftr-export-sibling = fomod_<言語> フォルダへ
ftr-export-sibling-hint = 翻訳済みの info.xml と ModuleConfig.xml を元の fomod フォルダの隣に書き込みます。元のファイルは変更されません。
ftr-export-inplace = 元のファイルに上書き
ftr-export-inplace-hint = それぞれのタイムスタンプ付き .bak コピーを作成してから、fomod/info.xml と fomod/ModuleConfig.xml を置き換えます。
ftr-force-explicit-order = 元の順序を維持
ftr-warn-order = 名前順に並べ替えられるリスト（order="Ascending"）は、MODマネージャーで翻訳後の名前によって並べ替え直されてしまいます。このオプションは order="Explicit" を強制し、オプションの現在の順序を維持します。
ftr-update = フォルダから更新
ftr-update-hint = FOMODをディスクから再度読み込み、翻訳とマージします。新規・変更・削除された文字列が報告されます。
ftr-preview-translated = 翻訳後のプレビュー
ftr-progress = { $done } / { $total } 翻訳済み
ftr-filter-all = すべて
ftr-filter-untranslated = 未翻訳
ftr-filter-review = 要確認
ftr-filter-issues = 問題あり
ftr-filter-locked = ロック済み
ftr-type-all = すべてのフィールド
ftr-type-names = 名前
ftr-type-descriptions = 説明
ftr-type-meta = MOD情報
ftr-search-hint = 原文、翻訳、コンテキストを検索…
ftr-next-untranslated = 次の未翻訳へ
ftr-show-whitespace = 空白と改行を表示
ftr-discard-question = 現在の翻訳には未保存の編集があります。破棄して別のFOMODを読み込みますか？
ftr-discard-yes = 破棄
ftr-unsaved-close = 翻訳に未保存の編集があります。
ftr-col-num = #
ftr-col-status = { "" }
ftr-col-context = コンテキスト
ftr-col-source = 原文
ftr-col-target = 翻訳
ftr-col-issues = { "" }
ftr-empty-hint = MODフォルダを開くか、アクティブなプロジェクトを読み込むと、翻訳可能な文字列が一覧表示されます。
ftr-empty-filter = 現在のフィルターに一致する文字列はありません。
ftr-select-row = 行を選択すると翻訳を編集できます。
ftr-copy-source = 原文をコピー
ftr-clear-target = クリア
ftr-lock = 翻訳しない
ftr-lock-hint = ロックされた文字列はそのまま書き込まれます（作成者、ウェブサイト、固有名詞など）。
ftr-note = メモ:
ftr-status-untranslated = 未翻訳
ftr-status-translated = 翻訳済み
ftr-status-auto = 自動入力されました — 確認してください
ftr-status-fuzzy = 翻訳後に原文が変更されました — 確認してください
ftr-status-obsolete = FOMODに存在しなくなりました
ftr-status-locked = ロック済み（そのまま書き込まれます）
ftr-field-info-name = MOD名 (info.xml)
ftr-field-module-name = インストーラのタイトル (ModuleConfig.xml)
ftr-field-author = 作成者
ftr-field-website = ウェブサイト
ftr-field-description = MODの説明
ftr-field-step = ステップ名
ftr-field-group = グループ名
ftr-field-plugin = オプション名
ftr-field-plugin-desc = オプションの説明
ftr-issue-empty = 翻訳が空です
ftr-issue-whitespace = 翻訳が空白のみです
ftr-issue-edge-whitespace = 先頭または末尾の空白が原文と異なります
ftr-issue-token = 保護されたトークンが異なります — 不足: { $missing } ; 余分: { $extra }
ftr-issue-newline-name = 名前に改行を含めることはできません
ftr-issue-control = XMLに保存できない文字が含まれています
ftr-issue-length = 原文と比べて長さが不自然です (×{ $ratio })
ftr-issue-identical = 原文と同一です
ftr-issue-duplicate = 同じ原文が { $key } では異なる翻訳になっています
ftr-issue-cdata = ここでは ]]> という並びは使用できません
ftr-load-error = FOMODを読み込めませんでした: { $error }
ftr-extracted = 翻訳可能な文字列が { $num } 件見つかりました。
ftr-sidecar-found = 既存の翻訳を読み込んでマージしました: 新規 { $new }、変更 { $changed }、削除 { $removed }。
ftr-saved = 翻訳を { $path } に保存しました
ftr-save-error = 翻訳を保存できませんでした: { $error }
ftr-save-first = 先にプロジェクトを保存してから翻訳してください。
ftr-export-success = 文字列 { $count } 件を { $path } に書き込みました
ftr-export-error = エクスポートに失敗しました: { $error }
ftr-export-blocked = エクスポートの前に、重大な問題 { $num } 件を修正する必要があります。
ftr-export-stale = FOMODが変更されたため、文字列 { $num } 件がスキップされました。「フォルダから更新」を使用してください。
ftr-update-report = 更新しました: 新規 { $new }、変更 { $changed }、移動 { $moved }、削除 { $removed }、変更なし { $unchanged }。
menu-edit = 編集
menu-undo = 元に戻す
menu-redo = やり直し
tree-title = プロジェクト
tree-mod-info = Mod情報
tree-steps = インストール手順
tree-required = 必須ファイル
tree-conditional = 条件付きインストール
tree-empty-steps = ステップはまだありません — + をクリックして追加してください。
tree-duplicate = 複製
tree-delete = 削除
tree-save-template = テンプレートとして保存…
tree-drop-hint = ここにドロップして移動
cond-set-label = 条件セット { $num }
inspector-empty = プロジェクトツリーで項目を選択するか、ステップを追加して始めてください。
count-options = オプション { $num } 件
count-files = ファイル { $num } 件
msg-deleted-undo = 削除しました。「元に戻す」(Ctrl+Z) で復元できます。
problems-title = 問題
problems-errors = エラー { $num } 件
problems-warnings = 警告 { $num } 件
btn-close = 閉じる
ftr-export-package = 翻訳パッケージとして (アーカイブ)
ftr-export-package-hint = アップロード可能な .zip または .7z を作成します。翻訳済みの info.xml と ModuleConfig.xml に README を加えたもの（パッチのみ）、または翻訳済みファイルを含むMOD全体（完全版）です。
ftr-package-full = MOD全体
ftr-package-full-hint = 翻訳済みの 2 つの XML ファイルだけでなく、MODのすべてのファイルをアーカイブに含めます。作者が再配布を許可していることを確認してください。
ftr-package-name-template = 名前:
ftr-readme-patch = このアーカイブには、「{ $name }」のインストーラの翻訳（{ $langname }）が含まれています (fomod/info.xml と fomod/ModuleConfig.xml)。元のMODに上書きインストールするか、MODマネージャーでマージして、翻訳済みファイルが元のファイルを置き換えるようにしてください。変更されるのはインストーラのテキストのみで、MOD本体のファイルは含まれていません。XIMOD Architect で作成。
ftr-readme-full = このアーカイブには、インストーラを翻訳（{ $langname }）した「{ $name }」が含まれています (fomod/info.xml と fomod/ModuleConfig.xml)。元のMODと同じようにインストールしてください。変更されているのはインストーラのテキストのみです。XIMOD Architect で作成。
ftr-apply-memory = メモリから入力
ftr-memory-size = 翻訳メモリ: この言語ペアのエントリ { $num } 件。保存した翻訳はすべて追加されます。
ftr-memory-applied = 翻訳メモリから文字列 { $num } 件を入力しました（「要確認」としてマーク）。
ftr-memory-suggestion = メモリの候補:
ftr-use-suggestion = 使用
ftr-propagate = 同一の原文に反映
ftr-propagate-hint = 原文が同じで未翻訳の他のすべての文字列に、この翻訳をコピーします。
ftr-propagated = 同一の文字列 { $num } 件を入力しました。
ftr-csv-export = CSV をエクスポート…
ftr-csv-import = CSV をインポート…
ftr-csv-imported = CSV ファイルから文字列 { $num } 件を更新しました。
ftr-csv-error = CSV エラー: { $error }
ftr-glossary = 用語集
ftr-glossary-source = 用語
ftr-glossary-target = 訳語
ftr-glossary-case = 大文字/小文字
ftr-glossary-dnt = 維持
ftr-glossary-add = 用語を追加
ftr-issue-glossary = 用語集: 「{ $term }」が指定どおりに翻訳されていません

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = アーカイブを開く…
filter-archive = Mod アーカイブ (zip, 7z)
msg-archive-opened = アーカイブを開きました（{ $num } 個のファイルを展開）: { $path }
msg-archive-reused = アーカイブは展開済みのため、{ $path } を再利用します
msg-archive-unsupported = アーカイブ形式「.{ $ext }」はサポートされていません。先に 7-Zip で展開してください（開けるのは .zip と .7z のみです）。
msg-archive-error = アーカイブを開く際にエラーが発生しました: { $error }
msg-archive-no-fomod = アーカイブ内に「fomod」フォルダーが見つかりません（{ $path }）
msg-archive-extracting = アーカイブを展開しています…
ftr-open-archive = Mod アーカイブを開く…
ftr-package-full-partial = この Mod は fomod フォルダーのみを含むアーカイブから開かれました。完全なパッケージには展開済みの Mod が必要です。
info-module-deps = Mod の必要条件
info-module-deps-hint = インストーラーの実行前に Mod 全体が必要とするファイルまたはフラグ (moduleDependencies)。不要な場合は空のままにしてください。
info-header-advanced = ヘッダーの詳細設定
info-title-position = タイトルの位置
info-title-colour = タイトルの色
info-title-colour-hint = 想定される形式: 16 進数 6 桁 (RRGGBB)
info-image-show = ヘッダー画像を表示
info-image-fade = ヘッダー画像をフェード
info-image-height = ヘッダー画像の高さ
info-attr-default = (既定)
file-always-install = 常に
file-always-install-hint = オプションが選択されていなくても、このファイルを常にインストールします (alwaysInstall)。
file-install-if-usable = 使用可能なら
file-install-if-usable-hint = オプションが選択されていなくても、使用可能であればこのファイルをインストールします (installIfUsable)。
msg-import-lossy = この FOMOD には XIMOD が編集できない構造が { $num } 件含まれています。プロジェクトの保存時に破棄されます。
fidelity-nested-deps = { $context } 内にネストされた依存関係グループがあります（サポートされるのは 1 階層のみ）
fidelity-game-dep = { $context } 内にゲームバージョンの要件 { $version } があります
fidelity-fomm-dep = { $context } 内に Mod マネージャーのバージョン要件 { $version } があります
fidelity-unknown = 「{ $parent }」内の要素「{ $element }」はサポートされていません（{ $context }）
loc-module = Mod の必要条件
loc-step = ステップ { $step }「{ $name }」
loc-installer = インストーラー

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = バックアップを復元…
backups-title = バックアップの復元
backups-empty = このプロジェクトにはまだバックアップがありません。以前のバージョンに上書き保存するたびにバックアップが作成されます。
backups-changes = 現在のプロジェクトとの差異: { $num } 件
btn-compare = 比較
btn-restore = 復元
btn-delete-backups = すべてのバックアップを削除
btn-delete-backups-confirm = もう一度クリックするとすべてのバックアップを削除します
msg-backup-restored = { $time } のバックアップをエディターに復元しました（未保存です。「元に戻す」で取り消せます）
msg-backups-deleted = { $num } 件のバックアップを削除しました
settings-backup-count = 保持するバックアップ数:
settings-backup-count-hint = 保存時に fomod/backups に保持する FOMOD XML の以前のバージョンの数 (0 = バックアップなし)。
settings-autosave-minutes = 復旧用コピーの自動保存間隔 (分):
settings-autosave-minutes-hint = この間隔で、変更されたすべてのプロジェクトの復旧用コピーが設定フォルダーに書き込まれます。次回起動時には、異常終了した場合にのみ提示されます (0 = 無効)。
settings-auto-masters = プラグインのマスターを条件として追加
settings-auto-masters-hint = プラグイン (.esp/.esm/.esl) をオプションに追加したとき、そのプラグインが必要とするマスターのうちゲームにもこの Mod にも含まれないものが、オプションの「Active」ファイル条件になります。
msg-author-from-plugin = プラグインのヘッダーから作者を入力しました: { $author }
msg-masters-added = { $plugin } のマスター { $num } 件をファイル条件として追加しました
issue-missing-master = { $plugin } は { $master } を必要としますが、この Mod に含まれておらず、依存関係としても宣言されていません
issue-esl-mismatch-flag = { $plugin } は拡張子が .esl ですが、light (ESL) フラグが設定されていません
issue-esl-eligible = { $plugin } は light としてフラグ付けできます（新規レコード { $num } 件、上限 { $limit }）
issue-esl-too-big = { $plugin } は light としてフラグ付けされていますが、light プラグインの規則を満たしていません（新規レコード { $num } 件、上限 { $limit }、または許容範囲外の FormID）
menu-plugin-report = プラグインレポート…
plugins-title = プラグインレポート
plugins-file = ファイル
plugins-kind = 種類
plugins-light = Light フラグ
plugins-masters = マスター
plugins-new-records = 新規レコード / 上限
plugins-eligible = Light 可
plugins-empty = このプロジェクトはプラグインファイル (.esp、.esm、.esl) をインストールしません。
plugins-unreadable = 読み取り不可

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = 最終的なファイルツリー
preview-total-size = インストール合計サイズ: { $size }
preview-tree-truncated = ツリーは省略されています: 展開するファイルが多すぎます（上記のサイズは一部のみです）。
preview-overwritten-by = { $plugin } によって上書き
preview-scenario = シナリオ:
preview-scenario-load = 読み込み
preview-scenario-save = 保存…
preview-scenario-delete = 削除
preview-scenario-name = シナリオ名
preview-scenario-saved = シナリオ「{ $name }」を fomod/scenarios に保存しました
preview-scenario-unresolved = シナリオの選択 { $num } 件がこのプロジェクトのどのオプションにも一致しません（名前変更または削除済み）
preview-scenario-none = (シナリオなし)
issue-unreachable-step = ステップ「{ $step }」は表示されることがありません: その表示条件が、それ以前のどのオプションも設定しないフラグ値を検査しています
issue-unreachable-option = オプション「{ $plugin }」は選択されることがありません: その使用可能タイプのパターンが、どのオプションも設定しないフラグ値を検査しています
issue-unreachable-cond = 条件付きファイルセット { $num } は適用されることがありません: その条件が、どのオプションも設定しないフラグ値を検査しています
size-option = インストールサイズ: { $size }（{ $num } ファイル）
size-missing = { $num } 件のソースが見つかりません
size-unknown = インストールサイズ: —（計測するには「検証」を実行してください）
menu-nexus-desc = Nexus 用説明文…
nexus-title = Nexus Mods 用説明文
nexus-format = 形式:
nexus-include-requirements = 必要条件
nexus-include-options = インストールオプション
nexus-include-install = インストール
nexus-include-changelog = 変更履歴
nexus-previous = 前のバージョン…
nexus-previous-none = (前のバージョンなし: 変更履歴は生成されません)
nexus-language = 言語:
nexus-language-source = (ソース)
nexus-sec-requirements = 必要条件
nexus-sec-options = インストールオプション
nexus-sec-install = インストール
nexus-sec-changelog = 変更履歴
nexus-install-text = この Mod には FOMOD インストーラが付属しています: Mod マネージャー（Vortex、Mod Organizer 2）でインストールし、インストーラでオプションを選択してください。
nexus-requires = 必要
nexus-step = ステップ
nexus-added = 追加
nexus-removed = 削除
nexus-changed = 変更
btn-copy = コピー
btn-save-as = 名前を付けて保存…
msg-copied = クリップボードにコピーしました
msg-saved-to = { $path } に保存しました

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = 名前の変更…
condeditor-rename-exists = 「{ $name }」という名前のフラグは既に存在します
condeditor-renamed = フラグ「{ $from }」を「{ $to }」に変更しました（{ $num } 件）
condeditor-delete-uses = すべての使用箇所を削除
condeditor-deleted-uses = フラグ「{ $name }」をすべての場所から削除しました（{ $num } 件）
condeditor-values-set = 設定される値:
condeditor-values-tested = 検査される値:
condeditor-value-never-set = { $value } — 検査されますが、どこにも設定されていません
condeditor-value-never-tested = { $value } — 設定されますが、どこでも検査されていません
condeditor-builder = 条件ビルダー
condeditor-builder-none = メインウィンドウでステップ、オプション、条件付きファイルセット、または Mod 情報を選択すると、ここでその条件を編集できます。
condeditor-builder-pattern = パターン:
condeditor-sentence-if = もし
condeditor-sentence-and = かつ
condeditor-sentence-or = または
condeditor-sentence-flag = フラグ { "{name}" } = { "{value}" }
condeditor-sentence-file = ファイル { "{name}" } が { "{value}" }
condeditor-sentence-empty = (条件なし: 常に真)
condeditor-sentence-then-visible = ならば ステップが表示されます
condeditor-sentence-then-type = ならば オプションは { $type } になります
condeditor-sentence-then-install = ならば ファイルがインストールされます
condeditor-sentence-then-module = ならば インストーラを実行できます（開始前に確認されます）
issue-flag-value-never-set = フラグ「{ $flag }」は値「{ $value }」で検査されますが、どのオプションもこの値を設定しません
issue-flag-never-used = フラグ「{ $flag }」は設定されますが、どこでも検査されていません
menu-project-strings = プロジェクトの文字列…
strings-title = プロジェクトの文字列
strings-search = テキスト、場所、またはキーを検索…
strings-kind-all = すべて
strings-kind-names = 名前
strings-kind-descriptions = 説明
strings-duplicates-only = 重複のみ
strings-replace-with = 置換後:
strings-case = 大文字と小文字を区別
strings-whole-word = 単語全体
strings-replace-current = 置換
strings-replace-all = すべて置換
strings-replaced = { $num } 件の文字列を置換しました
strings-dup-badge = ×{ $num }
strings-dup-hover = 同じテキスト:
strings-count = { $num } 件の文字列 · 重複グループ { $dups } 件
strings-col-location = 場所
strings-col-field = フィールド
strings-col-text = テキスト

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = アーカイブの内容…
filter-bethesda-archive = Bethesda アーカイブ (bsa, ba2)
archive-view-title = アーカイブの内容
archive-view-format = 形式:
archive-view-entries = { $num } 件のエントリ
archive-view-size = 展開後 { $size }
archive-view-search = パスを検索…
archive-view-col-path = パス
archive-view-col-size = サイズ
archive-view-col-compressed = 圧縮
archive-view-truncated = 一致するエントリのうち先頭の { $num } 件のみ表示しています — 検索条件を絞り込んでください。
archive-view-error = このアーカイブを読み取れません: { $error }
archive-view-hint = このアーカイブの内容を表示
issue-conflict-archive = 複数のアーカイブに同じアセット: 「{ $path }」は { $count } 件の参照（{ $locs }）によって格納されています — どれが使われるかはゲームのアーカイブ読み込み順で決まります。
issue-conflict-archive-loose = アーカイブとルーズファイル: 「{ $path }」はアーカイブに格納されると同時にルーズファイルとしてもインストールされます（{ $locs }）— ルーズファイルがアーカイブ内のものより優先されます。
preview-in-archive = (アーカイブ内)
preview-archived-size = うち { $size } はアーカイブに格納

# --- Project tree: expand / collapse menus
tree-expand = 展開
tree-collapse = 折りたたみ
tree-expand-all = すべて展開
tree-expand-selected = 選択項目を展開
tree-expand-from = 選択項目以下を展開
tree-collapse-all = すべて折りたたむ
tree-collapse-selected = 選択項目を折りたたむ
tree-collapse-from = 選択項目以下を折りたたむ
tree-expand-all-hint = すべての見出しを展開します
tree-expand-selected-hint = 選択した見出しのみを展開します
tree-expand-from-hint = 選択した見出しとその下のすべてを展開します
tree-collapse-all-hint = すべての見出しを折りたたみます
tree-collapse-selected-hint = 選択した見出しのみを折りたたみます
tree-collapse-from-hint = 選択した見出しとその下のすべてを折りたたみます

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = グループを追加
btn-remove-group-cond = グループを削除
dep-type-game = ゲームバージョン
dep-type-fomm = Mod マネージャーのバージョン
dep-group-hint = 「かつ」／「または」で結合した条件のグループです。グループは入れ子にできます。
condeditor-sentence-game = ゲームバージョン ≥ { "{value}" }
condeditor-sentence-fomm = Mod マネージャーのバージョン ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = 一意のテキスト
ftr-uniques-hint = 異なる原文ごとに 1 行だけ表示します。その行を翻訳すると、同じ原文を持つすべての文字列が一度に翻訳されます。
ftr-uniques-synced = 同一の文字列 { $num } 件を更新しました。
ftr-uniques-group = { $num } 件の文字列がこの原文を共有しています。その翻訳はすべてに適用されます。
